use oci_client::manifest::OciImageManifest;
use rusqlite::Connection;

/// Metadata for a stored OCI image.
#[derive(Debug, Clone)]
pub(crate) struct ImageEntry {
    pub id: i64,
    pub ref_registry: String,
    pub ref_repository: String,
    pub ref_mirror_registry: Option<String>,
    pub ref_tag: Option<String>,
    pub ref_digest: Option<String>,
    pub manifest: OciImageManifest,
}

impl ImageEntry {
    /// Inserts a new image entry into the database.
    pub(crate) fn insert(
        conn: &Connection,
        ref_registry: &str,
        ref_repository: &str,
        ref_tag: Option<&str>,
        ref_digest: Option<&str>,
        manifest: &str,
    ) -> anyhow::Result<()> {
        conn.execute(
            "INSERT INTO image (ref_registry, ref_repository, ref_tag, ref_digest, manifest) VALUES (?1, ?2, ?3, ?4, ?5)",
            (ref_registry, ref_repository, ref_tag, ref_digest, manifest),
        )?;
        Ok(())
    }

    /// Returns all currently stored images and their metadata.
    pub(crate) fn get_all(conn: &Connection) -> anyhow::Result<Vec<ImageEntry>> {
        let mut stmt = conn.prepare(
            "SELECT id, ref_registry, ref_repository, ref_mirror_registry, ref_tag, ref_digest, manifest FROM image",
        )?;

        let rows = stmt.query_map([], |row| {
            let manifest_json: String = row.get(6)?;
            let manifest: OciImageManifest = serde_json::from_str(&manifest_json).map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    6,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;

            Ok(ImageEntry {
                id: row.get(0)?,
                ref_registry: row.get(1)?,
                ref_repository: row.get(2)?,
                ref_mirror_registry: row.get(3)?,
                ref_tag: row.get(4)?,
                ref_digest: row.get(5)?,
                manifest,
            })
        })?;

        let mut entries = Vec::new();
        for row in rows {
            entries.push(row?);
        }
        Ok(entries)
    }
}
