//! `LessonSection` → elements.
//!
//! The match is exhaustive on purpose: adding a block type to the content model
//! without adding a renderer here is a compile error, not a blank spot in the
//! lesson. The 5 interactive widgets hand off to `widgets`, which needs the
//! block's state and, for controls, a view to write to.

use crate::components::{
    breakdown, callout, checklist, code_panel, diagram, steps, table, CalloutTone,
};
use crate::models::LessonSection;
use crate::screen::Screen;
use crate::widgets::{self, WidgetCtx};
use gpui_kit::component::button::{Button, ButtonVariants};
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
pub fn render_section(theme: &Theme, section: &LessonSection, ctx: &WidgetCtx<'_>) -> Div {
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
        } => {
            let panel = code_panel(theme, language, code, *status);
            // The lesson's whole pitch is "write Rust, not read about it" —
            // so every snippet is one click from the editor that runs it
            // (issue #8). Detached contexts (the content sweep) get the
            // panel alone; there is nowhere to jump to.
            match &ctx.view {
                Some(view) => {
                    let jump = view.clone();
                    let snippet = code.clone();
                    div().flex().flex_col().gap_2().child(panel).child(
                        Button::new("try-in-playground")
                            .ghost()
                            .text_size(px(12.))
                            .text_color(theme.muted_foreground)
                            .label("\u{25B6} TRY IN PLAYGROUND")
                            .on_click(move |_, _, cx| {
                                jump.update(cx, |view, cx| {
                                    view.playground.pending_code = Some(snippet.clone());
                                    view.navigate(Screen::Playground);
                                    cx.notify();
                                });
                            }),
                    )
                }
                None => panel,
            }
        }

        LessonSection::Breakdown { title, code, items } => breakdown(theme, title, code, items),

        LessonSection::Checklist { title, items } => checklist(theme, title, items),

        LessonSection::Diagram { title, code } => diagram(theme, title, code),

        LessonSection::Table { headers, rows } => table(theme, headers, rows),

        LessonSection::Steps { items } => steps(theme, items),

        // ── the five interactive widgets ──
        LessonSection::GapFill {
            intro,
            language,
            code,
            blanks,
            after,
        } => widgets::gap_fill(theme, ctx, intro, language, code, blanks, after),

        LessonSection::StepThrough {
            title,
            language,
            code,
            steps,
        } => widgets::step_through(theme, ctx, title, language, code, steps),

        LessonSection::GuessRun {
            prompt,
            language,
            code,
            options,
            correct,
            explanation,
            ..
        } => widgets::guess_run(
            theme,
            ctx,
            prompt,
            language,
            code,
            options,
            *correct,
            explanation,
        ),

        LessonSection::Dispatch {
            title,
            language,
            code,
            steps,
        } => widgets::dispatch(theme, ctx, title, language, code, steps),

        LessonSection::Pipeline {
            title,
            language,
            code,
            steps,
        } => widgets::pipeline(theme, ctx, title, language, code, steps),
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
