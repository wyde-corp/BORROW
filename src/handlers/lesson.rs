use axum::extract::{Path, State};
use axum::response::{Html, Redirect};
use sqlx::SqlitePool;

use crate::error::{AppError, Result};
use crate::models::Lesson;
use crate::services::progress as service;

pub async fn show(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Html<String>> {
    let data = service::lesson_page_data(&pool, id).await?;

    if data.lesson.is_none() {
        return Err(AppError::NotFound(format!("lesson {id}")));
    }

    Ok(Html(crate::views::lesson::show(&data)))
}

pub async fn complete(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Redirect> {
    let lesson: Option<Lesson> = service::lesson_by_id(&pool, id).await?;

    let Some(lesson) = lesson else {
        return Err(AppError::NotFound(format!("lesson {id}")));
    };

    service::mark_lesson_complete(&pool, id).await?;

    Ok(Redirect::to(&format!("/module/{}", lesson.module_id)))
}
