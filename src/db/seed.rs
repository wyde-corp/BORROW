use anyhow::Result;
use sqlx::SqlitePool;

use crate::content;

pub async fn run_if_empty(pool: &SqlitePool) -> Result<()> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM modules")
        .fetch_one(pool)
        .await?;

    if count > 0 {
        return Ok(());
    }

    for seed in content::all() {
        sqlx::query("INSERT INTO modules (id, slug, title, description, order_index) VALUES (?, ?, ?, ?, ?)")
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
    }

    Ok(())
}
