use super::components::{status_icon, StatusState};
use super::layout::{escape, page};
use crate::services::progress::ModulePageData;

pub fn show(data: &ModulePageData) -> String {
    let body = format!(
        r##"{lock_notice}
<header class="module-header">
    <span class="module-label">MODULE {num:02}</span>
    <h1>{title}</h1>
    <p class="lesson-subtitle">{description}</p>
    <div class="module-progress">
        {bar}
        <span class="progress-text">{done}/{total} LESSONS · {percent}%</span>
    </div>
</header>

<section class="module-section">
    <h2 class="section-heading">LESSONS</h2>
    <div class="lesson-list">{lessons}</div>
</section>

{exercises}

{projects}

<nav class="lesson-pagination">
    {prev}
    {next}
</nav>"##,
        lock_notice = lock_notice(data),
        num = data.module.as_ref().map(|m| m.order_index).unwrap_or(0),
        title = escape(&data.module.as_ref().map(|m| m.title.clone()).unwrap_or_default()),
        description = escape(&data.module.as_ref().map(|m| m.description.clone()).unwrap_or_default()),
        bar = super::components::progress_bar(percent(data), true),
        done = done_count(data),
        total = data.lessons.len(),
        percent = percent(data),
        lessons = lesson_rows(data),
        exercises = exercise_section(data),
        projects = project_section(data),
        prev = prev_link(data),
        next = next_link(data)
    );

    page("dashboard", &data.module.as_ref().map(|m| m.title.clone()).unwrap_or_default(), &body)
}

/// Advisory banner shown when a module has not been formally unlocked.
/// The content is readable regardless — this only sets expectations.
fn lock_notice(data: &ModulePageData) -> String {
    if !data.is_locked {
        return String::new();
    }

    // Already working on this module — telling them it is "not started" would
    // contradict the progress bar directly above.
    let touched = data
        .lessons
        .iter()
        .any(|l| data.completed_lessons.contains(&l.id))
        || data
            .exercises
            .iter()
            .any(|e| data.completed_exercises.contains(&e.id));

    if touched {
        return String::new();
    }

    let prev = data
        .prev_module
        .as_ref()
        .map(|m| escape(&m.title))
        .unwrap_or_else(|| "the previous module".to_string());

    format!(
        r##"<div class="lock-notice" role="note">
    <span class="lock-icon" aria-hidden="true">🔒</span>
    <div>
        <strong>Not started yet</strong>
        <p>You can read everything here, but working through <a href="/module/{id}">{prev}</a> first will make this a lot easier.</p>
    </div>
</div>"##,
        id = data.prev_module.as_ref().map(|m| m.id).unwrap_or(1),
        prev = prev
    )
}

fn done_count(data: &ModulePageData) -> usize {
    data.lessons
        .iter()
        .filter(|l| data.completed_lessons.contains(&l.id))
        .count()
}

fn percent(data: &ModulePageData) -> i64 {
    if data.lessons.is_empty() {
        0
    } else {
        done_count(data) as i64 * 100 / data.lessons.len() as i64
    }
}

fn lesson_rows(data: &ModulePageData) -> String {
    data.lessons
        .iter()
        .map(|l| {
            let done = data.completed_lessons.contains(&l.id);
            let state = if done { StatusState::Completed } else { StatusState::Available };
            let cls = if done { "completed" } else { "available" };

            format!(
                r##"<a href="/lesson/{id}" class="lesson-row {cls}">
    {icon}
    <span class="lesson-num">{num:02}</span>
    <span class="lesson-name">{title}</span>
    <span class="lesson-go" aria-hidden="true">→</span>
</a>"##,
                id = l.id,
                cls = cls,
                icon = status_icon(state),
                num = l.order_index,
                title = escape(&l.title)
            )
        })
        .collect()
}

fn exercise_section(data: &ModulePageData) -> String {
    if data.exercises.is_empty() {
        return String::new();
    }

    // Gradable exercises are reached through the run builder, which sequences
    // them, lets you jump, and grades you at the end. Listing all 106 of them
    // here was redundant with that and made the module page mostly exercise
    // links.
    let gradable = data
        .exercises
        .iter()
        .filter(|e| !e.is_open_ended())
        .count();

    // Open-ended exercises are the exception: there is no single right answer,
    // so a graded run cannot include them. This list is their only entry
    // point — removing it outright would orphan every module challenge and
    // BUILD IT.
    let open_ended: Vec<_> = data.exercises.iter().filter(|e| e.is_open_ended()).collect();

    let run_card = if gradable > 0 {
        format!(
            r##"<div class="practice-launch">
    <div>
        <strong>Ready to be tested?</strong>
        <p>{gradable} questions, taken one at a time. Jump anywhere, skip freely, and get a score at the end — or take the lot if you want the marathon.</p>
    </div>
    <a href="/practice/{id}" class="btn btn-primary">START PRACTICE RUN</a>
</div>"##,
            gradable = gradable,
            id = data.module.as_ref().map(|m| m.id).unwrap_or(1)
        )
    } else {
        String::new()
    };

    let open_section = if open_ended.is_empty() {
        String::new()
    } else {
        let rows: String = open_ended
            .iter()
            .map(|e| {
                let done = data.completed_exercises.contains(&e.id);
                let state = if done { StatusState::Completed } else { StatusState::Available };
                let cls = if done { "completed" } else { "available" };

                // Some prompts open with a code listing, so take the first line
                // only — a multi-line prompt would break the row layout.
                let prompt: String = e.prompt.lines().next().unwrap_or("").chars().take(120).collect();

                format!(
                    r##"<a href="/exercise/{id}" class="lesson-row {cls}">
    {icon}
    <span class="lesson-name">{prompt}</span>
    <span class="lesson-xp">+{xp} XP</span>
</a>"##,
                    id = e.id,
                    cls = cls,
                    icon = status_icon(state),
                    prompt = escape(prompt.trim()),
                    xp = e.xp_reward
                )
            })
            .collect();

        format!(
            r##"<details class="open-ended">
    <summary>
        <span class="open-ended-title">OPEN-ENDED PROJECTS</span>
        <span class="open-ended-count">{count}</span>
    </summary>
    <p class="open-ended-lede">No single right answer, so these sit outside the graded run. They are the ones worth writing out properly.</p>
    <div class="lesson-list">{rows}</div>
</details>"##,
            count = open_ended.len(),
            rows = rows
        )
    };

    format!(
        r##"<section class="module-section" id="practice">
    <h2 class="section-heading">PRACTICE</h2>
    {run_card}
    {open_section}
</section>"##,
        run_card = run_card,
        open_section = open_section
    )
}

fn project_section(data: &ModulePageData) -> String {
    if data.projects.is_empty() {
        return String::new();
    }

    let rows: String = data
        .projects
        .iter()
        .map(|p| {
            format!(
                r##"<a href="/project/{id}" class="lesson-row available">
    <span class="status-icon available" aria-hidden="true">◆</span>
    <span class="lesson-name">{title}</span>
    <span class="lesson-go" aria-hidden="true">→</span>
</a>"##,
                id = p.id,
                title = escape(&p.title)
            )
        })
        .collect();

    format!(
        r##"<section class="module-section" id="projects">
    <h2 class="section-heading">PROJECT</h2>
    <div class="lesson-list">{rows}</div>
</section>"##
    )
}

fn prev_link(data: &ModulePageData) -> String {
    data.prev_module
        .as_ref()
        .map(|m| {
            format!(
                r##"<a href="/module/{}" class="btn btn-ghost">← {}</a>"##,
                m.id,
                escape(&m.title)
            )
        })
        .unwrap_or_default()
}

fn next_link(data: &ModulePageData) -> String {
    data.next_module
        .as_ref()
        .map(|m| {
            format!(
                r##"<a href="/module/{}" class="btn btn-primary">{} →</a>"##,
                m.id,
                escape(&m.title)
            )
        })
        .unwrap_or_else(|| r##"<a href="/dashboard" class="btn btn-primary">BACK TO DASHBOARD</a>"##.to_string())
}
