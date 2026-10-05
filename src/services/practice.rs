//! Graded practice runs.
//!
//! A run is a learner's own selection of questions from one module, answered
//! through the runner and scored at the end. Nothing here writes to `progress`
//! or awards XP — practice is formative. That separation is the whole reason
//! these queries live in their own file rather than alongside
//! [`crate::services::progress`], which is the module that owns credit.

use anyhow::Result;
use sqlx::SqlitePool;

use crate::models::Exercise;

/// The answerable questions of a module, in curriculum order.
///
/// Open-ended kinds are filtered out: a run is graded, and there is nothing to
/// grade a `challenge` or `build_it` against.
pub async fn gradable_exercises(pool: &SqlitePool, module_id: i64) -> Result<Vec<Exercise>> {
    let all = crate::services::progress::exercises_for_module(pool, module_id).await?;

    Ok(all
        .into_iter()
        .filter(|e| !e.is_open_ended())
        .collect())
}

/// How a question went the last time it was answered in any run of this module.
/// Drives the "you got this right last time" chips in the picker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttemptState {
    Correct,
    Incorrect,
}

#[derive(Debug, Clone)]
pub struct Run {
    pub id: i64,
    pub module_id: i64,
    pub label: String,
    pub created_at: String,
    pub finished_at: Option<String>,
}

impl Run {
    pub fn is_finished(&self) -> bool {
        self.finished_at.is_some()
    }
}

/// One question in a run, with its 1-based position and the exercise behind it.
#[derive(Debug, Clone)]
pub struct RunItem {
    pub position: i64,
    pub exercise: Exercise,
}

/// The answer the learner gave a question, for the results screen.
#[derive(Debug, Clone)]
pub struct Answer {
    pub position: i64,
    pub exercise_id: i64,
    pub correct: bool,
    pub given: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BuilderData {
    pub module_id: i64,
    pub module_title: String,
    pub module_order: i64,
    pub exercises: Vec<Exercise>,
    pub lesson_titles: Vec<(i64, String)>,
    /// Latest state per exercise id, across every run of this module.
    pub past: Vec<(i64, AttemptState)>,
    pub unfinished: Option<Run>,
    pub history: Vec<(Run, i64, i64)>,
}

#[derive(Debug, Clone)]
pub struct RunPageData {
    pub run: Run,
    pub items: Vec<RunItem>,
    pub answers: Vec<(i64, bool, String)>,
    pub module_title: String,
    pub module_order: i64,
}

#[derive(Debug, Clone)]
pub struct ResultsData {
    pub run: Run,
    pub items: Vec<RunItem>,
    pub answers: Vec<Answer>,
}

pub async fn run_by_id(pool: &SqlitePool, run_id: i64) -> Result<Option<Run>> {
    Ok(sqlx::query_as::<_, (i64, i64, String, String, Option<String>)>(
        "SELECT id, module_id, label, created_at, finished_at FROM practice_runs WHERE id = ?",
    )
    .bind(run_id)
    .fetch_optional(pool)
    .await?
    .map(|(id, module_id, label, created_at, finished_at)| Run {
        id,
        module_id,
        label,
        created_at,
        finished_at,
    }))
}

/// Every question in a run, ordered by position.
pub async fn run_items(pool: &SqlitePool, run_id: i64) -> Result<Vec<RunItem>> {
    #[derive(sqlx::FromRow)]
    struct Row {
        position: i64,
        exercise_id: i64,
    }

    let rows: Vec<Row> = sqlx::query_as(
        "SELECT position, exercise_id FROM practice_run_items WHERE run_id = ? ORDER BY position",
    )
    .bind(run_id)
    .fetch_all(pool)
    .await?;

    let mut items = Vec::with_capacity(rows.len());
    for row in rows {
        if let Some(exercise) = crate::services::progress::exercise_by_id(pool, row.exercise_id).await? {
            items.push(RunItem {
                position: row.position,
                exercise,
            });
        }
    }

    Ok(items)
}

/// `(exercise_id, correct, given)` for everything answered in this run.
pub async fn run_answers(pool: &SqlitePool, run_id: i64) -> Result<Vec<(i64, bool, String)>> {
    Ok(sqlx::query_as::<_, (i64, bool, Option<String>)>(
        "SELECT exercise_id, correct != 0, given FROM practice_answers WHERE run_id = ?",
    )
    .bind(run_id)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|(id, correct, given)| (id, correct, given.unwrap_or_default()))
    .collect())
}

/// Creates a run from an explicit, already-validated list of exercise ids.
pub async fn create_run(
    pool: &SqlitePool,
    module_id: i64,
    label: &str,
    exercise_ids: &[i64],
) -> Result<Run> {
    let mut tx = pool.begin().await?;

    sqlx::query("INSERT INTO practice_runs (module_id, label) VALUES (?, ?)")
        .bind(module_id)
        .bind(label)
        .execute(&mut *tx)
        .await?;

    let run_id: i64 = sqlx::query_scalar("SELECT last_insert_rowid()")
        .fetch_one(&mut *tx)
        .await?;

    for (i, exercise_id) in exercise_ids.iter().enumerate() {
        sqlx::query("INSERT INTO practice_run_items (run_id, position, exercise_id) VALUES (?, ?, ?)")
            .bind(run_id)
            .bind(i as i64 + 1)
            .bind(exercise_id)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;

    Ok(Run {
        id: run_id,
        module_id,
        label: label.to_string(),
        created_at: String::new(),
        finished_at: None,
    })
}

/// Records an answer, replacing any previous one for the same question.
///
/// Deliberately does not call `record_exercise_result`, and deliberately does
/// not insert into `progress`. A wrong answer here must be invisible to the
/// module page and the XP total.
pub async fn record_answer(
    pool: &SqlitePool,
    run_id: i64,
    exercise_id: i64,
    given: &str,
    correct: bool,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO practice_answers (run_id, exercise_id, correct, given) VALUES (?, ?, ?, ?)
         ON CONFLICT(run_id, exercise_id) DO UPDATE SET correct = excluded.correct,
                                                      given = excluded.given,
                                                      answered_at = datetime('now')",
    )
    .bind(run_id)
    .bind(exercise_id)
    .bind(correct as i32)
    .bind(given)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn finish_run(pool: &SqlitePool, run_id: i64) -> Result<()> {
    sqlx::query("UPDATE practice_runs SET finished_at = datetime('now') WHERE id = ?")
        .bind(run_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn delete_run(pool: &SqlitePool, run_id: i64) -> Result<()> {
    // Items and answers cascade.
    sqlx::query("DELETE FROM practice_runs WHERE id = ?")
        .bind(run_id)
        .execute(pool)
        .await?;

    Ok(())
}

/// The most recent run of this module that has not been graded yet.
pub async fn unfinished_run(pool: &SqlitePool, module_id: i64) -> Result<Option<Run>> {
    let row = sqlx::query_as::<_, (i64, i64, String, String, Option<String>)>(
        "SELECT id, module_id, label, created_at, finished_at FROM practice_runs
         WHERE module_id = ? AND finished_at IS NULL
         ORDER BY id DESC LIMIT 1",
    )
    .bind(module_id)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|(id, module_id, label, created_at, finished_at)| Run {
        id,
        module_id,
        label,
        created_at,
        finished_at,
    }))
}

/// Recent finished runs, newest first, with their correct and total counts.
pub async fn run_history(pool: &SqlitePool, module_id: i64) -> Result<Vec<(Run, i64, i64)>> {
    #[derive(sqlx::FromRow)]
    struct Row {
        id: i64,
        label: String,
        created_at: String,
        total: i64,
        correct: i64,
    }

    let rows: Vec<Row> = sqlx::query_as(
        "SELECT r.id, r.label, r.created_at,
                COUNT(i.exercise_id) AS total,
                COALESCE(SUM(a.correct), 0) AS correct
         FROM practice_runs r
         JOIN practice_run_items i ON i.run_id = r.id
         LEFT JOIN practice_answers a ON a.run_id = r.id AND a.exercise_id = i.exercise_id
         WHERE r.module_id = ? AND r.finished_at IS NOT NULL
         GROUP BY r.id
         ORDER BY r.id DESC
         LIMIT 8",
    )
    .bind(module_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| {
            (
                Run {
                    id: row.id,
                    module_id,
                    label: row.label,
                    created_at: row.created_at,
                    finished_at: Some(String::new()),
                },
                row.correct,
                row.total,
            )
        })
        .collect())
}

/// Latest answer per exercise across every run of this module.
///
/// The picker uses this to show what you already know, so it takes the most
/// recent answer rather than any answer — getting a question right on the
/// second attempt should stop it looking wrong.
pub async fn past_states(pool: &SqlitePool, module_id: i64) -> Result<Vec<(i64, AttemptState)>> {
    // `practice_answers` has a composite primary key, so there is no `id`
    // column to order by — SQLite's implicit rowid is used instead to break
    // ties between answers recorded in the same second.
    Ok(sqlx::query_as::<_, (i64, bool)>(
        "SELECT a.exercise_id, a.correct != 0
         FROM practice_answers a
         JOIN practice_runs r ON r.id = a.run_id
         WHERE r.module_id = ?
           AND a.rowid = (
               SELECT a2.rowid FROM practice_answers a2
               JOIN practice_runs r2 ON r2.id = a2.run_id
               WHERE r2.module_id = ? AND a2.exercise_id = a.exercise_id
               ORDER BY a2.answered_at DESC, a2.rowid DESC
               LIMIT 1
           )",
    )
    .bind(module_id)
    .bind(module_id)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|(id, correct)| {
        (
            id,
            if correct {
                AttemptState::Correct
            } else {
                AttemptState::Incorrect
            },
        )
    })
    .collect())
}

/// Everything the run builder page needs.
///
/// `None` only when the module does not exist. A real query failure stays an
/// error, so a broken database surfaces as a 500 rather than being disguised as
/// a 404 — otherwise this is impossible to debug from the outside.
pub async fn builder_data(pool: &SqlitePool, module_id: i64) -> Result<Option<BuilderData>> {
    let Some(module) = crate::services::progress::module_by_id(pool, module_id).await? else {
        return Ok(None);
    };

    let exercises = gradable_exercises(pool, module_id).await?;

    let mut lessons = crate::services::progress::lessons_for_module(pool, module_id).await?;
    lessons.sort_by_key(|l| l.order_index);
    let lesson_titles: Vec<(i64, String)> =
        lessons.into_iter().map(|l| (l.id, l.title)).collect();

    Ok(Some(BuilderData {
        module_id,
        module_title: module.title,
        module_order: module.order_index,
        exercises,
        lesson_titles,
        past: past_states(pool, module_id).await?,
        unfinished: unfinished_run(pool, module_id).await?,
        history: run_history(pool, module_id).await?,
    }))
}

pub async fn run_page_data(pool: &SqlitePool, run_id: i64) -> Result<Option<RunPageData>> {
    let Some(run) = run_by_id(pool, run_id).await? else {
        return Ok(None);
    };

    let module = crate::services::progress::module_by_id(pool, run.module_id).await?;

    Ok(Some(RunPageData {
        items: run_items(pool, run_id).await?,
        answers: run_answers(pool, run_id).await?,
        module_title: module.as_ref().map(|m| m.title.clone()).unwrap_or_default(),
        module_order: module.as_ref().map(|m| m.order_index).unwrap_or(0),
        run,
    }))
}

pub async fn results_data(pool: &SqlitePool, run_id: i64) -> Result<Option<ResultsData>> {
    let Some(page) = run_page_data(pool, run_id).await? else {
        return Ok(None);
    };

    let mut answers: Vec<Answer> = sqlx::query_as::<_, (i64, bool, Option<String>)>(
        "SELECT exercise_id, correct != 0, given FROM practice_answers WHERE run_id = ?",
    )
    .bind(run_id)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|(exercise_id, correct, given)| Answer {
        position: 0,
        exercise_id,
        correct,
        given,
    })
    .collect();

    for item in &page.items {
        if let Some(a) = answers.iter_mut().find(|a| a.exercise_id == item.exercise.id) {
            a.position = item.position;
        }
    }
    answers.sort_by_key(|a| a.position);

    Ok(Some(ResultsData {
        items: page.items,
        answers,
        run: page.run,
    }))
}

/// Picks `n` questions spread evenly across `total`.
///
/// `idx = (k * total) / n` rather than a stride between the first and last:
/// it cannot produce a duplicate and cannot fall off the end, and the same `n`
/// always selects the same set so two attempts are comparable.
pub fn spread(total: usize, n: usize) -> Vec<usize> {
    if total == 0 || n == 0 {
        return Vec::new();
    }

    let n = n.min(total);
    (0..n).map(|k| (k * total) / n).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The "quick check" sample. Every requirement here is load-bearing: a
    /// duplicate or out-of-range index would produce a run that cannot be
    /// created, because (run_id, position) is a primary key.
    #[test]
    fn spread_returns_exactly_n_distinct_in_range_indices() {
        for total in [1_usize, 2, 7, 20, 68, 106] {
            for n in 1..=total {
                let picks = spread(total, n);

                assert_eq!(picks.len(), n, "total={total} n={n}");
                assert!(picks.iter().all(|p| *p < total), "out of range: {picks:?}");

                let mut sorted = picks.clone();
                sorted.dedup();
                assert_eq!(sorted.len(), n, "duplicate index in {picks:?}");
            }
        }
    }

    /// A run that covers the module should not be all from the first lesson.
    /// For module 5 that means lesson 22 must be reachable, not just lesson 1.
    #[test]
    fn spread_samples_across_the_whole_module() {
        let picks = spread(106, 20);

        assert!(picks.first().is_some_and(|p| *p < 10), "should start near the front");
        assert!(
            picks.last().is_some_and(|p| *p > 95),
            "should reach the end, got {:?}",
            picks.last()
        );
    }

    #[test]
    fn spread_clamps_a_request_larger_than_the_module() {
        assert_eq!(spread(5, 50).len(), 5);
        assert_eq!(spread(5, 5), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn spread_of_nothing_is_nothing() {
        assert!(spread(0, 10).is_empty());
        assert!(spread(10, 0).is_empty());
    }

    /// Deterministic, so two attempts at "20 questions" are comparable.
    #[test]
    fn spread_is_stable_for_the_same_inputs() {
        assert_eq!(spread(106, 20), spread(106, 20));
    }
}

/// Database-level tests for the one invariant that matters most: practice
/// must never leak into credit.
///
/// A wrong answer written into `progress` would silently mark the exercise
/// complete, because `is_exercise_completed` is a bare EXISTS over
/// `exercise_id`. The failure is invisible on screen and irreversible in the
/// learner's record, so it is pinned here rather than left to review.
#[cfg(test)]
mod db_tests {
    use sqlx::sqlite::SqlitePoolOptions;
    use sqlx::SqlitePool;

    use super::*;

    async fn pool() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .expect("in-memory sqlite");

        sqlx::migrate!("./migrations").run(&pool).await.expect("migrate");

        sqlx::query("INSERT INTO modules (id, slug, title, description, order_index) VALUES (1, 'm', 'M', 'd', 1)")
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query(
            "INSERT INTO exercises (id, module_id, exercise_type, prompt, correct_answer, explanation, xp_reward, order_index)
             VALUES (10, 1, 'mcq', 'q', '1', 'because', 25, 1)",
        )
        .execute(&pool)
        .await
        .unwrap();

        pool
    }

    /// Rows in `progress` — the credit table practice must never touch.
    async fn progress_rows(pool: &SqlitePool) -> i64 {
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM progress")
            .fetch_one(pool)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn a_correct_practice_answer_awards_no_xp() {
        let pool = pool().await;

        let run = create_run(&pool, 1, "test run", &[10]).await.unwrap();
        record_answer(&pool, run.id, 10, "1", true).await.unwrap();

        let stored: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM practice_answers WHERE run_id = ?")
            .bind(run.id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(stored, 1, "the attempt itself must be recorded");

        assert_eq!(
            progress_rows(&pool).await,
            0,
            "practice must not write to progress, even for a right answer"
        );
    }

    #[tokio::test]
    async fn a_wrong_practice_answer_does_not_complete_the_exercise() {
        let pool = pool().await;

        let run = create_run(&pool, 1, "test run", &[10]).await.unwrap();
        record_answer(&pool, run.id, 10, "0", false).await.unwrap();

        assert!(
            !crate::services::progress::is_exercise_completed(&pool, 10)
                .await
                .unwrap(),
            "a wrong practice answer must not mark the exercise complete"
        );
        assert_eq!(progress_rows(&pool).await, 0);
    }

    /// The control: the standalone exercise flow must keep behaving as it did.
    /// If this regresses, practice has either broken XP or replaced it.
    #[tokio::test]
    async fn the_standalone_flow_still_awards_xp() {
        let pool = pool().await;

        let xp = crate::services::progress::record_exercise_result(&pool, 10, true)
            .await
            .unwrap();

        assert_eq!(xp, 25);
        assert_eq!(progress_rows(&pool).await, 1);
        assert!(crate::services::progress::is_exercise_completed(&pool, 10)
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn a_wrong_standalone_answer_awards_nothing() {
        let pool = pool().await;

        let xp = crate::services::progress::record_exercise_result(&pool, 10, false)
            .await
            .unwrap();

        assert_eq!(xp, 0);
        assert_eq!(progress_rows(&pool).await, 0);
    }

    #[tokio::test]
    async fn re_answering_updates_in_place() {
        let pool = pool().await;

        let run = create_run(&pool, 1, "test run", &[10]).await.unwrap();
        record_answer(&pool, run.id, 10, "0", false).await.unwrap();
        record_answer(&pool, run.id, 10, "1", true).await.unwrap();

        let (rows, correct): (i64, bool) =
            sqlx::query_as("SELECT COUNT(*), MAX(correct != 0) FROM practice_answers WHERE run_id = ?")
                .bind(run.id)
                .fetch_one(&pool)
                .await
                .unwrap();

        assert_eq!(rows, 1, "re-answering must upsert, not duplicate");
        assert!(correct, "the latest answer wins");
    }

    #[tokio::test]
    async fn open_ended_kinds_are_excluded_from_runs() {
        let pool = pool().await;

        sqlx::query(
            "INSERT INTO exercises (id, module_id, exercise_type, prompt, correct_answer, explanation, xp_reward, order_index)
             VALUES (11, 1, 'challenge', 'c', '0', 'b', 100, 2)",
        )
        .execute(&pool)
        .await
        .unwrap();

        let gradable = gradable_exercises(&pool, 1).await.unwrap();
        assert_eq!(gradable.len(), 1, "the challenge must not be offered");
        assert_eq!(gradable[0].id, 10);
    }

    #[tokio::test]
    async fn past_state_reflects_the_most_recent_answer() {
        let pool = pool().await;

        let first = create_run(&pool, 1, "first", &[10]).await.unwrap();
        record_answer(&pool, first.id, 10, "0", false).await.unwrap();
        assert_eq!(past_states(&pool, 1).await.unwrap(), vec![(10, AttemptState::Incorrect)]);

        // Getting it right on a later run should stop it looking wrong in the
        // picker — otherwise revision is aimed at questions already understood.
        let second = create_run(&pool, 1, "second", &[10]).await.unwrap();
        record_answer(&pool, second.id, 10, "1", true).await.unwrap();
        assert_eq!(past_states(&pool, 1).await.unwrap(), vec![(10, AttemptState::Correct)]);
    }

    #[tokio::test]
    async fn unfinished_run_ignores_finished_ones() {
        let pool = pool().await;

        let done = create_run(&pool, 1, "done", &[10]).await.unwrap();
        record_answer(&pool, done.id, 10, "1", true).await.unwrap();
        finish_run(&pool, done.id).await.unwrap();

        let live = create_run(&pool, 1, "live", &[10]).await.unwrap();

        assert_eq!(unfinished_run(&pool, 1).await.unwrap().unwrap().id, live.id);
    }
}
