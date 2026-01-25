use anyhow::Context;
use std::collections::HashSet;
use std::path::Path;

use super::config::StateInfo;
use super::models::{ImageEntry, KnownPackage, Migrations};
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

        Ok(Self { state_info, conn })
    }

    pub(crate) async fn insert(
        &self,
        reference: &Reference,
        image: ImageData,
    ) -> anyhow::Result<()> {
        let digest = reference.digest().map(|s| s.to_owned()).or(image.digest);
        let manifest = serde_json::to_string(&image.manifest)?;
        ImageEntry::insert(
            &self.conn,
            reference.registry(),
            reference.repository(),
            reference.tag(),
            digest.as_deref(),
            &manifest,
        )?;

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
        Ok(())
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
}
