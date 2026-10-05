use axum::extract::State;

use crate::error::Result;
use crate::models::Module;
use crate::services::progress as service;

pub async fn curriculum(State(pool): State<sqlx::SqlitePool>) -> Result<axum::Json<Vec<Module>>> {
    let modules = service::all_modules(&pool).await?;
    Ok(axum::Json(modules))
}

pub async fn stats(State(pool): State<sqlx::SqlitePool>) -> Result<axum::Json<serde_json::Value>> {
    let data = service::progress_summary(&pool).await?;

    Ok(axum::Json(serde_json::json!({
        "total_xp": data.total_xp,
        "completed_lessons": data.completed_lessons,
        "total_lessons": data.total_lessons,
        "completed_exercises": data.completed_exercises,
        "total_modules": data.total_modules,
    })))
}
