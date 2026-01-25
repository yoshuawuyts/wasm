use oci_client::manifest::OciImageManifest;
use rusqlite::Connection;

/// Metadata for a stored OCI image.
#[derive(Debug, Clone)]
pub struct ImageEntry {
    #[allow(dead_code)] // Used in database schema
    id: i64,
    pub ref_registry: String,
    pub ref_repository: String,
    pub ref_mirror_registry: Option<String>,
    pub ref_tag: Option<String>,
    pub ref_digest: Option<String>,
    pub manifest: OciImageManifest,
}

impl ImageEntry {
    /// Returns the full reference string for this image (e.g., "ghcr.io/user/repo:tag").
    pub fn reference(&self) -> String {
        let mut reference = format!("{}/{}", self.ref_registry, self.ref_repository);
        if let Some(tag) = &self.ref_tag {
            reference.push(':');
            reference.push_str(tag);
        } else if let Some(digest) = &self.ref_digest {
            reference.push('@');
            reference.push_str(digest);
        }
        reference
    }

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

    /// Deletes an image entry by its full reference string.
    pub(crate) fn delete_by_reference(
        conn: &Connection,
        registry: &str,
        repository: &str,
        tag: Option<&str>,
        digest: Option<&str>,
    ) -> anyhow::Result<bool> {
        let rows_affected = match (tag, digest) {
            (Some(tag), Some(digest)) => conn.execute(
                "DELETE FROM image WHERE ref_registry = ?1 AND ref_repository = ?2 AND ref_tag = ?3 AND ref_digest = ?4",
                (registry, repository, tag, digest),
            )?,
            (Some(tag), None) => conn.execute(
                "DELETE FROM image WHERE ref_registry = ?1 AND ref_repository = ?2 AND ref_tag = ?3",
                (registry, repository, tag),
            )?,
            (None, Some(digest)) => conn.execute(
                "DELETE FROM image WHERE ref_registry = ?1 AND ref_repository = ?2 AND ref_digest = ?3",
                (registry, repository, digest),
            )?,
            (None, None) => conn.execute(
                "DELETE FROM image WHERE ref_registry = ?1 AND ref_repository = ?2",
                (registry, repository),
            )?,
        };
        Ok(rows_affected > 0)
    }
}
