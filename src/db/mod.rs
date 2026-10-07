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

    let pool = SqlitePoolOptions::new()
        .max_connections(8)
        .connect_with(options)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    seed::ensure_content(&pool).await?;

    Ok(pool)
}
