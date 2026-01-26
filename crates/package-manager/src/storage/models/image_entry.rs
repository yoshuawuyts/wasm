use oci_client::manifest::OciImageManifest;
use rusqlite::Connection;

/// Result of an insert operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsertResult {
    /// The entry was inserted successfully.
    Inserted,
    /// The entry already existed in the database.
    AlreadyExists,
}

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
    /// Size of the image on disk in bytes
    pub size_on_disk: u64,
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

    /// Checks if an image entry with the given reference already exists.
    pub(crate) fn exists(
        conn: &Connection,
        ref_registry: &str,
        ref_repository: &str,
        ref_tag: Option<&str>,
        ref_digest: Option<&str>,
    ) -> anyhow::Result<bool> {
        let count: i64 = match (ref_tag, ref_digest) {
            (Some(tag), Some(digest)) => conn.query_row(
                "SELECT COUNT(*) FROM image WHERE ref_registry = ?1 AND ref_repository = ?2 AND ref_tag = ?3 AND ref_digest = ?4",
                (ref_registry, ref_repository, tag, digest),
                |row| row.get(0),
            )?,
            (Some(tag), None) => conn.query_row(
                "SELECT COUNT(*) FROM image WHERE ref_registry = ?1 AND ref_repository = ?2 AND ref_tag = ?3 AND ref_digest IS NULL",
                (ref_registry, ref_repository, tag),
                |row| row.get(0),
            )?,
            (None, Some(digest)) => conn.query_row(
                "SELECT COUNT(*) FROM image WHERE ref_registry = ?1 AND ref_repository = ?2 AND ref_tag IS NULL AND ref_digest = ?3",
                (ref_registry, ref_repository, digest),
                |row| row.get(0),
            )?,
            (None, None) => conn.query_row(
                "SELECT COUNT(*) FROM image WHERE ref_registry = ?1 AND ref_repository = ?2 AND ref_tag IS NULL AND ref_digest IS NULL",
                (ref_registry, ref_repository),
                |row| row.get(0),
            )?,
        };
        Ok(count > 0)
    }

    /// Inserts a new image entry into the database if it doesn't already exist.
    /// Returns `InsertResult::AlreadyExists` if the entry already exists,
    /// or `InsertResult::Inserted` if it was successfully inserted.
    pub(crate) fn insert(
        conn: &Connection,
        ref_registry: &str,
        ref_repository: &str,
        ref_tag: Option<&str>,
        ref_digest: Option<&str>,
        manifest: &str,
        size_on_disk: u64,
    ) -> anyhow::Result<InsertResult> {
        // Check if entry already exists
        if Self::exists(conn, ref_registry, ref_repository, ref_tag, ref_digest)? {
            return Ok(InsertResult::AlreadyExists);
        }

        conn.execute(
            "INSERT INTO image (ref_registry, ref_repository, ref_tag, ref_digest, manifest, size_on_disk) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (ref_registry, ref_repository, ref_tag, ref_digest, manifest, size_on_disk as i64),
        )?;
        Ok(InsertResult::Inserted)
    }

    /// Returns all currently stored images and their metadata, ordered alphabetically by repository.
    pub(crate) fn get_all(conn: &Connection) -> anyhow::Result<Vec<ImageEntry>> {
        let mut stmt = conn.prepare(
            "SELECT id, ref_registry, ref_repository, ref_mirror_registry, ref_tag, ref_digest, manifest, size_on_disk FROM image ORDER BY ref_repository ASC, ref_registry ASC",
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
            let size_on_disk: i64 = row.get(7)?;

            Ok(ImageEntry {
                id: row.get(0)?,
                ref_registry: row.get(1)?,
                ref_repository: row.get(2)?,
                ref_mirror_registry: row.get(3)?,
                ref_tag: row.get(4)?,
                ref_digest: row.get(5)?,
                manifest,
                size_on_disk: size_on_disk as u64,
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

    /// Create a new ImageEntry for testing purposes.
    #[doc(hidden)]
    #[allow(clippy::too_many_arguments)]
    pub fn new_for_test(
        id: i64,
        ref_registry: String,
        ref_repository: String,
        ref_mirror_registry: Option<String>,
        ref_tag: Option<String>,
        ref_digest: Option<String>,
        manifest: OciImageManifest,
        size_on_disk: u64,
    ) -> Self {
        Self {
            id,
            ref_registry,
            ref_repository,
            ref_mirror_registry,
            ref_tag,
            ref_digest,
            manifest,
            size_on_disk,
        }
    }
}
