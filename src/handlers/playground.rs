//! The RUN endpoint behind every TRY THIS button.
//!
//! Deliberately JSON-only: this is called by the editor, not by a form post, and
//! there is no server-rendered playground page. The code comes from the browser
//! in the request body, so nothing here reads the database.

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use sqlx::SqlitePool;

use crate::services::sandbox;

/// A cap on how much code we will compile in one request.
///
/// Every request is a compile the server agreed to perform, and a lesson paste
/// or a curious `1_000_000` can be arbitrarily large. Sized well above any
/// realistic lesson program, so ordinary content never hits it.
const MAX_CODE_BYTES: usize = 256 * 1024;

/// Compilation is a blocking subprocess. Handing it to `spawn_blocking` keeps
/// one learner's `loop {}` from stalling the executor for every other request
/// on the server.
pub async fn run(State(_pool): State<SqlitePool>, Json(req): Json<RunRequest>) -> Response {
    if !sandbox::is_enabled() {
        return (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({
                "status": "unavailable",
                "message": "The playground is turned off because this server is not bound to localhost.",
            })),
        )
            .into_response();
    }

    if !within_limit(&req.code) {
        return (
            StatusCode::PAYLOAD_TOO_LARGE,
            Json(serde_json::json!({
                "status": "unavailable",
                "message": "That is more code than the playground will compile at once.",
            })),
        )
            .into_response();
    }

    let compiled = tokio::task::spawn_blocking(move || sandbox::run(&req.code)).await;

    match compiled {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(err) => {
            // The blocking task itself failed, which means the process is in a
            // state we should not keep running code in. Report it as
            // unavailable rather than taking the request down with it.
            tracing::error!(%err, "sandbox task failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "status": "unavailable",
                    "message": "The playground hit an internal error. Restart the server and try again.",
                })),
            )
                .into_response()
        }
    }
}

#[derive(serde::Deserialize)]
pub struct RunRequest {
    pub code: String,
}

/// Enforced separately from the handler so it is a plain function of the code
/// rather than a branch buried in the request path — this is the thing worth
/// testing, not the surrounding plumbing.
fn within_limit(code: &str) -> bool {
    code.len() <= MAX_CODE_BYTES
}

#[cfg(test)]
mod tests {
    use super::{within_limit, MAX_CODE_BYTES};

    /// A pasted lesson panel must go through. This is the side of the cap that
    /// would quietly break the feature if it moved.
    #[test]
    fn a_realistic_lesson_program_is_accepted() {
        // The largest single panel in the course, generously padded.
        let program = "fn main() {\n    println!(\"x\");\n}\n".repeat(1_000);
        assert!(within_limit(&program), "{} bytes was refused", program.len());
    }

    /// A deliberately huge body must be refused, so a paste cannot turn into a
    /// long-running rustc. Asserted against the cap's real behaviour rather
    /// than against the constant, so tightening or loosening it stays honest.
    #[test]
    fn a_pathological_body_is_refused() {
        let huge = "a".repeat(MAX_CODE_BYTES + 1);
        assert!(!within_limit(&huge));
    }

    /// The boundary itself: a body exactly at the cap is still allowed, one byte
    /// over is not. An off-by-one here would reject the largest legitimate
    /// program while claiming to enforce the limit.
    #[test]
    fn the_cap_is_inclusive_at_its_boundary() {
        let exact = "a".repeat(MAX_CODE_BYTES);
        assert!(within_limit(&exact));
        assert!(!within_limit(&format!("{exact}a")));
    }
}
