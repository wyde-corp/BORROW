use super::components::{status_icon, StatusState};
use super::layout::page_with_progress;
use crate::services::progress::DashboardData;

pub fn show(data: &DashboardData) -> String {
    let body = format!(
        r##"<section class="continue-learning">
    <h2 class="section-heading">YOUR RUST JOURNEY</h2>
    {continue_card}
</section>

<section class="curriculum">
    <h2 class="section-heading">CURRICULUM</h2>
    <div class="module-list">{rows}</div>
</section>"##,
        continue_card = continue_card(data),
        rows = module_rows(data)
    );

    page_with_progress(
        "dashboard",
        "Dashboard",
        &body,
        percent(data),
        data.total_xp,
    )
}

fn percent(data: &DashboardData) -> i64 {
    if data.total_lessons > 0 {
        data.completed_lessons.len() as i64 * 100 / data.total_lessons
    } else {
        0
    }
}

fn continue_card(data: &DashboardData) -> String {
    match data.first_incomplete_lesson {
        Some(lesson_id) => {
            let lesson = data
                .lessons_per_module
                .iter()
                .flatten()
                .find(|l| l.id == lesson_id);

            let module = data
                .modules
                .iter()
                .find(|m| lesson.map(|l| l.module_id == m.id).unwrap_or(false));

            let title = lesson.map(|l| l.title.clone()).unwrap_or_default();
            let label = module
                .map(|m| format!("MODULE {:02} — {}", m.order_index, m.title))
                .unwrap_or_default();
            let view_module = module
                .map(|m| format!(r##"<a href="/module/{}" class="btn btn-ghost">VIEW MODULE</a>"##, m.id))
                .unwrap_or_default();

            format!(
                r##"<div class="continue-card">
    <span class="module-label">{label}</span>
    <h3>{title}</h3>
    <div class="continue-actions">
        <a href="/lesson/{id}" class="btn btn-primary">CONTINUE</a>
        {view_module}
    </div>
</div>"##,
                label = super::layout::escape(&label),
                title = super::layout::escape(&title),
                id = lesson_id,
                view_module = view_module
            )
        }
        None => r##"<div class="continue-card">
    <h3>CURRICULUM COMPLETE</h3>
    <p>You have finished every lesson. Nice work.</p>
</div>"##
            .to_string(),
    }
}

fn module_rows(data: &DashboardData) -> String {
    let mut rows = String::new();

    // Whether the NEXT module is open. A module stays open once you have
    // started it or finished it, so a fresh account still sees module 01 and
    // everything you have actually begun — not a single dead row.
    let mut next_is_open = true;

    for (i, module) in data.modules.iter().enumerate() {
        let lessons = data.lessons_per_module.get(i).cloned().unwrap_or_default();
        let done = lessons
            .iter()
            .filter(|l| data.completed_lessons.contains(&l.id))
            .count();
        let total = lessons.len();
        let complete = total > 0 && done == total;
        let started = done > 0;

        let (state, status) = if complete {
            (StatusState::Completed, "completed")
        } else if started {
            (StatusState::InProgress, "in-progress")
        } else if next_is_open {
            (StatusState::Available, "available")
        } else {
            (StatusState::Locked, "locked")
        };

        next_is_open = started || complete;

        let percent = if total > 0 {
            done as i64 * 100 / total as i64
        } else {
            0
        };

        // Locked modules stay readable — a lock is a recommendation, not a
        // wall. Without this the only way in is to finish the previous module,
        // which leaves a new account unable to look ahead at all.
        let title = if state == StatusState::Locked {
            format!(
                r##"<a href="/module/{}" class="module-title locked-title" title="Not started yet — you can still read ahead">{}{}</a>"##,
                module.id,
                lock_glyph(),
                super::layout::escape(&module.title)
            )
        } else {
            format!(
                r##"<a href="/module/{}" class="module-title">{}</a>"##,
                module.id,
                super::layout::escape(&module.title)
            )
        };

        let meta = if total > 0 {
            format!(
                r##"<span class="module-meta">{done}/{total} LESSONS</span><span class="module-bar"><span style="width:{percent}%"></span></span>"##,
                done = done,
                total = total,
                percent = percent
            )
        } else {
            r##"<span class="module-meta">Content coming soon</span>"##.to_string()
        };

        rows.push_str(&format!(
            r##"<div class="module-row {status}">
    <span class="module-num">{num:02}</span>
    {icon}
    <div class="module-body">{title}{meta}</div>
</div>"##,
            status = status,
            num = module.order_index,
            icon = status_icon(state),
            title = title,
            meta = meta
        ));
    }

    rows
}

fn lock_glyph() -> &'static str {
    r##"<span class="lock-glyph" aria-label="Locked">🔒</span>"##
}
