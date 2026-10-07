use anyhow::Result;
use sqlx::SqlitePool;

use crate::models::{Exercise, Lesson, Module, Project};

#[derive(Debug, Clone)]
pub struct DashboardData {
    pub modules: Vec<Module>,
    pub lessons_per_module: Vec<Vec<Lesson>>,
    pub completed_lessons: Vec<i64>,
    pub total_lessons: i64,
    pub total_xp: i64,
    pub first_incomplete_lesson: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct ModulePageData {
    pub module: Option<Module>,
    pub lessons: Vec<Lesson>,
    pub exercises: Vec<Exercise>,
    pub projects: Vec<Project>,
    pub completed_lessons: Vec<i64>,
    pub completed_exercises: Vec<i64>,
    pub prev_module: Option<Module>,
    pub next_module: Option<Module>,
    /// True when the previous module has not been started yet. The module is
    /// still readable — this only drives the advisory notice, not access.
    pub is_locked: bool,
}

#[derive(Debug, Clone)]
pub struct LessonPageData {
    pub lesson: Option<Lesson>,
    pub module: Option<Module>,
    pub prev_lesson: Option<Lesson>,
    pub next_lesson: Option<Lesson>,
    pub total_lessons: i64,
    pub current_order: i64,
    pub is_completed: bool,
}

#[derive(Debug, Clone)]
pub struct ProgressSummary {
    pub total_modules: i64,
    pub completed_lessons: i64,
    pub total_lessons: i64,
    pub completed_exercises: i64,
    pub total_xp: i64,
    pub modules: Vec<Module>,
}

pub async fn all_modules(pool: &SqlitePool) -> Result<Vec<Module>> {
    Ok(
        sqlx::query_as::<_, Module>("SELECT * FROM modules ORDER BY order_index")
            .fetch_all(pool)
            .await?,
    )
}

pub async fn module_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Module>> {
    Ok(
        sqlx::query_as::<_, Module>("SELECT * FROM modules WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?,
    )
}

pub async fn lesson_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Lesson>> {
    Ok(
        sqlx::query_as::<_, Lesson>("SELECT * FROM lessons WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?,
    )
}

pub async fn lessons_for_module(pool: &SqlitePool, module_id: i64) -> Result<Vec<Lesson>> {
    Ok(sqlx::query_as::<_, Lesson>(
        "SELECT * FROM lessons WHERE module_id = ? ORDER BY order_index",
    )
    .bind(module_id)
    .fetch_all(pool)
    .await?)
}

pub async fn exercises_for_module(pool: &SqlitePool, module_id: i64) -> Result<Vec<Exercise>> {
    Ok(sqlx::query_as::<_, Exercise>(
        "SELECT * FROM exercises WHERE module_id = ? ORDER BY order_index",
    )
    .bind(module_id)
    .fetch_all(pool)
    .await?)
}

pub async fn projects_for_module(pool: &SqlitePool, module_id: i64) -> Result<Vec<Project>> {
    Ok(sqlx::query_as::<_, Project>(
        "SELECT * FROM projects WHERE module_id = ? ORDER BY order_index",
    )
    .bind(module_id)
    .fetch_all(pool)
    .await?)
}

pub async fn exercise_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Exercise>> {
    Ok(
        sqlx::query_as::<_, Exercise>("SELECT * FROM exercises WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?,
    )
}

pub async fn project_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Project>> {
    Ok(
        sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?,
    )
}

pub async fn next_exercise_id(pool: &SqlitePool, exercise_id: i64) -> Result<Option<i64>> {
    Ok(
        sqlx::query_scalar::<_, i64>("SELECT id FROM exercises WHERE id > ? ORDER BY id LIMIT 1")
            .bind(exercise_id)
            .fetch_optional(pool)
            .await?,
    )
}

pub async fn completed_lesson_ids(pool: &SqlitePool) -> Result<Vec<i64>> {
    Ok(
        sqlx::query_scalar::<_, i64>("SELECT lesson_id FROM progress WHERE lesson_id IS NOT NULL")
            .fetch_all(pool)
            .await?,
    )
}

/// How many of a module's own lessons are complete. Scoped to the module —
/// a global count would unlock the wrong thing.
pub async fn completed_lesson_count_for_module(pool: &SqlitePool, module_id: i64) -> Result<i64> {
    Ok(sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM progress p JOIN lessons l ON l.id = p.lesson_id WHERE l.module_id = ?",
    )
    .bind(module_id)
    .fetch_one(pool)
    .await?)
}

pub async fn completed_exercise_ids(pool: &SqlitePool) -> Result<Vec<i64>> {
    Ok(sqlx::query_scalar::<_, i64>(
        "SELECT exercise_id FROM progress WHERE exercise_id IS NOT NULL",
    )
    .fetch_all(pool)
    .await?)
}

pub async fn is_lesson_completed(pool: &SqlitePool, lesson_id: i64) -> Result<bool> {
    Ok(
        sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM progress WHERE lesson_id = ?)")
            .bind(lesson_id)
            .fetch_one(pool)
            .await?,
    )
}

pub async fn is_exercise_completed(pool: &SqlitePool, exercise_id: i64) -> Result<bool> {
    Ok(
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM progress WHERE exercise_id = ?)",
        )
        .bind(exercise_id)
        .fetch_one(pool)
        .await?,
    )
}

pub async fn mark_lesson_complete(pool: &SqlitePool, lesson_id: i64) -> Result<bool> {
    if is_lesson_completed(pool, lesson_id).await? {
        return Ok(false);
    }

    let module_id = sqlx::query_scalar::<_, i64>("SELECT module_id FROM lessons WHERE id = ?")
        .bind(lesson_id)
        .fetch_optional(pool)
        .await?;

    if let Some(module_id) = module_id {
        sqlx::query("INSERT INTO progress (module_id, lesson_id, xp_earned) VALUES (?, ?, ?)")
            .bind(module_id)
            .bind(lesson_id)
            .bind(LESSON_XP)
            .execute(pool)
            .await?;
    }

    Ok(true)
}

pub async fn record_exercise_result(
    pool: &SqlitePool,
    exercise_id: i64,
    correct: bool,
) -> Result<i64> {
    let exercise = exercise_by_id(pool, exercise_id).await?;

    let Some(exercise) = exercise else {
        return Ok(0);
    };

    if !correct {
        return Ok(0);
    }

    if is_exercise_completed(pool, exercise_id).await? {
        return Ok(0);
    }

    sqlx::query("INSERT INTO progress (module_id, exercise_id, xp_earned) VALUES (?, ?, ?)")
        .bind(exercise.module_id)
        .bind(exercise.id)
        .bind(exercise.xp_reward)
        .execute(pool)
        .await?;

    Ok(exercise.xp_reward)
}

pub async fn total_xp(pool: &SqlitePool) -> Result<i64> {
    Ok(
        sqlx::query_scalar::<_, i64>("SELECT COALESCE(SUM(xp_earned), 0) FROM progress")
            .fetch_one(pool)
            .await?,
    )
}

pub const LESSON_XP: i64 = 10;

pub async fn dashboard_data(pool: &SqlitePool) -> Result<DashboardData> {
    let modules = all_modules(pool).await?;

    let mut lessons_per_module = Vec::with_capacity(modules.len());
    for m in &modules {
        lessons_per_module.push(lessons_for_module(pool, m.id).await?);
    }

    let completed_lessons = completed_lesson_ids(pool).await?;

    let total_lessons = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM lessons")
        .fetch_one(pool)
        .await?;

    let first_incomplete_lesson = sqlx::query_scalar::<_, i64>(
        "SELECT id FROM lessons WHERE id NOT IN (SELECT lesson_id FROM progress WHERE lesson_id IS NOT NULL) ORDER BY module_id, order_index LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;

    Ok(DashboardData {
        modules,
        lessons_per_module,
        completed_lessons,
        total_lessons,
        total_xp: total_xp(pool).await?,
        first_incomplete_lesson,
    })
}

pub async fn module_page_data(pool: &SqlitePool, module_id: i64) -> Result<ModulePageData> {
    let module = module_by_id(pool, module_id).await?;

    let lessons = lessons_for_module(pool, module_id).await?;
    let exercises = exercises_for_module(pool, module_id).await?;
    let projects = projects_for_module(pool, module_id).await?;

    let prev_module = sqlx::query_as::<_, Module>(
        "SELECT * FROM modules WHERE order_index < (SELECT order_index FROM modules WHERE id = ?) ORDER BY order_index DESC LIMIT 1",
    )
    .bind(module_id)
    .fetch_optional(pool)
    .await?;

    let next_module = sqlx::query_as::<_, Module>(
        "SELECT * FROM modules WHERE order_index > (SELECT order_index FROM modules WHERE id = ?) ORDER BY order_index LIMIT 1",
    )
    .bind(module_id)
    .fetch_optional(pool)
    .await?;

    let completed_lessons = completed_lesson_ids(pool).await?;

    // A module is locked only while the module before it is completely
    // untouched. Completing it is never required to read ahead.
    let is_locked = match (&module, &prev_module) {
        (Some(m), Some(prev)) if m.order_index > 1 => {
            let prev_lesson_ids: Vec<i64> = lessons_for_module(pool, prev.id)
                .await?
                .into_iter()
                .map(|l| l.id)
                .collect();

            !completed_lessons
                .iter()
                .any(|id| prev_lesson_ids.contains(id))
        }
        _ => false,
    };

    Ok(ModulePageData {
        module,
        lessons,
        exercises,
        projects,
        completed_lessons,
        completed_exercises: completed_exercise_ids(pool).await?,
        prev_module,
        next_module,
        is_locked,
    })
}

pub async fn lesson_page_data(pool: &SqlitePool, lesson_id: i64) -> Result<LessonPageData> {
    let lesson = lesson_by_id(pool, lesson_id).await?;

    let module = match &lesson {
        Some(l) => module_by_id(pool, l.module_id).await?,
        None => None,
    };

    let (prev_lesson, next_lesson, total_lessons, current_order) = match &lesson {
        Some(l) => {
            let prev = sqlx::query_as::<_, Lesson>(
                "SELECT * FROM lessons WHERE module_id = ? AND order_index < ? ORDER BY order_index DESC LIMIT 1",
            )
            .bind(l.module_id)
            .bind(l.order_index)
            .fetch_optional(pool)
            .await?;

            let next = sqlx::query_as::<_, Lesson>(
                "SELECT * FROM lessons WHERE module_id = ? AND order_index > ? ORDER BY order_index LIMIT 1",
            )
            .bind(l.module_id)
            .bind(l.order_index)
            .fetch_optional(pool)
            .await?;

            let total =
                sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM lessons WHERE module_id = ?")
                    .bind(l.module_id)
                    .fetch_one(pool)
                    .await?;

            (prev, next, total, l.order_index)
        }
        None => (None, None, 0, 0),
    };

    Ok(LessonPageData {
        lesson,
        module,
        prev_lesson,
        next_lesson,
        total_lessons,
        current_order,
        is_completed: is_lesson_completed(pool, lesson_id).await?,
    })
}

pub async fn progress_summary(pool: &SqlitePool) -> Result<ProgressSummary> {
    let modules = all_modules(pool).await?;

    Ok(ProgressSummary {
        total_modules: modules.len() as i64,
        completed_lessons: completed_lesson_ids(pool).await?.len() as i64,
        total_lessons: sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM lessons")
            .fetch_one(pool)
            .await?,
        completed_exercises: completed_exercise_ids(pool).await?.len() as i64,
        total_xp: total_xp(pool).await?,
        modules,
    })
}
