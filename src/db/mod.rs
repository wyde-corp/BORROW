pub mod seed;

use anyhow::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;

use crate::config;

/// The web server's entry point, which reads `BORROW_DB_URL`. The desktop app
/// calls [`init_at`] directly, so this reads as dead code there.
#[allow(dead_code)]
pub async fn init() -> Result<SqlitePool> {
    init_at(&config::database_url()).await
}

/// [`init`] against an explicit URL, so tests can point at a throwaway file
/// instead of the app's database.
pub async fn init_at(url: &str) -> Result<SqlitePool> {
    let options = SqliteConnectOptions::from_str(url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .foreign_keys(true);

    open(options).await
}

/// The desktop app's database: `BORROW_DB_URL` when set, otherwise the file in
/// the per-user data directory.
///
/// A path rather than a URL on purpose: a user's home directory contains
/// spaces and, often, non-ASCII names, and turning one into a `sqlite:` URL is
/// where that goes wrong. `init_at` stays for the web and for tests, whose
/// URLs are ours to choose.
#[allow(dead_code)] // called by the desktop app only; the web uses `init`
pub async fn init_app() -> Result<SqlitePool> {
    match std::env::var("BORROW_DB_URL") {
        Ok(url) if !url.is_empty() => init_at(&url).await,
        _ => init_file(&crate::config::app_database_path()).await,
    }
}

/// [`init_at`] against a filesystem path, creating its directory if needed.
pub async fn init_file(path: &std::path::Path) -> Result<SqlitePool> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|err| anyhow::anyhow!("create {}: {err}", parent.display()))?;
        }
    }

    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .foreign_keys(true);

    open(options).await
}

/// The pool every entry point shares: connect, migrate, seed.
async fn open(options: SqliteConnectOptions) -> Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(8)
        .connect_with(options)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    seed::ensure_content(&pool).await?;

    Ok(pool)
}

#[cfg(test)]
mod file_init_tests {
    use super::*;

    /// `init_file` makes its directory — the per-user data directory does not
    /// exist on a first launch, and a database that cannot be created is an
    /// app that cannot start.
    #[test]
    fn a_missing_directory_is_created_and_the_database_opens() {
        let dir = std::env::temp_dir().join(format!("borrow-init-file-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);

        let path = dir.join("nested").join("borrow.db");
        let runtime = tokio::runtime::Runtime::new().expect("tokio runtime");
        let pool = runtime
            .block_on(init_file(&path))
            .expect("init_file creates its directory and migrates");

        assert!(path.exists(), "the database file should now exist");
        runtime.block_on(pool.close());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
