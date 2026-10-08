//! Database access from the desktop app.
//!
//! sqlx is async and gpui's event loop is not, so a tokio runtime is created
//! once and kept for the process's life. Queries are run with `block_on`.
//!
//! ## Why blocking
//!
//! It would be more idiomatic to thread futures through every click handler
//! and notify on completion. The cost is a state machine spread across the UI
//! — "querying / loaded / failed" for each read — buying nothing measurable,
//! because these are local SQLite reads and writes against a file we own:
//! microseconds, not milliseconds. Nothing here reaches the network or waits on
//! another process, so the UI thread is never at risk of a stall beyond those
//! microseconds.
//!
//! If anything in here ever *could* take a lock or do slow work, it must move
//! onto `cx.background_executor()` instead. The comment is on `block_on` for
//! exactly that reason.

use std::sync::Arc;

use anyhow::Result;
use gpui_kit::Global;
use tokio::runtime::Runtime;

use crate::db;

/// The process's database handle, registered as a gpui global so any view can
/// record progress without passing a pool through its constructors.
pub struct Db {
    runtime: Arc<Runtime>,
    pool: sqlx::SqlitePool,
}

impl Global for Db {}

impl Db {
    /// Opens the app's database, applying migrations and seeding if empty.
    ///
    /// The location is `db::init_app`'s decision — the per-user data
    /// directory, because an installed app has no cwd of its own — and
    /// seeding happens here rather than in a view, so the cost lands once on
    /// launch and no click handler can discover halfway through that the
    /// schema is not there yet.
    pub fn connect() -> Result<Self> {
        let runtime = Arc::new(Runtime::new().map_err(|err| anyhow::anyhow!("tokio: {err}"))?);
        let pool = runtime.block_on(db::init_app())?;

        Ok(Self { runtime, pool })
    }

    /// Opens an arbitrary database URL.
    ///
    /// Exists for `ui_tests`: writing a practice run must not reach for the
    /// real database, so a test hands in a throwaway file and migrates and
    /// seeds that instead. Nothing in the shipped app calls it — hence the
    /// allow — but the tests are the only reason it is here.
    #[allow(dead_code)]
    pub fn connect_with(url: &str) -> Result<Self> {
        let runtime = Arc::new(Runtime::new().map_err(|err| anyhow::anyhow!("tokio: {err}"))?);
        let pool = runtime.block_on(db::init_at(url))?;

        Ok(Self { runtime, pool })
    }

    pub fn pool(&self) -> &sqlx::SqlitePool {
        &self.pool
    }

    /// Runs a query to completion on the app's runtime.
    ///
    /// See the module docs: this is deliberately synchronous, and everything
    /// callable through it must stay local and fast.
    pub fn block_on<F: std::future::Future>(&self, future: F) -> F::Output {
        self.runtime.block_on(future)
    }
}
