//! `LessonSection` → elements.
//!
//! The match is exhaustive on purpose: adding a block type to the content model
//! without adding a renderer here is a compile error, not a blank spot in the
//! lesson. The 5 interactive widgets render their readable content for now —
//! prompt, code and explanation — and gain interaction later, rather than
//! shipping a control that does nothing.

use crate::components::{
    breakdown, callout, checklist, code_panel, diagram, label, steps, table, CalloutTone,
};
use crate::models::LessonSection;
use gpui_kit::component::Theme;
use gpui_kit::*;

/// Chooses a tone from the callout's title, exactly as the site's
/// `tone_for` does — the content has never carried a tone field.
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

/// Renders one block.
pub fn render_section(theme: &Theme, section: &LessonSection) -> Div {
    match section {
        LessonSection::Text { text } => paragraphs(theme, text),
        LessonSection::Paragraphs { items } => {
            let mut out = div().flex().flex_col().gap_3();
            for item in items {
                out = out.child(plain_text(theme, item));
            }
            out
        }

        LessonSection::Callout { title, text } => callout(theme, title, text, tone_for(title)),

        LessonSection::Code {
            language,
            code,
            status,
        } => code_panel(theme, language, code, *status),

        LessonSection::Breakdown { title, code, items } => breakdown(theme, title, code, items),

        LessonSection::Checklist { title, items } => checklist(theme, title, items),

        LessonSection::Diagram { title, code } => diagram(theme, title, code),

        LessonSection::Table { headers, rows } => table(theme, headers, rows),

        LessonSection::Steps { items } => steps(theme, items),

        // ── interactive widgets, rendered as their readable content ──
        LessonSection::GapFill {
            intro,
            language,
            code,
            blanks,
            after,
        } => widget(
            theme,
            "Fill the blanks",
            intro,
            language,
            code,
            &blanks
                .iter()
                .map(|b| b.hint.clone())
                .filter(|h| !h.is_empty())
                .collect::<Vec<_>>(),
            after,
        ),

        LessonSection::StepThrough {
            title,
            language,
            code,
            steps,
        } => widget(
            theme,
            title,
            "",
            language,
            code,
            &steps.iter().map(|s| s.note.clone()).collect::<Vec<_>>(),
            "",
        ),

        LessonSection::GuessRun {
            prompt,
            language,
            code,
            options,
            explanation,
            ..
        } => widget(
            theme,
            "Guess what it does",
            prompt,
            language,
            code,
            &options.clone(),
            explanation,
        ),

        LessonSection::Dispatch {
            title,
            language,
            code,
            steps,
        } => widget(
            theme,
            title,
            "",
            language,
            code,
            &steps
                .iter()
                .map(|s| format!("static: {} — dynamic: {}", s.static_note, s.dynamic_note))
                .collect::<Vec<_>>(),
            "",
        ),

        LessonSection::Pipeline {
            title,
            language,
            code,
            steps,
        } => widget(
            theme,
            title,
            "",
            language,
            code,
            &steps.iter().map(|s| s.note.clone()).collect::<Vec<_>>(),
            "",
        ),
    }
}

/// Prose split into paragraphs on blank lines, matching how the site treats
/// `\n\n` inside a `text` block.
fn paragraphs(theme: &Theme, text: &str) -> Div {
    let mut out = div().flex().flex_col().gap_3();

    for paragraph in text.split("\n\n") {
        if !paragraph.trim().is_empty() {
            out = out.child(plain_text(theme, paragraph.trim()));
        }
    }

    out
}

fn plain_text(theme: &Theme, text: &str) -> Div {
    div()
        .text_size(px(13.5))
        .text_color(theme.foreground)
        .child(text.to_string())
}

/// A heading, an optional intro, the code, an optional list of points, and
/// closing text.
///
/// Used by all five interactive widgets until they gain their own controls —
/// the point is that the lesson still reads in order, with nothing missing but
/// the interaction itself.
fn widget(
    theme: &Theme,
    title: &str,
    intro: &str,
    language: &str,
    code: &str,
    points: &[String],
    after: &str,
) -> Div {
    let mut out = div()
        .my_4()
        .rounded_md()
        .border_1()
        .border_color(theme.border)
        .bg(theme.secondary)
        .p_4()
        .flex()
        .flex_col()
        .gap_3()
        .child(label(theme, title));

    if !intro.trim().is_empty() {
        out = out.child(plain_text(theme, intro.trim()));
    }

    out = out.child(
        code_panel(theme, language, code, crate::models::CodeStatus::Valid)
            .mx_0()
            .my_0(),
    );

    if !points.is_empty() {
        let mut list = div().flex().flex_col().gap_2();
        for point in points {
            list = list.child(
                div()
                    .flex()
                    .flex_row()
                    .gap_2()
                    .child(
                        div()
                            .text_size(px(12.))
                            .text_color(theme.primary)
                            .child("›"),
                    )
                    .child(
                        div()
                            .text_size(px(13.))
                            .text_color(theme.foreground)
                            .child(point.clone()),
                    ),
            );
        }
        out = out.child(list);
    }

    if !after.trim().is_empty() {
        out = out.child(plain_text(theme, after.trim()));
    }

    out
}
