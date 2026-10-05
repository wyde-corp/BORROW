use super::components::progress_bar;
use super::layout::page;
use crate::services::progress::ProgressSummary;

pub fn show(data: &ProgressSummary) -> String {
    let percent = if data.total_lessons > 0 {
        data.completed_lessons * 100 / data.total_lessons
    } else {
        0
    };

    let module_rows: String = data
        .modules
        .iter()
        .map(|m| {
            format!(
                r##"<div class="progress-module">
    <span class="status-icon available" aria-hidden="true">○</span>
    <span class="module-title">{title}</span>
</div>"##,
                title = super::layout::escape(&m.title)
            )
        })
        .collect();

    let body = format!(
        r##"<header class="module-header">
    <span class="module-label">YOUR PROGRESS</span>
    <h1>Rust Fundamentals</h1>
    <div class="module-progress">
        {bar}
        <span class="progress-text">{done}/{total} LESSONS · {percent}%</span>
    </div>
</header>

<section class="stats-grid">
    <div class="stat-card"><span class="stat-value">{lessons}</span><span class="stat-label">LESSONS COMPLETED</span></div>
    <div class="stat-card"><span class="stat-value">{exercises}</span><span class="stat-label">EXERCISES COMPLETED</span></div>
    <div class="stat-card"><span class="stat-value">{modules}</span><span class="stat-label">MODULES</span></div>
    <div class="stat-card"><span class="stat-value">{xp}</span><span class="stat-label">XP EARNED</span></div>
</section>

<section class="module-section">
    <h2 class="section-heading">CURRICULUM</h2>
    <div class="lesson-list">{rows}</div>
</section>"##,
        bar = progress_bar(percent, true),
        done = data.completed_lessons,
        total = data.total_lessons,
        percent = percent,
        lessons = data.completed_lessons,
        exercises = data.completed_exercises,
        modules = data.total_modules,
        xp = data.total_xp,
        rows = module_rows
    );

    page("dashboard", "Progress", &body)
}
