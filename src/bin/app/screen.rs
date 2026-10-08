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
use crate::models::{Exercise, GapBlank, LessonContent, LessonSection};
use crate::services::practice;
use crate::services::progress::{self, DashboardData, ProgressSummary};
use crate::services::sandbox::{self, RunResult};
use crate::widgets::{BlockState, WidgetCtx};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{EditorState, InputState};
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
    /// Module completion, XP, and the continue card.
    Dashboard,
    /// Totals across the whole course.
    Progress,
    /// The code editor and its console.
    Playground,
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

/// Everything the playground keeps between renders.
///
/// The editor buffer itself lives in its own keyed element state — it needs
/// `&mut Window` to create, which only `render` has — so this holds the handle
/// plus what the run produced.
#[derive(Default)]
pub(crate) struct PlaygroundState {
    /// Created on the first draw of the screen and reused after that.
    pub(crate) editor: Option<Entity<EditorState>>,
    pub(crate) result: Option<RunResult>,
    /// Click-to-result wall clock, for the "round trip" line.
    pub(crate) elapsed_ms: Option<u128>,
    pub(crate) running: bool,
    /// The in-flight run. Dropping it cancels, which is how a second press of
    /// RUN stops a compile — the web aborts its fetch the same way.
    run: Option<Task<RunResult>>,
    /// The single open "WHY THIS HAPPENS" disclosure.
    pub(crate) explanation_open: bool,
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
    /// Which hint disclosures are open. A set rather than a count, because the
    /// site's hints are independent `<details>` — a learner can open the third
    /// without reading the first.
    hints_open: std::collections::HashSet<usize>,
    /// Loaded once when the nav entry is clicked, then kept until the next
    /// click reloads it — never re-queried per frame.
    dashboard: Option<DashboardData>,
    progress: Option<ProgressSummary>,
    pub(crate) playground: PlaygroundState,
    /// Interaction state for the lesson on screen, one entry per block.
    pub(crate) widgets: Vec<BlockState>,
    /// Which lesson `widgets` belongs to — entering another one starts clean
    /// even if some navigation path forgot to clear.
    widgets_lesson: Option<i64>,
}

impl AppView {
    pub fn new(modules: Vec<ModuleSeed>) -> Self {
        Self {
            modules,
            screen: Screen::Home,
            selected: None,
            result: None,
            practice: PracticeState::default(),
            hints_open: std::collections::HashSet::new(),
            dashboard: None,
            progress: None,
            playground: PlaygroundState::default(),
            widgets: Vec::new(),
            widgets_lesson: None,
        }
    }

    fn clear_answer(&mut self) {
        self.selected = None;
        self.result = None;
        self.hints_open.clear();
        self.widgets.clear();
    }

    fn clear_practice(&mut self) {
        self.practice = PracticeState::default();
    }

    /// Leaves for `screen`, closing hints and any graded result on the way.
    ///
    /// The field stays private so nothing changes screens without clearing the
    /// transient state first; `dashboard.rs` uses this to navigate.
    pub(crate) fn navigate(&mut self, screen: Screen) {
        self.clear_answer();
        self.screen = screen;
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

    /// The screen position of a database module id.
    fn module_index(&self, id: i64) -> Option<usize> {
        self.modules.iter().position(|seed| seed.module.id == id)
    }

    /// The screen position of a database lesson id — `(module, lesson)`.
    fn lesson_index(&self, id: i64) -> Option<(usize, usize)> {
        self.modules.iter().enumerate().find_map(|(module, seed)| {
            seed.lessons
                .iter()
                .position(|lesson| lesson.id == id)
                .map(|lesson| (module, lesson))
        })
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

    /// One lesson.
    ///
    /// `&mut self` because the widgets keep their state here rather than in
    /// the elements, and gap-fill's inputs have to exist before anything can
    /// render them.
    fn lesson_screen(
        &mut self,
        view: Entity<AppView>,
        window: &mut Window,
        cx: &mut Context<Self>,
        module: usize,
        lesson: usize,
    ) -> Div {
        let Some(seed) = self.module(module) else {
            return self.shell(cx.theme(), view, "Lesson not found", "", None, div());
        };
        let total = seed.lessons.len();
        let Some(record) = seed.lessons.get(lesson) else {
            return self.shell(
                cx.theme(),
                view,
                "Lesson not found",
                "",
                Some(module),
                div(),
            );
        };

        // Taken by value before anything below mutates `self`.
        let title = record.title.clone();
        let lesson_id = record.id;
        let parsed = Self::parse(record);

        if self.widgets_lesson != Some(lesson_id) {
            self.widgets.clear();
            self.widgets_lesson = Some(lesson_id);
        }
        let blocks = parsed
            .as_ref()
            .map(|content| content.sections.len())
            .unwrap_or(0);
        // A grow-and-trim rather than `Vec::resize`, which would want
        // `BlockState: Clone` — and a clone of an in-flight run task is not
        // something this state should ever have.
        while self.widgets.len() < blocks {
            self.widgets.push(BlockState::default());
        }
        self.widgets.truncate(blocks);

        // Gap-fill's inputs need a window, so they are made here rather than
        // in the renderer: the same key hands back the same input on every
        // draw, and the key carries the lesson so one lesson's blanks never
        // pick up another's text.
        if let Some(content) = parsed.as_ref() {
            for (position, section) in content.sections.iter().enumerate() {
                if let LessonSection::GapFill { blanks, .. } = section {
                    self.ensure_gap_inputs(position, blanks, lesson_id, window, cx);
                }
            }
        }

        // After the keyed state above, which needs `&mut cx`.
        let theme = cx.theme();
        let mut sections = div().flex().flex_col().gap_6();

        match parsed {
            Some(content) => {
                for (position, section) in content.sections.iter().enumerate() {
                    let ctx = WidgetCtx {
                        index: position,
                        state: &self.widgets[position],
                        view: Some(view.clone()),
                    };
                    sections = sections.child(render_section(theme, section, &ctx));
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
            pager = pager.child(
                Button::new("prev")
                    .secondary()
                    .label("\u{2190} PREV")
                    .on_click(move |_, _, cx| {
                        to.update(cx, |v, cx| {
                            v.clear_answer();
                            v.screen = Screen::Lesson {
                                module,
                                lesson: lesson - 1,
                            };
                            cx.notify();
                        });
                    }),
            );
        }

        if lesson + 1 < total {
            let to = view.clone();
            pager = pager.child(
                Button::new("next")
                    .primary()
                    .label("NEXT \u{2192}")
                    .on_click(move |_, _, cx| {
                        to.update(cx, |v, cx| {
                            v.clear_answer();
                            v.screen = Screen::Lesson {
                                module,
                                lesson: lesson + 1,
                            };
                            cx.notify();
                        });
                    }),
            );
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
                            .child(title.clone()),
                    ),
            )
            .child(sections)
            .child(pager);

        self.shell(theme, view, &title, "", Some(module), body)
    }

    /// Makes gap-fill's inputs for one block — one per blank.
    ///
    /// Called from the lesson screen rather than the renderer because
    /// creating input state needs `&mut Window`. A block that has never been
    /// drawn starts empty, so navigating away and back starts the learner
    /// over, the way the site does.
    fn ensure_gap_inputs(
        &mut self,
        block: usize,
        blanks: &[GapBlank],
        lesson: i64,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let fresh = self
            .widgets
            .get(block)
            .map(|state| state.inputs.is_empty())
            .unwrap_or(true);

        let mut inputs = Vec::with_capacity(blanks.len());
        for position in 0..blanks.len() {
            inputs.push(window.use_keyed_state(
                format!("gap-{lesson}-{block}-{position}"),
                cx,
                InputState::new,
            ));
        }

        if fresh {
            for input in &inputs {
                input.update(cx, |state, cx| state.set_value("", window, cx));
            }
        }

        if let Some(state) = self.widgets.get_mut(block) {
            state.inputs = inputs;
        }
    }

    /// The dashboard. Data is loaded by the nav entry, so an empty state here
    /// means the read failed — the same "could not be loaded" shape practice
    /// uses rather than a spinner that never resolves.
    fn dashboard_screen(&self, theme: &Theme, view: Entity<AppView>) -> Div {
        let Some(data) = self.dashboard.as_ref() else {
            return self.shell(
                theme,
                view,
                "Dashboard",
                "Your progress could not be loaded.",
                None,
                div(),
            );
        };

        let body = crate::dashboard::dashboard(
            theme,
            view.clone(),
            data,
            |id| self.module_index(id),
            |id| self.lesson_index(id),
        );
        self.shell(theme, view, "Dashboard", "", None, body)
    }

    /// The progress totals screen.
    fn progress_screen(&self, theme: &Theme, view: Entity<AppView>) -> Div {
        let Some(data) = self.progress.as_ref() else {
            return self.shell(
                theme,
                view,
                "Progress",
                "Your progress could not be loaded.",
                None,
                div(),
            );
        };

        let body = crate::dashboard::progress(theme, data);
        self.shell(theme, view, "Progress", "", None, body)
    }

    /// The playground.
    ///
    /// The editor's element state is keyed rather than kept on the view
    /// because creating it needs `&mut Window`, which only `render` has — the
    /// same shape Button uses for its focus handle.
    fn playground_screen(
        &mut self,
        view: Entity<AppView>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Div {
        let editor = window.use_keyed_state("playground-code", cx, |window, cx| {
            EditorState::new(window, cx)
                .default_value(crate::playground::STARTER_CODE)
                .language("rust")
                .line_number(true)
        });
        self.playground.editor = Some(editor.clone());

        // Taken after the keyed state above, which needs `&mut cx`: the theme
        // borrows it too, and the two cannot overlap.
        let theme = cx.theme();
        let body = crate::playground::body(theme, view.clone(), &self.playground, &editor, cx);
        self.shell(
            theme,
            view,
            "Playground",
            "Write Rust, press RUN, and read what the compiler says.",
            None,
            body,
        )
    }

    /// The frame every screen shares: navbar, scrollable content column.
    fn shell(
        &self,
        theme: &Theme,
        view: Entity<AppView>,
        title: &str,
        subtitle: &str,
        active_module: Option<usize>,
        body: impl IntoElement,
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
    /// Hints, each behind its own disclosure — the `<details>` the site uses.
    ///
    /// Reveal state lives on the view rather than in a widget because the
    /// toggle writes to it, and the whole hint row is a Button so the
    /// disclosure is reachable and operable from the keyboard like the cards.
    fn hints_panel(&self, theme: &Theme, exercise: &Exercise, view: Entity<AppView>) -> Div {
        let hints = exercise_view::hints_of(exercise);
        if hints.is_empty() {
            return div();
        }

        let mut list = div().flex().flex_col().gap_2();

        for (index, hint) in hints.iter().enumerate() {
            let open = self.hints_open.contains(&index);
            let number = index + 1;
            let toggling = view.clone();

            let mut row = Button::new(format!("hint-{index}"))
                .ghost()
                .rounded_sm()
                .border_1()
                .border_color(if open { theme.info } else { theme.border })
                .p_3()
                .flex()
                .flex_col()
                .gap_1()
                .on_click(move |_, _, cx| {
                    toggling.update(cx, |view, cx| {
                        if !view.hints_open.remove(&index) {
                            view.hints_open.insert(index);
                        }
                        cx.notify();
                    });
                })
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .text_size(px(12.))
                                .text_color(theme.info)
                                .child(if open { "▾" } else { "▸" }),
                        )
                        .child(
                            div()
                                .text_size(px(11.))
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.muted_foreground)
                                .child(format!("HINT {number}")),
                        ),
                );

            if open {
                row = row.child(
                    div()
                        .text_size(px(13.))
                        .text_color(theme.foreground)
                        .child(hint.clone()),
                );
            }

            list = list.child(row);
        }

        div()
            .rounded_md()
            .border_1()
            .border_color(theme.border)
            .bg(theme.secondary)
            .p_4()
            .flex()
            .flex_col()
            .gap_3()
            .child(label(theme, "HINTS"))
            .child(list)
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
        let chosen = self.selected;

        let mut body = div()
            .flex()
            .flex_col()
            .gap_5()
            .child(exercise_view::question(theme, record))
            .child(self.hints_panel(theme, record, view.clone()));

        // Every seeded exercise carries options, so this is the branch the whole
        // course reaches: radios, and a submit labelled by kind. The two arms
        // that used to follow it were unreachable — that is what made 41
        // exercises look answerless when in fact they were only mis-labelled.
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
                        .label(exercise_view::submit_label(record))
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
        // Checked before correctness: an open-ended exercise has no right
        // answer, and `correct` for one is only whether a picked option matched
        // a stored index — reporting that as "Correct" would be inventing a
        // grade the course never set.
        let (mark, colour, words) = if record.is_open_ended() {
            ("✓", theme.success, "Recorded")
        } else if result.correct {
            ("✓", theme.success, "Correct")
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
            .child(exercise_view::question(theme, exercise))
            .child(self.hints_panel(theme, exercise, view.clone()));

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
    let to_dashboard = to_home.clone();
    let to_progress = to_home.clone();
    let to_playground = to_home.clone();

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
        )
        .child(
            Button::new("nav-dashboard")
                .ghost()
                .text_size(px(13.))
                .text_color(theme.muted_foreground)
                .child("DASHBOARD")
                .on_click(move |_, _, cx| {
                    let data = load_dashboard(cx);
                    to_dashboard.update(cx, |view, cx| {
                        view.dashboard = data;
                        view.screen = Screen::Dashboard;
                        cx.notify();
                    });
                }),
        )
        .child(
            Button::new("nav-progress")
                .ghost()
                .text_size(px(13.))
                .text_color(theme.muted_foreground)
                .child("PROGRESS")
                .on_click(move |_, _, cx| {
                    let data = load_progress(cx);
                    to_progress.update(cx, |view, cx| {
                        view.progress = data;
                        view.screen = Screen::Progress;
                        cx.notify();
                    });
                }),
        )
        .child(
            Button::new("nav-playground")
                .ghost()
                .text_size(px(13.))
                .text_color(theme.muted_foreground)
                .child("PLAYGROUND")
                .on_click(move |_, _, cx| {
                    to_playground.update(cx, |view, cx| {
                        view.clear_answer();
                        view.screen = Screen::Playground;
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let view = cx.entity();

        match self.screen.clone() {
            Screen::Home => self.home(theme, view),
            Screen::Module(index) => self.module_screen(theme, view, index),
            Screen::Lesson { module, lesson } => {
                self.lesson_screen(view, window, cx, module, lesson)
            }
            Screen::Exercise { module, exercise } => {
                self.exercise_screen(theme, view, module, exercise)
            }
            Screen::Practice { module } => self.practice_screen(theme, view, module),
            Screen::PracticeRun { module } => self.practice_run_screen(theme, view, module),
            Screen::PracticeResults { module } => self.practice_results_screen(theme, view, module),
            Screen::Dashboard => self.dashboard_screen(theme, view),
            Screen::Progress => self.progress_screen(theme, view),
            Screen::Playground => self.playground_screen(view, window, cx),
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

/// The dashboard's data, or nothing if it cannot be read.
fn load_dashboard(cx: &App) -> Option<DashboardData> {
    with_db(cx, |db| {
        db.block_on(progress::dashboard_data(db.pool())).ok()
    })
}

/// The progress totals, or nothing if they cannot be read.
fn load_progress(cx: &App) -> Option<ProgressSummary> {
    with_db(cx, |db| {
        db.block_on(progress::progress_summary(db.pool())).ok()
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

/// Compiles and runs the editor's buffer — or cancels the run in flight.
///
/// One entry point for the RUN button, the ⌘/CTRL+ENTER handler and the test
/// that drives the same path. The compile itself goes to the background
/// executor: `sandbox::run` waits on `rustc` for seconds, and doing that on
/// the UI thread would freeze the window for the whole timeout.
///
/// Named for what it runs, not for where: `start_run` was already the
/// practice runner's entry point, and two of them in one module is one too
/// many.
pub(crate) fn run_program(view: Entity<AppView>, editor: &Entity<EditorState>, cx: &mut App) {
    // A second press while one is in flight cancels it. The button stays live
    // in the loading state for exactly this reason — greying it out would
    // take away the only control the learner has over a long compile.
    if view.read(cx).playground.running {
        view.update(cx, |view, cx| {
            view.playground.run.take();
            view.playground.running = false;
            cx.notify();
        });
        return;
    }

    let code = editor.read(cx).value().to_string();
    if code.trim().is_empty() {
        return;
    }

    let executor = cx.background_executor().clone();
    let runner = view.clone();
    let task = cx.spawn(async move |cx| {
        let started = std::time::Instant::now();
        let result = executor.spawn(async move { sandbox::run(&code) }).await;
        let elapsed = started.elapsed().as_millis();

        cx.update(|cx| {
            runner.update(cx, |view, cx| {
                view.playground.result = Some(result.clone());
                view.playground.elapsed_ms = Some(elapsed);
                view.playground.running = false;
                cx.notify();
            });
        });

        result
    });

    view.update(cx, |view, cx| {
        view.playground.result = None;
        view.playground.elapsed_ms = None;
        view.playground.explanation_open = false;
        view.playground.running = true;
        view.playground.run = Some(task);
        cx.notify();
    });
}

/// Restores the starter program and clears every trace of the last run —
/// back to a clean slate, not just clean output.
pub(crate) fn reset_editor(
    view: Entity<AppView>,
    editor: &Entity<EditorState>,
    window: &mut Window,
    cx: &mut App,
) {
    editor.update(cx, |state, cx| {
        state.set_value(crate::playground::STARTER_CODE, window, cx);
    });
    view.update(cx, |view, cx| {
        view.playground.run.take();
        view.playground.result = None;
        view.playground.elapsed_ms = None;
        view.playground.running = false;
        view.playground.explanation_open = false;
        cx.notify();
    });
}

/// Puts the caret on the line a diagnostic points at and focuses the editor.
///
/// The sandbox shifts diagnostics back onto the learner's numbering before
/// they reach us, so `line` is already a line in the buffer as shown.
pub(crate) fn jump_to_line(
    editor: &Entity<EditorState>,
    line: usize,
    window: &mut Window,
    cx: &mut App,
) {
    let target = crate::component::input::Position {
        line: line.saturating_sub(1) as u32,
        character: 0,
    };
    editor.update(cx, |state, cx| {
        state.set_cursor_position(target, window, cx);
    });
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
mod ui_tests {
    use super::{
        answer_question, exercise_view, finish_run, load_builder, load_dashboard, load_progress,
        run_program, start_run, AppView, LessonContent, LessonSection, Screen,
    };
    use gpui_kit::{TestAppContext, TestDispatcher, VisualTestContext};

    /// Applies the BORROW_ theme and opens the app at home on this context.
    ///
    /// Takes `&mut TestAppContext` rather than building its own so the returned
    /// window context can borrow it — returning a reference to a local built
    /// inside would not compile.
    fn open(cx: &mut TestAppContext) -> (gpui_kit::Entity<AppView>, &mut VisualTestContext) {
        cx.update(|cx| {
            gpui_kit::init(cx);
            let _ = crate::theme::apply(cx);
        });

        let (view, window) = cx.add_window_view(|_, _| AppView::new(crate::content::all()));
        window.update(|window, cx| window.draw(cx).clear(cx));
        (view, window)
    }

    /// Opens the app on a database of its own.
    ///
    /// Practice writes, so a test needs a pool — but not *the* pool. Each test
    /// gets its own file, migrated and seeded fresh, rather than writing rows
    /// into the app's `borrow.db` or racing another test over one shared file.
    fn open_with_db<'a>(
        cx: &'a mut TestAppContext,
        name: &'static str,
    ) -> (gpui_kit::Entity<AppView>, &'a mut VisualTestContext) {
        let path = std::env::temp_dir().join(format!("borrow-ui-{name}-{}.db", std::process::id()));
        let _ = std::fs::remove_file(&path);

        let url = format!("sqlite:{}?mode=rwc", path.display());
        let db = crate::app_db::Db::connect_with(&url).expect("test database migrates and seeds");
        cx.update(|cx| cx.set_global(db));

        open(cx)
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

        // First stop is the BORROW_ wordmark in the navbar, then the three
        // nav entries, then the first module card. If the cards were plain
        // divs again the last stop would not be a card, and Enter would leave
        // us on Home.
        for _ in 0..5 {
            cx.update(|window, cx| window.focus_next(cx));
        }
        press(cx, "enter");
        cx.run_until_parked();
        cx.update(|window, cx| window.draw(cx).clear(cx));

        assert_eq!(
            screen_of(&view, cx),
            "Module(0)",
            "Enter on a focused module card should navigate"
        );
    }

    /// The nav entries reach both new screens with data behind them.
    ///
    /// Walking focus rather than setting `screen` directly is the point: it
    /// proves the buttons are in the focus order and that clicking them loads
    /// the data. `draw` is the assertion, as everywhere else here.
    #[test]
    fn nav_opens_the_dashboard_and_the_progress_screen() {
        let mut cx = TestAppContext::build(TestDispatcher::new(0), Some("nav_dashboard"));
        let (view, cx) = open_with_db(&mut cx, "nav_dashboard");

        // Wordmark, then DASHBOARD.
        for _ in 0..2 {
            cx.update(|window, cx| window.focus_next(cx));
        }
        press(cx, "enter");
        cx.run_until_parked();
        cx.update(|window, cx| window.draw(cx).clear(cx));

        assert_eq!(screen_of(&view, cx), "Dashboard");
        let loaded = cx.update(|_, cx| view.read(cx).dashboard.is_some());
        assert!(loaded, "the nav entry should load the dashboard data");

        // Focus is still on DASHBOARD, so the next stop is PROGRESS.
        cx.update(|window, cx| window.focus_next(cx));
        press(cx, "enter");
        cx.run_until_parked();
        cx.update(|window, cx| window.draw(cx).clear(cx));

        assert_eq!(screen_of(&view, cx), "Progress");
        let totals = cx.update(|_, cx| view.read(cx).progress.clone());
        let totals = totals.expect("the nav entry should load the progress totals");
        assert!(totals.total_lessons > 0, "seeded course has lessons");
        assert!(totals.total_modules > 0, "seeded course has modules");
    }

    /// Both screens must draw against the seeded database, not an empty state.
    #[test]
    fn dashboard_and_progress_render_with_seeded_data() {
        let mut cx = TestAppContext::build(TestDispatcher::new(0), Some("dash_progress"));
        let (view, cx) = open_with_db(&mut cx, "dash_progress");

        cx.update(|_, cx| {
            let dashboard = load_dashboard(cx);
            let progress = load_progress(cx);
            view.update(cx, |view, cx| {
                view.dashboard = dashboard;
                view.progress = progress;
                view.screen = Screen::Dashboard;
                cx.notify();
            });
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));

        assert_eq!(screen_of(&view, cx), "Dashboard");
        let data = cx.update(|_, cx| view.read(cx).dashboard.clone());
        let data = data.expect("dashboard data loaded");
        assert!(!data.modules.is_empty(), "the curriculum has modules");
        assert_eq!(
            data.lessons_per_module.len(),
            data.modules.len(),
            "every module carries its lessons"
        );

        // The continue card needs a target: the first incomplete lesson must
        // resolve to a screen position, or the card renders with no buttons.
        if let Some(lesson_id) = data.first_incomplete_lesson {
            let target = cx.update(|_, cx| view.read(cx).lesson_index(lesson_id));
            assert!(
                target.is_some(),
                "lesson {lesson_id} should be on the screen somewhere"
            );
        }

        cx.update(|_, cx| {
            view.update(cx, |view, cx| {
                view.screen = Screen::Progress;
                cx.notify();
            });
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));

        assert_eq!(screen_of(&view, cx), "Progress");
        let totals = cx.update(|_, cx| view.read(cx).progress.clone());
        let totals = totals.expect("progress data loaded");
        assert!(totals.total_lessons > 0);
        assert!(totals.total_xp >= 0);
    }

    /// The playground must draw with a real editor behind it.
    ///
    /// The editor's state only exists after the first draw — it is keyed on
    /// the window — so this is also what proves the buffer carries the
    /// starter program and that RUN therefore starts out usable.
    #[test]
    fn playground_draws_with_the_starter_program() {
        let mut cx = TestAppContext::build(TestDispatcher::new(0), Some("playground"));
        let (view, cx) = open(&mut cx);

        cx.update(|_, cx| {
            view.update(cx, |view, cx| {
                view.navigate(Screen::Playground);
                cx.notify();
            });
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert_eq!(screen_of(&view, cx), "Playground");

        let buffer = cx.update(|_, cx| {
            view.read(cx)
                .playground
                .editor
                .as_ref()
                .map(|editor| editor.read(cx).value().to_string())
        });
        let buffer = buffer.expect("the first draw creates the editor state");
        assert_eq!(
            buffer,
            crate::playground::STARTER_CODE,
            "the editor opens on the site's starter program"
        );
        assert!(!buffer.trim().is_empty(), "RUN is enabled for it");
    }

    /// Compile-and-run through the same entry point the button calls, then
    /// draw the console with the result behind it.
    ///
    /// `draw` is the assertion again: the timing line, stdout and the status
    /// all have to survive a real render, and a panic here is a screen that
    /// would break on a learner's machine.
    #[test]
    fn a_program_runs_and_the_console_reports_it() {
        use crate::services::sandbox::Status;

        let mut cx = TestAppContext::build(TestDispatcher::new(0), Some("playground_run"));
        let (view, cx) = open(&mut cx);

        cx.update(|_, cx| {
            view.update(cx, |view, cx| {
                view.navigate(Screen::Playground);
                cx.notify();
            });
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));

        cx.update(|_, cx| {
            let editor = view
                .read(cx)
                .playground
                .editor
                .clone()
                .expect("the first draw creates the editor state");
            run_program(view.clone(), &editor, cx);
        });
        cx.run_until_parked();

        let result = cx.update(|_, cx| view.read(cx).playground.result.clone());
        let result = result.expect("the run should have finished");
        assert_eq!(result.status, Status::Ok, "the starter program runs");
        assert!(
            result.stdout.contains("hello"),
            "its output should reach the console, got {:?}",
            result.stdout
        );

        cx.update(|window, cx| window.draw(cx).clear(cx));
    }

    /// A program that will not compile has to arrive as structured
    /// diagnostics, and the disclosure that hides the long explanation has to
    /// toggle without taking the screen down.
    #[test]
    fn a_compile_error_renders_diagnostics_and_its_disclosure() {
        let mut cx = TestAppContext::build(TestDispatcher::new(0), Some("playground_error"));
        let (view, cx) = open(&mut cx);

        cx.update(|_, cx| {
            view.update(cx, |view, cx| {
                view.navigate(Screen::Playground);
                cx.notify();
            });
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));

        // Replace the buffer through the setter RESET uses, so the editor and
        // the run see the same text a learner would type.
        cx.update(|window, cx| {
            let editor = view
                .read(cx)
                .playground
                .editor
                .clone()
                .expect("the first draw creates the editor state");
            editor.update(cx, |state, cx| {
                state.set_value("fn main() { let x: i32 = \"nope\"; }", window, cx);
            });
        });

        cx.update(|_, cx| {
            let editor = view.read(cx).playground.editor.clone().expect("editor");
            run_program(view.clone(), &editor, cx);
        });
        cx.run_until_parked();

        let count = cx.update(|_, cx| {
            view.read(cx)
                .playground
                .result
                .as_ref()
                .map(|result| result.diagnostics.len())
        });
        assert!(
            count.unwrap_or(0) > 0,
            "a type error should reach the console as a diagnostic, got {count:?}"
        );
        cx.update(|window, cx| window.draw(cx).clear(cx));

        // Open the "WHY THIS HAPPENS" disclosure and draw again: it hides the
        // long write-up, and rendering it is the part that can break.
        cx.update(|_, cx| {
            view.update(cx, |view, cx| {
                view.playground.explanation_open = true;
                cx.notify();
            });
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));
    }

    /// The shortcut the hint promises: ⌘/CTRL+ENTER runs what is in the
    /// buffer while the editor has focus.
    ///
    /// The key event reaches the wrapper because the editor sits inside it —
    /// if that ever stops being true the hint is lying, and this is the test
    /// that would say so.
    #[test]
    fn the_run_shortcut_fires_from_the_editor() {
        let mut cx = TestAppContext::build(TestDispatcher::new(0), Some("playground_shortcut"));
        let (view, cx) = open(&mut cx);

        cx.update(|_, cx| {
            view.update(cx, |view, cx| {
                view.navigate(Screen::Playground);
                cx.notify();
            });
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));

        cx.update(|window, cx| {
            let editor = view
                .read(cx)
                .playground
                .editor
                .clone()
                .expect("the first draw creates the editor state");
            editor.update(cx, |state, cx| state.focus(window, cx));
        });

        press(cx, "cmd-enter");
        cx.run_until_parked();

        let ran = cx.update(|_, cx| view.read(cx).playground.result.is_some());
        assert!(ran, "the shortcut should have started a run");
    }

    /// The first lesson carrying a widget of each kind, straight out of the
    /// seeded content — a content edit that drops a widget would fail here
    /// rather than leave the widget's renderer unexercised.
    fn widget_lessons(
        view: &gpui_kit::Entity<AppView>,
        cx: &gpui_kit::App,
    ) -> Vec<(&'static str, usize, usize)> {
        let wanted = ["gap", "step", "guess", "dispatch", "pipeline"];
        let mut found: Vec<(&'static str, usize, usize)> = Vec::new();

        for (module, seed) in view.read(cx).modules.iter().enumerate() {
            for (lesson, record) in seed.lessons.iter().enumerate() {
                let Ok(content) = serde_json::from_str::<LessonContent>(&record.content) else {
                    continue;
                };
                for section in &content.sections {
                    let kind = match section {
                        LessonSection::GapFill { .. } => "gap",
                        LessonSection::StepThrough { .. } => "step",
                        LessonSection::GuessRun { .. } => "guess",
                        LessonSection::Dispatch { .. } => "dispatch",
                        LessonSection::Pipeline { .. } => "pipeline",
                        _ => continue,
                    };
                    if wanted.contains(&kind) && !found.iter().any(|(k, ..)| *k == kind) {
                        found.push((kind, module, lesson));
                    }
                }
            }
        }

        found
    }

    /// The first gap-fill block: where it is, which block it is, its answers.
    fn first_gap(
        view: &gpui_kit::Entity<AppView>,
        cx: &gpui_kit::App,
    ) -> Option<(usize, usize, usize, Vec<String>)> {
        for (module, seed) in view.read(cx).modules.iter().enumerate() {
            for (lesson, record) in seed.lessons.iter().enumerate() {
                let Ok(content) = serde_json::from_str::<LessonContent>(&record.content) else {
                    continue;
                };
                for (block, section) in content.sections.iter().enumerate() {
                    if let LessonSection::GapFill { blanks, .. } = section {
                        if blanks.is_empty() {
                            continue;
                        }
                        return Some((
                            module,
                            lesson,
                            block,
                            blanks.iter().map(|blank| blank.answer.clone()).collect(),
                        ));
                    }
                }
            }
        }
        None
    }

    /// The first guess-then-run block: where it is, which block it is, its
    /// program.
    fn first_guess(
        view: &gpui_kit::Entity<AppView>,
        cx: &gpui_kit::App,
    ) -> Option<(usize, usize, usize, String)> {
        for (module, seed) in view.read(cx).modules.iter().enumerate() {
            for (lesson, record) in seed.lessons.iter().enumerate() {
                let Ok(content) = serde_json::from_str::<LessonContent>(&record.content) else {
                    continue;
                };
                for (block, section) in content.sections.iter().enumerate() {
                    if let LessonSection::GuessRun { code, .. } = section {
                        return Some((module, lesson, block, code.clone()));
                    }
                }
            }
        }
        None
    }

    /// Every interactive widget, rendered with a view behind it.
    ///
    /// The course sweep takes the fallback path — no view, no window. This is
    /// the path a learner takes: inputs created by the lesson screen, then
    /// each widget's state driven to an edge and drawn again. `draw` is the
    /// assertion throughout; the step counter goes to `usize::MAX` on purpose,
    /// because a clamped step must not panic the frame counter.
    #[test]
    fn every_interactive_widget_renders_against_seeded_content() {
        let mut cx = TestAppContext::build(TestDispatcher::new(0), Some("widgets"));
        let (view, cx) = open(&mut cx);

        let found = cx.update(|_, cx| widget_lessons(&view, cx));
        assert_eq!(
            found.len(),
            5,
            "the seeded course should teach with every widget, got {found:?}"
        );

        for (kind, module, lesson) in found {
            cx.update(|_, cx| {
                view.update(cx, |view, cx| {
                    view.navigate(Screen::Lesson { module, lesson });
                    cx.notify();
                });
            });
            cx.update(|window, cx| window.draw(cx).clear(cx));
            assert_eq!(
                screen_of(&view, cx),
                format!("Lesson {{ module: {module}, lesson: {lesson} }}")
            );

            if kind == "gap" {
                let inputs = cx.update(|_, cx| {
                    view.read(cx)
                        .widgets
                        .iter()
                        .map(|state| state.inputs.len())
                        .sum::<usize>()
                });
                assert!(inputs > 0, "a gap lesson should create its inputs");
            }

            cx.update(|_, cx| {
                view.update(cx, |view, cx| {
                    for state in &mut view.widgets {
                        state.step = usize::MAX;
                        state.checked = true;
                        state.verdicts = vec![Some(true), Some(false), None];
                        state.revealed = true;
                        state.choice = Some(0);
                    }
                    cx.notify();
                });
            });
            cx.update(|window, cx| window.draw(cx).clear(cx));
        }
    }

    /// The gap-fill loop end to end: type, check, read the verdict, reset.
    ///
    /// Driven through the same functions the buttons call, because a button's
    /// closure is not reachable from a test without simulating a focus walk
    /// through a lesson full of controls.
    #[test]
    fn gaps_grade_what_was_typed_and_reset_clears_it() {
        use crate::widgets::{check_gaps, reset_gaps};

        let mut cx = TestAppContext::build(TestDispatcher::new(0), Some("gap_fill"));
        let (view, cx) = open(&mut cx);

        let (module, lesson, block, answers) = cx
            .update(|_, cx| first_gap(&view, cx))
            .expect("the course has a gap-fill block");

        cx.update(|_, cx| {
            view.update(cx, |view, cx| {
                view.navigate(Screen::Lesson { module, lesson });
                cx.notify();
            });
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));

        let inputs = cx.update(|_, cx| {
            view.read(cx)
                .widgets
                .get(block)
                .map(|state| state.inputs.clone())
                .unwrap_or_default()
        });
        assert_eq!(
            inputs.len(),
            answers.len(),
            "one input per blank, from the lesson screen"
        );

        // The first blank gets the right answer, the second — if there is
        // one — something that is not it.
        cx.update(|window, cx| {
            inputs[0].update(cx, |state, cx| {
                state.set_value(answers[0].clone(), window, cx)
            });
            if let Some(second) = inputs.get(1) {
                second.update(cx, |state, cx| {
                    state.set_value("definitely wrong", window, cx)
                });
            }
        });

        cx.update(|_, cx| check_gaps(view.clone(), block, &answers, cx));
        let verdicts = cx.update(|_, cx| view.read(cx).widgets[block].verdicts.clone());
        assert_eq!(verdicts.first().copied().flatten(), Some(true));
        if answers.len() > 1 {
            assert_eq!(verdicts.get(1).copied().flatten(), Some(false));
        }
        cx.update(|window, cx| window.draw(cx).clear(cx));

        cx.update(|window, cx| reset_gaps(view.clone(), block, window, cx));
        let cleared = cx.update(|_, cx| {
            let state = &view.read(cx).widgets[block];
            (state.checked, state.verdicts.len())
        });
        assert_eq!(cleared, (false, 0), "RESET forgets the last verdict");
        let text = cx.update(|_, cx| inputs[0].read(cx).value().to_string());
        assert!(text.is_empty(), "RESET empties the field, got {text:?}");
        cx.update(|window, cx| window.draw(cx).clear(cx));
    }

    /// The guess widget compiles through the same path its button takes, and
    /// the graded result renders.
    #[test]
    fn a_guess_compiles_and_the_widget_reports_it() {
        use crate::widgets::run_guess;

        let mut cx = TestAppContext::build(TestDispatcher::new(0), Some("guess_run"));
        let (view, cx) = open(&mut cx);

        let (module, lesson, block, code) = cx
            .update(|_, cx| first_guess(&view, cx))
            .expect("the course has a guess-then-run block");

        cx.update(|_, cx| {
            view.update(cx, |view, cx| {
                view.navigate(Screen::Lesson { module, lesson });
                cx.notify();
            });
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));

        cx.update(|_, cx| {
            view.update(cx, |view, cx| {
                if let Some(state) = view.widgets.get_mut(block) {
                    state.choice = Some(0);
                }
                cx.notify();
            });
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));

        cx.update(|_, cx| run_guess(view.clone(), block, code, cx));
        cx.run_until_parked();

        let graded = cx.update(|_, cx| view.read(cx).widgets[block].guess.is_some());
        assert!(graded, "the program should have been run and reported");
        cx.update(|window, cx| window.draw(cx).clear(cx));
    }

    /// The practice builder must draw against real seeded data, not an empty
    /// state. A panic here is a broken screen — `draw` is the assertion.
    #[test]
    fn practice_builder_renders_with_seeded_data() {
        let mut cx = TestAppContext::build(TestDispatcher::new(0), Some("practice_builder"));
        let (view, cx) = open_with_db(&mut cx, "practice_builder");

        cx.update(|_, cx| {
            let module_id = view.read(cx).modules[1].module.id;
            let builder = load_builder(cx, module_id);
            view.update(cx, |view, cx| {
                view.clear_practice();
                view.practice.builder = builder;
                view.screen = Screen::Practice { module: 1 };
                cx.notify();
            });
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));

        assert_eq!(screen_of(&view, cx), "Practice { module: 1 }");

        let exercises = cx.update(|_, cx| {
            view.read(cx)
                .practice
                .builder
                .as_ref()
                .map(|builder| builder.exercises.len())
        });
        assert!(
            exercises.unwrap_or(0) > 0,
            "the builder should carry the module's exercises, got {exercises:?}"
        );
    }

    /// Start a run, answer a question, finish — each step rendered.
    ///
    /// This is the coverage the screen grabs could not give: the run and
    /// results screens were wired in phase 4 and never seen to render.
    #[test]
    fn a_run_can_be_started_answered_and_finished() {
        let mut cx = TestAppContext::build(TestDispatcher::new(0), Some("practice_run"));
        let (view, cx) = open_with_db(&mut cx, "practice_run");

        cx.update(|_, cx| {
            let module_id = view.read(cx).modules[1].module.id;
            let page = start_run(cx, module_id, Some(3));
            view.update(cx, |view, cx| {
                view.clear_practice();
                view.practice.run = page;
                view.screen = Screen::PracticeRun { module: 1 };
                cx.notify();
            });
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));

        assert_eq!(screen_of(&view, cx), "PracticeRun { module: 1 }");

        let requested = cx.update(|_, cx| {
            view.read(cx)
                .practice
                .run
                .as_ref()
                .map(|page| page.items.len())
        });
        assert_eq!(requested, Some(3), "three questions were asked for");

        // Answer the first question and finish the run, rendering both.
        cx.update(|_, cx| {
            let page = view.read(cx).practice.run.clone().expect("run loaded");
            let run_id = page.run.id;
            let exercise = page.items[0].exercise.clone();

            let _ = answer_question(cx, run_id, &exercise, Some(0));
            let results = finish_run(cx, run_id);

            view.update(cx, |view, cx| {
                view.practice.results = results;
                view.screen = Screen::PracticeResults { module: 1 };
                cx.notify();
            });
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));

        assert_eq!(screen_of(&view, cx), "PracticeResults { module: 1 }");

        let answers = cx.update(|_, cx| {
            view.read(cx)
                .practice
                .results
                .as_ref()
                .map(|results| results.answers.len())
        });
        assert_eq!(
            answers,
            Some(1),
            "the one answer recorded should be in the results"
        );
    }

    /// Hints are disclosures, one per hint, matching the site's `<details>`.
    ///
    /// The panel is rendered on every draw, so a panic in it fails here rather
    /// than on a learner's screen; the state assertions check that revealing
    /// one does not reveal the rest, and that leaving the exercise closes them.
    #[test]
    fn hints_reveal_independently_and_reset_per_exercise() {
        let mut cx = TestAppContext::build(TestDispatcher::new(0), Some("hints"));
        let (view, cx) = open(&mut cx);

        // Find a seeded exercise that actually has hints, so the test does not
        // rot when content moves between modules.
        let spot = cx.update(|_, cx| {
            let view = view.read(cx);
            view.modules.iter().enumerate().find_map(|(m, seed)| {
                seed.exercises
                    .iter()
                    .enumerate()
                    .find(|(_, exercise)| !exercise_view::hints_of(exercise).is_empty())
                    .map(|(e, _)| (m, e))
            })
        });

        let (module, exercise) = spot.expect("the course has an exercise with hints");

        cx.update(|_, cx| {
            view.update(cx, |view, cx| {
                view.clear_answer();
                view.screen = Screen::Exercise { module, exercise };
                cx.notify();
            });
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert_eq!(
            screen_of(&view, cx),
            format!("Exercise {{ module: {module}, exercise: {exercise} }}")
        );
        assert_eq!(cx.update(|_, cx| view.read(cx).hints_open.len()), 0);

        // Opening one hint reveals that one.
        cx.update(|_, cx| {
            view.update(cx, |view, cx| {
                view.hints_open.insert(0);
                cx.notify();
            });
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));
        assert_eq!(
            cx.update(|_, cx| { view.read(cx).hints_open.iter().copied().collect::<Vec<_>>() }),
            vec![0],
            "one hint open should not open the others"
        );

        // Leaving the exercise closes them, so a later one starts clean.
        cx.update(|_, cx| {
            view.update(cx, |view, cx| {
                view.clear_answer();
                cx.notify();
            });
        });
        assert_eq!(cx.update(|_, cx| view.read(cx).hints_open.len()), 0);
    }
}
