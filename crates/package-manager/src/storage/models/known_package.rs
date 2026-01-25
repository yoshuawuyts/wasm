use rusqlite::Connection;

/// A known package that persists in the database even after local deletion.
/// This is used to track packages the user has seen or searched for.
#[derive(Debug, Clone)]
pub struct KnownPackage {
    #[allow(dead_code)]
    id: i64,
    pub registry: String,
    pub repository: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
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
            format!("{}:{}" , self.reference(), tag)
        } else {
            format!("{}:latest", self.reference())
        }
    }

    /// Inserts or updates a known package in the database.
    /// If the package already exists, updates the last_seen_at timestamp.
    /// Also adds the tag if provided.
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

        // If a tag was provided, add it to the tags table
        if let Some(tag) = tag {
            let package_id: i64 = conn.query_row(
                "SELECT id FROM known_package WHERE registry = ?1 AND repository = ?2",
                (registry, repository),
                |row| row.get(0),
            )?;

            conn.execute(
                "INSERT INTO known_package_tag (known_package_id, tag) VALUES (?1, ?2)
                 ON CONFLICT(known_package_id, tag) DO UPDATE SET last_seen_at = datetime('now')",
                (package_id, tag),
            )?;
        }

        Ok(())
    }

    /// Helper to fetch tags for a package by its ID.
    fn fetch_tags(conn: &Connection, package_id: i64) -> Vec<String> {
        let mut stmt = conn
            .prepare(
                "SELECT tag FROM known_package_tag WHERE known_package_id = ?1 ORDER BY last_seen_at DESC",
            )
            .ok();

        if let Some(ref mut stmt) = stmt {
            stmt.query_map([package_id], |row| row.get(0))
                .ok()
                .map(|rows| rows.filter_map(|r| r.ok()).collect())
                .unwrap_or_default()
        } else {
            Vec::new()
        }
    }

    /// Search for known packages by a query string.
    /// Searches in both registry and repository fields.
    pub(crate) fn search(conn: &Connection, query: &str) -> anyhow::Result<Vec<KnownPackage>> {
        let search_pattern = format!("%{}%", query);
        let mut stmt = conn.prepare(
            "SELECT id, registry, repository, description, last_seen_at, created_at 
             FROM known_package 
             WHERE registry LIKE ?1 OR repository LIKE ?1
             ORDER BY last_seen_at DESC
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
            let tags = Self::fetch_tags(conn, id);
            packages.push(KnownPackage {
                id,
                registry,
                repository,
                description,
                tags,
                last_seen_at,
                created_at,
            });
        }
        Ok(packages)
    }

    /// Get all known packages, ordered by last seen.
    pub(crate) fn get_all(conn: &Connection) -> anyhow::Result<Vec<KnownPackage>> {
        let mut stmt = conn.prepare(
            "SELECT id, registry, repository, description, last_seen_at, created_at 
             FROM known_package 
             ORDER BY last_seen_at DESC
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
            let tags = Self::fetch_tags(conn, id);
            packages.push(KnownPackage {
                id,
                registry,
                repository,
                description,
                tags,
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
                let tags = Self::fetch_tags(conn, id);
                Ok(Some(KnownPackage {
                    id,
                    registry,
                    repository,
                    description,
                    tags,
                    last_seen_at,
                    created_at,
                }))
            }
            None => Ok(None),
        }
    }
}
