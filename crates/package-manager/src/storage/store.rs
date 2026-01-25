use anyhow::Context;

use super::config::StateInfo;
use super::models::{ImageEntry, Migrations};
use futures_concurrency::prelude::*;
use oci_client::{Reference, client::ImageData};
use rusqlite::Connection;

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
        let state_info = StateInfo::new_at(data_dir, migration_info);

        Ok(Self { state_info, conn })
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
            let cache = self.state_info.layers_dir();
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
