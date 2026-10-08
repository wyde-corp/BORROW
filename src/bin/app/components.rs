//! The shared parts of a lesson, as `div()` builders.
//!
//! These are the desktop equivalents of `views/components.rs`. Two differences
//! from the web originals, both deliberate:
//!
//! - they take `&Theme` rather than reading it out of a render context. The
//!   builders then need no `App`, so a test can build a real theme, render every
//!   block in the course, and assert nothing panics — without a window.
//! - they return `Div`, one concrete type, so `render_section` can match on the
//!   content enum and still compile to a single value per arm. Rust's exhaustive
//!   match is what guarantees every block type has a renderer; a count in a test
//!   would only catch variants that happen to appear in the seeded course.

use gpui_kit::component::Theme;
use gpui_kit::*;

/// What a callout is for. The tones are the ones the site distinguishes; they
/// differ only in the accent bar, exactly as `.callout-*` does in `style.css`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalloutTone {
    Idea,
    Practice,
    Challenge,
    Project,
    Error,
}

impl CalloutTone {
    fn accent(self, theme: &Theme) -> Hsla {
        match self {
            CalloutTone::Idea => theme.primary,
            CalloutTone::Practice => theme.success,
            CalloutTone::Challenge => theme.warning,
            CalloutTone::Project => theme.info,
            CalloutTone::Error => theme.danger,
        }
    }

    fn title_color(self, theme: &Theme) -> Hsla {
        match self {
            CalloutTone::Idea => theme.muted_foreground,
            _ => self.accent(theme),
        }
    }
}

/// Whether a code panel is plain, deliberately broken, or its correction.
///
/// The site's lesson content carries `CodeStatus`; re-declaring it here would
/// mean keeping two enums in step, so the real one is re-used.
pub use crate::models::CodeStatus;

/// A short uppercase label, for callers that already hold a theme.
pub fn label(theme: &Theme, text: impl Into<SharedString>) -> Div {
    div()
        .text_size(px(10.))
        .text_color(theme.muted_foreground)
        .font_weight(FontWeight::BOLD)
        .child(text.into())
}

/// A code panel with the language header, the deliberate-invalidity banner,
/// and monospace body — the `.code-block` of `style.css`.
pub fn code_panel(theme: &Theme, language: &str, code: &str, status: CodeStatus) -> Div {
    let banner = match status {
        CodeStatus::Valid => None,
        CodeStatus::Broken => Some((
            "THIS DOES NOT COMPILE",
            theme.danger,
            theme.danger_foreground,
        )),
        CodeStatus::Corrected => Some(("CORRECTED", theme.success, theme.success_foreground)),
    };

    let mut header = div()
        .flex()
        .flex_row()
        .items_center()
        .gap_2()
        .px_3()
        .py_2()
        .text_size(px(11.))
        .text_color(theme.muted_foreground)
        .child(language.to_uppercase());

    if let Some((text, bg, fg)) = banner {
        header = header.child(
            div()
                .px_2()
                .py_1()
                .rounded_sm()
                .bg(bg)
                .text_color(fg)
                .text_size(px(10.))
                .font_weight(FontWeight::BOLD)
                .child(text),
        );
    }

    div()
        .my_4()
        .rounded_md()
        .border_1()
        .border_color(theme.border)
        .bg(theme.secondary)
        .overflow_hidden()
        .flex()
        .flex_col()
        .child(header.border_b_1().border_color(theme.border))
        .child(
            div()
                .px_4()
                .py_3()
                .bg(theme.background)
                .font_family(theme.mono_font_family.clone())
                .text_size(px(12.5))
                .text_color(theme.foreground)
                .child(code.to_string()),
        )
}

/// An aside with an accent bar. The bar is the only thing that differs by tone,
/// matching `.callout` in the stylesheet.
pub fn callout(theme: &Theme, title: &str, text: &str, tone: CalloutTone) -> Div {
    div()
        .my_4()
        .px_4()
        .py_3()
        .rounded_sm()
        .border_l_1()
        .border_color(tone.accent(theme))
        .bg(theme.secondary)
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .text_size(px(10.))
                .text_color(tone.title_color(theme))
                .font_weight(FontWeight::BOLD)
                .child(title.to_string()),
        )
        .child(
            div()
                .text_size(px(13.))
                .text_color(theme.foreground)
                .child(text.to_string()),
        )
}

/// A ticked list of abilities. The tick is drawn from the success token rather
/// than a colour of its own, so it stays inside the palette.
pub fn checklist(theme: &Theme, title: &str, items: &[String]) -> Div {
    let mut list = div().flex().flex_col().gap_2();

    for item in items {
        list = list.child(
            div()
                .flex()
                .flex_row()
                .items_start()
                .gap_2()
                .child(
                    div()
                        .text_size(px(13.))
                        .text_color(theme.success)
                        .child("✓"),
                )
                .child(
                    div()
                        .text_size(px(13.))
                        .text_color(theme.foreground)
                        .child(item.clone()),
                ),
        );
    }

    div()
        .my_4()
        .rounded_md()
        .border_1()
        .border_color(theme.border)
        .bg(theme.secondary)
        .p_4()
        .flex()
        .flex_col()
        .gap_3()
        .child(label(theme, title))
        .child(list)
}

/// Source on the left, per-line explanation on the right.
///
/// The two columns stack on a narrow window rather than shrinking until the
/// code is unreadable: code that no longer fits on one line has already stopped
/// doing its job.
pub fn breakdown(theme: &Theme, title: &str, code: &str, items: &[String]) -> Div {
    let mut numbered = div().flex().flex_col().gap_1();

    for (i, line) in code.lines().enumerate() {
        numbered = numbered.child(
            div()
                .flex()
                .flex_row()
                .gap_3()
                .child(
                    div()
                        .text_size(px(11.))
                        .text_color(theme.muted_foreground)
                        .w(px(22.))
                        .child((i + 1).to_string()),
                )
                .child(
                    div()
                        .text_size(px(12.5))
                        .text_color(theme.foreground)
                        .child(line.to_string()),
                ),
        );
    }

    let mut notes = div().flex().flex_col().gap_2();
    for (i, item) in items.iter().enumerate() {
        notes = notes.child(
            div()
                .flex()
                .flex_row()
                .items_start()
                .gap_2()
                .child(
                    div()
                        .text_size(px(11.))
                        .text_color(theme.primary)
                        .child(format!("{}. ", i + 1)),
                )
                .child(
                    div()
                        .text_size(px(13.))
                        .text_color(theme.foreground)
                        .child(item.clone()),
                ),
        );
    }

    div()
        .my_4()
        .rounded_md()
        .border_1()
        .border_color(theme.border)
        .bg(theme.secondary)
        .p_4()
        .flex()
        .flex_col()
        .gap_3()
        .child(label(theme, title))
        .child(
            div()
                .flex()
                .flex_col()
                .gap_4()
                .child(
                    div()
                        .font_family(theme.mono_font_family.clone())
                        .child(numbered),
                )
                .child(notes),
        )
}

/// A monospace panel for ASCII diagrams.
pub fn diagram(theme: &Theme, title: &str, code: &str) -> Div {
    div()
        .my_4()
        .rounded_md()
        .border_1()
        .border_color(theme.border)
        .bg(theme.secondary)
        .p_4()
        .flex()
        .flex_col()
        .gap_3()
        .child(label(theme, title))
        .child(
            div()
                .font_family(theme.mono_font_family.clone())
                .text_size(px(12.5))
                .text_color(theme.foreground)
                .child(code.to_string()),
        )
}

/// A data table. Tick cells (✓ and →) take the success token, as they do on
/// the site.
pub fn table(theme: &Theme, headers: &[String], rows: &[Vec<String>]) -> Div {
    let header_cells = div()
        .flex()
        .flex_row()
        .gap_4()
        .pb_2()
        .border_b_1()
        .border_color(theme.border)
        .children(headers.iter().map(|h| {
            div()
                .text_size(px(11.))
                .text_color(theme.muted_foreground)
                .font_weight(FontWeight::BOLD)
                .child(h.clone())
        }));

    let mut body = div().flex().flex_col().gap_2().pt_2();
    for row in rows {
        body = body.child(
            div()
                .flex()
                .flex_row()
                .gap_4()
                .children(row.iter().map(|cell| {
                    let tick = cell == "✓" || cell == "→";
                    div()
                        .text_size(px(13.))
                        .text_color(if tick {
                            theme.success
                        } else {
                            theme.foreground
                        })
                        .child(cell.clone())
                })),
        );
    }

    div()
        .my_4()
        .rounded_md()
        .border_1()
        .border_color(theme.border)
        .bg(theme.secondary)
        .p_4()
        .flex()
        .flex_col()
        .child(header_cells)
        .child(body)
}

/// The four states the site's `.status-icon` distinguishes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusState {
    Locked,
    Available,
    InProgress,
    Completed,
}

/// The glyph for one state — `✓`, `●`, `○` on the site.
pub fn status_glyph(state: StatusState) -> &'static str {
    match state {
        StatusState::Locked | StatusState::Available => "○",
        StatusState::InProgress => "●",
        StatusState::Completed => "✓",
    }
}

/// Its colour, from the same tokens `style.css` gives `.status-icon`.
pub fn status_color(theme: &Theme, state: StatusState) -> Hsla {
    match state {
        StatusState::Locked | StatusState::Available => theme.muted_foreground,
        StatusState::InProgress => theme.primary,
        StatusState::Completed => theme.success,
    }
}

/// `.progress-bar` — an elevated track with an accent fill at `percent`.
///
/// `large` is `.progress-bar.large`: full width instead of the 120px inline
/// bar. The track colour is `--elevated` and the fill `--accent`, as in
/// `style.css`.
pub fn progress_bar(theme: &Theme, percent: i64, large: bool) -> AnyElement {
    let bar = component::progress::Progress::new(format!("progress-{percent}-{large}"))
        .value(percent.clamp(0, 100) as f32)
        .color(theme.primary)
        .h(if large { px(8.) } else { px(6.) });

    if large {
        bar.w_full().into_any_element()
    } else {
        bar.w(px(120.)).into_any_element()
    }
}

/// An ordered list of steps.
pub fn steps(theme: &Theme, items: &[String]) -> Div {
    let mut list = div().flex().flex_col().gap_3().pl_1();

    for (i, item) in items.iter().enumerate() {
        list = list.child(
            div()
                .flex()
                .flex_row()
                .items_start()
                .gap_3()
                .child(
                    div()
                        .text_size(px(12.))
                        .text_color(theme.primary)
                        .font_weight(FontWeight::BOLD)
                        .w(px(18.))
                        .child(format!("{}.", i + 1)),
                )
                .child(
                    div()
                        .text_size(px(13.))
                        .text_color(theme.foreground)
                        .child(item.clone()),
                ),
        );
    }

    div().my_4().flex().flex_col().child(list)
}
