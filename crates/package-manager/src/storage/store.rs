use super::config::Config;
use super::models::ImageEntry;
use anyhow::Context;
use futures_concurrency::prelude::*;
use oci_client::{Reference, client::ImageData};
use rusqlite::Connection;

/// A migration that can be applied to the database.
struct Migration {
    version: u32,
    name: &'static str,
    sql: &'static str,
}

/// All migrations in order. Each migration is run exactly once.
const MIGRATIONS: &[Migration] = &[Migration {
    version: 1,
    name: "init",
    sql: include_str!("./migrations/01_init.sql"),
}];

#[derive(Debug)]
pub(crate) struct Store {
    pub(crate) config: Config,
    conn: Connection,
}

impl Store {
    /// Open the store and run any pending migrations.
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
        Self::run_migrations(&conn)?;

        Ok(Self { config, conn })
    }

    /// Initialize the migrations table and run all pending migrations.
    fn run_migrations(conn: &Connection) -> anyhow::Result<()> {
        // Create the migrations table if it doesn't exist
        conn.execute_batch(include_str!("./migrations/00_migrations.sql"))?;

        // Get the current migration version
        let current_version: u32 = conn
            .query_row(
                "SELECT COALESCE(MAX(version), 0) FROM migrations",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        // Run all migrations that haven't been applied yet
        for migration in MIGRATIONS {
            if migration.version > current_version {
                conn.execute_batch(migration.sql).with_context(|| {
                    format!(
                        "Failed to run migration {}: {}",
                        migration.version, migration.name
                    )
                })?;

                conn.execute(
                    "INSERT INTO migrations (version) VALUES (?1)",
                    [migration.version],
                )?;
            }
        }

        Ok(())
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
