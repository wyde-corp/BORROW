use anyhow::Result;
use sqlx::SqlitePool;

use crate::content;
use crate::content::ModuleSeed;

/// Brings the database's content up to date with the code.
///
/// This used to seed only when the `modules` table was completely empty, which
/// meant a module authored *after* the first run never appeared: the whole
/// curriculum is seeded as placeholder rows up front, so `Borrowing` sat there
/// with zero lessons and the dashboard read "Content coming soon" forever. The
/// only way to see it was to delete the database, which also discarded progress.
///
/// So there are two paths. An empty database is seeded in full, as before. A
/// database that already has modules gets the modules whose content is missing
/// filled in — a module with no lessons that the code now has lessons for.
/// Existing modules are never rewritten and progress is never touched; the only
/// writes are to modules that have no content to lose.
pub async fn ensure_content(pool: &SqlitePool) -> Result<()> {
    let modules: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM modules")
        .fetch_one(pool)
        .await?;

    if modules == 0 {
        for seed in content::all() {
            insert_module(pool, &seed).await?;
        }
        return Ok(());
    }

    for seed in content::all() {
        if seed.lessons.is_empty() {
            continue;
        }

        let lessons: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM lessons WHERE module_id = ?")
            .bind(seed.module.id)
            .fetch_one(pool)
            .await?;

        if lessons == 0 {
            tracing::info!(module = %seed.module.slug, "content was missing, seeding it");

            // A placeholder module carries no lessons or exercises, but it does
            // carry a stub "Coming soon" project. Clear any children that are
            // there so the authored set is the only set. Lessons were just
            // counted as zero, so nothing of value is removed.
            sqlx::query("DELETE FROM exercises WHERE module_id = ?")
                .bind(seed.module.id)
                .execute(pool)
                .await?;
            sqlx::query("DELETE FROM projects WHERE module_id = ?")
                .bind(seed.module.id)
                .execute(pool)
                .await?;

            insert_module(pool, &seed).await?;
        }
    }

    Ok(())
}

async fn insert_module(pool: &SqlitePool, seed: &ModuleSeed) -> Result<()> {
    // Upsert the module row rather than a plain INSERT: on the fill-in path the
    // placeholder row already exists, and its generic description should be
    // replaced by the authored one. `id` is stable, so progress keyed on the
    // module is unaffected.
    sqlx::query(
        "INSERT INTO modules (id, slug, title, description, order_index) VALUES (?, ?, ?, ?, ?) \
         ON CONFLICT(id) DO UPDATE SET slug = excluded.slug, title = excluded.title, \
         description = excluded.description, order_index = excluded.order_index",
    )
    .bind(seed.module.id)
    .bind(&seed.module.slug)
    .bind(&seed.module.title)
    .bind(&seed.module.description)
    .bind(seed.module.order_index)
    .execute(pool)
    .await?;

    for lesson in &seed.lessons {
        sqlx::query("INSERT INTO lessons (id, module_id, title, content, order_index) VALUES (?, ?, ?, ?, ?)")
            .bind(lesson.id)
            .bind(lesson.module_id)
            .bind(&lesson.title)
            .bind(&lesson.content)
            .bind(lesson.order_index)
            .execute(pool)
            .await?;
    }

    for exercise in &seed.exercises {
        sqlx::query(
            "INSERT INTO exercises (id, module_id, lesson_id, exercise_type, prompt, code, code_b, options, correct_answer, explanation, xp_reward, order_index) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(exercise.id)
        .bind(exercise.module_id)
        .bind(exercise.lesson_id)
        .bind(&exercise.exercise_type)
        .bind(&exercise.prompt)
        .bind(&exercise.code)
        .bind(&exercise.code_b)
        .bind(&exercise.options)
        .bind(&exercise.correct_answer)
        .bind(&exercise.explanation)
        .bind(exercise.xp_reward)
        .bind(exercise.order_index)
        .execute(pool)
        .await?;
    }

    for project in &seed.projects {
        sqlx::query("INSERT INTO projects (id, module_id, title, description, requirements, order_index) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(project.id)
            .bind(project.module_id)
            .bind(&project.title)
            .bind(&project.description)
            .bind(&project.requirements)
            .bind(project.order_index)
            .execute(pool)
            .await?;
    }

    tracing::info!(
        module = %seed.module.slug,
        lessons = seed.lessons.len(),
        exercises = seed.exercises.len(),
        projects = seed.projects.len(),
        "seeded module"
    );

    Ok(())
}
