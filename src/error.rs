use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    Anyhow(anyhow::Error),
    NotFound(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Database(e) => write!(f, "database error: {e}"),
            AppError::Anyhow(e) => write!(f, "internal error: {e}"),
            AppError::NotFound(what) => write!(f, "not found: {what}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Anyhow(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::Database(_) | AppError::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
        };

        if status == StatusCode::INTERNAL_SERVER_ERROR {
            tracing::error!(error = %self, "request failed");
        }

        let title = match status {
            StatusCode::NOT_FOUND => "NOT FOUND",
            _ => "SOMETHING WENT WRONG",
        };

        let body = format!(
            r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>BORROW_ — {title}</title>
    <link rel="stylesheet" href="/static/style.css">
</head>
<body>
    <nav class="navbar">
        <a href="/" class="logo">BORROW_</a>
        <div class="nav-links">
            <a href="/dashboard">Learn</a>
        </div>
    </nav>
    <main class="content">
        <div class="error-panel">
            <span class="module-label">{title}</span>
            <h1>Something went wrong</h1>
            <p>We couldn't load this page.</p>
            <a href="/dashboard" class="btn btn-primary">TRY AGAIN</a>
        </div>
    </main>
</body>
</html>"##
        );

        (status, axum::response::Html(body)).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
