//! The dashboard and progress bodies, ported from the web's
//! `views/dashboard.rs` and `views/progress.rs`.
//!
//! This file only builds markup. The screens own navigation and the database,
//! and the ids coming out of SQLite are resolved to indices into
//! `AppView::modules` by the caller — so a click here sets a [`Screen`] and
//! nothing else, exactly like the module cards on home.

use crate::components::{label, progress_bar, status_color, status_glyph, StatusState};
use crate::screen::{AppView, ModuleLock, Screen};
use crate::services::progress::{DashboardData, ProgressSummary};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Theme;
use gpui_kit::*;

/// Completion as a whole-course percentage.
fn percent(done: usize, total: i64) -> i64 {
    if total > 0 {
        done as i64 * 100 / total
    } else {
        0
    }
}

/// The dashboard body: overall bar, the continue card, the module list.
///
/// `module_index` and `lesson_index` translate database ids into positions in
/// `AppView::modules`. The data comes from `services::progress::dashboard_data`,
/// which reads the same tables the site does.
///
/// `locks[i]` is the gating verdict for module `i`: a progression lock is
/// dimmed and inert (wait your turn), a premium lock is dimmed but *clicks
/// through to the paywall* — a closed door with a handle on it.
pub fn dashboard(
    theme: &Theme,
    view: Entity<AppView>,
    data: &DashboardData,
    locks: &[ModuleLock],
    module_index: impl Fn(i64) -> Option<usize>,
    lesson_index: impl Fn(i64) -> Option<(usize, usize)>,
) -> Div {
    let mut body = div().flex().flex_col().gap_6();

    // The strip the site puts in the navbar: how far through the course.
    let overall = percent(data.completed_lessons.len(), data.total_lessons);
    body = body.child(
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_4()
            .child(
                div()
                    .flex_1()
                    .min_w(px(0.))
                    .child(progress_bar(theme, overall, true)),
            )
            .child(
                div()
                    .text_size(px(12.))
                    .text_color(theme.muted_foreground)
                    .font_weight(FontWeight::BOLD)
                    .child(format!("{overall}% · {} XP", data.total_xp)),
            ),
    );

    // YOUR RUST JOURNEY
    let mut journey = div().flex().flex_col().gap_3();
    journey = journey
        .child(label(theme, "YOUR RUST JOURNEY"))
        .child(continue_card(
            theme,
            view.clone(),
            data,
            &module_index,
            &lesson_index,
        ));
    body = body.child(journey);

    // CURRICULUM
    let mut curriculum = div().flex().flex_col().gap_3();
    curriculum = curriculum.child(label(theme, "CURRICULUM"));

    let mut rows = div().flex().flex_col().gap_2();

    for (position, module) in data.modules.iter().enumerate() {
        let lessons = data
            .lessons_per_module
            .get(position)
            .cloned()
            .unwrap_or_default();
        let done = lessons
            .iter()
            .filter(|lesson| data.completed_lessons.contains(&lesson.id))
            .count();
        let total = lessons.len();
        let lock = locks.get(position).copied().unwrap_or(ModuleLock::Open);
        let is_unlocked = lock == ModuleLock::Open;

        let state = if !is_unlocked {
            StatusState::Locked
        } else if total > 0 && done == total {
            StatusState::Completed
        } else if done > 0 {
            StatusState::InProgress
        } else {
            StatusState::Available
        };

        rows = rows.child(module_row(
            theme,
            view.clone(),
            module,
            (module.order_index, done, total, state),
            module_index(module.id),
            lock,
        ));
    }

    body = body.child(curriculum.child(rows));
    body
}

/// The continue card: the next lesson, or the finished-course note.
fn continue_card(
    theme: &Theme,
    view: Entity<AppView>,
    data: &DashboardData,
    module_index: &impl Fn(i64) -> Option<usize>,
    lesson_index: &impl Fn(i64) -> Option<(usize, usize)>,
) -> Div {
    let card = div()
        .rounded_md()
        .border_1()
        .border_color(theme.border)
        .bg(theme.secondary)
        .p_5()
        .flex()
        .flex_col()
        .gap_3();

    let Some(lesson_id) = data.first_incomplete_lesson else {
        return card
            .child(
                div()
                    .text_size(px(15.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.foreground)
                    .child("CURRICULUM COMPLETE"),
            )
            .child(
                div()
                    .text_size(px(13.))
                    .text_color(theme.muted_foreground)
                    .child("You have finished every lesson. Nice work."),
            );
    };

    let lesson = data
        .lessons_per_module
        .iter()
        .flatten()
        .find(|lesson| lesson.id == lesson_id);
    let module = data.modules.iter().find(|module| {
        lesson
            .map(|lesson| lesson.module_id == module.id)
            .unwrap_or(false)
    });

    let title = lesson
        .map(|lesson| lesson.title.clone())
        .unwrap_or_default();
    let module_label = module
        .map(|module| format!("MODULE {:02} — {}", module.order_index, module.title))
        .unwrap_or_default();

    let mut actions = div().flex().flex_row().gap_3();

    if let Some((module, position)) = lesson_index(lesson_id) {
        let jump = view.clone();
        actions = actions.child(
            Button::new("continue")
                .primary()
                .label("CONTINUE")
                .on_click(move |_, _, cx| {
                    jump.update(cx, |view, cx| {
                        view.navigate(Screen::Lesson {
                            module,
                            lesson: position,
                        });
                        cx.notify();
                    });
                }),
        );
    }

    if let Some(module) = module_index(lesson.map(|lesson| lesson.module_id).unwrap_or_default()) {
        let jump = view.clone();
        actions = actions.child(
            Button::new("view-module")
                .secondary()
                .label("VIEW MODULE")
                .on_click(move |_, _, cx| {
                    jump.update(cx, |view, cx| {
                        view.navigate(Screen::Module(module));
                        cx.notify();
                    });
                }),
        );
    }

    card.child(label(theme, module_label))
        .child(
            div()
                .text_size(px(18.))
                .font_weight(FontWeight::BOLD)
                .text_color(theme.foreground)
                .child(title),
        )
        .child(actions)
}

/// One row of the curriculum list: number, status, title, progress.
///
/// A progression-locked row is dimmed, carries a lock, and is not a Button.
/// A premium-locked row looks the same but *is* a Button — it opens the
/// paywall, because the only way through is to subscribe.
fn module_row(
    theme: &Theme,
    view: Entity<AppView>,
    module: &crate::models::Module,
    progress: (i64, usize, usize, StatusState),
    index: Option<usize>,
    lock: ModuleLock,
) -> Div {
    let (num, done, total, state) = progress;
    let is_unlocked = lock == ModuleLock::Open;
    let percent = if total > 0 {
        done as i64 * 100 / total as i64
    } else {
        0
    };

    let heading = match (index, lock) {
        (Some(index), ModuleLock::Open) => {
            let jump = view;
            div().child(
                Button::new(format!("module-title-{index}"))
                    .ghost()
                    .text_size(px(14.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.foreground)
                    .child(module.title.clone())
                    .on_click(move |_, _, cx| {
                        jump.update(cx, |view, cx| {
                            view.navigate(Screen::Module(index));
                            cx.notify();
                        });
                    }),
            )
        }
        // Paywall: same closed look, but the click opens the upgrade
        // screen — the only row where "locked" has somewhere to go.
        (Some(index), ModuleLock::Premium) => {
            let jump = view;
            div().child(
                Button::new(format!("module-title-premium-{index}"))
                    .ghost()
                    .text_size(px(14.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.muted_foreground)
                    .child(module.title.clone())
                    .on_click(move |_, _, cx| {
                        jump.update(cx, |view, cx| {
                            view.navigate(Screen::Upgrade);
                            cx.notify();
                        });
                    }),
            )
        }
        // Progression: plain text, not a control. Nothing to click.
        _ => div().child(
            div()
                .text_size(px(14.))
                .font_weight(FontWeight::BOLD)
                .text_color(theme.muted_foreground)
                .child(module.title.clone()),
        ),
    };

    let meta = if total > 0 {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_3()
            .child(
                div()
                    .text_size(px(11.))
                    .text_color(theme.muted_foreground)
                    .child(format!("{done}/{total} LESSONS")),
            )
            .child(progress_bar(theme, percent, false))
    } else {
        div()
            .text_size(px(11.))
            .text_color(theme.muted_foreground)
            .child("Content coming soon")
    };

    let body = div().flex().flex_col().gap_2().child(heading).child(meta);

    let row = div()
        .flex()
        .flex_row()
        .items_center()
        .gap_4()
        .rounded_md()
        .border_1()
        .border_color(theme.border)
        .bg(theme.secondary)
        .px_4()
        .py_3()
        .child(
            div()
                .w(px(24.))
                .text_size(px(12.))
                .text_color(theme.muted_foreground)
                .child(format!("{num:02}")),
        )
        .child(
            div()
                .w(px(20.))
                .text_size(px(14.))
                .text_color(status_color(theme, state))
                .child(if is_unlocked {
                    status_glyph(state)
                } else {
                    "\u{1F512}"
                }),
        )
        .child(div().flex_1().min_w(px(0.)).child(body));

    if is_unlocked {
        row
    } else {
        row.opacity(0.5)
    }
}

/// The progress body: the overall bar, four stats, the curriculum list.
pub fn progress(theme: &Theme, data: &ProgressSummary) -> Div {
    let percent = if data.total_lessons > 0 {
        data.completed_lessons * 100 / data.total_lessons
    } else {
        0
    };

    let mut stats = div().flex().flex_row().gap_3().flex_wrap();
    for (value, name) in [
        (data.completed_lessons, "LESSONS COMPLETED"),
        (data.completed_exercises, "EXERCISES COMPLETED"),
        (data.total_modules, "MODULES"),
        (data.total_xp, "XP EARNED"),
    ] {
        stats = stats.child(
            div()
                .flex_1()
                .min_w(px(140.))
                .rounded_md()
                .border_1()
                .border_color(theme.border)
                .bg(theme.secondary)
                .p_4()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_size(px(24.))
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.primary)
                        .child(value.to_string()),
                )
                .child(
                    div()
                        .text_size(px(11.))
                        .text_color(theme.muted_foreground)
                        .child(name),
                ),
        );
    }

    let mut list = div().flex().flex_col().gap_2();
    for module in &data.modules {
        list = list.child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_3()
                .child(
                    div()
                        .w(px(20.))
                        .text_size(px(14.))
                        .text_color(status_color(theme, StatusState::Available))
                        .child(status_glyph(StatusState::Available)),
                )
                .child(
                    div()
                        .text_size(px(14.))
                        .text_color(theme.foreground)
                        .child(module.title.clone()),
                ),
        );
    }

    div()
        .flex()
        .flex_col()
        .gap_6()
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .child(label(theme, "YOUR PROGRESS"))
                .child(
                    div()
                        .text_size(px(24.))
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.foreground)
                        .child("Rust Fundamentals"),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_4()
                        .child(
                            div()
                                .flex_1()
                                .min_w(px(0.))
                                .child(progress_bar(theme, percent, true)),
                        )
                        .child(
                            div()
                                .text_size(px(12.))
                                .text_color(theme.muted_foreground)
                                .child(format!(
                                    "{}/{} LESSONS · {percent}%",
                                    data.completed_lessons, data.total_lessons
                                )),
                        ),
                ),
        )
        .child(stats)
        .child(
            div()
                .flex()
                .flex_col()
                .gap_3()
                .child(label(theme, "CURRICULUM"))
                .child(list),
        )
}
