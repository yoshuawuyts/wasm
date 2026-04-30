//! `cargo xtask sql` — database migration tooling.
//!
//! With Rust-defined SeaORM migrations under
//! `crates/component-package-manager-migration/`, the legacy schema-diff
//! tooling (sqlite3def, schema.sql) is gone. The only remaining task is a
//! sanity-check that exercises the migrator against real backends — this is
//! what `cargo xtask sql check` does, and what `cargo xtask test` runs in CI.

#![allow(clippy::print_stdout, clippy::print_stderr)]

use anyhow::{Context, Result};

/// `cargo xtask sql install` — kept for backwards compatibility with old CI
/// scripts; today there's nothing to install.
pub(crate) fn install() {
    println!("`cargo xtask sql install` is a no-op since the SeaORM port.");
    println!("Migrations live under crates/component-package-manager-migration/.");
}

/// `cargo xtask sql migrate` — placeholder. Hand-author migration files
/// directly; there's no diff-based generator any more.
pub(crate) fn migrate(_name: &str) -> Result<()> {
    anyhow::bail!(
        "`cargo xtask sql migrate` is no longer supported. \
         Hand-author a new migration under \
         crates/component-package-manager-migration/src/migrations/ \
         and register it in `Migrator::migrations()`."
    );
}

/// `cargo xtask sql check` — apply migrations to ephemeral databases.
///
/// Always runs against in-memory SQLite. When `COMPONENT_DATABASE_URL` is
/// set to a Postgres URL, also runs against that database (CI uses this to
/// verify the Postgres schema applies cleanly).
pub(crate) fn check() -> Result<()> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("building tokio runtime")?;
    runtime.block_on(check_async())
}

async fn check_async() -> Result<()> {
    use component_package_manager_migration::{Migrator, MigratorTrait};
    use sea_orm::Database;

    println!("Applying migrations to in-memory SQLite...");
    let sqlite_db = Database::connect("sqlite::memory:")
        .await
        .context("connecting to in-memory SQLite")?;
    Migrator::up(&sqlite_db, None)
        .await
        .context("running migrations against SQLite")?;
    println!("  OK");

    if let Ok(url) = std::env::var("COMPONENT_DATABASE_URL")
        && (url.starts_with("postgres://") || url.starts_with("postgresql://"))
    {
        println!("Applying migrations to {url}...");
        let pg_db = Database::connect(url.clone())
            .await
            .with_context(|| format!("connecting to {url}"))?;
        // Reset the database first so the test is repeatable. Drop and re-up.
        Migrator::down(&pg_db, None)
            .await
            .context("rolling back Postgres migrations")?;
        Migrator::up(&pg_db, None)
            .await
            .context("running migrations against Postgres")?;
        println!("  OK");
    } else {
        println!("Skipping Postgres check (set COMPONENT_DATABASE_URL=postgres://... to enable).");
    }
    Ok(())
}
