use axum::extract::{Path, State};
use axum::response::Html;
use sqlx::SqlitePool;

use crate::error::{AppError, Result};
use crate::services::progress as service;
use crate::views;

#[derive(Debug, serde::Deserialize)]
pub struct ExerciseSubmit {
    /// Only choice-based exercises post this. Open-ended ones (challenge,
    /// build_it) post a `step_N` checkbox per requirement instead, so the
    /// field has to be optional or their forms are rejected with a 422.
    #[serde(default)]
    pub answer: Option<String>,
}

pub async fn show(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Html<String>> {
    let exercise = service::exercise_by_id(&pool, id).await?;

    let Some(exercise) = exercise else {
        return Err(AppError::NotFound(format!("exercise {id}")));
    };

    let next = match service::next_exercise_id(&pool, id).await? {
        Some(next_id) => service::exercise_by_id(&pool, next_id).await?,
        None => None,
    };

    Ok(Html(views::exercise::show(Some(&exercise), next.as_ref())))
}

pub async fn submit(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    axum::extract::Form(form): axum::extract::Form<ExerciseSubmit>,
) -> Result<Html<String>> {
    let exercise = service::exercise_by_id(&pool, id).await?;

    let Some(exercise) = exercise else {
        return Err(AppError::NotFound(format!("exercise {id}")));
    };

    // Open-ended work cannot be graded, so submitting it counts as completion
    // rather than correctness. Choice-based exercises still need a real answer.
    let is_open_ended = exercise.is_open_ended();

    let (shown_answer, correct) = match (&form.answer, is_open_ended) {
        (Some(a), _) => (a.clone(), exercise.correct_answer == *a),
        (None, true) => ("Submitted for self-review".to_string(), false),
        (None, false) => (String::new(), false),
    };

    // An open-ended submission is the completion signal, so it earns the XP.
    let xp = service::record_exercise_result(&pool, id, correct || is_open_ended).await?;

    let next = match service::next_exercise_id(&pool, id).await? {
        Some(next_id) => service::exercise_by_id(&pool, next_id).await?,
        None => None,
    };

    Ok(Html(views::exercise::result(
        Some(&exercise),
        &shown_answer,
        correct,
        xp,
        next.as_ref(),
    )))
}
