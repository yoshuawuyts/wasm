use rusqlite::Connection;

/// The type of a tag, used to distinguish release tags from signatures and attestations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagType {
    /// A regular release tag (e.g., "1.0.0", "latest")
    Release,
    /// A signature tag (ending in ".sig")
    Signature,
    /// An attestation tag (ending in ".att")
    Attestation,
}

impl TagType {
    /// Determine the tag type from a tag string.
    pub fn from_tag(tag: &str) -> Self {
        if tag.ends_with(".sig") {
            TagType::Signature
        } else if tag.ends_with(".att") {
            TagType::Attestation
        } else {
            TagType::Release
        }
    }

    /// Convert to the database string representation.
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            TagType::Release => "release",
            TagType::Signature => "signature",
            TagType::Attestation => "attestation",
        }
    }
}

/// A known package that persists in the database even after local deletion.
/// This is used to track packages the user has seen or searched for.
#[derive(Debug, Clone)]
pub struct KnownPackage {
    #[allow(dead_code)]
    id: i64,
    pub registry: String,
    pub repository: String,
    pub description: Option<String>,
    /// Release tags (regular version tags like "1.0.0", "latest")
    pub tags: Vec<String>,
    /// Signature tags (tags ending in ".sig")
    pub signature_tags: Vec<String>,
    /// Attestation tags (tags ending in ".att")
    pub attestation_tags: Vec<String>,
    pub last_seen_at: String,
    pub created_at: String,
}

impl KnownPackage {
    /// Returns the full reference string for this package (e.g., "ghcr.io/user/repo").
    pub fn reference(&self) -> String {
        format!("{}/{}", self.registry, self.repository)
    }

    /// Returns the full reference string with the most recent tag.
    pub fn reference_with_tag(&self) -> String {
        if let Some(tag) = self.tags.first() {
            format!("{}:{}", self.reference(), tag)
        } else {
            format!("{}:latest", self.reference())
        }
    }

    /// Inserts or updates a known package in the database.
    /// If the package already exists, updates the last_seen_at timestamp.
    /// Also adds the tag if provided, classifying it by type.
    pub(crate) fn upsert(
        conn: &Connection,
        registry: &str,
        repository: &str,
        tag: Option<&str>,
        description: Option<&str>,
    ) -> anyhow::Result<()> {
        conn.execute(
            "INSERT INTO known_package (registry, repository, description) VALUES (?1, ?2, ?3)
             ON CONFLICT(registry, repository) DO UPDATE SET 
                last_seen_at = datetime('now'),
                description = COALESCE(excluded.description, known_package.description)",
            (registry, repository, description),
        )?;

        // If a tag was provided, add it to the tags table with its type
        if let Some(tag) = tag {
            let package_id: i64 = conn.query_row(
                "SELECT id FROM known_package WHERE registry = ?1 AND repository = ?2",
                (registry, repository),
                |row| row.get(0),
            )?;

            let tag_type = TagType::from_tag(tag);
            conn.execute(
                "INSERT INTO known_package_tag (known_package_id, tag, tag_type) VALUES (?1, ?2, ?3)
                 ON CONFLICT(known_package_id, tag) DO UPDATE SET last_seen_at = datetime('now'), tag_type = ?3",
                (package_id, tag, tag_type.as_str()),
            )?;
        }

        Ok(())
    }

    /// Helper to fetch tags for a package by its ID, separated by type.
    /// Returns (release_tags, signature_tags, attestation_tags).
    fn fetch_tags_by_type(
        conn: &Connection,
        package_id: i64,
    ) -> (Vec<String>, Vec<String>, Vec<String>) {
        let mut release_tags = Vec::new();
        let mut signature_tags = Vec::new();
        let mut attestation_tags = Vec::new();

        let mut stmt = match conn.prepare(
            "SELECT tag, tag_type FROM known_package_tag WHERE known_package_id = ?1 ORDER BY last_seen_at DESC",
        ) {
            Ok(stmt) => stmt,
            Err(_) => return (release_tags, signature_tags, attestation_tags),
        };

        let rows = match stmt.query_map([package_id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        }) {
            Ok(rows) => rows,
            Err(_) => return (release_tags, signature_tags, attestation_tags),
        };

        for row in rows.flatten() {
            let (tag, tag_type) = row;
            match tag_type.as_str() {
                "signature" => signature_tags.push(tag),
                "attestation" => attestation_tags.push(tag),
                _ => release_tags.push(tag),
            }
        }

        (release_tags, signature_tags, attestation_tags)
    }

    /// Search for known packages by a query string.
    /// Searches in both registry and repository fields.
    pub(crate) fn search(conn: &Connection, query: &str) -> anyhow::Result<Vec<KnownPackage>> {
        let search_pattern = format!("%{}%", query);
        let mut stmt = conn.prepare(
            "SELECT id, registry, repository, description, last_seen_at, created_at 
             FROM known_package 
             WHERE registry LIKE ?1 OR repository LIKE ?1
             ORDER BY repository ASC, registry ASC
             LIMIT 100",
        )?;

        let rows = stmt.query_map([&search_pattern], |row| {
            let id: i64 = row.get(0)?;
            Ok((
                id,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
            ))
        })?;

        let mut packages = Vec::new();
        for row in rows {
            let (id, registry, repository, description, last_seen_at, created_at) = row?;
            let (tags, signature_tags, attestation_tags) = Self::fetch_tags_by_type(conn, id);
            packages.push(KnownPackage {
                id,
                registry,
                repository,
                description,
                tags,
                signature_tags,
                attestation_tags,
                last_seen_at,
                created_at,
            });
        }
        Ok(packages)
    }

    /// Get all known packages, ordered alphabetically by repository.
    pub(crate) fn get_all(conn: &Connection) -> anyhow::Result<Vec<KnownPackage>> {
        let mut stmt = conn.prepare(
            "SELECT id, registry, repository, description, last_seen_at, created_at 
             FROM known_package 
             ORDER BY repository ASC, registry ASC
             LIMIT 100",
        )?;

        let rows = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            Ok((
                id,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
            ))
        })?;

        let mut packages = Vec::new();
        for row in rows {
            let (id, registry, repository, description, last_seen_at, created_at) = row?;
            let (tags, signature_tags, attestation_tags) = Self::fetch_tags_by_type(conn, id);
            packages.push(KnownPackage {
                id,
                registry,
                repository,
                description,
                tags,
                signature_tags,
                attestation_tags,
                last_seen_at,
                created_at,
            });
        }
        Ok(packages)
    }

    /// Get a known package by registry and repository.
    #[allow(dead_code)]
    pub(crate) fn get(
        conn: &Connection,
        registry: &str,
        repository: &str,
    ) -> anyhow::Result<Option<KnownPackage>> {
        let mut stmt = conn.prepare(
            "SELECT id, registry, repository, description, last_seen_at, created_at 
             FROM known_package 
             WHERE registry = ?1 AND repository = ?2",
        )?;

        let mut rows = stmt.query_map([registry, repository], |row| {
            let id: i64 = row.get(0)?;
            Ok((
                id,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
            ))
        })?;

        match rows.next() {
            Some(row) => {
                let (id, registry, repository, description, last_seen_at, created_at) = row?;
                let (tags, signature_tags, attestation_tags) = Self::fetch_tags_by_type(conn, id);
                Ok(Some(KnownPackage {
                    id,
                    registry,
                    repository,
                    description,
                    tags,
                    signature_tags,
                    attestation_tags,
                    last_seen_at,
                    created_at,
                }))
            }
            None => Ok(None),
        }
    }
}
