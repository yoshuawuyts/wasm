use rusqlite::Connection;

/// An interface extracted from a component image.
#[derive(Debug, Clone)]
pub struct InterfaceEntry {
    #[allow(dead_code)] // Only used in database queries, not accessed in Rust
    id: i64,
    pub image_id: i64,
    pub name: String,
    pub interface_type: String,
}

impl InterfaceEntry {
    /// Insert a new interface entry into the database.
    pub(crate) fn insert(
        conn: &Connection,
        image_id: i64,
        name: &str,
        interface_type: &str,
    ) -> anyhow::Result<()> {
        conn.execute(
            "INSERT INTO interface (image_id, name, interface_type) VALUES (?1, ?2, ?3)",
            (image_id, name, interface_type),
        )?;
        Ok(())
    }

    /// Get all interfaces for a specific image.
    pub(crate) fn get_by_image_id(
        conn: &Connection,
        image_id: i64,
    ) -> anyhow::Result<Vec<InterfaceEntry>> {
        let mut stmt = conn.prepare(
            "SELECT id, image_id, name, interface_type FROM interface WHERE image_id = ?1",
        )?;

        let rows = stmt.query_map([image_id], |row| {
            Ok(InterfaceEntry {
                id: row.get(0)?,
                image_id: row.get(1)?,
                name: row.get(2)?,
                interface_type: row.get(3)?,
            })
        })?;

        let mut entries = Vec::new();
        for row in rows {
            entries.push(row?);
        }
        Ok(entries)
    }

    /// Delete all interfaces for a specific image.
    pub(crate) fn delete_by_image_id(conn: &Connection, image_id: i64) -> anyhow::Result<()> {
        conn.execute("DELETE FROM interface WHERE image_id = ?1", [image_id])?;
        Ok(())
    }

    /// Get all interfaces in the database.
    pub(crate) fn get_all(conn: &Connection) -> anyhow::Result<Vec<InterfaceEntry>> {
        let mut stmt =
            conn.prepare("SELECT id, image_id, name, interface_type FROM interface")?;

        let rows = stmt.query_map([], |row| {
            Ok(InterfaceEntry {
                id: row.get(0)?,
                image_id: row.get(1)?,
                name: row.get(2)?,
                interface_type: row.get(3)?,
            })
        })?;

        let mut entries = Vec::new();
        for row in rows {
            entries.push(row?);
        }
        Ok(entries)
    }
}
