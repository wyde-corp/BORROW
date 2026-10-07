//! Screens and navigation: home → module → lesson.
//!
//! One `AppView` owns the screen, and buttons capture a handle to the view so a
//! click can update that state and request a re-render. Nothing here is async —
//! `content::all()` is compiled in, so switching screens costs a render rather
//! than a query.

use crate::app_db::Db;
use crate::components::label;
use crate::content::ModuleSeed;
use crate::exercise as exercise_view;
use crate::lesson::render_section;
use crate::models::{Exercise, LessonContent};
use crate::services::practice;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::*;
// `overflow_y_scroll` is on StatefulInteractiveElement, not InteractiveElement,
// and a stateful element needs an id to hang its state on.
use gpui_kit::*;
use gpui_kit::{InteractiveElement, StatefulInteractiveElement};

/// Where the app is showing. Module and lesson indices point into
/// [`AppView::modules`], which is fixed for the process's lifetime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Screen {
    Home,
    Module(usize),
    Lesson {
        module: usize,
        lesson: usize,
    },
    Exercise {
        module: usize,
        exercise: usize,
    },
    /// Practice builder for a module.
    Practice {
        module: usize,
    },
    /// One question of an active run.
    PracticeRun {
        module: usize,
    },
    /// After finishing a run.
    PracticeResults {
        module: usize,
    },
}

/// Everything the practice screens need between renders, all loaded once on
/// navigation rather than re-queried per frame.
#[derive(Default)]
struct PracticeState {
    builder: Option<practice::BuilderData>,
    run: Option<practice::RunPageData>,
    results: Option<practice::ResultsData>,
    position: usize,
    selected: Option<usize>,
}

/// Which of the two answer fields an option row writes to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AnswerSlot {
    Exercise,
    PracticeQuestion,
}

pub struct AppView {
    modules: Vec<ModuleSeed>,
    screen: Screen,
    /// Which option the learner has picked on the open exercise, and what the
    /// last submission returned. Both are transient: navigating away and back
    /// clears them, so a graded result is never left on screen for an exercise
    /// that has not been answered again.
    selected: Option<usize>,
    result: Option<exercise_view::Submission>,
    practice: PracticeState,
}

impl AppView {
    pub fn new(modules: Vec<ModuleSeed>) -> Self {
        Self {
            modules,
            screen: Screen::Home,
            selected: None,
            result: None,
            practice: PracticeState::default(),
        }
    }

    fn clear_answer(&mut self) {
        self.selected = None;
        self.result = None;
    }

    fn clear_practice(&mut self) {
        self.practice = PracticeState::default();
    }

    /// Parses one lesson's stored JSON.
    ///
    /// A malformed record renders as a callout rather than panicking: content
    /// is static and committed, so a failure here is a content bug, and taking
    /// down the app over it would hide the lesson that has it.
    fn parse(lesson: &crate::models::Lesson) -> Option<LessonContent> {
        match serde_json::from_str::<LessonContent>(&lesson.content) {
            Ok(content) => Some(content),
            Err(err) => {
                eprintln!(
                    "lesson {} ({}) failed to parse: {err}",
                    lesson.id, lesson.title
                );
                None
            }
        }
    }

    fn module(&self, index: usize) -> Option<&ModuleSeed> {
        self.modules.get(index)
    }

    fn home(&self, theme: &Theme, view: Entity<AppView>) -> Div {
        let mut list = div().flex().flex_col().gap_3();

        for (index, seed) in self.modules.iter().enumerate() {
            let title = seed.module.title.clone();
            let description = seed.module.description.clone();
            let lesson_count = seed.lessons.len();
            let jump_to = view.clone();

            // A Button rather than a div with on_click: gpui-component's Button
            // owns focus, so the card is reachable with Tab and activates on
            // Enter or Space without a second focus path to maintain. Its
            // content stays ours — ParentElement lets the card hold the same
            // title/description layout the div had.
            let card = Button::new(format!("module-{index}"))
                .ghost()
                .rounded_md()
                .border_1()
                .border_color(theme.border)
                .bg(theme.secondary)
                .p_4()
                .flex()
                .flex_col()
                .gap_1()
                .on_click(move |_, _, cx| {
                    jump_to.update(cx, |view, cx| {
                        view.screen = Screen::Module(index);
                        cx.notify();
                    });
                })
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_baseline()
                        .justify_between()
                        .child(
                            div()
                                .text_size(px(15.))
                                .text_color(theme.foreground)
                                .font_weight(FontWeight::BOLD)
                                .child(title),
                        )
                        .child(
                            div()
                                .text_size(px(11.))
                                .text_color(theme.muted_foreground)
                                .child(format!("{lesson_count} lessons")),
                        ),
                )
                .child(
                    div()
                        .text_size(px(13.))
                        .text_color(theme.muted_foreground)
                        .child(description),
                );

            list = list.child(card);
        }

        self.shell(
            theme,
            view,
            "BORROW_",
            "Learn Rust without the 700-page detour.",
            None,
            list,
        )
    }

    fn module_screen(&self, theme: &Theme, view: Entity<AppView>, index: usize) -> Div {
        let Some(seed) = self.module(index) else {
            return self.shell(theme, view, "Module not found", "", None, div());
        };

        let title = seed.module.title.clone();
        let description = seed.module.description.clone();
        let mut list = div().flex().flex_col().gap_2();

        for (position, lesson) in seed.lessons.iter().enumerate() {
            let id = lesson.id;
            let lesson_title = lesson.title.clone();
            let module_index = index;
            let jump_to = view.clone();

            list = list.child(
                Button::new(format!("lesson-{id}"))
                    .ghost()
                    .p_3()
                    .rounded_md()
                    .border_1()
                    .border_color(theme.border)
                    .bg(theme.secondary)
                    .on_click(move |_, _, cx| {
                        jump_to.update(cx, |view, cx| {
                            view.screen = Screen::Lesson {
                                module: module_index,
                                lesson: position,
                            };
                            cx.notify();
                        });
                    })
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .w(px(28.))
                            .text_size(px(11.))
                            .text_color(theme.muted_foreground)
                            .child(format!("{:02}", position + 1)),
                    )
                    .child(
                        div()
                            .text_size(px(14.))
                            .text_color(theme.foreground)
                            .child(lesson_title),
                    ),
            );
        }

        let mut exercise_list = div().flex().flex_col().gap_2();

        for (position, record) in seed.exercises.iter().enumerate() {
            let id = record.id;
            let prompt = record
                .prompt
                .lines()
                .next()
                .unwrap_or(&record.prompt)
                .trim()
                .to_string();
            let module_index = index;
            let jump_to = view.clone();

            exercise_list = exercise_list.child(
                Button::new(format!("exercise-{id}"))
                    .ghost()
                    .p_3()
                    .rounded_md()
                    .border_1()
                    .border_color(theme.border)
                    .bg(theme.secondary)
                    .on_click(move |_, _, cx| {
                        jump_to.update(cx, |view, cx| {
                            view.clear_answer();
                            view.screen = Screen::Exercise {
                                module: module_index,
                                exercise: position,
                            };
                            cx.notify();
                        });
                    })
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .w(px(28.))
                            .text_size(px(11.))
                            .text_color(theme.muted_foreground)
                            .child(format!("{:02}", position + 1)),
                    )
                    .child(
                        div()
                            .text_size(px(13.5))
                            .text_color(theme.foreground)
                            .child(prompt),
                    ),
            );
        }

        let mut body = div()
            .flex()
            .flex_col()
            .gap_5()
            .child(
                div()
                    .text_size(px(13.))
                    .text_color(theme.muted_foreground)
                    .child(description),
            )
            .child(label(theme, format!("LESSONS ({})", seed.lessons.len())))
            .child(list)
            .child(label(
                theme,
                format!("EXERCISES ({})", seed.exercises.len()),
            ))
            .child(exercise_list);

        // Practice is reached from here rather than from the nav: it is
        // per-module by nature, and its builder needs the module's id to find
        // the runs belonging to it.
        let target = view.clone();
        let practice_module = index;
        let module_id = seed.module.id;
        body = body.child(
            Button::new("practice")
                .primary()
                .label("PRACTICE THIS MODULE")
                .on_click(move |_, _, cx| {
                    let data = load_builder(cx, module_id);
                    target.update(cx, |view, cx| {
                        view.clear_practice();
                        view.practice.builder = data;
                        view.screen = Screen::Practice {
                            module: practice_module,
                        };
                        cx.notify();
                    });
                }),
        );

        self.shell(theme, view, &title, "", Some(index), body)
    }

    fn lesson_screen(
        &self,
        theme: &Theme,
        view: Entity<AppView>,
        module: usize,
        lesson: usize,
    ) -> Div {
        let Some(seed) = self.module(module) else {
            return self.shell(theme, view, "Lesson not found", "", None, div());
        };
        let total = seed.lessons.len();
        let Some(record) = seed.lessons.get(lesson) else {
            return self.shell(theme, view, "Lesson not found", "", Some(module), div());
        };

        let mut sections = div().flex().flex_col().gap_6();

        match Self::parse(record) {
            Some(content) => {
                for section in &content.sections {
                    sections = sections.child(render_section(theme, section));
                }
            }
            None => {
                sections = sections.child(callout(
                    theme,
                    "Lesson unavailable",
                    "This lesson's content could not be read.",
                ))
            }
        }

        let mut pager = div().flex().flex_row().gap_3();

        if lesson > 0 {
            let to = view.clone();
            pager = pager.child(Button::new("prev").secondary().label("← PREV").on_click(
                move |_, _, cx| {
                    to.update(cx, |v, cx| {
                        v.screen = Screen::Lesson {
                            module,
                            lesson: lesson - 1,
                        };
                        cx.notify();
                    });
                },
            ));
        }

        if lesson + 1 < total {
            let to = view.clone();
            pager = pager.child(Button::new("next").primary().label("NEXT →").on_click(
                move |_, _, cx| {
                    to.update(cx, |v, cx| {
                        v.screen = Screen::Lesson {
                            module,
                            lesson: lesson + 1,
                        };
                        cx.notify();
                    });
                },
            ));
        }

        let body = div()
            .flex()
            .flex_col()
            .gap_6()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(label(theme, format!("LESSON {:02} / {total}", lesson + 1)))
                    .child(
                        div()
                            .text_size(px(24.))
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.foreground)
                            .child(record.title.clone()),
                    ),
            )
            .child(sections)
            .child(pager);

        self.shell(theme, view, &record.title, "", Some(module), body)
    }

    /// The frame every screen shares: navbar, scrollable content column.
    fn shell(
        &self,
        theme: &Theme,
        view: Entity<AppView>,
        title: &str,
        subtitle: &str,
        active_module: Option<usize>,
        body: Div,
    ) -> Div {
        let to_home = view.clone();
        // An `Option` rather than a usize, because module index 0 is the
        // first module and cannot double as "no module selected".
        let module_name = active_module
            .and_then(|index| self.module(index))
            .map(|s| s.module.title.clone())
            .unwrap_or_default();

        div()
            .size_full()
            .bg(theme.background)
            .text_color(theme.foreground)
            .flex()
            .flex_col()
            .child(navbar(theme, to_home, module_name))
            .child(
                div()
                    .id("screen-scroll")
                    .flex_1()
                    .min_h(px(0.))
                    .overflow_y_scroll()
                    .px_6()
                    .py_8()
                    .child(
                        div()
                            .mx_auto()
                            .max_w(px(820.))
                            .flex()
                            .flex_col()
                            .gap_5()
                            .child(heading(theme, title, subtitle))
                            .child(body),
                    ),
            )
    }
    /// One exercise: the question, the answer controls, and the result.
    fn exercise_screen(
        &self,
        theme: &Theme,
        view: Entity<AppView>,
        module: usize,
        exercise: usize,
    ) -> Div {
        let Some(seed) = self.module(module) else {
            return self.shell(theme, view, "Exercise not found", "", None, div());
        };
        let total = seed.exercises.len();
        let Some(record) = seed.exercises.get(exercise) else {
            return self.shell(theme, view, "Exercise not found", "", None, div());
        };

        let options = exercise_view::options_of(record);
        let open_ended = record.is_open_ended();
        let chosen = self.selected;

        let mut body = div()
            .flex()
            .flex_col()
            .gap_5()
            .child(exercise_view::question(theme, record));

        if !options.is_empty() {
            body = body.child(self.option_list(
                theme,
                &options,
                chosen,
                AnswerSlot::Exercise,
                view.clone(),
            ));

            if self.result.is_none() {
                body = body.child(
                    Button::new("check")
                        .primary()
                        .label("CHECK ANSWER")
                        .disabled(chosen.is_none())
                        .on_click({
                            let answer = record.clone();
                            let picked = chosen;
                            let reported = view.clone();
                            move |_, _, cx| {
                                let submission = {
                                    let db = cx.global::<Db>();
                                    exercise_view::submit(db, &answer, picked)
                                };
                                reported.update(cx, |view, cx| {
                                    view.result = Some(submission);
                                    cx.notify();
                                });
                            }
                        }),
                );
            }
        } else if open_ended {
            body = body.child(
                Button::new("complete")
                    .primary()
                    .label("MARK COMPLETE")
                    .on_click({
                        let answer = record.clone();
                        let reported = view.clone();
                        move |_, _, cx| {
                            let submission = {
                                let db = cx.global::<Db>();
                                exercise_view::submit(db, &answer, None)
                            };
                            reported.update(cx, |view, cx| {
                                view.result = Some(submission);
                                cx.notify();
                            });
                        }
                    }),
            );
        } else {
            // fill_gap and refactor are graded against typed text. A text field
            // needs the editor's input state, which Phase 5 builds anyway, so
            // the question is readable here rather than half-answered.
            body = body.child(
                div()
                    .rounded_md()
                    .border_1()
                    .border_color(theme.border)
                    .bg(theme.secondary)
                    .p_4()
                    .text_size(px(13.))
                    .text_color(theme.muted_foreground)
                    .child("Type your answer once the editor arrives in a later phase — this exercise is written to grade a typed reply."),
            );
        }

        if let Some(result) = &self.result {
            body = body.child(self.feedback(theme, record, &options, result));
        }

        let heading = format!("Exercise {}/{}", exercise + 1, total);
        self.shell(theme, view, &heading, "", Some(module), body)
    }

    /// Choice rows; selecting one updates the view so CHECK ANSWER is enabled.
    fn option_list(
        &self,
        theme: &Theme,
        options: &[String],
        selected: Option<usize>,
        slot: AnswerSlot,
        view: Entity<AppView>,
    ) -> Div {
        let mut list = div().flex().flex_col().gap_2();

        for (index, text) in options.iter().enumerate() {
            let chosen = selected == Some(index);
            let accent = if chosen { theme.primary } else { theme.border };
            let picked = view.clone();

            list = list.child(
                Button::new(format!("option-{index}"))
                    .ghost()
                    .on_click(move |_, _, cx| {
                        picked.update(cx, |view, cx| {
                            match slot {
                                AnswerSlot::Exercise => {
                                    view.selected = Some(index);
                                    view.result = None;
                                }
                                AnswerSlot::PracticeQuestion => {
                                    view.practice.selected = Some(index);
                                }
                            }
                            cx.notify();
                        });
                    })
                    .flex()
                    .flex_row()
                    .items_start()
                    .gap_3()
                    .p_3()
                    .rounded_md()
                    .border_1()
                    .border_color(accent)
                    .bg(if chosen { theme.muted } else { theme.secondary })
                    .child(
                        div()
                            .w(px(16.))
                            .text_size(px(13.))
                            .text_color(if chosen {
                                theme.primary
                            } else {
                                theme.muted_foreground
                            })
                            .child(if chosen { "●" } else { "○" }),
                    )
                    .child(
                        div()
                            .text_size(px(13.5))
                            .text_color(theme.foreground)
                            .child(text.clone()),
                    ),
            );
        }

        list
    }

    /// What the last submission earned, with the explanation attached.
    fn feedback(
        &self,
        theme: &Theme,
        record: &crate::models::Exercise,
        options: &[String],
        result: &exercise_view::Submission,
    ) -> Div {
        let (mark, colour, words) = if result.correct {
            ("✓", theme.success, "Correct")
        } else if record.is_open_ended() {
            ("✓", theme.success, "Recorded")
        } else {
            ("✗", theme.danger, "Not quite")
        };

        let chosen = result
            .shown_answer
            .parse::<usize>()
            .ok()
            .and_then(|index| options.get(index))
            .cloned()
            .unwrap_or_else(|| result.shown_answer.clone());

        let mut panel = div()
            .rounded_md()
            .border_1()
            .border_color(colour)
            .bg(theme.secondary)
            .p_4()
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_3()
                    .child(div().text_size(px(16.)).text_color(colour).child(mark))
                    .child(
                        div()
                            .text_size(px(15.))
                            .font_weight(FontWeight::BOLD)
                            .text_color(colour)
                            .child(words),
                    )
                    .child(if result.xp > 0 {
                        div()
                            .text_size(px(12.))
                            .text_color(theme.warning)
                            .child(format!("+{} XP", result.xp))
                    } else {
                        div()
                    }),
            );

        if !chosen.is_empty() && !record.is_open_ended() {
            panel = panel.child(
                div()
                    .text_size(px(13.))
                    .text_color(theme.muted_foreground)
                    .child(format!("Your answer: {chosen}")),
            );
        }

        if !result.message.is_empty() {
            panel = panel.child(
                div()
                    .text_size(px(13.))
                    .text_color(theme.muted_foreground)
                    .child(result.message.clone()),
            );
        }

        if !record.explanation.trim().is_empty() {
            panel = panel.child(
                div()
                    .text_size(px(13.))
                    .text_color(theme.foreground)
                    .child(record.explanation.trim().to_string()),
            );
        }

        panel
    }

    /// The practice builder: resume an unfinished run, start a new one, and
    /// see how previous runs went.
    fn practice_screen(&self, theme: &Theme, view: Entity<AppView>, module: usize) -> Div {
        let Some(seed) = self.module(module) else {
            return self.shell(theme, view, "Practice not found", "", None, div());
        };
        let module_id = seed.module.id;

        let mut body = div().flex().flex_col().gap_5();

        match self.practice.builder.as_ref() {
            None => {
                body = body.child(
                    div()
                        .text_size(px(13.))
                        .text_color(theme.muted_foreground)
                        .child("Practice questions could not be loaded."),
                )
            }
            Some(data) => {
                let total = data.exercises.len();

                body = body.child(
                    div()
                        .text_size(px(13.))
                        .text_color(theme.muted_foreground)
                        .child(format!(
                            "{total} gradable questions. Practice never awards XP and never marks a lesson complete — wrong answers are yours to keep, not to lose credit for."
                        )),
                );

                if let Some(run) = &data.unfinished {
                    let run_id = run.id;
                    let target = view.clone();
                    let resumed = run.label.clone();
                    body = body.child(
                        Button::new("resume")
                            .primary()
                            .label(format!("RESUME — {resumed}"))
                            .on_click(move |_, _, cx| {
                                if let Some(page) = load_run(cx, run_id) {
                                    let first =
                                        page.answers.len().min(page.items.len()).saturating_sub(0);
                                    target.update(cx, |view, cx| {
                                        view.clear_answer();
                                        view.practice.run = Some(page);
                                        view.practice.position = first;
                                        view.practice.selected = None;
                                        view.screen = Screen::PracticeRun { module };
                                        cx.notify();
                                    });
                                }
                            }),
                    );
                }

                let mut actions = div().flex().flex_row().gap_3().flex_wrap();

                let all_count = total;
                let target_all = view.clone();
                actions = actions.child(
                    Button::new("all")
                        .primary()
                        .label(format!("PRACTICE ALL {all_count}"))
                        .on_click(move |_, _, cx| {
                            if let Some(page) = start_run(cx, module_id, None) {
                                target_all.update(cx, |view, cx| {
                                    view.clear_answer();
                                    view.practice.run = Some(page);
                                    view.practice.position = 0;
                                    view.practice.selected = None;
                                    view.screen = Screen::PracticeRun { module };
                                    cx.notify();
                                });
                            }
                        }),
                );

                if total > 10 {
                    let target_ten = view.clone();
                    actions = actions.child(
                        Button::new("ten")
                            .secondary()
                            .label("10 QUESTIONS")
                            .on_click(move |_, _, cx| {
                                if let Some(page) = start_run(cx, module_id, Some(10)) {
                                    target_ten.update(cx, |view, cx| {
                                        view.clear_answer();
                                        view.practice.run = Some(page);
                                        view.practice.position = 0;
                                        view.practice.selected = None;
                                        view.screen = Screen::PracticeRun { module };
                                        cx.notify();
                                    });
                                }
                            }),
                    );
                }

                body = body.child(actions);

                if !data.history.is_empty() {
                    let mut history = div().flex().flex_col().gap_2();

                    for (run, correct, history_total) in &data.history {
                        history = history.child(
                            div()
                                .flex()
                                .flex_row()
                                .justify_between()
                                .py_2()
                                .border_b_1()
                                .border_color(theme.border)
                                .child(
                                    div()
                                        .text_size(px(13.))
                                        .text_color(theme.foreground)
                                        .child(format!("{} · {}", run.label, run.created_at)),
                                )
                                .child(
                                    div()
                                        .text_size(px(13.))
                                        .text_color(if *correct == *history_total {
                                            theme.success
                                        } else {
                                            theme.warning
                                        })
                                        .child(format!("{correct}/{history_total}")),
                                ),
                        );
                    }

                    body = body.child(label(theme, "PAST RUNS")).child(history);
                }
            }
        }

        self.shell(theme, view, "Practice", "", Some(module), body)
    }

    /// One question of an active run: answer it, then move on.
    fn practice_run_screen(&self, theme: &Theme, view: Entity<AppView>, module: usize) -> Div {
        let Some(data) = self.practice.run.as_ref() else {
            return self.shell(theme, view, "Practice not found", "", Some(module), div());
        };
        let total = data.items.len();
        let position = self.practice.position.min(total.saturating_sub(1));

        let Some(item) = data.items.get(position) else {
            return self.shell(theme, view, "Practice not found", "", Some(module), div());
        };
        let exercise = &item.exercise;
        let options = exercise_view::options_of(exercise);
        let run_id = data.run.id;

        // An answer already on the run is the source of truth for whether this
        // question has been answered — it survives navigation back and forth
        // and a restart, where a field on the view would not.
        let answered = data.answers.iter().find(|(id, _, _)| *id == exercise.id);

        let mut body = div()
            .flex()
            .flex_col()
            .gap_5()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_baseline()
                    .gap_4()
                    .child(label(theme, format!("QUESTION {} / {total}", position + 1))),
            )
            .child(exercise_view::question(theme, exercise));

        if !options.is_empty() {
            body = body.child(self.option_list(
                theme,
                &options,
                self.practice.selected,
                AnswerSlot::PracticeQuestion,
                view.clone(),
            ));

            match answered {
                None => {
                    let chosen = self.practice.selected;
                    let target = view.clone();
                    let answer_for = exercise.clone();
                    body = body.child(
                        Button::new("check")
                            .primary()
                            .label("CHECK ANSWER")
                            .disabled(chosen.is_none())
                            .on_click(move |_, _, cx| {
                                let Some(index) = chosen else {
                                    return;
                                };
                                answer_question(cx, run_id, &answer_for, Some(index));
                                let page = load_run(cx, run_id);
                                target.update(cx, |view, cx| {
                                    view.practice.run = page;
                                    view.practice.selected = None;
                                    cx.notify();
                                });
                            }),
                    );
                }
                Some((_, correct, given)) => {
                    let (mark, colour, words) = if *correct {
                        ("✓", theme.success, "Correct")
                    } else {
                        ("✗", theme.danger, "Not quite")
                    };

                    let shown = given
                        .parse::<usize>()
                        .ok()
                        .and_then(|index| options.get(index))
                        .cloned()
                        .unwrap_or_else(|| given.clone());

                    let mut panel = div()
                        .rounded_md()
                        .border_1()
                        .border_color(colour)
                        .bg(theme.secondary)
                        .p_4()
                        .flex()
                        .flex_col()
                        .gap_3()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_3()
                                .child(div().text_size(px(16.)).text_color(colour).child(mark))
                                .child(
                                    div()
                                        .text_size(px(15.))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(colour)
                                        .child(words),
                                ),
                        );

                    if !shown.is_empty() {
                        panel = panel.child(
                            div()
                                .text_size(px(13.))
                                .text_color(theme.muted_foreground)
                                .child(format!("You answered: {shown}")),
                        );
                    }

                    if !exercise.explanation.trim().is_empty() {
                        panel = panel.child(
                            div()
                                .text_size(px(13.))
                                .text_color(theme.foreground)
                                .child(exercise.explanation.trim().to_string()),
                        );
                    }

                    body = body.child(panel);
                }
            }
        }

        // ── movement ──
        let mut pager = div().flex().flex_row().gap_3();

        if position > 0 {
            let target = view.clone();
            pager = pager.child(Button::new("prev").secondary().label("← PREV").on_click(
                move |_, _, cx| {
                    target.update(cx, |view, cx| {
                        view.practice.position = view.practice.position.saturating_sub(1);
                        view.practice.selected = None;
                        cx.notify();
                    });
                },
            ));
        }

        if position + 1 < total {
            let target = view.clone();
            pager = pager.child(Button::new("next").secondary().label("NEXT →").on_click(
                move |_, _, cx| {
                    target.update(cx, |view, cx| {
                        view.practice.position += 1;
                        view.practice.selected = None;
                        cx.notify();
                    });
                },
            ));
        } else {
            let target = view.clone();
            pager = pager.child(Button::new("finish").primary().label("FINISH →").on_click(
                move |_, _, cx| {
                    if let Some(results) = finish_run(cx, run_id) {
                        target.update(cx, |view, cx| {
                            view.practice.results = Some(results);
                            view.practice.selected = None;
                            view.screen = Screen::PracticeResults { module };
                            cx.notify();
                        });
                    }
                },
            ));
        }

        body = body.child(pager);

        self.shell(theme, view, &data.run.label, "", Some(module), body)
    }

    /// The score after a run, and a way to throw it away and start again.
    fn practice_results_screen(&self, theme: &Theme, view: Entity<AppView>, module: usize) -> Div {
        let Some(data) = self.practice.results.as_ref() else {
            return self.shell(theme, view, "Results not found", "", Some(module), div());
        };

        let total = data.items.len();
        let correct = data.answers.iter().filter(|answer| answer.correct).count();
        let perfect = correct == total;
        let colour = if perfect {
            theme.success
        } else if correct == 0 {
            theme.danger
        } else {
            theme.warning
        };

        let mut rows = div().flex().flex_col().gap_2();

        for item in &data.items {
            let answer = data
                .answers
                .iter()
                .find(|answer| answer.exercise_id == item.exercise.id);
            let answered_correct = answer.map(|a| a.correct).unwrap_or(false);
            let given = answer.and_then(|a| a.given.clone()).unwrap_or_default();

            let shown = given
                .parse::<usize>()
                .ok()
                .and_then(|index| {
                    exercise_view::options_of(&item.exercise)
                        .get(index)
                        .cloned()
                })
                .unwrap_or_else(|| given.clone());

            rows = rows.child(
                div()
                    .flex()
                    .flex_row()
                    .items_start()
                    .gap_3()
                    .py_2()
                    .border_b_1()
                    .border_color(theme.border)
                    .child(
                        div()
                            .text_size(px(14.))
                            .text_color(if answered_correct {
                                theme.success
                            } else {
                                theme.danger
                            })
                            .child(if answered_correct { "✓" } else { "✗" }),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div().text_size(px(13.)).text_color(theme.foreground).child(
                                    item.exercise
                                        .prompt
                                        .lines()
                                        .next()
                                        .unwrap_or("")
                                        .trim()
                                        .to_string(),
                                ),
                            )
                            .child(
                                div()
                                    .text_size(px(12.))
                                    .text_color(theme.muted_foreground)
                                    .child(if shown.is_empty() {
                                        "not answered".to_string()
                                    } else {
                                        format!("you said: {shown}")
                                    }),
                            ),
                    ),
            );
        }

        let mut body = div()
            .flex()
            .flex_col()
            .gap_5()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_baseline()
                    .gap_4()
                    .text_size(px(32.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(colour)
                    .child(format!("{correct}/{total}"))
                    .child(
                        div()
                            .text_size(px(13.))
                            .text_color(theme.muted_foreground)
                            .child(if perfect {
                                "every one correct"
                            } else {
                                "graded practice — no XP at stake"
                            }),
                    ),
            )
            .child(rows);

        let run_id = data.run.id;
        let module_id = self
            .module(module)
            .map(|seed| seed.module.id)
            .unwrap_or_default();
        let another = view.clone();

        body = body.child(div().flex().flex_row().gap_3()).child(
            Button::new("again")
                .primary()
                .label("START ANOTHER RUN")
                .on_click(move |_, _, cx| {
                    // Discard the finished run rather than leaving it to
                    // show up as unfinished later; its answers cascade.
                    let _ = discard_run(cx, run_id);
                    let builder = load_builder(cx, module_id);
                    another.update(cx, |view, cx| {
                        view.practice = PracticeState {
                            builder,
                            ..Default::default()
                        };
                        view.screen = Screen::Practice { module };
                        cx.notify();
                    });
                }),
        );

        self.shell(theme, view, &data.run.label, "", Some(module), body)
    }
}

/// The top strip: wordmark back to home, and where you are.
fn navbar(theme: &Theme, to_home: Entity<AppView>, module_name: String) -> Div {
    let mut bar = div()
        .flex()
        .flex_row()
        .items_center()
        .gap_6()
        .px_6()
        .py_4()
        .border_b_1()
        .border_color(theme.border)
        .bg(theme.secondary)
        .child(
            Button::new("nav-home")
                .ghost()
                .text_size(px(15.))
                .font_weight(FontWeight::BOLD)
                .text_color(theme.primary)
                .child("BORROW_")
                .on_click(move |_, _, cx| {
                    to_home.update(cx, |view, cx| {
                        view.screen = Screen::Home;
                        cx.notify();
                    });
                }),
        );

    if !module_name.is_empty() {
        bar = bar.child(
            div()
                .text_size(px(11.))
                .text_color(theme.muted_foreground)
                .child(module_name),
        );
    }

    bar
}

impl Render for AppView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let view = cx.entity();

        match self.screen.clone() {
            Screen::Home => self.home(theme, view),
            Screen::Module(index) => self.module_screen(theme, view, index),
            Screen::Lesson { module, lesson } => self.lesson_screen(theme, view, module, lesson),
            Screen::Exercise { module, exercise } => {
                self.exercise_screen(theme, view, module, exercise)
            }
            Screen::Practice { module } => self.practice_screen(theme, view, module),
            Screen::PracticeRun { module } => self.practice_run_screen(theme, view, module),
            Screen::PracticeResults { module } => self.practice_results_screen(theme, view, module),
        }
    }
}

/// Runs a database call inside a gpui callback.
///
/// The `&Db` borrow has to end before `view.update`, which needs `cx`
/// mutably; returning an owned value makes that ordering obvious rather than
/// something to reason about at every call site.
fn with_db<T>(cx: &App, f: impl FnOnce(&Db) -> T) -> T {
    f(cx.global::<Db>())
}

/// The practice builder's data for a module, or nothing if it cannot be read.
fn load_builder(cx: &App, module_id: i64) -> Option<practice::BuilderData> {
    with_db(cx, |db| {
        db.block_on(practice::builder_data(db.pool(), module_id))
            .ok()
            .flatten()
    })
}

/// Starts a run of `limit` questions — every gradable one when `None` — and
/// returns the page to render.
///
/// Questions come from `gradable_exercises`, which excludes open-ended work:
/// a practice run is scored, and there is nothing to score there.
fn start_run(cx: &App, module_id: i64, limit: Option<usize>) -> Option<practice::RunPageData> {
    with_db(cx, |db| {
        let pool = db.pool();
        let gradable = db
            .block_on(practice::gradable_exercises(pool, module_id))
            .ok()?;
        let take = limit.unwrap_or(gradable.len()).min(gradable.len());
        let ids: Vec<i64> = gradable.iter().take(take).map(|e| e.id).collect();

        if ids.is_empty() {
            return None;
        }

        let label = format!("{} questions", ids.len());
        let run = db
            .block_on(practice::create_run(pool, module_id, &label, &ids))
            .ok()?;
        db.block_on(practice::run_page_data(pool, run.id))
            .ok()
            .flatten()
    })
}

/// Reopens a run's page.
fn load_run(cx: &App, run_id: i64) -> Option<practice::RunPageData> {
    with_db(cx, |db| {
        db.block_on(practice::run_page_data(db.pool(), run_id))
            .ok()
            .flatten()
    })
}

/// Scores one answer and stores it, returning whether it was correct.
///
/// The write goes to `practice_answers` only — `record_answer` deliberately
/// never touches `progress`, so a wrong answer stays invisible to XP and to
/// the module page. That is why the table exists separately.
fn answer_question(cx: &App, run_id: i64, exercise: &Exercise, selected: Option<usize>) -> bool {
    with_db(cx, |db| {
        let (correct, given) = exercise_view::grade(exercise, selected, false);
        let _ = db.block_on(practice::record_answer(
            db.pool(),
            run_id,
            exercise.id,
            &given,
            correct,
        ));
        correct
    })
}

/// Marks a run finished and loads its results.
fn finish_run(cx: &App, run_id: i64) -> Option<practice::ResultsData> {
    with_db(cx, |db| {
        let _ = db.block_on(practice::finish_run(db.pool(), run_id));
        db.block_on(practice::results_data(db.pool(), run_id))
            .ok()
            .flatten()
    })
}

/// Throws a run away — items and answers cascade — for starting again.
fn discard_run(cx: &App, run_id: i64) -> bool {
    with_db(cx, |db| {
        db.block_on(practice::delete_run(db.pool(), run_id)).is_ok()
    })
}

fn heading(theme: &Theme, title: &str, subtitle: &str) -> Div {
    let mut head = div().flex().flex_col().gap_1();
    head = head.child(
        div()
            .text_size(px(28.))
            .font_weight(FontWeight::BOLD)
            .text_color(theme.foreground)
            .child(title.to_string()),
    );

    if !subtitle.is_empty() {
        head = head.child(
            div()
                .text_size(px(13.))
                .text_color(theme.muted_foreground)
                .child(subtitle.to_string()),
        );
    }

    head
}

/// Convenience used by the lesson view's unavailable-content fallback.
fn callout(theme: &Theme, title: &str, text: &str) -> Div {
    crate::components::callout(theme, title, text, crate::components::CalloutTone::Error)
}

/// Keyboard behaviour, checked without a compositor behind it.
///
/// The cards are `Button`s rather than `div().on_click` specifically so they
/// sit in the focus order and activate on Enter — a pointer-only list would
/// have looked identical on screen and been unreachable from a keyboard. This
/// is the test that says the difference is real.
///
/// Two things are deliberately avoided in here:
///
/// - `#[gpui_kit::test]` segfaults against this version of gpui;
/// - `use super::*` — the parent module does `use gpui_kit::*`, which carries
///   gpui's own `test` attribute macro, and that silently replaces Rust's
///   built-in `#[test]`. Every test here would have been gpui's macro, which
///   is what crashed the compiler. Hence the explicit imports.
#[cfg(test)]
mod keyboard_tests {
    use super::AppView;
    use gpui_kit::{TestAppContext, TestDispatcher, VisualTestContext};

    /// Applies the BORROW_ theme and opens the app at home on this context.
    ///
    /// Takes `&mut TestAppContext` rather than building its own so the returned
    /// window context can borrow it — returning a reference to a local built
    /// inside would not compile.
    fn open(
        cx: &mut TestAppContext,
    ) -> (gpui_kit::Entity<AppView>, &mut VisualTestContext) {
        cx.update(|cx| {
            gpui_kit::init(cx);
            let _ = crate::theme::apply(cx);
        });

        let (view, window) = cx.add_window_view(|_, _| AppView::new(crate::content::all()));
        window.update(|window, cx| window.draw(cx).clear(cx));
        (view, window)
    }

    fn screen_of(view: &gpui_kit::Entity<AppView>, cx: &mut VisualTestContext) -> String {
        cx.update(|_, cx| format!("{:?}", view.read(cx).screen))
    }

    fn press(cx: &mut VisualTestContext, key: &str) {
        let keystroke = gpui_kit::Keystroke::parse(key).expect("parses");
        cx.simulate_event(gpui_kit::KeyDownEvent {
            keystroke: keystroke.clone(),
            is_held: false,
            prefer_character_input: false,
        });
        cx.simulate_event(gpui_kit::KeyUpEvent { keystroke });
    }

    #[test]
    fn some_control_is_reachable_from_the_keyboard() {
        let mut cx = TestAppContext::build(TestDispatcher::new(0), Some("focus_exists"));
        let (view, cx) = open(&mut cx);

        // The floor under the whole accessibility claim: if nothing is
        // focusable, focus_next leaves focus nowhere.
        cx.update(|window, cx| window.focus_next(cx));
        let focused = cx.update(|window, cx| window.focused(cx).is_some());

        assert!(focused, "the first control should take focus on Tab");
        let _ = view;
    }

    #[test]
    fn tab_walks_from_the_wordmark_into_the_first_module_card() {
        let mut cx = TestAppContext::build(TestDispatcher::new(0), Some("tab_walks"));
        let (view, cx) = open(&mut cx);

        assert_eq!(screen_of(&view, cx), "Home");

        // First stop is the BORROW_ wordmark in the navbar, second the first
        // module card. If the cards were plain divs again the second stop
        // would not be a card, and Enter would leave us on Home.
        cx.update(|window, cx| window.focus_next(cx));
        cx.update(|window, cx| window.focus_next(cx));
        press(cx, "enter");
        cx.run_until_parked();
        cx.update(|window, cx| window.draw(cx).clear(cx));

        assert_eq!(
            screen_of(&view, cx),
            "Module(0)",
            "Enter on a focused module card should navigate"
        );
    }
}
