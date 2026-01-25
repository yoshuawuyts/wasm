use std::env;
use std::path::PathBuf;

use oci_client::Reference;

use crate::network::Client;
use crate::storage::{ImageEntry, Store};

/// Information about the current state of the package manager.
#[derive(Debug, Clone)]
pub struct StateInfo {
    /// Path to the current executable
    pub executable: PathBuf,
    /// Path to the data storage directory
    pub data_dir: PathBuf,
    /// Path to the image layers directory
    pub layers_dir: PathBuf,
    /// Path to the metadata database file
    pub metadata_file: PathBuf,
    /// Current migration version
    pub migration_current: u32,
    /// Total number of migrations available
    pub migration_total: u32,
}

/// A cache on disk
#[derive(Debug)]
pub struct Manager {
    client: Client,
    store: Store,
}

impl Manager {
    /// Create a new store at a location on disk.
    ///
    /// This may return an error if it fails to create the cache location on disk.
    pub async fn open() -> anyhow::Result<Self> {
        let client = Client::new();
        let store = Store::open().await?;

        Ok(Self { client, store })
    }

    // /// Create a new store at a location on disk.
    // ///
    // /// This may return an error if it fails to create the cache location on disk.
    // pub async fn with_config(config: Config) -> anyhow::Result<Self> {
    //     let client = Client::new();
    //     let store = Store::open().await?;

    //     Ok(Self { client, store })
    // }

    pub async fn pull(&self, reference: Reference) -> anyhow::Result<()> {
        let image = self.client.pull(&reference).await?;
        self.store.insert(&reference, image).await?;
        Ok(())
    }

    /// List all stored images and their metadata.
    pub fn list_all(&self) -> anyhow::Result<Vec<ImageEntry>> {
        self.store.list_all()
    }

    /// Get data from the store
    pub async fn get(&self, key: &str) -> cacache::Result<Vec<u8>> {
        cacache::read(&self.store.config.layers_dir(), key).await
    }

    /// Access the config
    pub fn config(&self) -> &crate::storage::Config {
        &self.store.config
    }

    /// Get information about the current state of the package manager.
    pub fn state_info(&self) -> anyhow::Result<StateInfo> {
        let migration_info = self.store.migration_info()?;
        Ok(StateInfo {
            executable: env::current_exe().unwrap_or_else(|_| PathBuf::from("unknown")),
            data_dir: self.store.config.data_dir().to_path_buf(),
            layers_dir: self.store.config.layers_dir().to_path_buf(),
            metadata_file: self.store.config.metadata_file().to_path_buf(),
            migration_current: migration_info.current,
            migration_total: migration_info.total,
        })
    }
}
