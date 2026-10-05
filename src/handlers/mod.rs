pub mod api;
pub mod dashboard;
pub mod exercise;
pub mod home;
pub mod lesson;
pub mod module;
pub mod playground;
pub mod practice;
pub mod progress;
pub mod project;

use axum::response::{Html, Redirect};

use crate::error::AppError;

pub async fn not_found() -> Result<Html<String>, AppError> {
    Err(AppError::NotFound("page".to_string()))
}

pub async fn method_not_allowed() -> Redirect {
    Redirect::to("/dashboard")
}
