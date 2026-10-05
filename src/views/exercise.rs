use super::components::{code_block, escape_html, CalloutTone};
use super::layout::{escape, page};
use crate::models::{CodeStatus, Exercise, LessonSection};

use serde_json::Value;

/// The badge text for an exercise kind.
///
/// Free-standing so the practice picker can label a row by format without
/// building a whole `ExerciseView`. `ExerciseView::badge` delegates here, so
/// there is still exactly one kind-to-label mapping in the codebase.
pub fn kind_label(kind: &str) -> &'static str {
    match kind {
        "predict_output" => "PREDICT THE OUTPUT",
        "fix_code" => "FIX THE CODE",
        "spot_difference" => "SPOT THE DIFFERENCE",
        "type_detective" => "TYPE DETECTIVE",
        "compiler_challenge" => "COMPILER CHALLENGE",
        "build_it" => "BUILD IT",
        "challenge" => "MODULE CHALLENGE",
        "fill_gap" => "FILL THE GAP",
        "signature_reading" => "READ THE SIGNATURE",
        "predict_ownership" => "PREDICT OWNERSHIP",
        "predict_compile" => "WILL IT COMPILE?",
        "memory_diagram" => "DRAW THE MEMORY",
        "trace_borrow" => "TRACE THE BORROW",
        "api_design" => "API DESIGN",
        "refactor" => "REFACTOR",
        _ => "PRACTICE",
    }
}

/// Everything an exercise view needs, decoded once.
pub struct ExerciseView<'a> {
    pub exercise: &'a Exercise,
    pub options: Vec<String>,
    pub hints: Vec<String>,
    pub code_b: Option<String>,
}

impl<'a> ExerciseView<'a> {
    pub fn new(exercise: &'a Exercise) -> Self {
        let payload: Value = exercise
            .options
            .as_ref()
            .and_then(|o| serde_json::from_str(o).ok())
            .unwrap_or(Value::Null);

        let options = if payload.is_object() {
            payload
                .get("options")
                .and_then(|v| v.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|s| s.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default()
        } else {
            payload
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter_map(|s| s.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default()
        };

        let hints = payload
            .get("hints")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|s| s.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        Self {
            exercise,
            options,
            hints,
            code_b: exercise.code_b.clone(),
        }
    }

    pub fn kind(&self) -> &str {
        &self.exercise.exercise_type
    }

    pub fn is_choice(&self) -> bool {
        matches!(
            self.kind(),
            "mcq"
                | "predict_output"
                | "fix_code"
                | "spot_difference"
                | "type_detective"
                | "compiler_challenge"
                | "signature_reading"
                | "predict_ownership"
                | "predict_compile"
                | "memory_diagram"
                | "trace_borrow"
                | "api_design"
        )
    }

    /// The `&str` slice, if the prompt contains one.
    pub fn prompt_code(&self) -> Option<String> {
        let (before, after) = self.exercise.prompt.split_once('\n')?;
        let code = format!("{before}\n{after}").trim().to_string();
        if code.is_empty() {
            None
        } else {
            Some(code)
        }
    }

    pub fn badge(&self) -> String {
        let label = kind_label(self.kind());

        format!(r##"<span class="exercise-kind">{label}</span>"##)
    }

    pub fn code_panels(&self) -> String {
        let lang = if self.kind() == "compiler_challenge" {
            "text"
        } else {
            "rust"
        };

        let mut out = String::new();

        if let Some(code) = &self.exercise.code {
            let rendered = if self.kind() == "fill_gap" {
                format!(
                    "<div class=\"code-block fill-gap\">\n        <div class=\"code-header\"><span class=\"code-lang\">RUST</span><button type=\"button\" class=\"copy-btn\" data-copy>COPY</button></div>\n        <pre><code>{}</code></pre>\n    </div>",
                    escape_html(code).replace("____", "<span class=\"blank\">____</span>")
                )
            } else {
                code_block(lang, code, true)
            };
            out.push_str(&rendered);
        }

        if let Some(code_b) = &self.code_b {
            out.push_str(&code_block(lang, code_b, true));
        }

        out
    }

    fn options_fieldset(&self) -> String {
        self.options_fieldset_state(None, false, None)
    }

    /// Options with a previously-chosen answer pre-selected, optionally locked
    /// for reading, and optionally relabelled.
    ///
    /// The practice runner needs all three: pre-selected when you jump back to
    /// a question you have already answered, locked once it has been graded so
    /// the page matches the answer that was recorded, and relabelled because it
    /// wraps this in its own fieldset with a "Your answer" legend. The override
    /// exists so the two do not stack up into two legends saying different
    /// things about the same group of radios.
    pub fn options_fieldset_state(
        &self,
        selected: Option<&str>,
        readonly: bool,
        legend_override: Option<&str>,
    ) -> String {
        if self.options.is_empty() {
            return String::new();
        }

        let is_checklist = self.kind() == "build_it" || self.kind() == "challenge";

        let rows: String = self
            .options
            .iter()
            .enumerate()
            .map(|(i, text)| {
                let value = i.to_string();

                let checked = match selected {
                    Some(sel) if sel == value => " checked",
                    _ => "",
                };

                let disabled = if readonly { " disabled" } else { "" };

                // A locked group is already excluded from validation, so
                // `required` on top of `disabled` is noise.
                let required = if readonly { "" } else { " required" };

                let body = if is_checklist {
                    format!(r##"<label class="option option-checklist">
    <input type="checkbox" name="step_{i}" value="{i}"{checked}{disabled}>
    <span class="option-text">{text}</span>
</label>"##)
                } else {
                    format!(r##"<label class="option">
    <input type="radio" name="answer" value="{i}"{required}{checked}{disabled}>
    <span class="option-text">{text}</span>
</label>"##)
                };

                body.replace("{required}", required)
                    .replace("{checked}", checked)
                    .replace("{disabled}", disabled)
                    .replace("{i}", &value)
                    .replace("{text}", &escape(text))
            })
            .collect();

        let legend = if let Some(custom) = legend_override {
            custom.to_string()
        } else if self.kind() == "build_it" {
            "Tick off each requirement as you complete it".to_string()
        } else if self.kind() == "challenge" {
            "Work through these in order".to_string()
        } else {
            "Choose one answer".to_string()
        };

        format!(
            r##"<fieldset class="options"><legend>{legend}</legend>{rows}</fieldset>"##,
            legend = legend,
            rows = rows
        )
    }

    pub fn hints_block(&self) -> String {
        if self.hints.is_empty() {
            return String::new();
        }

        let items: String = self
            .hints
            .iter()
            .enumerate()
            .map(|(i, h)| {
                format!(
                    r##"<li><details><summary>Hint {n}</summary><p>{text}</p></details></li>"##,
                    n = i + 1,
                    text = escape(h)
                )
            })
            .collect();

        format!(r##"<ul class="hints">{items}</ul>"##, items = items)
    }

    fn submit_label(&self) -> &'static str {
        match self.kind() {
            "build_it" | "refactor" | "fill_gap" => "MARK AS DONE",
            "challenge" => "CHECK MY WORK",
            _ => "CHECK ANSWER",
        }
    }
}

pub fn show(exercise: Option<&Exercise>, next: Option<&Exercise>) -> String {
    let Some(exercise) = exercise else {
        return page("practice", "Not found", r##"<h1>Exercise not found</h1>"##);
    };

    let view = ExerciseView::new(exercise);

    let prompt = if let Some(code) = view.prompt_code() {
        format!(
            "<h1 class=\"exercise-prompt\">{headline}</h1>{code}",
            headline = escape(exercise.prompt.split('\n').next().unwrap_or("")),
            code = code_block("rust", &code, false)
        )
    } else {
        format!(
            "<h1 class=\"exercise-prompt\">{}</h1>",
            escape(&exercise.prompt)
        )
    };

    let next_link = next
        .map(|e| format!(r##"<a href="/exercise/{}" class="btn btn-ghost">SKIP →</a>"##, e.id))
        .unwrap_or_default();

    // Open-ended exercises (challenge, build_it) post a step checklist
    // instead of an answer, but they still need a button — without one the
    // form is inert and the exercise can never be completed.
    let action = if !view.options.is_empty() || view.is_choice() {
        format!(
            r##"<button type="submit" class="btn btn-primary">{}</button>"##,
            view.submit_label()
        )
    } else {
        String::new()
    };

    let body = format!(
        r##"<header class="exercise-header">
    {badge}
    {prompt}
</header>

{panels}

<form action="/exercise/{id}/submit" method="post" class="exercise-form">
    {options}
    {hints}
    <div class="exercise-actions">
        {action}
        {next_link}
    </div>
</form>"##,
        badge = view.badge(),
        prompt = prompt,
        panels = view.code_panels(),
        id = exercise.id,
        options = view.options_fieldset(),
        hints = view.hints_block(),
        action = action,
        next_link = next_link
    );

    page("practice", "Practice", &body)
}

pub fn result(
    exercise: Option<&Exercise>,
    answer: &str,
    correct: bool,
    xp_awarded: i64,
    next: Option<&Exercise>,
) -> String {
    let view = exercise.map(ExerciseView::new);

    let is_open_ended = exercise.map(|e| e.is_open_ended()).unwrap_or(false);

    // Computed once: the XP badge is the same fragment in every branch, and
    // some branches render it inside a nested format!.
    let xp = if xp_awarded > 0 {
        format!(r##"<span class="xp-reward">+{xp_awarded} XP</span>"##)
    } else {
        String::new()
    };

    let feedback = match (view.as_ref(), correct) {
        // Open-ended work has no single right answer to check against, so it
        // never "passes". It gets the review panel regardless of what was
        // submitted — grading a design decision is the learner's job, and
        // telling them NOT QUITE for their attempt is simply wrong.
        (Some(v), _) if is_open_ended => {
            let explanation = format_explanation(&v.exercise.explanation);
            let body = r##"<details class="explanation" open>
            <summary>Full walkthrough</summary>
            <div class="explanation-body">EXPLANATION_BODY</div>
        </details>"##;
            format!(
                r##"<div class="feedback neutral" role="status">
    <span class="feedback-icon" aria-hidden="true">◆</span>
    <div>
        <strong>NICE WORK — CHECK YOUR ANSWER</strong>
        {xp}
        {body}
    </div>
</div>"##,
                xp = xp,
                body = body.replace("EXPLANATION_BODY", &explanation)
            )
        }
        (Some(v), true) => format!(
            r##"<div class="feedback correct" role="status">
    <span class="feedback-icon" aria-hidden="true">✓</span>
    <div>
        <strong>CORRECT</strong>
        <p>{explanation}</p>
        {xp}
    </div>
</div>"##,
            explanation = format_explanation(&v.exercise.explanation),
            xp = xp
        ),
        (Some(v), false) => {
            // Nothing to echo for open-ended work — there was no chosen answer.
            let answer_line = if is_open_ended {
                String::new()
            } else {
                format!(
                    "<p class=\"feedback-answer\">You answered: {}</p>",
                    escape(answer)
                )
            };
            let explanation = format_explanation(&v.exercise.explanation);
            let body = r##"<details class="explanation">
            <summary>Show the answer</summary>
            <div class="explanation-body">EXPLANATION_BODY</div>
        </details>"##;
            format!(
                r##"<div class="feedback incorrect" role="status">
    <span class="feedback-icon" aria-hidden="true">✗</span>
    <div>
        <strong>NOT QUITE</strong>
        {answer_line}
        {body}
    </div>
</div>"##,
                answer_line = answer_line,
                body = body.replace("EXPLANATION_BODY", &explanation)
            )
        }
        (None, _) => r##"<div class="feedback incorrect"><strong>EXERCISE NOT FOUND</strong></div>"##.to_string(),
    };

    let next_link = next
        .map(|e| format!(r##"<a href="/exercise/{}" class="btn btn-primary">NEXT EXERCISE →</a>"##, e.id))
        .unwrap_or_else(|| r##"<a href="/dashboard" class="btn btn-primary">BACK TO DASHBOARD →</a>"##.to_string());

    let retry = exercise
        .map(|e| format!(r##"<a href="/exercise/{}" class="btn btn-ghost">TRY AGAIN</a>"##, e.id))
        .unwrap_or_default();

    let body = format!(
        r##"{feedback}

<nav class="exercise-actions">
    {next_link}
    {retry}
</nav>"##,
        feedback = feedback,
        next_link = next_link,
        retry = retry
    );

    page("practice", "Result", &body)
}

fn format_explanation(markdownish: &str) -> String {
    escape(markdownish).replace("\n\n", "</p><p>")
}

pub fn render_section(section: &LessonSection) -> String {
    use super::components::{
        breakdown, broken_code_block, callout, checklist, code_block, corrected_code_block, diagram,
        gap_fill_block, guess_run_block, step_through_block, steps_block, table,
    };

    match section {
        LessonSection::Text { text } => {
            let escaped = escape(text);
            escaped
                .split("\n\n")
                .map(|p| format!("<p class=\"lesson-text\">{p}</p>"))
                .collect::<Vec<_>>()
                .join("\n")
        }
        LessonSection::Callout { title, text } => callout(title, text, tone_for(title)),
        LessonSection::Code { language, code, status } => match status {
            CodeStatus::Broken => broken_code_block(language, code),
            CodeStatus::Corrected => corrected_code_block(language, code),
            CodeStatus::Valid => code_block(language, code, true),
        },
        LessonSection::Breakdown { title, code, items } => breakdown(title, code, items),
        LessonSection::Checklist { title, items } => checklist(title, items),
        LessonSection::Diagram { title, code } => diagram(title, code),
        LessonSection::Table { headers, rows } => table(headers, rows),
        LessonSection::Steps { items } => steps_block(items),
        LessonSection::Paragraphs { items } => items
            .iter()
            .map(|p| {
                format!(
                    "<p class=\"lesson-text\">{}</p>",
                    escape(p).replace('\n', "<br>")
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
        LessonSection::GapFill {
            intro,
            language,
            code,
            blanks,
            after,
        } => gap_fill_block(intro, language, code, blanks, after),
        LessonSection::StepThrough {
            title,
            language,
            code,
            steps,
        } => step_through_block(title, language, code, steps),
        LessonSection::GuessRun {
            prompt,
            language,
            code,
            options,
            correct,
            explanation,
        } => guess_run_block(prompt, language, code, options, *correct, explanation),
    }
}

pub fn tone_for(title: &str) -> CalloutTone {
    let lower = title.to_lowercase();

    if lower.contains("practice") {
        CalloutTone::Practice
    } else if lower.contains("mini challenge") {
        CalloutTone::Challenge
    } else if lower.contains("project") {
        CalloutTone::Project
    } else {
        CalloutTone::Idea
    }
}

#[cfg(test)]
mod widget_tests {
    use super::render_section;
    use crate::models::{GapBlank, LessonSection, Step};

    #[test]
    fn gap_fill_renders_one_input_per_marker() {
        let section = LessonSection::GapFill {
            intro: "Fill in the field types.".into(),
            language: "rust".into(),
            code: "struct Topping {\n    name: ____,\n    extra_cost: ____,\n}".into(),
            blanks: vec![
                GapBlank {
                    id: "b1".into(),
                    answer: "String".into(),
                    hint: "owned text".into(),
                },
                GapBlank {
                    id: "b2".into(),
                    answer: "u32".into(),
                    hint: "whole number".into(),
                },
            ],
            after: String::new(),
        };

        let html = render_section(&section);

        assert!(html.contains("data-gap-fill"));
        assert_eq!(html.matches("data-gap data-blank-id").count(), 2);
        assert!(html.contains("data-answer=\"String\""));
        assert!(html.contains("Show answers"), "no-JS fallback must be present");
    }

    /// A `____` with no matching blank must stay visible rather than silently
    /// disappear — a content typo should look wrong, not lose a placeholder.
    #[test]
    fn gap_fill_leaves_an_unmatched_marker_visible() {
        let section = LessonSection::GapFill {
            intro: String::new(),
            language: "rust".into(),
            code: "let x = ____; let y = ____;".into(),
            blanks: vec![GapBlank {
                id: "b1".into(),
                answer: "1".into(),
                hint: String::new(),
            }],
            after: String::new(),
        };

        let html = render_section(&section);

        assert_eq!(html.matches("data-gap data-blank-id").count(), 1);
        assert!(html.contains("____"), "the unmatched marker must remain");
    }

    #[test]
    fn step_through_renders_controls_and_a_no_js_fallback() {
        let section = LessonSection::StepThrough {
            title: "Building a pizza".into(),
            language: "rust".into(),
            code: "let p = Pizza::new(Size::Medium);".into(),
            steps: vec![
                Step {
                    lines: vec![1],
                    note: "new runs".into(),
                },
                Step {
                    lines: vec![1],
                    note: "size stored".into(),
                },
            ],
        };

        let html = render_section(&section);

        assert!(html.contains("data-step-through"));
        assert!(html.contains("data-st-prev") && html.contains("data-st-next"));
        assert!(html.contains("st-fallback"), "no-JS fallback must be present");
        assert!(html.contains("data-line=\"1\""));
    }

    #[test]
    fn guess_run_renders_options_a_run_button_and_a_fallback() {
        let section = LessonSection::GuessRun {
            prompt: "What will this print?".into(),
            language: "rust".into(),
            code: "fn main() { println!(\"hi\"); }".into(),
            options: vec!["hi".into(), "nothing".into()],
            correct: 0,
            explanation: "println! writes to stdout.".into(),
        };

        let html = render_section(&section);

        assert!(html.contains("data-guess-run"));
        assert!(html.contains("GUESS, THEN RUN"));
        assert!(html.contains("data-config"));
        assert!(html.contains("Reveal the answer"), "no-JS fallback must be present");
    }

    /// Two widgets on one page must not share a radio group, or selecting an
    /// option in the second clears the first.
    #[test]
    fn two_guess_run_widgets_use_different_radio_groups() {
        let section = LessonSection::GuessRun {
            prompt: "?".into(),
            language: "rust".into(),
            code: "fn main() {}".into(),
            options: vec!["a".into(), "b".into()],
            correct: 0,
            explanation: "because".into(),
        };

        let group = |html: &str| {
            html.split("name=\"")
                .nth(1)
                .and_then(|s| s.split('"').next())
                .map(str::to_string)
                .unwrap()
        };

        let first = render_section(&section);
        let second = render_section(&section);

        assert_ne!(group(&first), group(&second));
    }
}
