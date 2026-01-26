use anyhow::Context;
use std::collections::HashSet;
use std::path::Path;

use super::config::StateInfo;
use super::models::{ImageEntry, InsertResult, KnownPackage, Migrations, WitInterface};
use futures_concurrency::prelude::*;
use oci_client::{Reference, client::ImageData};
use rusqlite::Connection;
use wit_parser::decoding::{decode, DecodedWasm};

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
    ) -> anyhow::Result<InsertResult> {
        let digest = reference.digest().map(|s| s.to_owned()).or(image.digest);
        let manifest_str = serde_json::to_string(&image.manifest)?;

        // Calculate total size on disk from all layers
        let size_on_disk: u64 = image.layers.iter().map(|l| l.data.len() as u64).sum();

        let (result, image_id) = ImageEntry::insert(
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
                    
                    // Try to extract WIT interface from this layer
                    if let Some(image_id) = image_id {
                        self.try_extract_wit_interface(image_id, data);
                    }
                }
            }
        }
        Ok(result)
    }
    
    /// Attempt to extract WIT interface from wasm component bytes.
    /// This is best-effort - if extraction fails, we silently skip.
    fn try_extract_wit_interface(&self, image_id: i64, wasm_bytes: &[u8]) {
        // Try to decode the wasm bytes as a component
        let decoded = match decode(wasm_bytes) {
            Ok(d) => d,
            Err(_) => return, // Not a valid wasm component, skip
        };
        
        // Extract metadata based on decoded type
        let (world_name, import_count, export_count) = match &decoded {
            DecodedWasm::WitPackage(resolve, package_id) => {
                let package = &resolve.packages[*package_id];
                // Use the first world name if available
                let world = package.worlds.iter().next().map(|(name, world_id)| {
                    let w = &resolve.worlds[*world_id];
                    (name.clone(), w.imports.len() as i32, w.exports.len() as i32)
                });
                world.unwrap_or((package.name.name.clone(), 0, 0))
            }
            DecodedWasm::Component(resolve, world_id) => {
                let world = &resolve.worlds[*world_id];
                (
                    world.name.clone(),
                    world.imports.len() as i32,
                    world.exports.len() as i32,
                )
            }
        };
        
        // Generate a WIT text representation from the decoded structure
        let wit_text = Self::generate_wit_text(&decoded);
        
        // Insert the WIT interface
        let wit_id = match WitInterface::insert(
            &self.conn,
            &wit_text,
            Some(&world_name),
            import_count,
            export_count,
        ) {
            Ok(id) => id,
            Err(_) => return, // Failed to insert, skip
        };
        
        // Link to image
        let _ = WitInterface::link_to_image(&self.conn, image_id, wit_id);
    }
    
    /// Generate WIT text representation from decoded component.
    fn generate_wit_text(decoded: &DecodedWasm) -> String {
        let resolve = decoded.resolve();
        let mut output = String::new();
        
        match decoded {
            DecodedWasm::WitPackage(_, package_id) => {
                let package = &resolve.packages[*package_id];
                output.push_str(&format!("package {};\n\n", package.name));
                
                // Print interfaces
                for (name, interface_id) in &package.interfaces {
                    output.push_str(&format!("interface {} {{\n", name));
                    let interface = &resolve.interfaces[*interface_id];
                    
                    // Print types
                    for (type_name, type_id) in &interface.types {
                        let type_def = &resolve.types[*type_id];
                        output.push_str(&format!("  type {}: {:?};\n", type_name, type_def.kind.as_str()));
                    }
                    
                    // Print functions
                    for (func_name, func) in &interface.functions {
                        let params: Vec<String> = func.params.iter()
                            .map(|(name, _ty)| name.clone())
                            .collect();
                        let has_result = func.result.is_some();
                        output.push_str(&format!(
                            "  func {}({}){};\n",
                            func_name,
                            params.join(", "),
                            if has_result { " -> ..." } else { "" }
                        ));
                    }
                    output.push_str("}\n\n");
                }
                
                // Print worlds
                for (name, world_id) in &package.worlds {
                    let world = &resolve.worlds[*world_id];
                    output.push_str(&format!("world {} {{\n", name));
                    
                    for (key, _item) in &world.imports {
                        output.push_str(&format!("  import {};\n", Self::world_key_to_string(key)));
                    }
                    for (key, _item) in &world.exports {
                        output.push_str(&format!("  export {};\n", Self::world_key_to_string(key)));
                    }
                    output.push_str("}\n\n");
                }
            }
            DecodedWasm::Component(_, world_id) => {
                let world = &resolve.worlds[*world_id];
                output.push_str(&format!("// Inferred component interface\n"));
                output.push_str(&format!("world {} {{\n", world.name));
                
                for (key, _item) in &world.imports {
                    output.push_str(&format!("  import {};\n", Self::world_key_to_string(key)));
                }
                for (key, _item) in &world.exports {
                    output.push_str(&format!("  export {};\n", Self::world_key_to_string(key)));
                }
                output.push_str("}\n");
            }
        }
        
        output
    }
    
    /// Convert a WorldKey to a string representation.
    fn world_key_to_string(key: &wit_parser::WorldKey) -> String {
        match key {
            wit_parser::WorldKey::Name(name) => name.clone(),
            wit_parser::WorldKey::Interface(id) => format!("interface-{:?}", id),
        }
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
    
    /// Get all WIT interfaces.
    pub(crate) fn list_wit_interfaces(&self) -> anyhow::Result<Vec<WitInterface>> {
        WitInterface::get_all(&self.conn)
    }
    
    /// Get all WIT interfaces with their associated component references.
    pub(crate) fn list_wit_interfaces_with_components(&self) -> anyhow::Result<Vec<(WitInterface, String)>> {
        WitInterface::get_all_with_images(&self.conn)
    }
}
