use axum::extract::{Path, State};
use axum::response::Html;
use sqlx::SqlitePool;

use crate::error::{AppError, Result};
use crate::services::progress as service;

pub async fn show(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Html<String>> {
    let data = service::module_page_data(&pool, id).await?;

    if data.module.is_none() {
        return Err(AppError::NotFound(format!("module {id}")));
    }

    Ok(Html(crate::views::module::show(&data)))
}
