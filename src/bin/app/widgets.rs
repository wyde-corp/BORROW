//! The five interactive lesson widgets.
//!
//! Ported from `static/interactive.js` — the behaviour — and the widget
//! blocks in `views/components.rs` — the structure. Two deliberate
//! differences, both because of what gpui can reach from here:
//!
//! - gap verdicts are computed on CHECK rather than per keystroke; gpui's
//!   single-line input exposes no change hook we can subscribe to from a
//!   render, and the site's own wrong-blank flag only ever fired on blur
//!   anyway;
//! - guess-then-run keeps its own smaller console, as the site's does: no
//!   gutter, no jump links, no storage.
//!
//! With no view behind them — the course sweep, which has no window — a
//! widget renders the readable fallback the site shows with JavaScript off,
//! so nothing in a lesson is ever unreadable.

use crate::components::{code_panel, label};
use crate::models::{DispatchStep, GapBlank, PipelineStep, Step};
use crate::screen::AppView;
use crate::services::sandbox::{self, RunResult};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::{Disableable, Theme};
use gpui_kit::*;

/// One block's interaction state, indexed by the block's place in its lesson.
///
/// One struct for all five widgets rather than five, because a block is only
/// ever one of them: the fields never meet in the same widget.
#[derive(Default)]
pub struct BlockState {
    /// gap-fill: the verdict per blank after CHECK — `None` means unjudged.
    pub verdicts: Vec<Option<bool>>,
    pub checked: bool,
    /// step-through / dispatch / pipeline: the step on show.
    pub step: usize,
    /// guess-then-run: the prediction, and whether it has been graded.
    pub choice: Option<usize>,
    pub guess: Option<RunResult>,
    pub running: bool,
    /// The reveal disclosure — gap answers, or the guess explanation.
    pub revealed: bool,
    /// gap-fill's inputs. Created by the lesson screen rather than here,
    /// because making one needs `&mut Window`.
    pub inputs: Vec<Entity<InputState>>,
    /// The in-flight guess run; dropping it is not offered, the button is
    /// disabled while one runs.
    pub guess_task: Option<gpui_kit::Task<RunResult>>,
}

/// What a widget needs to render: where it sits, what has happened in it, and
/// — when there is a window behind the render — somewhere to write.
pub struct WidgetCtx<'a> {
    /// The block's index in its lesson; the state is `view.widgets[index]`.
    pub index: usize,
    pub state: &'a BlockState,
    pub view: Option<Entity<AppView>>,
}

impl<'a> WidgetCtx<'a> {
    /// A context with nowhere to write: the controls render as fallback
    /// content. Used by the sweep, which has no view and no window.
    #[cfg(test)]
    pub fn detached(state: &'a BlockState) -> Self {
        Self {
            index: 0,
            state,
            view: None,
        }
    }
}

/// Whitespace in an answer is never meaningful, so it is normalised away.
/// Case is ignored only when the expected answer is entirely lowercase, so
/// `mut` accepts `MUT` but `String` does not accept `string`.
pub fn matches(given: &str, answer: &str) -> bool {
    fn normalise(text: &str) -> String {
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    let given = normalise(given);
    let answer = normalise(answer);

    if answer == answer.to_lowercase() {
        given.to_lowercase() == answer
    } else {
        given == answer
    }
}

/// Grades every blank against what is in the buffer and records the
/// verdicts.
///
/// What CHECK runs. Public so the test drives the same path rather than the
/// button's closure, which it cannot reach.
pub fn check_gaps(view: Entity<AppView>, block: usize, answers: &[String], cx: &mut App) {
    let verdicts = {
        let current = view.read(cx);
        let inputs = current
            .widgets
            .get(block)
            .map(|state| state.inputs.clone())
            .unwrap_or_default();

        inputs
            .iter()
            .enumerate()
            .map(|(position, input)| {
                let given = input.read(cx).value();
                answers.get(position).map(|answer| matches(&given, answer))
            })
            .collect::<Vec<_>>()
    };

    view.update(cx, |view, cx| {
        if let Some(state) = view.widgets.get_mut(block) {
            state.verdicts = verdicts;
            state.checked = true;
        }
        cx.notify();
    });
}

/// Empties every blank and forgets the last verdict.
pub fn reset_gaps(view: Entity<AppView>, block: usize, window: &mut Window, cx: &mut App) {
    let inputs = view
        .read(cx)
        .widgets
        .get(block)
        .map(|state| state.inputs.clone())
        .unwrap_or_default();

    for input in &inputs {
        input.update(cx, |state, cx| state.set_value("", window, cx));
    }

    view.update(cx, |view, cx| {
        if let Some(state) = view.widgets.get_mut(block) {
            state.verdicts.clear();
            state.checked = false;
        }
        cx.notify();
    });
}

/// Fill the blanks: an inline input per `____`, checked per blank.
pub fn gap_fill(
    theme: &Theme,
    ctx: &WidgetCtx<'_>,
    intro: &str,
    language: &str,
    code: &str,
    blanks: &[GapBlank],
    after: &str,
) -> Div {
    let mut out = frame(theme, "FILL THE GAPS");

    if !intro.trim().is_empty() {
        out = out.child(text(theme, intro.trim(), theme.foreground));
    }

    // The code, with each `____` replaced by its input — laid out line by
    // line so a gap inside a line stays inside it.
    let mut body = div()
        .px_3()
        .py_2()
        .font_family(theme.mono_font_family.clone())
        .text_size(px(12.5))
        .flex()
        .flex_col()
        .gap_1();

    let interactive = ctx.view.is_some() && !ctx.state.inputs.is_empty();
    let panel = div()
        .rounded_md()
        .border_1()
        .border_color(theme.border)
        .bg(theme.background)
        .flex()
        .flex_col()
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .px_3()
                .py_2()
                .border_b_1()
                .border_color(theme.border)
                .child(label(theme, language.to_uppercase())),
        );
    for line in code.split('\n') {
        let mut flow = div().flex().flex_row().flex_wrap().items_center().gap_1();
        for (position, part) in line.split("____").enumerate() {
            if position > 0 {
                flow = flow.child(blank(theme, ctx, position - 1, blanks, interactive));
            }
            if !part.is_empty() {
                flow = flow.child(text(theme, part, theme.foreground));
            }
        }
        body = body.child(flow);
    }
    out = out.child(panel.child(body));

    if let Some(view) = ctx.view.clone() {
        let block = ctx.index;
        let answers: Vec<String> = blanks.iter().map(|blank| blank.answer.clone()).collect();
        let total = blanks.len();

        let checking = view.clone();
        let gradeable = answers.clone();
        let check = Button::new(format!("widget-{block}-check"))
            .primary()
            .label("CHECK")
            .on_click(move |_, _, cx| {
                check_gaps(checking.clone(), block, &gradeable, cx);
            });

        let resetting = view.clone();
        let reset = Button::new(format!("widget-{block}-gap-reset"))
            .ghost()
            .label("RESET")
            .on_click(move |_, window, cx| {
                reset_gaps(resetting.clone(), block, window, cx);
            });

        let mut status = div()
            .text_size(px(12.5))
            .text_color(theme.muted_foreground)
            .child("");
        if ctx.state.checked {
            let right = ctx
                .state
                .verdicts
                .iter()
                .filter(|verdict| **verdict == Some(true))
                .count();
            let message = if right == total && total > 0 {
                "All correct.".to_string()
            } else {
                format!("{right} of {total} correct.")
            };
            status = status.child(message);
        }

        out = out.child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_3()
                .flex_wrap()
                .child(check)
                .child(reset)
                .child(status),
        );
    }

    // The answers, behind the same disclosure the site puts them in.
    let answers: Vec<(String, String)> = blanks
        .iter()
        .map(|blank| (blank.answer.clone(), blank.hint.clone()))
        .collect();
    out = out.child(disclosure(theme, ctx, "SHOW ANSWERS", |theme| {
        let mut list = div().flex().flex_col().gap_1();
        for (answer, hint) in &answers {
            let mut row = div().flex().flex_row().gap_2().child(
                div()
                    .font_family(theme.mono_font_family.clone())
                    .text_size(px(12.5))
                    .text_color(theme.warning)
                    .child(answer.clone()),
            );
            if !hint.trim().is_empty() {
                row = row.child(text(theme, &format!("— {hint}"), theme.muted_foreground));
            }
            list = list.child(row);
        }
        list
    }));

    if !after.trim().is_empty() {
        out = out.child(text(theme, after.trim(), theme.muted_foreground));
    }

    out
}

/// One blank: the input, and its verdict once CHECK has judged it.
fn blank(
    theme: &Theme,
    ctx: &WidgetCtx<'_>,
    position: usize,
    blanks: &[GapBlank],
    interactive: bool,
) -> Div {
    let hint = blanks.get(position).map(|blank| blank.hint.clone());

    // No view behind the render: the marker stays text, which is how the
    // course sweep and any future print view still read the gap.
    if !interactive {
        return div().text_color(theme.warning).child("____");
    }

    let Some(input) = ctx.state.inputs.get(position) else {
        return div().text_color(theme.warning).child("____");
    };

    let width = blanks
        .get(position)
        .map(|blank| blank.answer.chars().count().max(6))
        .unwrap_or(6);

    let mut row = div().flex().flex_row().items_center().gap_1().child(
        Input::new(input)
            .id(format!("gap-{}-{position}", ctx.index))
            .w(px(8. * width as f32 + 14.)),
    );

    // The verdict rides next to the field: gpui's Input owns its own border,
    // so colouring the box itself would mean fighting the component. The
    // verdict is what the learner is reading for anyway.
    match ctx.state.verdicts.get(position).copied().flatten() {
        Some(true) => row = row.child(glyph(theme, "✓", theme.success)),
        Some(false) => row = row.child(glyph(theme, "✗", theme.danger)),
        None => {
            if let Some(hint) = hint {
                if !hint.trim().is_empty() {
                    row = row.child(
                        div()
                            .text_size(px(11.))
                            .text_color(theme.muted_foreground)
                            .child(format!("· {hint}")),
                    );
                }
            }
        }
    }

    row
}

/// A snippet stepped through one line-group at a time.
pub fn step_through(
    theme: &Theme,
    ctx: &WidgetCtx<'_>,
    title: &str,
    language: &str,
    code: &str,
    steps: &[Step],
) -> Div {
    let mut out = frame(theme, "STEP THROUGH").child(label(theme, title));

    // Clamped before it is used anywhere: the step is stored, not trusted —
    // a lesson edit can leave a state pointing past the end, and `step + 1`
    // over the counter would panic rather than land on the last step.
    let step = ctx.state.step.min(steps.len().saturating_sub(1));
    let Some(active) = steps.get(step) else {
        return out;
    };

    out = out.child(code_block(theme, language, code, &active.lines.to_vec()));

    match ctx.view.clone() {
        Some(view) => {
            out = out.child(controls(theme, ctx, &view, step, steps.len(), None));
            out = out.child(note(theme, &active.note));
        }
        // The fallback list still tells the whole story, as it does on the
        // site with JavaScript off.
        None => {
            let mut list = div().flex().flex_col().gap_2();
            for step in steps {
                list = list.child(ref_line(theme, &step.lines, &step.note));
            }
            out = out.child(list);
        }
    }

    out
}

/// Static versus dynamic dispatch over the same call, advanced in tandem.
pub fn dispatch(
    theme: &Theme,
    ctx: &WidgetCtx<'_>,
    title: &str,
    language: &str,
    code: &str,
    steps: &[DispatchStep],
) -> Div {
    let mut out = frame(theme, "STATIC vs DYNAMIC").child(label(theme, title));

    // Clamped before it is used anywhere: the step is stored, not trusted —
    // a lesson edit can leave a state pointing past the end, and `step + 1`
    // over the counter would panic rather than land on the last step.
    let step = ctx.state.step.min(steps.len().saturating_sub(1));
    let Some(active) = steps.get(step) else {
        return out;
    };

    out = out.child(code_block(theme, language, code, &active.lines.to_vec()));

    match ctx.view.clone() {
        Some(view) => {
            out = out.child(controls(theme, ctx, &view, step, steps.len(), None));
            out = out.child(
                div()
                    .flex()
                    .flex_row()
                    .gap_3()
                    .flex_wrap()
                    .child(column(
                        theme,
                        "STATIC DISPATCH · generics",
                        &active.static_note,
                        theme.primary,
                    ))
                    .child(column(
                        theme,
                        "DYNAMIC DISPATCH · dyn Trait",
                        &active.dynamic_note,
                        theme.info,
                    )),
            );
        }
        None => {
            let mut list = div().flex().flex_col().gap_3();
            for step in steps {
                list = list.child(ref_line(theme, &step.lines, "static"));
                list = list.child(text(theme, &step.static_note, theme.foreground));
                list = list.child(text(theme, "dynamic", theme.info));
                list = list.child(text(theme, &step.dynamic_note, theme.foreground));
            }
            out = out.child(list);
        }
    }

    out
}

/// A pipeline of adapters, first lazy and then driven by a consumer.
pub fn pipeline(
    theme: &Theme,
    ctx: &WidgetCtx<'_>,
    title: &str,
    language: &str,
    code: &str,
    steps: &[PipelineStep],
) -> Div {
    let mut out = frame(theme, "LAZY → EAGER").child(label(theme, title));

    // Clamped before it is used anywhere: the step is stored, not trusted —
    // a lesson edit can leave a state pointing past the end, and `step + 1`
    // over the counter would panic rather than land on the last step.
    let step = ctx.state.step.min(steps.len().saturating_sub(1));
    let Some(active) = steps.get(step) else {
        return out;
    };

    out = out.child(code_block(theme, language, code, &active.lines.to_vec()));

    match ctx.view.clone() {
        Some(view) => {
            let eager = active.phase.eq_ignore_ascii_case("eager");
            let badge = div()
                .px_2()
                .py_1()
                .rounded_sm()
                .border_1()
                .border_color(if eager { theme.success } else { theme.warning })
                .text_size(px(10.))
                .font_weight(FontWeight::BOLD)
                .text_color(if eager { theme.success } else { theme.warning })
                .child(active.phase.to_uppercase());

            out = out.child(controls(theme, ctx, &view, step, steps.len(), Some(badge)));
            out = out.child(note(theme, &active.note));
        }
        None => {
            let mut list = div().flex().flex_col().gap_2();
            for step in steps {
                let eager = step.phase.eq_ignore_ascii_case("eager");
                list = list.child(
                    div()
                        .flex()
                        .flex_row()
                        .gap_2()
                        .child(glyph(
                            theme,
                            &step.phase.to_uppercase(),
                            if eager { theme.success } else { theme.warning },
                        ))
                        .child(ref_line(theme, &step.lines, &step.note)),
                );
            }
            out = out.child(list);
        }
    }

    out
}

/// Predict what the program does, then run it and compare.
///
/// Eight arguments because the block has seven fields and the two shared
/// ones; splitting them into a struct would only hide what is passed.
#[allow(clippy::too_many_arguments)]
pub fn guess_run(
    theme: &Theme,
    ctx: &WidgetCtx<'_>,
    prompt: &str,
    language: &str,
    code: &str,
    options: &[String],
    correct: usize,
    explanation: &str,
) -> Div {
    let mut out = frame(theme, "GUESS, THEN RUN").child(label(theme, prompt));
    out = out.child(code_panel(
        theme,
        language,
        code,
        crate::models::CodeStatus::Valid,
    ));

    let Some(view) = ctx.view.clone() else {
        // Nothing to click: the options and the explanation still read.
        for (position, option) in options.iter().enumerate() {
            let mark = if position == correct { "→" } else { " " };
            out = out.child(text(theme, &format!("{mark} {option}"), theme.foreground));
        }
        out = out.child(text(theme, explanation, theme.muted_foreground));
        return out;
    };

    let block = ctx.index;
    out = out.child(
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(text(theme, "What will happen?", theme.muted_foreground))
            .child({
                let mut list = div().flex().flex_col().gap_2();
                for (position, option) in options.iter().enumerate() {
                    let chosen = ctx.state.choice == Some(position);
                    let picking = view.clone();
                    let option_text = option.clone();

                    list = list.child(
                        Button::new(format!("widget-{block}-option-{position}"))
                            .ghost()
                            .rounded_md()
                            .border_1()
                            .border_color(if chosen { theme.primary } else { theme.border })
                            .bg(if chosen {
                                theme.primary.opacity(0.12)
                            } else {
                                theme.secondary
                            })
                            .p_3()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_3()
                            .child(glyph(
                                theme,
                                if chosen { "●" } else { "○" },
                                if chosen {
                                    theme.primary
                                } else {
                                    theme.muted_foreground
                                },
                            ))
                            .child(
                                div()
                                    .text_size(px(13.))
                                    .text_color(theme.foreground)
                                    .child(option_text),
                            )
                            .on_click(move |_, _, cx| {
                                picking.update(cx, |view, cx| {
                                    if let Some(state) = view.widgets.get_mut(block) {
                                        state.choice = Some(position);
                                    }
                                    cx.notify();
                                });
                            }),
                    );
                }
                list
            }),
    );

    // The submit button is inert until there is a prediction — the site's is
    // disabled from the first paint for the same reason.
    let can_guess = ctx.state.choice.is_some();
    let running = ctx.state.running;
    let submit = Button::new(format!("widget-{block}-guess"))
        .primary()
        .label(if running {
            "RUNNING…"
        } else {
            "GUESS, THEN RUN"
        })
        .disabled(!can_guess || running)
        .loading(running)
        .on_click({
            let target = view.clone();
            let program = code.to_string();
            move |_, _, cx| run_guess(target.clone(), block, program.clone(), cx)
        });

    let status = if running { "compiling" } else { "" };
    out = out.child(
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_3()
            .child(submit)
            .child(
                div()
                    .text_size(px(12.5))
                    .text_color(theme.muted_foreground)
                    .child(status),
            ),
    );

    if let Some(result) = ctx.state.guess.as_ref() {
        let right = ctx.state.choice == Some(correct);
        out = out.child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .child(
                    div()
                        .font_weight(FontWeight::BOLD)
                        .text_color(if right { theme.success } else { theme.primary })
                        .child(if right {
                            "✓ Your prediction was right."
                        } else {
                            "✗ Not quite — here is what actually happened."
                        }),
                )
                .child(text(theme, explanation, theme.muted_foreground))
                .child(run_output(theme, result)),
        );
    }

    out
}

/// Runs a guess through the same sandbox the playground uses.
///
/// Public so the test can drive the path the button takes rather than
/// simulating a click it cannot reach.
pub fn run_guess(view: Entity<AppView>, block: usize, code: String, cx: &mut App) {
    let executor = cx.background_executor().clone();
    let target = view.clone();
    let task = cx.spawn(async move |cx| {
        let result = executor.spawn(async move { sandbox::run(&code) }).await;

        cx.update(|cx| {
            target.update(cx, |view, cx| {
                if let Some(state) = view.widgets.get_mut(block) {
                    state.guess = Some(result.clone());
                    state.running = false;
                }
                cx.notify();
            });
        });

        result
    });

    view.update(cx, |view, cx| {
        if let Some(state) = view.widgets.get_mut(block) {
            state.guess = None;
            state.running = true;
            state.guess_task = Some(task);
        }
        cx.notify();
    });
}

// ── shared pieces ───────────────────────────────────────────────────────

/// The frame every widget sits in: badge, then whatever the widget adds.
fn frame(theme: &Theme, badge: &str) -> Div {
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
        .child(
            div().flex().flex_row().items_center().child(
                div()
                    .text_size(px(10.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.muted_foreground)
                    .child(badge.to_string()),
            ),
        )
}

/// The code panel the stepped widgets use: one row per line, so the active
/// lines can be lit without re-parsing anything.
fn code_block(theme: &Theme, language: &str, code: &str, active: &[usize]) -> Div {
    let mut header = div()
        .flex()
        .flex_row()
        .items_center()
        .px_3()
        .py_2()
        .border_b_1()
        .border_color(theme.border)
        .child(label(theme, language.to_uppercase()));

    if !active.is_empty() {
        header = header.child(
            div()
                .ml_auto()
                .px_2()
                .py(px(1.))
                .rounded_sm()
                .bg(theme.primary)
                .text_size(px(10.))
                .font_weight(FontWeight::BOLD)
                .text_color(theme.primary_foreground)
                .child(format!("LINES {}", join_lines(active))),
        );
    }

    let mut body = div()
        .px_3()
        .py_2()
        .font_family(theme.mono_font_family.clone())
        .text_size(px(12.5))
        .flex()
        .flex_col();

    for (position, line) in code.lines().enumerate() {
        let number = position + 1;
        let on = active.contains(&number);
        // A space rather than nothing: an empty line still has to take a
        // row's height, or the numbering and the highlight drift.
        let shown = if line.is_empty() { " " } else { line };

        body = body.child(
            div()
                .flex()
                .flex_row()
                .gap_3()
                .rounded_sm()
                .bg(if on {
                    theme.warning.opacity(0.16)
                } else {
                    theme.transparent
                })
                .border_l_2()
                .border_color(if on { theme.warning } else { theme.transparent })
                .child(
                    div()
                        .w(px(24.))
                        .text_color(theme.muted_foreground)
                        .child(number.to_string()),
                )
                .child(div().text_color(theme.foreground).child(shown.to_string())),
        );
    }

    div()
        .rounded_md()
        .border_1()
        .border_color(theme.border)
        .bg(theme.background)
        .overflow_hidden()
        .flex()
        .flex_col()
        .child(header)
        .child(body)
}

/// PREV, the counter, NEXT — with an optional leading badge (the pipeline's
/// phase chip), and disabled ends, exactly as the site's controls are.
fn controls(
    theme: &Theme,
    ctx: &WidgetCtx<'_>,
    view: &Entity<AppView>,
    step: usize,
    total: usize,
    leading: Option<Div>,
) -> Div {
    let block = ctx.index;

    let stepping_back = view.clone();
    let prev = Button::new(format!("widget-{block}-prev"))
        .ghost()
        .label("← PREV")
        .disabled(step == 0)
        .on_click(move |_, _, cx| {
            stepping_back.update(cx, |view, cx| {
                if let Some(state) = view.widgets.get_mut(block) {
                    state.step = state.step.saturating_sub(1);
                }
                cx.notify();
            });
        });

    let stepping_on = view.clone();
    let next = Button::new(format!("widget-{block}-next"))
        .ghost()
        .label("NEXT →")
        .disabled(step + 1 >= total)
        .on_click(move |_, _, cx| {
            stepping_on.update(cx, |view, cx| {
                if let Some(state) = view.widgets.get_mut(block) {
                    state.step = state.step.saturating_add(1);
                }
                cx.notify();
            });
        });

    let mut row = div().flex().flex_row().items_center().gap_3().flex_wrap();

    if let Some(leading) = leading {
        row = row.child(leading);
    }

    row.child(prev)
        .child(
            div()
                .flex_1()
                .text_size(px(12.))
                .text_color(theme.muted_foreground)
                .child(format!("Step {} of {total}", step + 1)),
        )
        .child(next)
}

/// The step note, in the boxed panel the site uses for it.
fn note(theme: &Theme, body: &str) -> Div {
    div()
        .px_3()
        .py_3()
        .rounded_sm()
        .bg(theme.background)
        .text_size(px(13.))
        .text_color(theme.foreground)
        .child(body.to_string())
}

/// "line 4, 6" plus the note, for the no-controls fallback lists.
fn ref_line(theme: &Theme, lines: &[usize], body: &str) -> Div {
    div()
        .flex()
        .flex_row()
        .items_baseline()
        .gap_2()
        .child(
            div()
                .text_size(px(11.))
                .text_color(theme.primary)
                .child(format!("line {}", join_lines(lines))),
        )
        .child(
            div()
                .text_size(px(13.))
                .text_color(theme.foreground)
                .child(body.to_string()),
        )
}

fn join_lines(lines: &[usize]) -> String {
    lines
        .iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

fn column(theme: &Theme, heading: &str, body: &str, accent: Hsla) -> Div {
    div()
        .flex_1()
        .min_w(px(200.))
        .rounded_md()
        .border_1()
        .border_color(theme.border)
        .bg(theme.background)
        .p_3()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .text_size(px(10.))
                .font_weight(FontWeight::BOLD)
                .text_color(accent)
                .child(heading.to_string()),
        )
        .child(
            div()
                .text_size(px(13.))
                .text_color(theme.foreground)
                .child(body.to_string()),
        )
}

/// A disclosure over content: hidden until the button is pressed, and plain
/// content when there is no view to press it — which is how the sweep still
/// reads what is behind it.
fn disclosure(
    theme: &Theme,
    ctx: &WidgetCtx<'_>,
    label_text: &str,
    open: impl Fn(&Theme) -> Div,
) -> Div {
    let mut out = div().flex().flex_col().gap_2();

    let (revealed, view) = match ctx.view.clone() {
        Some(view) => (ctx.state.revealed, Some(view)),
        None => (true, None),
    };

    if let Some(view) = view {
        let block = ctx.index;
        let toggling = view;
        let open_now = revealed;
        out = out.child(
            Button::new(format!("widget-{block}-reveal"))
                .ghost()
                .text_size(px(11.))
                .font_weight(FontWeight::BOLD)
                .text_color(theme.warning)
                .child(format!("{} {label_text}", if open_now { "▾" } else { "▸" }))
                .on_click(move |_, _, cx| {
                    toggling.update(cx, |view, cx| {
                        if let Some(state) = view.widgets.get_mut(block) {
                            state.revealed = !state.revealed;
                        }
                        cx.notify();
                    });
                }),
        );
    }

    if revealed {
        out = out.child(open(theme));
    }

    out
}

fn text(_theme: &Theme, body: &str, color: Hsla) -> Div {
    div()
        .text_size(px(13.))
        .text_color(color)
        .child(body.to_string())
}

fn glyph(_theme: &Theme, mark: &str, color: Hsla) -> Div {
    div()
        .text_size(px(12.))
        .text_color(color)
        .child(mark.to_string())
}

/// The guess widget's console: diagnostics, stderr, stdout — the site's
/// smaller copy of the editor's, with no gutter and no jump links.
fn run_output(theme: &Theme, result: &RunResult) -> Div {
    let mut panel = div()
        .rounded_md()
        .border_1()
        .border_color(theme.border)
        .bg(theme.background)
        .p_3()
        .font_family(theme.mono_font_family.clone())
        .text_size(px(12.5))
        .flex()
        .flex_col()
        .gap_2();

    if let Some(message) = &result.message {
        panel = panel.child(
            div()
                .text_color(theme.muted_foreground)
                .child(message.clone()),
        );
    }

    for diagnostic in &result.diagnostics {
        let mut card = div()
            .rounded_sm()
            .border_1()
            .border_color(theme.border)
            .bg(theme.secondary)
            .p_3()
            .flex()
            .flex_col()
            .gap_2();

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
                .text_color(if diagnostic.level == "error" {
                    theme.primary
                } else if diagnostic.level == "warning" {
                    theme.warning
                } else {
                    theme.muted_foreground
                })
                .child(diagnostic.level.to_uppercase()),
        );

        card = card.child(header).child(
            div()
                .text_size(px(12.5))
                .text_color(theme.foreground)
                .child(diagnostic.message.clone()),
        );

        for child in &diagnostic.children {
            card = card.child(
                div()
                    .text_size(px(12.))
                    .text_color(theme.muted_foreground)
                    .child(child.message.clone()),
            );
        }

        panel = panel.child(card);
    }

    if !result.stderr.trim().is_empty() {
        panel = panel.child(
            div()
                .text_color(theme.warning)
                .child(result.stderr.trim_end().to_string()),
        );
    }
    if !result.stdout.trim().is_empty() {
        panel = panel.child(
            div()
                .text_color(theme.foreground)
                .child(result.stdout.trim_end().to_string()),
        );
    }
    if result.diagnostics.is_empty()
        && result.stdout.trim().is_empty()
        && result.stderr.trim().is_empty()
        && result.message.is_none()
    {
        panel = panel.child(
            div()
                .text_color(theme.muted_foreground)
                .child("This program ran without printing anything."),
        );
    }

    panel
}

#[cfg(test)]
mod tests {
    // No `use super::*`: the parent does `use gpui_kit::*`, which carries
    // gpui's own `test` attribute and would silently replace Rust's.
    use super::matches;

    /// Whitespace never counts, and case only when the answer is lowercase —
    /// `mut` accepts `MUT`, `String` does not accept `string`.
    #[test]
    fn blank_answers_compare_the_way_the_site_does() {
        assert!(matches("MUT", "mut"));
        assert!(matches("  mut\t", "mut"));
        assert!(matches("let  x", "let x"));
        assert!(!matches("string", "String"));
        assert!(!matches("mut", "mutate"));
        assert!(!matches("", "mut"));
    }
}
