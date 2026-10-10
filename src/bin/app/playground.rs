//! Phase 5 — the playground: editor, RUN button, console, diagnostics.
//!
//! The web splits this across three files and so does this port:
//! `views/playground.rs` for the structure, `static/editor.js` for the
//! behaviour (the RUN state machine, diagnostic markup, jump-to-line), and
//! `.run-btn` / `.console` / `.diagnostic` in `static/style.css` for colour.
//! The states are named after the CSS ones so a mismatch is findable.

use crate::screen::{AppView, PlaygroundState};
use crate::services::sandbox::{Diagnostic, Status};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Editor, EditorState};
use gpui_kit::component::{Disableable, Theme};
use gpui_kit::*;

/// What the editor opens with — the site's `STARTER_CODE`, move semantics
/// and all: the first program a learner is likely to break on purpose.
pub const STARTER_CODE: &str = r#"fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s2}");
}"#;

/// The playground body: the editor, the actions row, the console.
///
/// The root div carries the Esc listener: the editor's own key context
/// consumes Escape for its dismissal actions but propagates it when there is
/// nothing to dismiss, so the handler on this ancestor sees it — the same
/// focus-path shape the Enter shortcut uses, one level up.
pub fn body(
    theme: &Theme,
    view: Entity<AppView>,
    state: &PlaygroundState,
    editor: &Entity<EditorState>,
    cx: &App,
) -> AnyElement {
    let cancel_view = view.clone();
    div()
        .id("playground-body")
        .flex()
        .flex_col()
        .gap_4()
        .on_key_down(move |event: &KeyDownEvent, _window, cx| {
            if event.keystroke.key == "escape" {
                cancel_view.update(cx, |view, cx| {
                    if view.playground.running {
                        crate::screen::cancel_run(view, cx);
                    }
                });
            }
        })
        .child(
            Editor::new(editor)
                .w_full()
                .h(px(340.))
                .rounded_md()
                .into_any_element(),
        )
        .child(actions(theme, view.clone(), state, editor, cx))
        .child(console(theme, view, state, editor))
        .into_any_element()
}

/// RUN, its hint and live status, plus RESET.
///
/// The eight states of `.run-btn` map onto Button's own: hover, active and
/// focus-visible are the Button's to draw (a second hover style on top panics
/// — see the handoff), and the remaining five are chosen here.
fn actions(
    theme: &Theme,
    view: Entity<AppView>,
    state: &PlaygroundState,
    editor: &Entity<EditorState>,
    cx: &App,
) -> Div {
    let empty = editor.read(cx).value().trim().is_empty();
    let running = state.running;

    // Label as the site has it: the outcome changes the word, not just the
    // colour, so success and error never ride on colour alone. While a run is
    // in flight the button becomes STOP — a live control with one job.
    let label = if running {
        "STOP"
    } else if state.result.is_some() {
        "RUN AGAIN"
    } else {
        "RUN"
    };

    // No `.loading(running)`: a loading Button is inert — it swallows clicks
    // before `on_click` sees them — and this button is the cancel control.
    let mut run = Button::new("run")
        .label(label)
        .disabled(empty && !running)
        .rounded_md()
        .min_w(px(132.))
        .min_h(px(44.))
        .text_size(px(13.))
        .font_weight(FontWeight::BOLD);

    run = if running {
        run.secondary().text_color(theme.warning)
    } else {
        match state.result.as_ref().map(|result| result.status) {
            // Success fills; error keeps the elevated surface and says so in
            // the console, exactly as the two `[data-state]` rules do.
            Some(Status::Ok) => run.success(),
            Some(_) => run.secondary().text_color(theme.primary),
            None => run.primary(),
        }
    };

    let running_view = view.clone();
    let running_editor = editor.clone();
    run = run.on_click(move |_, _window, cx| {
        crate::screen::run_program(running_view.clone(), &running_editor, cx);
    });

    let reset_view = view.clone();
    let reset_editor = editor.clone();
    let reset = Button::new("reset")
        .ghost()
        .label("RESET")
        .text_size(px(13.))
        .on_click(move |_, window, cx| {
            crate::screen::reset_editor(reset_view.clone(), &reset_editor, window, cx);
        });

    let status = if running { "compiling" } else { "" };

    div()
        .flex()
        .flex_row()
        .items_center()
        .gap_4()
        .child(run)
        .child(reset)
        .child(
            div()
                .text_size(px(11.))
                .text_color(theme.muted_foreground)
                .child(if running {
                    "ESC TO STOP"
                } else if cfg!(target_os = "macos") {
                    // The primary modifier is ⌘ on macOS and CTRL elsewhere —
                    // the same split the editor's own `secondary-enter`
                    // binding makes, which is the combination the shortcut
                    // rides on.
                    "⌘ + ENTER"
                } else {
                    "CTRL + ENTER"
                }),
        )
        .child(
            div()
                .text_size(px(11.))
                .text_color(theme.primary)
                .child(status),
        )
}

/// The console: the empty state, or timing plus whatever the run produced.
fn console(
    theme: &Theme,
    view: Entity<AppView>,
    state: &PlaygroundState,
    editor: &Entity<EditorState>,
) -> AnyElement {
    let mut panel = div()
        .id("console")
        .rounded_md()
        .border_1()
        .border_color(theme.border)
        .bg(theme.background)
        .p_4()
        .min_h(px(112.))
        .max_h(px(352.))
        .overflow_y_scroll()
        .font_family(theme.mono_font_family.clone())
        .text_size(px(13.))
        .flex()
        .flex_col()
        .gap_3();

    let Some(result) = state.result.as_ref() else {
        return panel
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
                    .py_6()
                    .text_color(theme.muted_foreground)
                    .child(
                        div()
                            .text_size(px(20.))
                            .text_color(theme.border)
                            .child(">_"),
                    )
                    .child(
                        div()
                            .text_size(px(12.))
                            .text_color(theme.muted_foreground)
                            .child(
                            "Nothing has run yet. Press RUN to compile and execute this program.",
                        ),
                    ),
            )
            .into_any_element();
    };

    // Timing first, as the site does — it is the one line that is always
    // there when something ran. A cancelled run never finished, so a timing
    // line would report zeroes as if they were measurements.
    let timing = match result.status {
        Status::Cancelled => String::new(),
        Status::Ok => format!(
            "compiled in {}ms · ran in {}ms",
            result.compile_ms, result.run_ms
        ),
        _ => format!("compiled in {}ms", result.compile_ms),
    };
    let timing = match state.elapsed_ms {
        Some(elapsed) if !timing.is_empty() => format!("{timing} · {elapsed}ms round trip"),
        _ => timing,
    };
    if !timing.is_empty() {
        panel = panel.child(
            div()
                .text_size(px(11.))
                .text_color(theme.muted_foreground)
                .child(timing),
        );
    }

    if let Some(message) = &result.message {
        panel = panel.child(notice(theme, message));
    }

    for (index, diagnostic) in result.diagnostics.iter().enumerate() {
        panel = panel.child(diagnostic_card(
            theme,
            view.clone(),
            editor,
            diagnostic,
            index,
            state.explanation_open,
        ));
    }

    if !result.stderr.trim().is_empty() {
        panel = panel.child(stream(&result.stderr, theme.warning));
    }
    if !result.stdout.trim().is_empty() {
        panel = panel.child(stream(&result.stdout, theme.foreground));
    }
    if result.output_truncated {
        panel = panel.child(notice(
            theme,
            "Output was too long to show in full — the beginning is above.",
        ));
    }

    let produced = !result.diagnostics.is_empty()
        || !result.stdout.trim().is_empty()
        || !result.stderr.trim().is_empty()
        || result.message.is_some();
    if !produced {
        panel = panel.child(notice(theme, "This program ran without printing anything."));
    }

    panel.into_any_element()
}

/// One compiler message: code and level, the message, jump links, the
/// related `help`/`note` lines, and the official explanation behind a
/// disclosure — `renderDiagnostic` in `editor.js`, element for element.
fn diagnostic_card(
    theme: &Theme,
    view: Entity<AppView>,
    editor: &Entity<EditorState>,
    diagnostic: &Diagnostic,
    index: usize,
    open: bool,
) -> Div {
    let level = diagnostic.level.as_str();
    let level_color = match level {
        "error" => theme.primary,
        "warning" => theme.warning,
        "note" | "help" => theme.info,
        _ => theme.muted_foreground,
    };

    let mut header = div().flex().flex_row().items_center().gap_2();

    if let Some(code) = &diagnostic.code {
        header = header.child(
            div()
                .text_size(px(12.))
                .font_weight(FontWeight::BOLD)
                .text_color(theme.primary)
                .child(code.clone()),
        );
    }
    header = header.child(
        div()
            .text_size(px(11.))
            .text_color(level_color)
            .child(level.to_uppercase()),
    );

    let mut card = div()
        .rounded_sm()
        .border_1()
        .border_color(theme.border)
        .bg(theme.secondary)
        .p_3()
        .flex()
        .flex_col()
        .gap_2()
        .child(header)
        .child(
            div()
                .text_size(px(13.))
                .text_color(theme.foreground)
                .child(diagnostic.message.clone()),
        );

    // The spans are already shifted back onto the learner's numbering by the
    // sandbox, so a jump line is a line in the editor as it stands.
    let mut jumps = div().flex().flex_row().gap_3().flex_wrap();
    let mut has_jumps = false;
    for span in diagnostic.spans.iter().filter(|span| span.is_primary) {
        if span.line_start == 0 {
            continue;
        }
        let line = span.line_start;
        let jumping = editor.clone();
        jumps = jumps.child(
            Button::new(format!("diag-{index}-jump-{line}"))
                .link()
                .text_size(px(11.))
                .text_color(theme.info)
                .child(format!("jump to line {line}"))
                .on_click(move |_, window, cx| {
                    crate::screen::jump_to_line(&jumping, line, window, cx);
                }),
        );
        has_jumps = true;
    }
    if has_jumps {
        card = card.child(jumps);
    }

    for child in &diagnostic.children {
        card = card.child(
            div()
                .text_size(px(12.))
                .text_color(theme.muted_foreground)
                .child(child.message.clone()),
        );
    }

    if let Some(explanation) = &diagnostic.explanation {
        let toggling = view.clone();
        card = card.child(
            Button::new(format!("diag-{index}-why"))
                .ghost()
                .text_size(px(11.))
                .font_weight(FontWeight::BOLD)
                .text_color(theme.muted_foreground)
                .child(if open {
                    "▾ WHY THIS HAPPENS"
                } else {
                    "▸ WHY THIS HAPPENS"
                })
                .on_click(move |_, _, cx| {
                    toggling.update(cx, |view, cx| {
                        view.playground.explanation_open = !view.playground.explanation_open;
                        cx.notify();
                    });
                }),
        );
        if open {
            card = card.child(
                div()
                    .text_size(px(12.))
                    .text_color(theme.muted_foreground)
                    .child(explanation.clone()),
            );
        }
    }

    card
}

/// A muted aside with the border-left the site's `.console-notice` has.
fn notice(theme: &Theme, text: &str) -> Div {
    div()
        .pl_3()
        .text_size(px(12.))
        .text_color(theme.muted_foreground)
        .child(text.to_string())
}

/// stdout or stderr as a preformatted block.
fn stream(text: &str, color: Hsla) -> Div {
    div().text_color(color).child(text.trim_end().to_string())
}
