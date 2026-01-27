use anyhow::Context;
use std::env;
use std::path::{Path, PathBuf};

use super::models::Migrations;

/// Information about the current state of the package manager.
#[derive(Debug, Clone)]
pub struct StateInfo {
    /// Path to the current executable
    executable: PathBuf,
    /// Path to the data storage directory
    data_dir: PathBuf,
    /// Path to the image layers directory
    layers_dir: PathBuf,
    /// Size of the layers directory in bytes
    layers_size: u64,
    /// Path to the metadata database file
    metadata_file: PathBuf,
    /// Size of the metadata file in bytes
    metadata_size: u64,
    /// Current migration version
    migration_current: u32,
    /// Total number of migrations available
    migration_total: u32,
}

impl StateInfo {
    /// Create a new StateInfo instance.
    pub fn new(
        migration_info: Migrations,
        layers_size: u64,
        metadata_size: u64,
    ) -> anyhow::Result<Self> {
        let data_dir = dirs::data_local_dir()
            .context("No local data dir known for the current OS")?
            .join("wasm");
        Ok(Self::new_at(
            data_dir,
            migration_info,
            layers_size,
            metadata_size,
        ))
    }

    /// Create a new StateInfo instance at a specific data directory.
    #[must_use]
    pub fn new_at(
        data_dir: PathBuf,
        migration_info: Migrations,
        layers_size: u64,
        metadata_size: u64,
    ) -> Self {
        Self {
            executable: env::current_exe().unwrap_or_else(|_| PathBuf::from("unknown")),
            layers_dir: data_dir.join("layers"),
            layers_size,
            metadata_file: data_dir.join("metadata.db3"),
            metadata_size,
            data_dir,
            migration_current: migration_info.current,
            migration_total: migration_info.total,
        }
    }

    /// Get the path to the current executable
    #[must_use]
    pub fn executable(&self) -> &Path {
        &self.executable
    }

    /// Get the location of the crate's data dir
    #[must_use]
    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    /// Get the location of the crate's cache dir
    #[must_use]
    pub fn layers_dir(&self) -> &Path {
        &self.layers_dir
    }

    /// Get the size of the layers directory in bytes
    #[must_use]
    pub fn layers_size(&self) -> u64 {
        self.layers_size
    }

    /// Get the location of the crate's metadata file
    #[must_use]
    pub fn metadata_file(&self) -> &Path {
        &self.metadata_file
    }

    /// Get the size of the metadata file in bytes
    #[must_use]
    pub fn metadata_size(&self) -> u64 {
        self.metadata_size
    }

    /// Get the current migration version
    #[must_use]
    pub fn migration_current(&self) -> u32 {
        self.migration_current
    }

    /// Get the total number of migrations available
    #[must_use]
    pub fn migration_total(&self) -> u32 {
        self.migration_total
    }
}
