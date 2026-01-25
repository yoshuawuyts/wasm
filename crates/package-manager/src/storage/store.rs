use super::config::Config;
use super::models::ImageEntry;
use anyhow::Context;
use futures_concurrency::prelude::*;
use oci_client::{Reference, client::ImageData};
use rusqlite::Connection;

#[derive(Debug)]
pub(crate) struct Store {
    pub(crate) config: Config,
    conn: Connection,
}

impl Store {
    pub(crate) async fn open() -> anyhow::Result<Self> {
        let config = Config::new()?;

        // TODO: remove me once we're done testing
        // tokio::fs::remove_dir_all(config.data_dir()).await?;

        let a = tokio::fs::create_dir_all(config.data_dir());
        let b = tokio::fs::create_dir_all(config.layers_dir());
        let _ = (a, b)
            .try_join()
            .await
            .context("Could not create config directories on disk")?;

        let conn = Connection::open(config.metadata_file())?;
        conn.execute_batch(include_str!("./migrations/01_init.sql"))?;

        Ok(Self { config, conn })
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

        for layer in &image.layers {
            let cache = self.config.layers_dir();
            let key = reference.whole().to_string();
            let data = &layer.data;
            let _integrity = cacache::write(&cache, &key, data).await?;
        }
        Ok(())
    }

    /// Returns all currently stored images and their metadata.
    pub(crate) fn list_all(&self) -> anyhow::Result<Vec<ImageEntry>> {
        ImageEntry::get_all(&self.conn)
    }
}
