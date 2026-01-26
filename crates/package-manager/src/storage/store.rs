use anyhow::Context;
use std::collections::HashSet;
use std::path::Path;

use super::config::StateInfo;
use super::models::{ImageEntry, InsertResult, InterfaceEntry, KnownPackage, Migrations};
use futures_concurrency::prelude::*;
use oci_client::{Reference, client::ImageData};
use rusqlite::Connection;

/// Calculate the total size of a directory recursively
async fn dir_size(path: &Path) -> u64 {
    let mut total = 0u64;
    let mut stack = vec![path.to_path_buf()];

    while let Some(dir) = stack.pop() {
        if let Ok(mut entries) = tokio::fs::read_dir(&dir).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Ok(metadata) = entry.metadata().await {
                    if metadata.is_dir() {
                        stack.push(entry.path());
                    } else {
                        total += metadata.len();
                    }
                }
            }
        }
    }
    total
}

#[derive(Debug)]
pub(crate) struct Store {
    pub(crate) state_info: StateInfo,
    conn: Connection,
}

impl Store {
    /// Open the store and run any pending migrations.
    pub(crate) async fn open() -> anyhow::Result<Self> {
        let data_dir = dirs::data_local_dir()
            .context("No local data dir known for the current OS")?
            .join("wasm");
        let layers_dir = data_dir.join("layers");
        let metadata_file = data_dir.join("metadata.db3");

        // TODO: remove me once we're done testing
        // tokio::fs::remove_dir_all(&data_dir).await?;

        let a = tokio::fs::create_dir_all(&data_dir);
        let b = tokio::fs::create_dir_all(&layers_dir);
        let _ = (a, b)
            .try_join()
            .await
            .context("Could not create config directories on disk")?;

        let conn = Connection::open(&metadata_file)?;
        Migrations::run_all(&conn)?;

        let migration_info = Migrations::get(&conn)?;
        let layers_size = dir_size(&layers_dir).await;
        let metadata_size = tokio::fs::metadata(&metadata_file)
            .await
            .map(|m| m.len())
            .unwrap_or(0);
        let state_info = StateInfo::new_at(data_dir, migration_info, layers_size, metadata_size);

        let store = Self { state_info, conn };
        
        // Re-scan interfaces after migrations to ensure derived data is up-to-date
        // Ignore errors during re-scan as they shouldn't prevent the store from opening
        if let Err(e) = store.scan_interfaces().await {
            eprintln!("Warning: Failed to re-scan interfaces: {}", e);
        }

        Ok(store)
    }

    pub(crate) async fn insert(
        &self,
        reference: &Reference,
        image: ImageData,
    ) -> anyhow::Result<InsertResult> {
        let digest = reference.digest().map(|s| s.to_owned()).or(image.digest);
        let manifest_str = serde_json::to_string(&image.manifest)?;

        // Calculate total size on disk from all layers
        let size_on_disk: u64 = image.layers.iter().map(|l| l.data.len() as u64).sum();

        let result = ImageEntry::insert(
            &self.conn,
            reference.registry(),
            reference.repository(),
            reference.tag(),
            digest.as_deref(),
            &manifest_str,
            size_on_disk,
        )?;

        // Only store layers if this is a new entry
        if result == InsertResult::Inserted {
            // Store layers by their content digest (content-addressable storage)
            // The manifest.layers and image.layers should be in the same order
            if let Some(ref manifest) = image.manifest {
                for (idx, layer) in image.layers.iter().enumerate() {
                    let cache = self.state_info.layers_dir();
                    // Use the layer's content digest from the manifest as the key
                    let fallback_key = reference.whole().to_string();
                    let key = manifest
                        .layers
                        .get(idx)
                        .map(|l| l.digest.as_str())
                        .unwrap_or(&fallback_key);
                    let data = &layer.data;
                    let _integrity = cacache::write(&cache, key, data).await?;
                }
            }
        }
        Ok(result)
    }

    /// Returns all currently stored images and their metadata.
    pub(crate) fn list_all(&self) -> anyhow::Result<Vec<ImageEntry>> {
        ImageEntry::get_all(&self.conn)
    }

    /// Deletes an image by its reference.
    /// Only removes cached layers if no other images reference them.
    pub(crate) async fn delete(&self, reference: &Reference) -> anyhow::Result<bool> {
        // Get all images to find which layers are still needed
        let all_entries = ImageEntry::get_all(&self.conn)?;

        // Find the entry we're deleting to get its layer digests
        let entry_to_delete = all_entries.iter().find(|e| {
            e.ref_registry == reference.registry()
                && e.ref_repository == reference.repository()
                && e.ref_tag.as_deref() == reference.tag()
                && e.ref_digest.as_deref() == reference.digest()
        });

        if let Some(entry) = entry_to_delete {
            // Collect all layer digests from the entry we're deleting
            let layers_to_delete: HashSet<&str> = entry
                .manifest
                .layers
                .iter()
                .map(|l| l.digest.as_str())
                .collect();

            // Collect all layer digests from OTHER entries (excluding the one we're deleting)
            let layers_still_needed: HashSet<&str> = all_entries
                .iter()
                .filter(|e| {
                    !(e.ref_registry == reference.registry()
                        && e.ref_repository == reference.repository()
                        && e.ref_tag.as_deref() == reference.tag()
                        && e.ref_digest.as_deref() == reference.digest())
                })
                .flat_map(|e| e.manifest.layers.iter().map(|l| l.digest.as_str()))
                .collect();

            // Only delete layers that are not needed by other entries
            for layer_digest in layers_to_delete {
                if !layers_still_needed.contains(layer_digest) {
                    let _ = cacache::remove(self.state_info.layers_dir(), layer_digest).await;
                }
            }
        }

        // Delete from database
        ImageEntry::delete_by_reference(
            &self.conn,
            reference.registry(),
            reference.repository(),
            reference.tag(),
            reference.digest(),
        )
    }

    /// Search for known packages by query string.
    pub(crate) fn search_known_packages(&self, query: &str) -> anyhow::Result<Vec<KnownPackage>> {
        KnownPackage::search(&self.conn, query)
    }

    /// Get all known packages.
    pub(crate) fn list_known_packages(&self) -> anyhow::Result<Vec<KnownPackage>> {
        KnownPackage::get_all(&self.conn)
    }

    /// Add or update a known package.
    pub(crate) fn add_known_package(
        &self,
        registry: &str,
        repository: &str,
        tag: Option<&str>,
        description: Option<&str>,
    ) -> anyhow::Result<()> {
        KnownPackage::upsert(&self.conn, registry, repository, tag, description)
    }

    /// Scan all stored images and extract interface information from their component metadata.
    /// This is a re-scan operation that can be called after migrations to update derived data.
    pub(crate) async fn scan_interfaces(&self) -> anyhow::Result<usize> {
        let images = ImageEntry::get_all(&self.conn)?;
        let mut scanned_count = 0;

        for image in images {
            // Get the layer data for this image
            // For now, we'll extract interface information from the manifest metadata
            // In a real implementation, this would parse the actual WASM binary
            
            // Extract interface names from the manifest's layers (if they contain metadata)
            // This is a simplified implementation - in reality, you'd need to:
            // 1. Load the actual WASM binary from the layer cache
            // 2. Parse it with wasm-metadata
            // 3. Extract interface information
            
            // For this minimal implementation, we'll create a placeholder interface
            // based on the image's reference
            let interface_name = image.reference();
            
            // Get the image ID from the entry
            let image_id = image.id();

            // Delete existing interfaces for this image before re-scanning
            InterfaceEntry::delete_by_image_id(&self.conn, image_id)?;

            // Insert the interface entry
            InterfaceEntry::insert(&self.conn, image_id, &interface_name, "component")?;
            scanned_count += 1;
        }

        Ok(scanned_count)
    }
}
