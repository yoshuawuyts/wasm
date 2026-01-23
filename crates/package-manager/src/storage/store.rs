use super::config::Config;
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
        tokio::fs::remove_dir_all(config.data_dir()).await?;

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
        self.conn.execute(
            "INSERT INTO image (ref_registry, ref_repository, ref_tag, ref_digest, manifest) VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                reference.registry(),
                reference.repository(),
                reference.tag(),
                reference.digest(),
                &serde_json::to_string(&image.manifest)?,
            ),
        )?;

        for layer in &image.layers {
            let cache = self.config.layers_dir();
            let key = reference.whole().to_string();
            let data = &layer.data;
            let _integrity = cacache::write(&cache, &key, data).await?;
        }
        Ok(())
    }
}
