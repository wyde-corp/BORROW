//! Exercises: parsing their payloads, grading them, and rendering the question.
//!
//! Grading is a deliberate copy of `handlers::exercise::submit` rather than a
//! call into it — that handler returns HTML — so the rule it encodes is
//! restated here: open-ended work cannot be graded, and submitting it counts as
//! completion instead, which is what earns the XP.

use crate::models::Exercise;
use crate::services::progress;
use gpui_kit::component::Theme;
use gpui_kit::*;

/// The exercise payload is `{"options": [...], "hints": [...]}` or `[]`.
fn payload(exercise: &Exercise) -> serde_json::Value {
    exercise
        .options
        .as_deref()
        .and_then(|raw| serde_json::from_str(raw).ok())
        .unwrap_or(serde_json::Value::Array(vec![]))
}

pub fn options_of(exercise: &Exercise) -> Vec<String> {
    payload(exercise)
        .get("options")
        .and_then(|v| v.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

pub fn hints_of(exercise: &Exercise) -> Vec<String> {
    payload(exercise)
        .get("hints")
        .and_then(|v| v.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

/// A checked answer and what it earned.
#[derive(Debug, Clone, Default)]
pub struct Submission {
    pub correct: bool,
    pub xp: i64,
    /// What the learner typed or picked, for showing back to them.
    pub shown_answer: String,
    /// Free-text explanation of the outcome, when neither ✓ nor ✗ says enough.
    pub message: String,
}

/// The grading rule, on its own.
///
/// Split out of [`submit`] because it is the rule the whole XP system rests on
/// and the only part worth testing without a database:
///
/// - a picked option is compared against `correct_answer`, which is the option
///   *index* as a string — the same value the site's radio buttons post;
/// - an open-ended exercise cannot be right or wrong, so it is recorded for
///   self-review, and `submit` turns `correct || is_open_ended` into completion;
/// - a choice left blank scores nothing rather than defaulting to an option.
pub fn grade(exercise: &Exercise, selected: Option<usize>, open_ended: bool) -> (bool, String) {
    let answer = selected.map(|i| i.to_string());

    match (answer, open_ended) {
        (Some(a), _) => (exercise.correct_answer == a, a),
        (None, true) => (false, "Submitted for self-review".to_string()),
        (None, false) => (false, String::new()),
    }
}

/// Grades an answer and records the result, returning what it was worth.
///
/// The `Some`/`None` arms are the web handler's. For an open-ended exercise
/// `selected` is always `None`, the answer is recorded as self-review, and the
/// completion signal — `correct || is_open_ended` — is what awards the XP.
pub fn submit(db: &crate::app_db::Db, exercise: &Exercise, selected: Option<usize>) -> Submission {
    let open_ended = exercise.is_open_ended();
    let (correct, shown_answer) = grade(exercise, selected, open_ended);

    let recorded = db.block_on(progress::record_exercise_result(
        db.pool(),
        exercise.id,
        correct || open_ended,
    ));

    let xp = match recorded {
        Ok(xp) => xp,
        // A failed write must not read as a correct answer, and the learner
        // should be told the result did not stick rather than seeing +XP that
        // will not be there next launch.
        Err(err) => {
            eprintln!("could not record exercise {}: {err}", exercise.id);
            return Submission {
                correct,
                xp: 0,
                shown_answer,
                message: "Your result could not be saved.".to_string(),
            };
        }
    };

    let message = if correct {
        String::new()
    } else if open_ended {
        "Recorded for self-review.".to_string()
    } else {
        String::new()
    };

    Submission {
        correct,
        xp,
        shown_answer,
        message,
    }
}

/// The question itself: prompt and code.
///
/// Hints live in `screen::AppView::hints_panel`, because revealing one needs to
/// write to the view — `question` has no handle to it and `exercise.rs` should
/// not depend on the screen module.
pub fn question(theme: &Theme, exercise: &Exercise) -> Div {
    let mut body = div()
        .flex()
        .flex_col()
        .gap_4()
        .child(prompt_block(theme, exercise));

    if let Some(code) = exercise
        .code
        .as_deref()
        .filter(|code| !code.trim().is_empty())
    {
        body = body.child(crate::components::code_panel(
            theme,
            "rust",
            code,
            crate::models::CodeStatus::Valid,
        ));
    }

    body
}

/// The prompt, split on its first newline if it carries an inline snippet.
fn prompt_block(theme: &Theme, exercise: &Exercise) -> Div {
    let prompt = exercise.prompt.trim();

    if let Some((first, rest)) = prompt.split_once('\n') {
        if !rest.trim().is_empty() {
            return div()
                .flex()
                .flex_col()
                .gap_3()
                .child(
                    div()
                        .text_size(px(13.5))
                        .text_color(theme.foreground)
                        .child(first.trim().to_string()),
                )
                .child(
                    div()
                        .rounded_md()
                        .border_1()
                        .border_color(theme.border)
                        .bg(theme.background)
                        .p_3()
                        .font_family(theme.mono_font_family.clone())
                        .text_size(px(12.5))
                        .text_color(theme.foreground)
                        .child(rest.trim().to_string()),
                );
        }
    }

    div()
        .text_size(px(13.5))
        .text_color(theme.foreground)
        .child(prompt.to_string())
}

#[cfg(test)]
mod tests {
    use super::grade;
    use crate::models::Exercise;

    fn exercise(kind: &str, correct_answer: &str) -> Exercise {
        Exercise {
            id: 1,
            module_id: 1,
            lesson_id: None,
            exercise_type: kind.to_string(),
            prompt: "What does this print?".to_string(),
            code: Some("fn main() { println!(\"1\"); }".to_string()),
            code_b: None,
            options: Some(r#"{"options":["2","1"],"hints":["read it line by line"]}"#.to_string()),
            correct_answer: correct_answer.to_string(),
            explanation: "It prints 1.".to_string(),
            xp_reward: 25,
            order_index: 1,
        }
    }

    /// The answer posted by the site's radios is the option index as a string,
    /// so that is what this compares — getting the shape wrong would make every
    /// correct choice grade as wrong.
    #[test]
    fn a_picked_index_scores_against_the_stored_index() {
        let item = exercise("mcq", "1");

        assert_eq!(grade(&item, Some(1), false), (true, "1".to_string()));
        assert_eq!(grade(&item, Some(0), false), (false, "0".to_string()));
    }

    /// Nothing chosen is not a default of the first option.
    #[test]
    fn a_blank_choice_scores_nothing() {
        let item = exercise("mcq", "1");
        assert_eq!(grade(&item, None, false), (false, String::new()));
    }

    /// Open-ended work cannot be graded, and self-review is what `submit`
    /// converts into a completion — which is where its XP comes from.
    #[test]
    fn open_ended_submissions_record_for_self_review() {
        let item = exercise("build_it", "anything");

        let (correct, shown) = grade(&item, None, true);
        assert!(!correct, "there is no right answer to score");
        assert_eq!(shown, "Submitted for self-review");

        // The rule submit applies: correct OR open-ended.
        assert!(correct || item.is_open_ended());
    }

    /// `challenge` and `build_it` are the open-ended types; a multiple-choice
    /// exercise must not be one, or it would earn XP without being graded.
    #[test]
    fn open_endedness_is_what_decides_completion() {
        let graded = exercise("mcq", "1");
        assert!(!graded.is_open_ended(), "mcq is graded, so not open-ended");

        let freeform = exercise("challenge", "n/a");
        assert!(freeform.is_open_ended());
    }
}
