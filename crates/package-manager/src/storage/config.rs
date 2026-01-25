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
    /// Path to the metadata database file
    metadata_file: PathBuf,
    /// Current migration version
    migration_current: u32,
    /// Total number of migrations available
    migration_total: u32,
}

impl StateInfo {
    pub fn new(migration_info: Migrations) -> anyhow::Result<Self> {
        let data_dir = dirs::data_local_dir()
            .context("No local data dir known for the current OS")?
            .join("wasm");
        Ok(Self::new_at(data_dir, migration_info))
    }

    pub fn new_at(data_dir: PathBuf, migration_info: Migrations) -> Self {
        Self {
            executable: env::current_exe().unwrap_or_else(|_| PathBuf::from("unknown")),
            layers_dir: data_dir.join("layers"),
            metadata_file: data_dir.join("metadata.db3"),
            data_dir,
            migration_current: migration_info.current,
            migration_total: migration_info.total,
        }
    }

    /// Get the path to the current executable
    pub fn executable(&self) -> &Path {
        &self.executable
    }

    /// Get the location of the crate's data dir
    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    /// Get the location of the crate's cache dir
    pub fn layers_dir(&self) -> &Path {
        &self.layers_dir
    }

    /// Get the location of the crate's metadata file
    pub fn metadata_file(&self) -> &Path {
        &self.metadata_file
    }

    /// Get the current migration version
    pub fn migration_current(&self) -> u32 {
        self.migration_current
    }

    /// Get the total number of migrations available
    pub fn migration_total(&self) -> u32 {
        self.migration_total
    }
}
