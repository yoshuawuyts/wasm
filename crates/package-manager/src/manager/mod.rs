use anyhow::Context;
use futures_concurrency::prelude::*;
use oci_client::Reference;
use oci_client::client::{ClientConfig, ClientProtocol, ImageData};
use oci_wasm::WasmClient;

mod config;
use crate::network::Client;
pub use config::Config;

/// A cache on disk
pub struct Manager {
    config: Config,
    client: Client,
}

impl std::fmt::Debug for Manager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("config", &self.config)
            .finish()
    }
}

impl Manager {
    /// Create a new store at a location on disk.
    ///
    /// This may return an error if it fails to create the cache location on disk.
    pub async fn open() -> anyhow::Result<Self> {
        let config = Config::new()?;
        Self::with_config(config).await
    }

    /// Create a new store at a location on disk.
    ///
    /// This may return an error if it fails to create the cache location on disk.
    pub async fn with_config(config: Config) -> anyhow::Result<Self> {
        let a = tokio::fs::create_dir_all(config.data_dir());
        let b = tokio::fs::create_dir_all(config.metadata_dir());
        let c = tokio::fs::create_dir_all(config.layers_dir());
        let _ = (a, b, c)
            .try_join()
            .await
            .context("Could not create config directories on disk")?;

        let client = Client::new();

        Ok(Self { config, client })
    }

    pub async fn pull(&self, reference: Reference) -> anyhow::Result<cacache::Integrity> {
        let image = self.client.pull(&reference).await?;
        let integrity = self.insert(reference, image).await?;
        Ok(integrity)
    }

    /// Insert data into the store
    async fn insert(
        &self,
        reference: Reference,
        image: ImageData,
    ) -> cacache::Result<cacache::Integrity> {
        dbg!(image.config.media_type);
        dbg!(image.manifest);
        for layer in image.layers {
            match dbg!(layer.media_type.as_str()) {
                "application/wasm" => {
                    let mut key = reference.whole().to_string();
                    key.push_str("+wasm");
                    let data = &layer.data;
                    let integrity = cacache::write(&self.config.layers_dir(), &key, data).await?;
                    return Ok(integrity);
                }
                _ => {}
            }
        }
        todo!()
    }

    /// Get data from the store
    pub async fn get(&self, key: &str) -> cacache::Result<Vec<u8>> {
        cacache::read(&self.config.layers_dir(), key).await
    }

    /// Access the config
    pub fn config(&self) -> &Config {
        &self.config
    }
}
