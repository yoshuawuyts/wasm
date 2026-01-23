use oci_client::Reference;

use crate::network::Client;
use crate::storage::Store;

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

    /// Get data from the store
    pub async fn get(&self, key: &str) -> cacache::Result<Vec<u8>> {
        cacache::read(&self.store.config.layers_dir(), key).await
    }

    /// Access the config
    pub fn config(&self) -> &crate::storage::Config {
        &self.store.config
    }
}
