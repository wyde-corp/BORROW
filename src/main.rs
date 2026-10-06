mod config;
mod content;
mod db;
mod error;
mod handlers;
mod models;
mod services;
mod views;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let pool = db::init().await?;
    let address = config::address();

    let app = Router::new()
        .route("/", get(handlers::home::index))
        .route("/dashboard", get(handlers::dashboard::show))
        .route("/module/:id", get(handlers::module::show))
        .route("/lesson/:id", get(handlers::lesson::show))
        .route("/lesson/:id/complete", post(handlers::lesson::complete))
        .route("/exercise/:id", get(handlers::exercise::show))
        .route("/exercise/:id/submit", post(handlers::exercise::submit))
        .route("/project/:id", get(handlers::project::show))
        .route("/practice/:module_id", get(handlers::practice::builder))
        .route(
            "/practice/:module_id/start",
            post(handlers::practice::start),
        )
        .route("/practice/run/:run_id", get(handlers::practice::run_start))
        .route(
            "/practice/run/:run_id/:position",
            get(handlers::practice::question),
        )
        .route(
            "/practice/run/:run_id/:position",
            post(handlers::practice::answer),
        )
        .route(
            "/practice/run/:run_id/results",
            get(handlers::practice::results),
        )
        .route(
            "/practice/run/:run_id/restart",
            post(handlers::practice::restart),
        )
        .route("/playground/run", post(handlers::playground::run))
        .route("/playground", get(handlers::playground::page))
        .route("/progress", get(handlers::progress::show))
        .route("/api/curriculum", get(handlers::api::curriculum))
        .route("/api/progress", get(handlers::api::stats))
        .nest_service("/static", ServeDir::new("static"))
        .fallback(handlers::not_found)
        .method_not_allowed_fallback(handlers::method_not_allowed)
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(address).await?;

    tracing::info!("BORROW_ listening on http://{}", address);

    // The playground runs code with the server's own privileges, so being bound
    // to a reachable interface is worth a loud boot line rather than a silent
    // 403 the first time somebody presses RUN.
    if config::playground_allowed() {
        tracing::info!("playground enabled (loopback bind)");
    } else {
        tracing::warn!(
            "playground DISABLED — the server is not bound to loopback, so learner code \
             would run with the server's privileges for anyone who can reach it. \
             Set BORROW_ALLOW_PLAYGROUND=1 to override."
        );
    }

    axum::serve(listener, app).await?;

    Ok(())
}
