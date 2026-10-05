//! The practice runner: build a run, work through it, get graded.
//!
//! Question rendering is deliberately borrowed from [`super::exercise`] rather
//! than reimplemented — the badge, code panels, hints and options are identical
//! to the standalone exercise page, so the two flows cannot drift apart.

use super::components::{code_block, progress_bar};
use super::exercise::ExerciseView;
use super::layout::{escape, page};
use crate::models::Exercise;
use crate::services::practice::{AttemptState, BuilderData, ResultsData, RunPageData};

// ── Run builder ─────────────────────────────────────────────────────────

pub fn builder(data: &BuilderData) -> String {
    let total = data.exercises.len();

    if total == 0 {
        let body = format!(
            r##"<header class="practice-header">
    <span class="module-label">MODULE {num:02}</span>
    <h1>Practice — {title}</h1>
</header>

<p class="practice-empty">This module has no gradable practice questions yet. The open-ended exercises can still be done from the <a href="/module/{id}">module page</a>.</p>"##,
            num = data.module_order,
            title = escape(&data.module_title),
            id = data.module_id
        );
        return page("practice", "Practice", &body);
    }

    let body = format!(
        r##"{resume}
<header class="practice-header">
    <span class="module-label">MODULE {num:02}</span>
    <h1>Practice — {title}</h1>
    <p class="practice-lede">{total} questions you can be graded on. Open-ended exercises are not included, because there is no single right answer to check against.</p>
</header>

<section class="run-options">
    <form class="run-card" action="/practice/{id}/start" method="post">
        <h2>Take everything</h2>
        <p>The whole module, in curriculum order. Nothing is left out.</p>
        <input type="hidden" name="mode" value="all">
        <button type="submit" class="btn btn-primary">START ALL {total} QUESTIONS</button>
    </form>

    <form class="run-card" action="/practice/{id}/start" method="post">
        <h2>Quick check</h2>
        <p>A sample spread evenly across the module, so it covers every lesson rather than just the first few.</p>
        <input type="hidden" name="mode" value="count">
        <div class="run-count">
            <label for="run-n">How many questions?</label>
            <input type="number" id="run-n" name="n" value="20" min="1" max="{total}">
            <button type="submit" class="btn btn-primary">START</button>
        </div>
    </form>
</section>

<section class="picker-section">
    <h2 class="section-heading">PICK YOUR OWN</h2>
    <p class="practice-lede">Tick the questions you want. These are numbered the same way as a full run, so &ldquo;question 42&rdquo; means the same thing wherever you see it.</p>
    {filters}
    <form class="picker" action="/practice/{id}/start" method="post">
        <input type="hidden" name="mode" value="custom">
        {groups}
        <div class="picker-footer">
            <span class="picker-count" data-picker-count>{total} selectable</span>
            <button type="submit" class="btn btn-primary" data-picker-submit disabled>START SELECTED</button>
        </div>
    </form>
</section>

{history}"##,
        resume = resume_banner(data),
        num = data.module_order,
        title = escape(&data.module_title),
        id = data.module_id,
        total = total,
        filters = picker_filters(),
        groups = picker_groups(data),
        history = history_section(data)
    );

    page("practice", "Practice", &body)
}

/// Offers to pick up the run already in progress. Losing an 80-question run
/// because you closed the tab would be the most annoying bug in the feature.
fn resume_banner(data: &BuilderData) -> String {
    let Some(run) = &data.unfinished else {
        return String::new();
    };

    format!(
        r##"<div class="resume-banner" role="note">
    <div>
        <strong>You have a run in progress</strong>
        <p>{label} — started {created}.</p>
    </div>
    <a href="/practice/run/{id}" class="btn btn-primary">RESUME</a>
</div>"##,
        label = escape(&run.label),
        created = escape(&run.created_at),
        id = run.id
    )
}

fn picker_filters() -> String {
    // The filtering is progressive enhancement: without JS the checkboxes all
    // work, you just select by hand. The buttons hide themselves if JS is off.
    r##"<div class="picker-filters" data-picker-filters hidden>
    <button type="button" data-filter="all">All</button>
    <button type="button" data-filter="unanswered">Unanswered</button>
    <button type="button" data-filter="incorrect">Incorrect</button>
    <button type="button" data-filter="correct">Correct</button>
    <button type="button" data-filter="clear">Clear</button>
</div>"##
    .to_string()
}

/// One group per lesson, so a 106-question picker stays navigable.
fn picker_groups(data: &BuilderData) -> String {
    let mut out = String::new();

    for (lesson_id, lesson_title) in &data.lesson_titles {
        let rows: String = data
            .exercises
            .iter()
            .enumerate()
            .filter(|(_, e)| e.lesson_id == Some(*lesson_id))
            .map(|(i, e)| picker_row(i + 1, e, &data.past))
            .collect();

        if rows.is_empty() {
            continue;
        }

        out.push_str(&format!(
            r##"<details class="picker-group" open>
    <summary><span class="picker-lesson">{title}</span></summary>
    <div class="picker-rows">{rows}</div>
</details>"##,
            title = escape(lesson_title),
            rows = rows
        ));
    }

    // Exercises with no lesson attached (the two module challenges are filtered
    // out, but be safe rather than silently dropping a question).
    let orphans: String = data
        .exercises
        .iter()
        .enumerate()
        .filter(|(_, e)| e.lesson_id.is_none())
        .map(|(i, e)| picker_row(i + 1, e, &data.past))
        .collect();

    if !orphans.is_empty() {
        out.push_str(&format!(
            r##"<details class="picker-group" open>
    <summary><span class="picker-lesson">Ungrouped</span></summary>
    <div class="picker-rows">{orphans}</div>
</details>"##
        ));
    }

    out
}

fn picker_row(number: usize, exercise: &Exercise, past: &[(i64, AttemptState)]) -> String {
    let (state, glyph, spoken) = match past.iter().find(|(id, _)| *id == exercise.id) {
        Some((_, AttemptState::Correct)) => ("correct", "✓", "Answered correctly"),
        Some((_, AttemptState::Incorrect)) => ("incorrect", "✗", "Answered incorrectly"),
        None => ("unanswered", "○", "Not answered yet"),
    };

    // The label is the teaching signal, so it is not truncated in Rust. CSS
    // clamps it to two lines. Cutting it here meant picking between questions
    // on a fragment of a sentence.
    let label = exercise.prompt.lines().next().unwrap_or("").trim();

    format!(
        r##"<label class="picker-row state-{state}" data-state="{state}">
    <input type="checkbox" name="q" value="{number}">
    <span class="picker-num">{number}</span>
    <span class="picker-kind">{kind}</span>
    <span class="picker-label">{label}</span>
    <span class="picker-state"><span aria-hidden="true">{glyph}</span><span class="sr-only">{spoken}</span></span>
</label>"##,
        state = state,
        number = number,
        kind = super::exercise::kind_label(&exercise.exercise_type),
        label = escape(label),
        glyph = glyph,
        spoken = spoken
    )
}

fn history_section(data: &BuilderData) -> String {
    if data.history.is_empty() {
        return String::new();
    }

    let rows: String = data
        .history
        .iter()
        .map(|(run, correct, total)| {
            let pct = if *total > 0 {
                correct * 100 / total
            } else {
                0
            };

            format!(
                r##"<tr>
    <td><a href="/practice/run/{id}/results">{label}</a></td>
    <td>{correct} / {total}</td>
    <td>{pct}%</td>
    <td>{created}</td>
</tr>"##,
                id = run.id,
                label = escape(&run.label),
                correct = correct,
                total = total,
                pct = pct,
                created = escape(&run.created_at)
            )
        })
        .collect();

    format!(
        r##"<section class="history-section">
    <h2 class="section-heading">RECENT RUNS</h2>
    <table class="history-table">
        <thead><tr><th>Run</th><th>Score</th><th>%</th><th>Started</th></tr></thead>
        <tbody>{rows}</tbody>
    </table>
</section>"##,
        rows = rows
    )
}

// ── Question ─────────────────────────────────────────────────────────────

pub fn question(
    data: &RunPageData,
    exercise: &Exercise,
    position: i64,
    answered: Option<bool>,
) -> String {
    let view = ExerciseView::new(exercise);
    let total = data.items.len();
    let is_last = position >= total as i64;

    // On revisit, pre-select whatever was chosen last time so the learner can
    // see their own answer rather than a blank form.
    let previous = data
        .answers
        .iter()
        .find(|(id, _, _)| *id == exercise.id)
        .map(|(_, _, given)| given.clone());

    let body = format!(
        r##"{run_header}
{badge}
{prompt_block}
{panels}

<form action="/practice/run/{run}/{pos}" method="post" class="exercise-form">
    {options}
    {hints}
    <div class="exercise-actions">
        <button type="submit" class="btn btn-primary">CHECK ANSWER</button>
    </div>
</form>

{answered_note}
{nav}"##,
        run_header = run_header(data, position),
        badge = view.badge(),
        prompt_block = prompt_block(&view, exercise),
        panels = view.code_panels(),
        run = data.run.id,
        pos = position,
        options = view.options_fieldset_state(previous.as_deref(), false, None),
        hints = view.hints_block(),
        answered_note = match previous {
            Some(_) if answered == Some(true) => {
                r##"<p class="run-note done">Already answered correctly. Answer again to change it.</p>"##
                    .to_string()
            }
            Some(_) => {
                r##"<p class="run-note todo">Already answered. Answer again to try it properly.</p>"##
                    .to_string()
            }
            None => String::new(),
        },
        nav = nav_row(data, position, is_last)
    );

    page("practice", "Practice", &body)
}

/// A question that has just been graded: feedback, locked options, and an
/// explicit way on. Deliberately does not auto-redirect — the explanation is
/// the point of answering.
pub fn answered(
    data: &RunPageData,
    exercise: &Exercise,
    position: i64,
    given: &str,
    correct: bool,
) -> String {
    let view = ExerciseView::new(exercise);
    let total = data.items.len();
    let is_last = position >= total as i64;

    let feedback = if correct {
        r##"<div class="feedback correct" role="status">
    <span class="feedback-icon" aria-hidden="true">✓</span>
    <div>
        <strong>CORRECT</strong>
        <p>{explanation}</p>
    </div>
</div>"##
    } else {
        r##"<div class="feedback incorrect" role="status">
    <span class="feedback-icon" aria-hidden="true">✗</span>
    <div>
        <strong>NOT QUITE</strong>
        <p class="feedback-answer">You chose: {given}</p>
        <p>The right answer was: {right}</p>
        <details class="explanation" open>
            <summary>Why</summary>
            <div class="explanation-body">{explanation}</div>
        </details>
    </div>
</div>"##
    };

    // On the last question there is no "next" — go straight to the grade.
    let onward = if is_last {
        format!(
            r##"<a href="/practice/run/{}/results" class="btn btn-primary">SEE YOUR GRADE →</a>"##,
            data.run.id
        )
    } else {
        format!(
            r##"<a href="/practice/run/{run}/{next}" class="btn btn-primary">NEXT QUESTION →</a>"##,
            run = data.run.id,
            next = position + 1
        )
    };

    let body = format!(
        r##"{run_header}
{badge}
{prompt_block}
{panels}

{feedback}

{options}

<div class="exercise-actions">
    {onward}
</div>

{nav}"##,
        run_header = run_header(data, position),
        badge = view.badge(),
        prompt_block = prompt_block(&view, exercise),
        panels = view.code_panels(),
        feedback = feedback
            .replace("{explanation}", &escape(&exercise.explanation))
            .replace("{given}", &escape(given))
            .replace("{right}", &option_text(exercise, &exercise.correct_answer)),
        onward = onward,
        options = view.options_fieldset_state(Some(given), true, Some("Your answer")),
        nav = nav_row(data, position, is_last)
    );

    page("practice", "Practice", &body)
}

/// `QUESTION n OF m`, a progress bar, and the run's label.
fn run_header(data: &RunPageData, position: i64) -> String {
    let total = data.items.len() as i64;
    let answered = data.answers.len() as i64;
    let percent = if total > 0 {
        answered * 100 / total
    } else {
        0
    };

    format!(
        r##"<header class="practice-run-header">
    <span class="module-label">MODULE {num:02} · {module}</span>
    <h1>QUESTION {position} OF {total}</h1>
    <p class="run-label">{label}</p>
    <div class="module-progress">
        {bar}
        <span class="progress-text">{answered} of {total} answered</span>
    </div>
</header>"##,
        num = data.module_order,
        module = escape(&data.module_title),
        position = position,
        total = total,
        label = escape(&data.run.label),
        bar = progress_bar(percent, false),
        answered = answered
    )
}

/// A prompt whose first line may itself be a code listing.
fn prompt_block(view: &ExerciseView, exercise: &Exercise) -> String {
    match view.prompt_code() {
        Some(code) => format!(
            r##"<div class="run-prompt"><span class="run-prompt-head">{}</span>{code}</div>"##,
            escape(exercise.prompt.split('\n').next().unwrap_or("")),
            code = code_block("rust", &code, true)
        ),
        None => format!(
            r##"<p class="run-prompt plain">{}</p>"##,
            escape(&exercise.prompt)
        ),
    }
}

/// The text of whichever option the answer index points at.
fn option_text(exercise: &Exercise, answer: &str) -> String {
    let view = ExerciseView::new(exercise);

    answer
        .parse::<usize>()
        .ok()
        .and_then(|i| view.options.get(i).cloned())
        .unwrap_or_else(|| answer.to_string())
}

fn nav_row(data: &RunPageData, position: i64, is_last: bool) -> String {
    let prev = if position > 1 {
        format!(
            r##"<a href="/practice/run/{run}/{p}" class="btn btn-ghost">← PREVIOUS</a>"##,
            run = data.run.id,
            p = position - 1
        )
    } else {
        String::new()
    };

    let next = if is_last {
        format!(
            r##"<a href="/practice/run/{}/results" class="btn btn-ghost">FINISH &amp; GRADE →</a>"##,
            data.run.id
        )
    } else {
        format!(
            r##"<a href="/practice/run/{run}/{n}" class="btn btn-ghost">NEXT →</a>"##,
            run = data.run.id,
            n = position + 1
        )
    };

    format!(
        r##"<nav class="run-nav">
    {prev}
    {palette}
    {next}
</nav>"##,
        prev = prev,
        palette = jump_palette(data, position),
        next = next
    )
}

/// Numbered links to every question, marked by state. The only reason a
/// 106-question run is navigable at all.
///
/// State is carried by a glyph and a screen-reader phrase, not by colour —
/// the design spec (§27) forbids colour alone, and green-on-grey is invisible
/// to a protanope and to anyone on a washed-out screen.
fn jump_palette(data: &RunPageData, current: i64) -> String {
    let cells: String = data
        .items
        .iter()
        .map(|item| {
            let (state, glyph, spoken) =
                match data.answers.iter().find(|(id, _, _)| *id == item.exercise.id) {
                    Some((_, true, _)) => ("correct", "✓", "answered correctly"),
                    Some((_, false, _)) => ("incorrect", "✗", "answered incorrectly"),
                    None => ("unanswered", "○", "not answered"),
                };

            let here = if item.position == current { " here" } else { "" };

            format!(
                r##"<a href="/practice/run/{run}/{p}" class="jump-cell {state}{here}" title="Question {p} — {spoken}"><span class="jump-glyph" aria-hidden="true">{glyph}</span><span class="jump-num">{p}</span><span class="sr-only">{spoken}</span></a>"##,
                run = data.run.id,
                p = item.position,
                state = state,
                here = here,
                spoken = spoken,
                glyph = glyph
            )
        })
        .collect();

    format!(
        r##"<details class="jump-palette">
    <summary>JUMP TO QUESTION</summary>
    <div class="jump-grid">{cells}</div>
</details>"##
    )
}

// ── Results ──────────────────────────────────────────────────────────────

pub fn results(data: &ResultsData) -> String {
    let total = data.items.len();
    let correct = data.answers.iter().filter(|a| a.correct).count();
    let wrong = data.answers.iter().filter(|a| !a.correct).count();
    let unanswered = total - data.answers.len();
    let percent = ((correct * 100) as i64).checked_div(total as i64).unwrap_or(0);

    let verdict = match percent {
        90..=100 => "THAT IS THE STUFF",
        70..=89 => "SOLID — PATCH THE GAPS",
        50..=69 => "GETTING THERE",
        25..=49 => "WORTH ANOTHER PASS",
        _ => "GO BACK THROUGH THE LESSONS",
    };

    let body = format!(
        r##"<header class="results-header">
    <span class="module-label">MODULE PRACTICE</span>
    <h1>Your grade</h1>
    <p class="results-verdict">{verdict}</p>
    <p class="results-score"><span class="results-percent">{percent}%</span></p>
    <p class="results-counts">{correct} correct · {wrong} incorrect · {unanswered} not answered</p>
    <p class="results-run">{label}</p>
</header>

<div class="results-note">
    <strong>Practice does not award XP.</strong>
    <p>Nothing here changes your module progress or your XP total — that still comes from the <a href="/module/{module}">module page</a> and the standalone exercises. This is here to show you what you know right now.</p>
</div>

<section class="results-review">
    <h2 class="section-heading">REVIEW</h2>
    {rows}
</section>

<div class="results-actions">
    <form action="/practice/{module}/start" method="post">
        <input type="hidden" name="mode" value="all">
        <button type="submit" class="btn btn-primary">NEW RUN</button>
    </form>
    <form action="/practice/run/{run}/restart" method="post">
        <button type="submit" class="btn btn-ghost">RETRY THESE QUESTIONS</button>
    </form>
    <a href="/module/{module}" class="btn btn-ghost">BACK TO MODULE</a>
</div>"##,
        verdict = verdict,
        percent = percent,
        correct = correct,
        wrong = wrong,
        unanswered = unanswered,
        label = escape(&data.run.label),
        rows = review_rows(data),
        module = data.run.module_id,
        run = data.run.id
    );

    page("practice", "Practice results", &body)
}

fn review_rows(data: &ResultsData) -> String {
    data.items
        .iter()
        .map(|item| {
            let answer = data
                .answers
                .iter()
                .find(|a| a.exercise_id == item.exercise.id);

            let (state, chip, verdict) = match answer {
                Some(a) if a.correct => ("correct", "✓", "Correct"),
                Some(_) => ("incorrect", "✗", "Incorrect"),
                None => ("unanswered", "·", "Not answered"),
            };

            let detail = match answer {
                Some(a) => format!(
                    r##"<p class="review-detail"><span class="review-gave">You: {given}</span> <span class="review-right">Right: {right}</span></p>"##,
                    given = escape(a.given.as_deref().unwrap_or("—")),
                    right = escape(&option_text(&item.exercise, &item.exercise.correct_answer))
                ),
                None => String::new(),
            };

            format!(
                r##"<details class="review-row {state}">
    <summary>
        <span class="review-num">{pos}</span>
        <span class="review-chip">{chip}</span>
        <span class="review-label">{label}</span>
        <span class="review-verdict">{verdict}</span>
    </summary>
    {detail}
    <p class="review-actions"><a href="/practice/run/{run}/{pos}">Open this question →</a></p>
</details>"##,
                state = state,
                pos = item.position,
                chip = chip,
                label = escape(
                    item.exercise
                        .prompt
                        .lines()
                        .next()
                        .unwrap_or("")
                ),
                verdict = verdict,
                detail = detail,
                run = data.run.id
            )
        })
        .collect()
}
