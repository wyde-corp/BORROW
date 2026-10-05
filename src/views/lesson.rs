use super::components::callout;
use super::layout::{escape, page};
use crate::models::LessonContent;
use crate::services::progress::LessonPageData;

pub fn show(data: &LessonPageData) -> String {
    let title = data
        .lesson
        .as_ref()
        .map(|l| l.title.clone())
        .unwrap_or_else(|| "Lesson not found".to_string());

    let module_title = data
        .module
        .as_ref()
        .map(|m| m.title.clone())
        .unwrap_or_default();

    let module_id = data.module.as_ref().map(|m| m.id).unwrap_or(0);

    let body = format!(
        r##"<a href="/module/{module_id}" class="back-link">← {module_title}</a>

<header class="lesson-header">
    <span class="module-label">LESSON {order:02} / {total}</span>
    <h1>{title}</h1>
    {badge}
</header>

<div class="lesson-content">{sections}</div>

<nav class="lesson-pagination">
    {prev}
    <span class="page-indicator">{order} / {total}</span>
    {next}
</nav>"##,
        module_id = module_id,
        module_title = escape(&module_title),
        order = data.current_order,
        total = data.total_lessons,
        title = escape(&title),
        badge = badge(data.is_completed),
        sections = sections(data),
        prev = prev_link(data),
        next = next_link(data)
    );

    page("dashboard", &title, &body)
}

fn badge(completed: bool) -> String {
    if completed {
        r##"<span class="completion-badge">✓ COMPLETED</span>"##.to_string()
    } else {
        String::new()
    }
}

pub fn sections(data: &LessonPageData) -> String {
    let Some(lesson) = &data.lesson else {
        return callout(
            "Error",
            "This lesson could not be loaded.",
            super::components::CalloutTone::Error,
        );
    };

    let content: LessonContent = match serde_json::from_str(&lesson.content) {
        Ok(c) => c,
        Err(err) => {
            tracing::error!(lesson_id = lesson.id, %err, "failed to parse lesson content");
            return callout(
                "Error",
                "This lesson could not be loaded.",
                super::components::CalloutTone::Error,
            );
        }
    };

    content
        .sections
        .iter()
        .map(super::exercise::render_section)
        .collect::<Vec<_>>()
        .join("\n")
}

fn prev_link(data: &LessonPageData) -> String {
    data.prev_lesson
        .as_ref()
        .map(|l| format!(r##"<a href="/lesson/{}" class="btn btn-ghost">← PREV</a>"##, l.id))
        .unwrap_or_default()
}

fn next_link(data: &LessonPageData) -> String {
    if !data.is_completed {
        let id = data.lesson.as_ref().map(|l| l.id).unwrap_or(0);
        return format!(
            r##"<form action="/lesson/{id}/complete" method="post" class="inline-form">
    <button type="submit" class="btn btn-primary">MARK COMPLETE →</button>
</form>"##
        );
    }

    data.next_lesson
        .as_ref()
        .map(|l| format!(r##"<a href="/lesson/{}" class="btn btn-primary">NEXT →</a>"##, l.id))
        .unwrap_or_else(|| {
            let module_id = data.module.as_ref().map(|m| m.id).unwrap_or(0);
            format!(r##"<a href="/module/{module_id}" class="btn btn-primary">MODULE DONE →</a>"##)
        })
}
