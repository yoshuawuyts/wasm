use anyhow::Context;
use rusqlite::Connection;

/// A migration that can be applied to the database.
struct MigrationDef {
    version: u32,
    name: &'static str,
    sql: &'static str,
}

/// All migrations in order. Each migration is run exactly once.
const MIGRATIONS: &[MigrationDef] = &[
    MigrationDef {
        version: 1,
        name: "init",
        sql: include_str!("../migrations/01_init.sql"),
    },
    MigrationDef {
        version: 2,
        name: "known_packages",
        sql: include_str!("../migrations/02_known_packages.sql"),
    },
    MigrationDef {
        version: 3,
        name: "known_package_tags",
        sql: include_str!("../migrations/03_known_package_tags.sql"),
    },
    MigrationDef {
        version: 4,
        name: "image_size",
        sql: include_str!("../migrations/04_image_size.sql"),
    },
];

/// Information about the current migration state.
#[derive(Debug, Clone)]
pub struct Migrations {
    /// The current migration version applied to the database.
    pub current: u32,
    /// The total number of migrations available.
    pub total: u32,
}

impl Migrations {
    /// Initialize the migrations table and run all pending migrations.
    pub(crate) fn run_all(conn: &Connection) -> anyhow::Result<()> {
        // Create the migrations table if it doesn't exist
        conn.execute_batch(include_str!("../migrations/00_migrations.sql"))?;

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

    /// Returns information about the current migration state.
    pub(crate) fn get(conn: &Connection) -> anyhow::Result<Self> {
        let current: u32 = conn
            .query_row(
                "SELECT COALESCE(MAX(version), 0) FROM migrations",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);
        let total = MIGRATIONS.last().map(|m| m.version).unwrap_or(0);
        Ok(Self { current, total })
    }
}
