//! Progress sync: the local completions, and the account's copy of them.
//!
//! Both directions are union merges — a completion is a fact that happened
//! or did not, so replays, races and multi-device edits converge instead of
//! overwriting. The push carries the *whole* local set every time (a few
//! hundred rows at most), which makes the call idempotent by construction.
//!
//! Every async function here runs on the database's tokio runtime (see
//! `app_db::Db::runtime`): sqlx and reqwest both need a reactor, and the UI
//! thread has none.

use anyhow::Result;
use gpui_kit::{App, Entity};
use sqlx::SqlitePool;

use crate::screen::AppView;
use crate::services::remote::{self, ProgressRow};

/// Launch sync: push the local set, then pull the account's — which is also
/// the one-time adoption of any progress this device made before it had an
/// account (the union merge makes "adopt" and "sync" the same operation).
///
/// Both halves run on the database runtime; failures are logged, never
/// fatal. The local database stays the working copy either way.
pub fn spawn_initial(view: &Entity<AppView>, cx: &App) {
    let Some(site) = crate::config::convex_site_url() else {
        return;
    };
    let Some(token) = view
        .read(cx)
        .session
        .as_ref()
        .map(|session| session.token.clone())
    else {
        return;
    };
    let Some(db) = cx.try_global::<crate::app_db::Db>() else {
        return;
    };
    let runtime = db.runtime();
    let pool = db.pool().clone();

    runtime.spawn(async move {
        if let Err(err) = push(&pool, &site, &token).await {
            eprintln!("sync: push failed: {err:#}");
        }
        match pull_into(&pool, &site, &token).await {
            Ok(0) => {}
            Ok(count) => eprintln!("sync: merged {count} completions from the account"),
            Err(err) => eprintln!("sync: pull failed: {err:#}"),
        }
    });
}

/// Every local completion, shaped for the push endpoint. The epoch
/// conversion happens in SQL: `completed_at` is SQLite datetime text, and
/// `strftime` is the one parser guaranteed to agree with how the rows were
/// written.
pub async fn local_rows(pool: &SqlitePool) -> Result<Vec<ProgressRow>> {
    let rows = sqlx::query_as::<_, (String, i64, i64, i64)>(
        r#"
        SELECT 'lesson', lesson_id,
               CAST(COALESCE(strftime('%s', completed_at), 0) AS INTEGER) * 1000,
               xp_earned
          FROM progress WHERE lesson_id IS NOT NULL
        UNION ALL
        SELECT 'exercise', exercise_id,
               CAST(COALESCE(strftime('%s', completed_at), 0) AS INTEGER) * 1000,
               xp_earned
          FROM progress WHERE exercise_id IS NOT NULL
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|(kind, item_id, completed_at, xp)| ProgressRow {
            kind,
            item_id,
            completed_at: completed_at.max(0) as u64,
            xp,
        })
        .collect())
}

/// Pushes the full local set. The server ignores rows it already has.
pub async fn push(pool: &SqlitePool, site: &str, token: &str) -> Result<()> {
    let rows = local_rows(pool).await?;
    remote::push_progress(site, token, &rows).await
}

/// Pulls the account's rows and merges the ones this device is missing.
/// Returns how many rows were actually inserted.
pub async fn pull_into(pool: &SqlitePool, site: &str, token: &str) -> Result<usize> {
    let rows = remote::pull_progress(site, token).await?;
    let mut inserted = 0;
    for row in &rows {
        if merge_one(pool, row).await? {
            inserted += 1;
        }
    }
    Ok(inserted)
}

/// One row of the union merge. A row that already exists — here or under
/// the other id shape — is left exactly as it is; an id that resolves to no
/// local lesson or exercise is skipped rather than inserted blind (it would
/// be invisible to every query and unprunable).
async fn merge_one(pool: &SqlitePool, row: &ProgressRow) -> Result<bool> {
    if row.item_id <= 0 {
        return Ok(false);
    }

    match row.kind.as_str() {
        "lesson" => {
            let exists: bool =
                sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM progress WHERE lesson_id = ?)")
                    .bind(row.item_id)
                    .fetch_one(pool)
                    .await?;
            if exists {
                return Ok(false);
            }
            let module: Option<i64> =
                sqlx::query_scalar("SELECT module_id FROM lessons WHERE id = ?")
                    .bind(row.item_id)
                    .fetch_optional(pool)
                    .await?;
            let Some(module_id) = module else {
                return Ok(false);
            };

            sqlx::query(
                "INSERT INTO progress (module_id, lesson_id, exercise_id, xp_earned, completed_at, user_id)
                 VALUES (?, ?, NULL, ?, datetime(? / 1000, 'unixepoch'), 'local')",
            )
            .bind(module_id)
            .bind(row.item_id)
            .bind(row.xp)
            .bind(row.completed_at as i64)
            .execute(pool)
            .await?;
            Ok(true)
        }
        "exercise" => {
            let exists: bool =
                sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM progress WHERE exercise_id = ?)")
                    .bind(row.item_id)
                    .fetch_one(pool)
                    .await?;
            if exists {
                return Ok(false);
            }
            let module: Option<i64> =
                sqlx::query_scalar("SELECT module_id FROM exercises WHERE id = ?")
                    .bind(row.item_id)
                    .fetch_optional(pool)
                    .await?;
            let Some(module_id) = module else {
                return Ok(false);
            };

            sqlx::query(
                "INSERT INTO progress (module_id, lesson_id, exercise_id, xp_earned, completed_at, user_id)
                 VALUES (?, NULL, ?, ?, datetime(? / 1000, 'unixepoch'), 'local')",
            )
            .bind(module_id)
            .bind(row.item_id)
            .bind(row.xp)
            .bind(row.completed_at as i64)
            .execute(pool)
            .await?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::{local_rows, merge_one, ProgressRow};
    use sqlx::SqlitePool;

    /// The push payload is built in SQL; this pins the shape against the
    /// fixture database the migrations seed, so a column rename or a
    /// strftime mistake shows up as a failing test rather than as a sync
    /// that silently drops rows.
    #[tokio::test]
    async fn local_rows_reads_the_seeded_progress_shape() {
        let path = std::env::temp_dir().join(format!("borrow-sync-{}.db", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let url = format!("sqlite:{}?mode=rwc", path.display());
        // Migrates *and seeds* — the pull test below needs a real lesson row
        // to resolve against, and the seed is what the shipped app writes.
        let pool: SqlitePool = crate::db::init_at(&url).await.expect("migrate and seed");

        // One completion, written the way the app writes it — against a
        // real seeded lesson, because the schema enforces the foreign keys
        // this table's rows point at.
        let lesson_id = first_lesson_id(&pool).await;
        let module_id: i64 = sqlx::query_scalar("SELECT module_id FROM lessons WHERE id = ?")
            .bind(lesson_id)
            .fetch_one(&pool)
            .await
            .expect("lesson's module");
        sqlx::query(
            "INSERT INTO progress (module_id, lesson_id, exercise_id, xp_earned) VALUES (?, ?, NULL, 0)",
        )
        .bind(module_id)
        .bind(lesson_id)
        .execute(&pool)
        .await
        .expect("insert");

        let rows = local_rows(&pool).await.expect("local rows");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].kind, "lesson");
        assert_eq!(rows[0].item_id, lesson_id);
        assert_eq!(rows[0].xp, 0);
        assert!(rows[0].completed_at > 0, "datetime text becomes epoch ms");

        let _ = std::fs::remove_file(&path);
    }

    /// The pull side: a remote completion for a lesson this device knows
    /// about lands locally; one for an id it does not is skipped.
    #[tokio::test]
    async fn pull_merges_known_ids_and_skips_unknown_ones() {
        let path = std::env::temp_dir().join(format!("borrow-pull-{}.db", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let url = format!("sqlite:{}?mode=rwc", path.display());
        let pool: SqlitePool = crate::db::init_at(&url).await.expect("migrate and seed");

        let known = ProgressRow {
            kind: "lesson".into(),
            item_id: first_lesson_id(&pool).await,
            completed_at: 1_791_600_000_000,
            xp: 0,
        };
        let unknown = ProgressRow {
            kind: "lesson".into(),
            item_id: 999_999,
            completed_at: 1_791_600_000_000,
            xp: 0,
        };

        assert!(merge_one(&pool, &known).await.expect("merge known"));
        assert!(!merge_one(&pool, &known).await.expect("idempotent"));
        assert!(!merge_one(&pool, &unknown)
            .await
            .expect("unknown id skipped"));

        let stored: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM progress")
            .fetch_one(&pool)
            .await
            .expect("count");
        assert_eq!(stored, 1);

        let _ = std::fs::remove_file(&path);
    }

    /// The id of the first seeded lesson — looked up rather than hard-coded,
    /// so a content edit cannot make the test lie about what exists.
    async fn first_lesson_id(pool: &SqlitePool) -> i64 {
        sqlx::query_scalar("SELECT id FROM lessons ORDER BY id LIMIT 1")
            .fetch_one(pool)
            .await
            .expect("seeded lessons exist")
    }
}
