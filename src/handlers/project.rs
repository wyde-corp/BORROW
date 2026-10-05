use axum::extract::{Path, State};
use axum::response::Html;
use sqlx::SqlitePool;

use crate::error::{AppError, Result};
use crate::services::progress as service;
use crate::views;

pub async fn show(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Html<String>> {
    let project = service::project_by_id(&pool, id).await?;

    let Some(project) = project else {
        return Err(AppError::NotFound(format!("project {id}")));
    };

    let module = service::module_by_id(&pool, project.module_id).await?;

    // A project unlocks when every lesson in its own module is done. Both
    // sides of the comparison are scoped to the module, so progress
    // elsewhere cannot unlock this.
    let total = service::lessons_for_module(&pool, project.module_id).await?.len() as i64;
    let done = service::completed_lesson_count_for_module(&pool, project.module_id).await?;
    let unlocked = total > 0 && done >= total;

    Ok(Html(views::project::show(
        &project,
        module.as_ref(),
        unlocked,
    )))
}
