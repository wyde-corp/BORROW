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
    /// Opens the database, applying migrations and seeding content if empty.
    ///
    /// Seeding happens once, against an empty database, so the cost lands on
    /// first launch rather than in any particular view.
    pub fn connect() -> Result<Self> {
        let runtime = Arc::new(Runtime::new().map_err(|err| anyhow::anyhow!("tokio: {err}"))?);
        let pool = runtime.block_on(db::init())?;

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
