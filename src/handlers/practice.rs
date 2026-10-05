use axum::extract::{Path, State};
use axum::response::{Html, IntoResponse, Redirect, Response};
use sqlx::SqlitePool;

use crate::error::{AppError, Result};
use crate::services::practice as service;

/// The run builder. The landing page for everything practice-related.
pub async fn builder(
    State(pool): State<SqlitePool>,
    Path(module_id): Path<i64>,
) -> Result<Html<String>> {
    let Some(data) = service::builder_data(&pool, module_id).await? else {
        return Err(AppError::NotFound(format!("module {module_id}")));
    };

    Ok(Html(crate::views::practice::builder(&data)))
}

/// Reads the raw form body and pulls out the three values the run builder posts.
///
/// Deliberately not `axum::extract::Form`: a hand-picked run sends one `q`
/// field per ticked checkbox, and `serde_urlencoded` cannot collect repeated keys
/// into a sequence — it rejects the body outright. There is a test below that
/// pins this, because getting it wrong fails silently rather than loudly.
struct StartForm {
    mode: String,
    n: Option<usize>,
    q: Vec<String>,
}

impl StartForm {
    fn parse(body: &str) -> Self {
        let pairs = form_pairs(body);

        let get = |key: &str| -> Option<String> {
            pairs
                .iter()
                .rev()
                .find(|(k, _)| k == key)
                .map(|(_, v)| v.clone())
        };

        let mode = get("mode").unwrap_or_else(|| "all".to_string());
        let n = get("n").and_then(|v| v.parse::<usize>().ok());

        // Every `q`, in submission order.
        let q: Vec<String> = pairs
            .iter()
            .filter(|(k, _)| k == "q")
            .map(|(_, v)| v.clone())
            .collect();

        Self { mode, n, q }
    }
}

/// Splits `a=1&b=2` into pairs, percent-decoding both halves.
///
/// Small enough to keep here rather than take a dependency, and it keeps the
/// `Vec` behaviour the picker depends on.
fn form_pairs(body: &str) -> Vec<(String, String)> {
    body.split('&')
        .filter(|part| !part.is_empty())
        .map(|part| match part.split_once('=') {
            Some((k, v)) => (percent_decode(k), percent_decode(v)),
            None => (percent_decode(part), String::new()),
        })
        .collect()
}

fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b'%' if i + 2 < bytes.len() => {
                let hex = &input[i + 1..i + 3];
                match u8::from_str_radix(hex, 16) {
                    Ok(byte) => {
                        out.push(byte);
                        i += 3;
                    }
                    Err(_) => {
                        out.push(bytes[i]);
                        i += 1;
                    }
                }
            }
            other => {
                out.push(other);
                i += 1;
            }
        }
    }

    String::from_utf8_lossy(&out).into_owned()
}

pub async fn start(
    State(pool): State<SqlitePool>,
    Path(module_id): Path<i64>,
    body: axum::body::Bytes,
) -> Result<Redirect> {
    let form = StartForm::parse(&String::from_utf8_lossy(&body));

    let exercises = service::gradable_exercises(&pool, module_id).await?;

    if exercises.is_empty() {
        return Err(AppError::NotFound(format!(
            "no practice exercises in module {module_id}"
        )));
    }

    let total = exercises.len();

    let picks: Vec<usize> = match form.mode.as_str() {
        "all" => (0..total).collect(),
        "count" => {
            let n = form.n.unwrap_or(20).clamp(1, total);
            service::spread(total, n)
        }
        "custom" => {
            let picked = parse_selection(&form.q, total);
            if picked.is_empty() {
                return Ok(Redirect::to(&format!("/practice/{module_id}")));
            }
            picked
        }
        _ => (0..total).collect(),
    };

    // De-duplicate defensively: a duplicate would break the (run_id, position)
    // primary key, and that surfaces as a 500 rather than anything actionable.
    let mut seen = std::collections::HashSet::new();
    let ids: Vec<i64> = picks
        .iter()
        .filter(|p| seen.insert(**p))
        .map(|p| exercises[*p].id)
        .collect();

    if ids.is_empty() {
        return Ok(Redirect::to(&format!("/practice/{module_id}")));
    }

    let label = match form.mode.as_str() {
        "all" => format!("All {total} questions"),
        "count" => format!("{} questions", ids.len()),
        _ => format!("{} chosen questions", ids.len()),
    };

    let run = service::create_run(&pool, module_id, &label, &ids).await?;

    Ok(Redirect::to(&format!("/practice/run/{}", run.id)))
}

/// Converts the picker's 1-based question numbers into 0-based indices.
///
/// Out-of-range and unparseable entries are dropped rather than rejected, so a
/// stale bookmark or a hand-edited form cannot produce a 500. The caller's
/// empty-result redirect is the user-visible outcome.
fn parse_selection(raw: &[String], total: usize) -> Vec<usize> {
    let mut out: Vec<usize> = Vec::new();

    for part in raw.iter().flat_map(|s| s.split(',')) {
        let Ok(n) = part.trim().parse::<usize>() else {
            continue;
        };

        if n == 0 || n > total {
            continue;
        }

        out.push(n - 1);
    }

    out
}

/// `/practice/run/:id` — send them to the first question they have not answered.
pub async fn run_start(
    State(pool): State<SqlitePool>,
    Path(run_id): Path<i64>,
) -> Result<Redirect> {
    let Some(data) = service::run_page_data(&pool, run_id).await? else {
        return Err(AppError::NotFound(format!("run {run_id}")));
    };

    if data.items.is_empty() {
        return Err(AppError::NotFound(format!("run {run_id}")));
    }

    if data.run.is_finished() {
        return Ok(Redirect::to(&format!("/practice/run/{run_id}/results")));
    }

    let first_unanswered = data
        .items
        .iter()
        .find(|item| !data.answers.iter().any(|(id, _, _)| *id == item.exercise.id))
        .map(|item| item.position)
        .unwrap_or(data.items[0].position);

    Ok(Redirect::to(&format!("/practice/run/{run_id}/{first_unanswered}")))
}

pub async fn question(
    State(pool): State<SqlitePool>,
    Path((run_id, position)): Path<(i64, i64)>,
) -> Result<Response> {
    let Some(data) = service::run_page_data(&pool, run_id).await? else {
        return Err(AppError::NotFound(format!("run {run_id}")));
    };

    if data.items.is_empty() {
        return Err(AppError::NotFound(format!("run {run_id}")));
    }

    if data.run.is_finished() {
        return Ok(Redirect::to(&format!("/practice/run/{run_id}/results")).into_response());
    }

    let Some(item) = data.items.iter().find(|i| i.position == position) else {
        return Err(AppError::NotFound(format!("question {position} in run {run_id}")));
    };

    let answered = data
        .answers
        .iter()
        .find(|(id, _, _)| *id == item.exercise.id)
        .map(|(_, correct, _)| *correct);

    Ok(Html(crate::views::practice::question(
        &data,
        &item.exercise,
        position,
        answered,
    ))
    .into_response())
}

#[derive(Debug, serde::Deserialize)]
pub struct AnswerForm {
    pub answer: Option<String>,
}

/// Grade one question, then advance. The last question finishes the run.
pub async fn answer(
    State(pool): State<SqlitePool>,
    Path((run_id, position)): Path<(i64, i64)>,
    axum::extract::Form(form): axum::extract::Form<AnswerForm>,
) -> Result<Response> {
    let Some(data) = service::run_page_data(&pool, run_id).await? else {
        return Err(AppError::NotFound(format!("run {run_id}")));
    };

    let Some(item) = data.items.iter().find(|i| i.position == position) else {
        return Err(AppError::NotFound(format!("question {position} in run {run_id}")));
    };

    let exercise = &item.exercise;

    // Unanswered is treated as incorrect rather than skipped, so a blank
    // submission still records an attempt and still shows up in the results.
    let given = form.answer.unwrap_or_default();
    let correct = given == exercise.correct_answer;

    service::record_answer(&pool, run_id, exercise.id, &given, correct).await?;

    let is_last = position >= data.items.len() as i64;

    if is_last {
        service::finish_run(&pool, run_id).await?;
        return Ok(Redirect::to(&format!("/practice/run/{run_id}/results")).into_response());
    }

    // Shown on the same page, alongside the feedback, rather than bouncing to
    // the next question — the learner needs a moment to read why.
    Ok(Html(crate::views::practice::answered(
        &data,
        &item.exercise,
        position,
        &given,
        correct,
    ))
    .into_response())
}

pub async fn results(
    State(pool): State<SqlitePool>,
    Path(run_id): Path<i64>,
) -> Result<Html<String>> {
    let Some(data) = service::results_data(&pool, run_id).await? else {
        return Err(AppError::NotFound(format!("run {run_id}")));
    };

    Ok(Html(crate::views::practice::results(&data)))
}

pub async fn restart(
    State(pool): State<SqlitePool>,
    Path(run_id): Path<i64>,
) -> Result<Redirect> {
    let Some(run) = service::run_by_id(&pool, run_id).await? else {
        return Err(AppError::NotFound(format!("run {run_id}")));
    };

    // The same questions again, as a fresh run. The old one stays in history.
    let ids: Vec<i64> = service::run_items(&pool, run_id)
        .await?
        .iter()
        .map(|i| i.exercise.id)
        .collect();

    if ids.is_empty() {
        service::delete_run(&pool, run_id).await?;
        return Ok(Redirect::to(&format!("/practice/{}", run.module_id)));
    }

    let fresh = service::create_run(&pool, run.module_id, &run.label, &ids).await?;

    Ok(Redirect::to(&format!("/practice/run/{}", fresh.id)))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A hand-picked run sends one `q` field per ticked checkbox, so the body
    /// arrives as `mode=custom&q=3&q=7&q=12` — repeated keys, not one joined
    /// value. This is load-bearing: this used to go through
    /// `serde_urlencoded`, which rejects repeated keys outright
    /// (`invalid type: string "3", expected a sequence`) and would have made
    /// every hand-picked run silently select nothing.
    #[test]
    fn start_form_reads_repeated_question_keys() {
        let form = StartForm::parse("mode=custom&q=3&q=7&q=12");

        assert_eq!(form.mode, "custom");
        assert_eq!(form.q, vec!["3", "7", "12"]);
    }

    /// `mode=all` and `mode=count` post no `q` at all.
    #[test]
    fn start_form_tolerates_a_missing_selection() {
        let all = StartForm::parse("mode=all");
        assert_eq!(all.mode, "all");
        assert!(all.q.is_empty());

        let count = StartForm::parse("mode=count&n=20");
        assert_eq!(count.n, Some(20));
        assert!(count.q.is_empty());
    }

    #[test]
    fn start_form_defaults_to_everything_when_mode_is_missing() {
        let form = StartForm::parse("");
        assert_eq!(form.mode, "all");
        assert!(form.q.is_empty());
    }

    #[test]
    fn form_pairs_percent_decodes_both_halves() {
        let pairs = form_pairs("mode=custom&q=3&note=a%20b&plus=c+d");
        assert_eq!(pairs[0], ("mode".to_string(), "custom".to_string()));
        assert_eq!(pairs[1], ("q".to_string(), "3".to_string()));
        assert_eq!(pairs[2], ("note".to_string(), "a b".to_string()));
        assert_eq!(pairs[3], ("plus".to_string(), "c d".to_string()));
    }

    /// A malformed percent escape must not panic or drop the field.
    #[test]
    fn form_pairs_survives_malformed_escapes() {
        let pairs = form_pairs("q=%zz&tail");
        assert_eq!(pairs[0], ("q".to_string(), "%zz".to_string()));
        assert_eq!(pairs[1], ("tail".to_string(), String::new()));
    }

    fn picks(values: &[&str]) -> Vec<String> {
        values.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn selection_parser_converts_1_based_input_to_indices() {
        assert_eq!(parse_selection(&picks(&["1", "5", "9"]), 12), vec![0, 4, 8]);
    }

    /// A stale bookmark or hand-edited form must not become a 500.
    #[test]
    fn selection_parser_drops_garbage_instead_of_failing() {
        assert_eq!(
            parse_selection(&picks(&["1", " abc ", "99", "0", "3"]), 5),
            vec![0, 2]
        );
        assert!(parse_selection(&picks(&[]), 5).is_empty());
        assert!(parse_selection(&picks(&["nonsense"]), 5).is_empty());
    }
}
