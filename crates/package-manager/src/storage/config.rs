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

    /// Creates a new StateInfo for testing purposes.
    #[cfg(any(test, feature = "test-helpers"))]
    #[must_use]
    pub fn new_for_testing() -> Self {
        Self {
            executable: PathBuf::from("/usr/local/bin/wasm"),
            data_dir: PathBuf::from("/home/user/.local/share/wasm"),
            layers_dir: PathBuf::from("/home/runner/.local/share/wasm/layers"),
            layers_size: 1024 * 1024 * 10, // 10 MB
            metadata_file: PathBuf::from("/home/user/.local/share/wasm/metadata.db3"),
            metadata_size: 1024 * 64, // 64 KB
            migration_current: 3,
            migration_total: 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_migrations() -> Migrations {
        Migrations {
            current: 3,
            total: 5,
        }
    }

    #[test]
    fn test_state_info_new_at() {
        let data_dir = PathBuf::from("/test/data");
        let state_info = StateInfo::new_at(data_dir.clone(), test_migrations(), 1024, 512);

        assert_eq!(state_info.data_dir(), data_dir);
        assert_eq!(state_info.layers_dir(), data_dir.join("layers"));
        assert_eq!(state_info.metadata_file(), data_dir.join("metadata.db3"));
        assert_eq!(state_info.layers_size(), 1024);
        assert_eq!(state_info.metadata_size(), 512);
        assert_eq!(state_info.migration_current(), 3);
        assert_eq!(state_info.migration_total(), 5);
    }

    #[test]
    fn test_state_info_executable() {
        let data_dir = PathBuf::from("/test/data");
        let state_info = StateInfo::new_at(data_dir, test_migrations(), 0, 0);

        // executable() should return something (either the actual exe or "unknown")
        let exe = state_info.executable();
        assert!(!exe.as_os_str().is_empty());
    }

    #[test]
    fn test_state_info_sizes() {
        let data_dir = PathBuf::from("/test/data");

        // Test with various sizes
        let state_info = StateInfo::new_at(data_dir.clone(), test_migrations(), 0, 0);
        assert_eq!(state_info.layers_size(), 0);
        assert_eq!(state_info.metadata_size(), 0);

        let state_info = StateInfo::new_at(data_dir.clone(), test_migrations(), 1024 * 1024, 1024);
        assert_eq!(state_info.layers_size(), 1024 * 1024);
        assert_eq!(state_info.metadata_size(), 1024);
    }
}
