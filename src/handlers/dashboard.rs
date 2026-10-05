use axum::extract::State;
use axum::response::Html;
use sqlx::SqlitePool;

use crate::error::Result;
use crate::services::progress as service;
use crate::views;

pub async fn show(State(pool): State<SqlitePool>) -> Result<Html<String>> {
    let data = service::dashboard_data(&pool).await?;
    Ok(Html(views::dashboard::show(&data)))
}
