use oci_client::Reference;

use crate::network::Client;
use crate::storage::{ImageEntry, StateInfo, Store};

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
        cacache::read(self.store.state_info.layers_dir(), key).await
    }

    /// Get information about the current state of the package manager.
    pub fn state_info(&self) -> StateInfo {
        self.store.state_info.clone()
    }

    /// Delete an image from the store by its reference.
    pub async fn delete(&self, reference: Reference) -> anyhow::Result<bool> {
        self.store.delete(&reference).await
    }
}
