//! Screens and navigation: home → module → lesson.
//!
//! One `AppView` owns the screen, and buttons capture a handle to the view so a
//! click can update that state and request a re-render. Nothing here is async —
//! `content::all()` is compiled in, so switching screens costs a render rather
//! than a query.

use crate::app_db::Db;
use crate::auth::{self, StoredSession};
use crate::components::label;
use crate::content::ModuleSeed;
use crate::exercise as exercise_view;
use crate::lesson::render_section;
use crate::models::{Exercise, GapBlank, LessonContent, LessonSection};
use crate::services::practice;
use crate::services::progress::{self, DashboardData, ProgressSummary};
use crate::services::remote::{self as api, FetchError, Me};
use crate::services::sandbox::{self, RunResult};
use crate::widgets::{BlockState, WidgetCtx};
use gpui_kit::component::avatar::Avatar;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{EditorState, InputState};
use gpui_kit::component::Size;
use gpui_kit::component::*;
// `overflow_y_scroll` is on StatefulInteractiveElement, not InteractiveElement,
// and a stateful element needs an id to hang its state on.
use gpui_kit::*;
use gpui_kit::{InteractiveElement, StatefulInteractiveElement};
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

/// Where the app is showing. Module and lesson indices point into
/// [`AppView::modules`], which is fixed for the process's lifetime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Screen {
    /// The login gate — the only screen a signed-out app can reach.
    Login,
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
    /// The $5/month paywall, reached from a locked module.
    Upgrade,
    /// Signed-in identity: plan status, billing, sign out.
    Account,
    /// The how-to-use tour.
    Help,
}

/// Why a module is closed, so the UI can offer the right door.
///
/// A progression lock is study advice — wait your turn. A premium lock is a
/// paywall — here is the way through.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleLock {
    Open,
    Progression,
    Premium,
}

/// Modules 1–3 are free; everything from 4 on needs a subscription.
/// Zero-based: index 3 is "module 4".
pub const FREE_MODULES: usize = 3;

/// What the login and billing flows are doing between clicks. Tasks are
/// held so a flow survives its own awaits — dropping a `Task` cancels it.
#[derive(Default)]
pub(crate) struct AuthUi {
    login_pending: bool,
    login_task: Option<Task<()>>,
    /// The one-shot entitlement refresh, held so it is not cancelled
    /// mid-flight by its own drop. `pub(crate)` because the launch path
    /// arms it from `app.rs` inside the same entity update it adopts the
    /// session in.
    pub(crate) refresh_task: Option<Task<()>>,
    login_error: Option<String>,
    /// Hosted checkout is open in the browser; the app polls entitlement
    /// until it lands or the window gives up.
    checkout_pending: bool,
    checkout_task: Option<Task<()>>,
    billing_error: Option<String>,
    /// The daily entitlement refresh, running for the app's whole life.
    pub(crate) daily_task: Option<Task<()>>,
    /// CHANGE PICTURE in flight: picked file on its way to Convex.
    avatar_pending: bool,
    avatar_task: Option<Task<()>>,
    avatar_error: Option<String>,
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
    /// The in-flight run. Dropping it abandons the result — not the process:
    /// the sandbox owns the child, so [`Self::cancel`] is what kills it.
    run: Option<Task<RunResult>>,
    /// The flag that stops the child behind the in-flight run. Raised on
    /// cancel and cleared when the run finishes or is replaced.
    cancel: Option<sandbox::CancelFlag>,
    /// The editor's Enter subscription, made on the first draw. Kept for as
    /// long as the view lives: a dropped subscription is a silent shortcut.
    shortcut: Option<Subscription>,
    /// The single open "WHY THIS HAPPENS" disclosure.
    pub(crate) explanation_open: bool,
    /// A lesson's code block, carried here by "TRY IN PLAYGROUND" and
    /// dropped into the editor on the next draw — the editor needs
    /// `&mut Window`, so the swap cannot happen at click time.
    pub(crate) pending_code: Option<String>,
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
    /// The signed-in account, or `None` behind the login gate. The
    /// entitlement inside it is the cached last-known state — see
    /// [`StoredSession::premium`]: never recomputed locally, never expired
    /// offline.
    pub(crate) session: Option<StoredSession>,
    pub(crate) auth: AuthUi,
    /// Where the learner has been, newest last — what `← BACK` walks out
    /// of. Recorded in `render`, so *every* transition lands here no matter
    /// which handler set the screen.
    history: Vec<Screen>,
    /// The screen the last render saw; how `record_history` notices a
    /// change without re-recording a `back()`'s own write.
    last_recorded: Option<Screen>,
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
            session: None,
            auth: AuthUi::default(),
            history: Vec::new(),
            last_recorded: None,
        }
    }

    /// Records a screen transition into `history`, called once per render.
    ///
    /// Recording here rather than in `navigate` means the direct
    /// `view.screen = …` writes (the pager, the navbar, the snippet jumps)
    /// are captured too — without rewriting twenty call sites to remember
    /// to ask. Login is never part of the trail: `← BACK` must not walk
    /// out through the gate.
    fn record_history(&mut self) {
        let current = self.screen.clone();
        if let Some(previous) = self.last_recorded.clone() {
            let changed = previous != current;
            let past_the_gate = previous != Screen::Login && current != Screen::Login;
            let not_already_there = self.history.last() != Some(&previous);
            if changed && past_the_gate && not_already_there {
                self.history.push(previous);
                if self.history.len() > 30 {
                    self.history.remove(0);
                }
            }
        }
        self.last_recorded = Some(current);
    }

    /// Pops one screen off the trail. No history: no-op — there is no
    /// "before" to return to, and inventing one (Home?) would lie.
    pub(crate) fn go_back(&mut self) {
        if let Some(previous) = self.history.pop() {
            self.clear_answer();
            self.screen = previous.clone();
            self.last_recorded = Some(previous);
        }
    }

    /// Adopts (or drops) the account and moves the screen to match: a login
    /// lands on Home, a sign-out falls back to the gate. Refreshes go
    /// through [`Self::session`] directly so they never yank the screen
    /// out from under the learner.
    pub(crate) fn adopt_session(&mut self, session: Option<StoredSession>) {
        self.session = session;
        self.screen = if self.session.is_some() {
            Screen::Home
        } else {
            Screen::Login
        };
    }

    /// The cached premium flag — see [`StoredSession::premium`].
    pub(crate) fn premium(&self) -> bool {
        self.session
            .as_ref()
            .map(auth::StoredSession::premium)
            .unwrap_or(false)
    }

    /// What stands between the learner and module `index`.
    ///
    /// Premium *adds to* the course's sequential design rather than
    /// replacing it: paying unlocks the paywall, not the ability to skip
    /// module 4 before finishing module 3.
    pub(crate) fn module_lock(
        &self,
        index: usize,
        completed_lessons: &[i64],
        completed_exercises: &[i64],
    ) -> ModuleLock {
        let progression_ok = index == 0
            || self.module_is_complete(index - 1, completed_lessons, completed_exercises);
        if !progression_ok {
            return ModuleLock::Progression;
        }
        if index >= FREE_MODULES && !self.premium() {
            return ModuleLock::Premium;
        }
        ModuleLock::Open
    }

    /// Whether module `index` can be opened at all. The `bool` face of
    /// [`Self::module_lock`] — what the gating tests speak.
    #[allow(dead_code)] // read by `ui_tests`; the UI asks for the lock itself
    fn module_is_unlocked(
        &self,
        index: usize,
        completed_lessons: &[i64],
        completed_exercises: &[i64],
    ) -> bool {
        self.module_lock(index, completed_lessons, completed_exercises) == ModuleLock::Open
    }

    /// Whether a screen that belongs to `module_index` should hand over to
    /// the paywall instead. Progression is irrelevant here: if the module
    /// needs premium and there is no premium, that is the answer.
    fn needs_premium(&self, module_index: usize) -> bool {
        module_index >= FREE_MODULES && !self.premium()
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

    /// Whether every lesson and every exercise in module `index` is done.
    ///
    /// Exercises count: a module's challenges and BUILD IT briefs are part of
    /// the work, and open-ended ones still record completion when submitted
    /// (`record_exercise_result` receives `correct || open_ended`).
    fn module_is_complete(
        &self,
        index: usize,
        completed_lessons: &[i64],
        completed_exercises: &[i64],
    ) -> bool {
        let Some(seed) = self.modules.get(index) else {
            return false;
        };
        !seed.lessons.is_empty()
            && seed
                .lessons
                .iter()
                .all(|l| completed_lessons.contains(&l.id))
            && seed
                .exercises
                .iter()
                .all(|e| completed_exercises.contains(&e.id))
    }

    /// The landing screen: what BORROW_ is and why it is worth starting.
    ///
    /// The curriculum itself lives on the Dashboard (reached by the nav or
    /// VIEW CURRICULUM); this screen's job is to sell the product and hand
    /// the learner straight into their next lesson.
    fn home(&self, theme: &Theme, view: Entity<AppView>, cx: &App) -> Div {
        let (completed_lessons, _) = completed_progress(cx);

        // The first lesson not yet complete, in course order. Fresh account:
        // lesson 1 of module 1. Everything done: nowhere to resume, so the
        // button points at the dashboard instead.
        let first_incomplete = self.modules.iter().enumerate().find_map(|(m, seed)| {
            seed.lessons
                .iter()
                .position(|l| !completed_lessons.contains(&l.id))
                .map(|l| (m, l))
        });

        let mut ctas = div().flex().flex_row().gap_3().pt_2();

        match first_incomplete {
            Some((m, l)) => {
                let jump = view.clone();
                ctas = ctas.child(
                    Button::new("start-learning")
                        .primary()
                        .label("START LEARNING")
                        .on_click(move |_, _, cx| {
                            jump.update(cx, |view, cx| {
                                view.clear_answer();
                                view.screen = Screen::Lesson {
                                    module: m,
                                    lesson: l,
                                };
                                cx.notify();
                            });
                        }),
                );
            }
            None => {
                let jump = view.clone();
                ctas = ctas.child(
                    Button::new("start-learning")
                        .primary()
                        .label("VIEW PROGRESS")
                        .on_click(move |_, _, cx| {
                            let data = load_dashboard(cx);
                            jump.update(cx, |view, cx| {
                                view.dashboard = data;
                                view.screen = Screen::Dashboard;
                                cx.notify();
                            });
                        }),
                );
            }
        }

        let jump = view.clone();
        ctas = ctas.child(
            Button::new("view-curriculum")
                .secondary()
                .label("VIEW CURRICULUM")
                .on_click(move |_, _, cx| {
                    let data = load_dashboard(cx);
                    jump.update(cx, |view, cx| {
                        view.dashboard = data;
                        view.screen = Screen::Dashboard;
                        cx.notify();
                    });
                }),
        );

        let module_count = self.modules.len();
        let lesson_count: usize = self.modules.iter().map(|s| s.lessons.len()).sum();
        let exercise_count: usize = self.modules.iter().map(|s| s.exercises.len()).sum();

        let hero = div()
            .flex()
            .flex_col()
            .gap_3()
            .pt_8()
            .pb_2()
            .child(
                div()
                    .text_size(px(40.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.foreground)
                    .child("LEARN RUST"),
            )
            .child(
                div()
                    .text_size(px(20.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.primary)
                    .child("WITHOUT THE 700-PAGE DETOUR."),
            )
            .child(
                div()
                    .text_size(px(14.))
                    .text_color(theme.muted_foreground)
                    .max_w(px(560.))
                    .child(
                        "Short lessons. Exercises that check your answers as you go. \
                         A playground backed by the real compiler. Learn the \
                         fundamentals by writing Rust, not by reading about it.",
                    ),
            )
            .child(ctas);

        let stat = |value: String, name: String| {
            div()
                .flex()
                .flex_col()
                .gap_1()
                .min_w(px(100.))
                .child(
                    div()
                        .text_size(px(24.))
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.primary)
                        .child(value),
                )
                .child(
                    div()
                        .text_size(px(11.))
                        .text_color(theme.muted_foreground)
                        .child(name),
                )
        };

        let stats = div()
            .flex()
            .flex_row()
            .gap_10()
            .py_4()
            .my_4()
            .border_t_1()
            .border_b_1()
            .border_color(theme.border)
            .child(stat(module_count.to_string(), "MODULES".to_string()))
            .child(stat(lesson_count.to_string(), "LESSONS".to_string()))
            .child(stat(exercise_count.to_string(), "EXERCISES".to_string()));

        let feature = |title: String, description: String| {
            div()
                .flex_1()
                .min_w(px(280.))
                .p_4()
                .rounded_md()
                .border_1()
                .border_color(theme.border)
                .bg(theme.secondary)
                .flex()
                .flex_col()
                .gap_2()
                .child(
                    div()
                        .text_size(px(13.))
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.foreground)
                        .child(title),
                )
                .child(
                    div()
                        .text_size(px(12.))
                        .text_color(theme.muted_foreground)
                        .child(description),
                )
        };

        let features = div()
            .flex()
            .flex_row()
            .gap_3()
            .flex_wrap()
            .child(feature(
                "INTERACTIVE LESSONS".to_string(),
                "Fill-in-the-gap code, step-through walkthroughs, and \
                 guess-then-run exercises that compile your answer with the \
                 real compiler."
                    .to_string(),
            ))
            .child(feature(
                "PRACTICE RUNS".to_string(),
                "Graded question sets taken one at a time. Jump anywhere, \
                 skip freely, and get a scored result at the end."
                    .to_string(),
            ))
            .child(feature(
                "CODE PLAYGROUND".to_string(),
                "A full Rust editor backed by real rustc. Write, run, and \
                 read the compiler's actual diagnostics."
                    .to_string(),
            ))
            .child(feature(
                "PROGRESS TRACKING".to_string(),
                "XP, completion bars, and a dashboard that always shows \
                 exactly where you left off."
                    .to_string(),
            ));

        let body = div()
            .flex()
            .flex_col()
            .gap_2()
            .child(hero)
            .child(stats)
            .child(features);

        // No shell heading — the hero is the heading.
        self.shell(theme, view, "", "", None, body)
    }

    fn module_screen(&self, theme: &Theme, view: Entity<AppView>, index: usize, cx: &App) -> Div {
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
                    .with_size(Size::Size(px(200.)))
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
                    .child(
                        div()
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
                                    .font_weight(FontWeight::BOLD)
                                    .child(lesson_title),
                            ),
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
                    .with_size(Size::Size(px(200.)))
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
                    .child(
                        div()
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

        let target = view.clone();
        let practice_module = index;
        let module_id = seed.module.id;
        let next_module_target = target.clone();
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

        let (completed_lessons, completed_exercises) = completed_progress(cx);
        let module_done = self.module_is_complete(index, &completed_lessons, &completed_exercises);

        if module_done && index + 1 < self.modules.len() {
            let next_index = index + 1;
            let next_title = self.modules[next_index].module.title.clone();
            let jump_to_module = next_module_target.clone();
            body = body.child(
                div().flex().flex_row().justify_center().pt_4().child(
                    Button::new("next-module")
                        .primary()
                        // Custom size opts out of Medium's fixed h_8: the
                        // padding and label define the height, so the text
                        // is not squeezed into a 32px band (the clipping in
                        // the report).
                        .with_size(Size::Size(px(50.)))
                        .text_size(px(15.))
                        .px_5()
                        .py_3()
                        .label(format!("NEXT MODULE: {next_title} \u{2192}"))
                        .on_click(move |_, _, cx| {
                            jump_to_module.update(cx, |view, cx| {
                                view.screen = Screen::Module(next_index);
                                cx.notify();
                            });
                        }),
                ),
            );
        } else if !module_done {
            // The lessons can be finished while the exercises are not — the
            // state the first test pass got stuck in, where the module felt
            // done and every "go on" button was invisible. Point at the
            // exercises that are actually left instead of hiding the door.
            let lessons_done = seed
                .lessons
                .iter()
                .all(|lesson| completed_lessons.contains(&lesson.id));
            if lessons_done {
                if let Some((position, remaining)) = next_open_exercise(seed, &completed_exercises)
                {
                    let jump = view.clone();
                    body = body.child(
                        div().flex().flex_row().justify_center().pt_4().child(
                            Button::new("continue-exercises")
                                .primary()
                                // Same fix as NEXT MODULE: hug the content
                                // instead of clipping into Medium's h_8.
                                .with_size(Size::Size(px(50.)))
                                .text_size(px(15.))
                                .px_5()
                                .py_3()
                                .label(format!(
                                    "CONTINUE \u{2014} {remaining} EXERCISE{} LEFT \u{2192}",
                                    if remaining == 1 { "" } else { "S" }
                                ))
                                .on_click(move |_, _, cx| {
                                    jump.update(cx, |view, cx| {
                                        view.clear_answer();
                                        view.screen = Screen::Exercise {
                                            module: index,
                                            exercise: position,
                                        };
                                        cx.notify();
                                    });
                                }),
                        ),
                    );
                }
            }
        }

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

        let is_completed = cx
            .try_global::<Db>()
            .map(|db| {
                db.block_on(progress::is_lesson_completed(db.pool(), lesson_id))
                    .unwrap_or(false)
            })
            .unwrap_or(false);

        if is_completed && lesson + 1 < total {
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
        } else if !is_completed {
            // Completion is recorded here — nothing else in the desktop app
            // marks a lesson done, and without this the dashboard, the
            // progress bars, and progressive unlocking would never advance.
            let to = view.clone();
            pager = pager.child(
                Button::new("complete")
                    .primary()
                    .label("MARK COMPLETE \u{2192}")
                    .on_click(move |_, _, cx| {
                        if let Some(db) = cx.try_global::<Db>() {
                            let _ =
                                db.block_on(progress::mark_lesson_complete(db.pool(), lesson_id));
                        }
                        to.update(cx, |v, cx| {
                            v.clear_answer();
                            cx.notify();
                        });
                        // The row exists locally now; tell the account.
                        spawn_push_now(to.read(cx), cx);
                    }),
            );
        } else {
            // Last lesson, already complete: hand the learner to the next
            // module once the whole module is done, otherwise back to the
            // module page where the remaining exercises live.
            let (done_lessons, done_exercises) = completed_progress(cx);
            if self.module_is_complete(module, &done_lessons, &done_exercises)
                && module + 1 < self.modules.len()
            {
                let next_index = module + 1;
                let next_title = self.modules[next_index].module.title.clone();
                let to = view.clone();
                pager = pager.child(
                    Button::new("next-module")
                        .primary()
                        .label(format!("NEXT MODULE: {next_title} \u{2192}"))
                        .on_click(move |_, _, cx| {
                            to.update(cx, |v, cx| {
                                v.clear_answer();
                                v.screen = Screen::Module(next_index);
                                cx.notify();
                            });
                        }),
                );
            } else if let Some((position, remaining)) = self
                .module(module)
                .and_then(|seed| next_open_exercise(seed, &done_exercises))
            {
                // Lessons are done but the module is not: "MODULE DONE" was
                // a lie. The exercises are the way on — say so, and go
                // straight to the first one that is left.
                let to = view.clone();
                pager = pager.child(
                    Button::new("continue-exercises")
                        .primary()
                        .label(format!(
                            "CONTINUE \u{2014} {remaining} EXERCISE{} LEFT \u{2192}",
                            if remaining == 1 { "" } else { "S" }
                        ))
                        .on_click(move |_, _, cx| {
                            to.update(cx, |v, cx| {
                                v.clear_answer();
                                v.screen = Screen::Exercise {
                                    module,
                                    exercise: position,
                                };
                                cx.notify();
                            });
                        }),
                );
            } else {
                let to = view.clone();
                pager = pager.child(
                    Button::new("module-done")
                        .primary()
                        .label("MODULE DONE \u{2192}")
                        .on_click(move |_, _, cx| {
                            to.update(cx, |v, cx| {
                                v.clear_answer();
                                v.screen = Screen::Module(module);
                                cx.notify();
                            });
                        }),
                );
            }
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
    fn dashboard_screen(&self, theme: &Theme, view: Entity<AppView>, cx: &App) -> Div {
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

        let (completed_lessons, completed_exercises) = completed_progress(cx);
        let locks: Vec<ModuleLock> = (0..self.modules.len())
            .map(|i| self.module_lock(i, &completed_lessons, &completed_exercises))
            .collect();

        let body = crate::dashboard::dashboard(
            theme,
            view.clone(),
            data,
            &locks,
            |id| self.module_index(id),
            |id| self.lesson_index(id),
        );
        self.shell(theme, view, "Dashboard", "", None, body)
    }

    /// The login gate: wordmark, the two providers, and nothing else. The
    /// hard gate from the design — no account, no app — so this screen has
    /// no navbar to wander off with.
    fn login_screen(&self, theme: &Theme, view: Entity<AppView>, _cx: &App) -> Div {
        let configured = crate::config::convex_site_url().is_some();
        let google_ready = crate::config::google_client_id().is_some();
        let github_ready = crate::config::github_client_id().is_some();
        let pending = self.auth.login_pending;

        let mut buttons = div().flex().flex_col().gap_3().w(px(300.));
        for (key, text, ready) in [
            ("google", "CONTINUE WITH GOOGLE", google_ready),
            ("github", "CONTINUE WITH GITHUB", github_ready),
        ] {
            let enabled = configured && ready && !pending;
            let jump = view.clone();
            buttons = buttons.child(
                Button::new(format!("login-{key}"))
                    .primary()
                    .label(text)
                    .disabled(!enabled)
                    .on_click(move |_, _, cx| {
                        jump.update(cx, |view, cx| start_login(view, cx, key));
                    }),
            );
        }

        let mut status = div().flex().flex_col().gap_2().items_center();
        if pending {
            status = status.child(
                div()
                    .text_size(px(12.))
                    .text_color(theme.muted_foreground)
                    .child("WAITING FOR THE BROWSER…"),
            );
        }
        if let Some(error) = &self.auth.login_error {
            status = status.child(
                div()
                    .text_size(px(12.))
                    .text_color(theme.warning)
                    .child(error.clone()),
            );
        }
        if !configured {
            status = status.child(
                div()
                    .text_size(px(12.))
                    .text_color(theme.warning)
                    .child("Sign-in is not configured for this build. See SETUP.md."),
            );
        }

        let card = div()
            .rounded_md()
            .border_1()
            .border_color(theme.border)
            .bg(theme.secondary)
            .p_8()
            .flex()
            .flex_col()
            .gap_5()
            .items_center()
            .child(
                div()
                    .text_size(px(26.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.primary)
                    .child("BORROW_"),
            )
            .child(
                div()
                    .text_size(px(16.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.foreground)
                    .child("SIGN IN TO CONTINUE"),
            )
            .child(
                div()
                    .text_size(px(13.))
                    .text_color(theme.muted_foreground)
                    .text_align(TextAlign::Center)
                    .child("Modules 1–3 are free. Premium unlocks modules 4–11 for $5/month."),
            )
            .child(buttons)
            .child(status);

        div()
            .size_full()
            .bg(theme.background)
            .text_color(theme.foreground)
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .child(card)
    }

    /// The paywall. Reached from a locked module, or offered as
    /// confirmation once premium lands — same screen, two states.
    fn upgrade_screen(&self, theme: &Theme, view: Entity<AppView>, _cx: &App) -> Div {
        let premium = self.premium();
        let has_billing = self
            .session
            .as_ref()
            .map(|session| session.has_billing)
            .unwrap_or(false);
        let entitlement = self
            .session
            .as_ref()
            .and_then(|session| session.entitlement.as_ref().cloned());
        let checkout_pending = self.auth.checkout_pending;

        let mut body = div().flex().flex_col().gap_5();

        if premium {
            body = body
                .child(
                    div()
                        .text_size(px(22.))
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.primary)
                        .child("YOU'RE PREMIUM"),
                )
                .child(
                    div()
                        .text_size(px(14.))
                        .text_color(theme.muted_foreground)
                        .child("Modules 4–11 are unlocked. Your progress syncs to your account."),
                );

            let jump = view.clone();
            body = body.child(
                Button::new("upgrade-start")
                    .primary()
                    .label("START LEARNING")
                    .on_click(move |_, _, cx| {
                        jump.update(cx, |view, cx| {
                            view.navigate(Screen::Home);
                            cx.notify();
                        });
                    }),
            );
        } else {
            body = body
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_baseline()
                        .gap_3()
                        .child(
                            div()
                                .text_size(px(34.))
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.foreground)
                                .child("$5"),
                        )
                        .child(
                            div()
                                .text_size(px(13.))
                                .text_color(theme.muted_foreground)
                                .font_weight(FontWeight::BOLD)
                                .child("/ MONTH"),
                        ),
                )
                .child(
                    div().flex().flex_col().gap_2().child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(premium_line(
                                theme,
                                "Modules 4–11: lessons, exercises, projects, practice",
                            ))
                            .child(premium_line(theme, "Progress synced to your account"))
                            .child(premium_line(
                                theme,
                                "Cancel anytime from the billing portal",
                            )),
                    ),
                );

            // Where the last known window stands — grace counting down, or
            // already closed.
            if let Some(entitlement) = &entitlement {
                let message = match entitlement.reason.as_str() {
                    "grace" | "paid_through" => entitlement
                        .access_ends_at
                        .map(|ends| {
                            format!("Premium access ends {}.", in_words(ends, auth::now_ms()))
                        })
                        .unwrap_or_default(),
                    "expired" => {
                        "Your subscription ended. Renewing brings every module back.".to_string()
                    }
                    _ => String::new(),
                };
                if !message.is_empty() {
                    body = body.child(
                        div()
                            .text_size(px(13.))
                            .text_color(theme.warning)
                            .child(message),
                    );
                }
            }

            if let Some(error) = &self.auth.billing_error {
                body = body.child(
                    div()
                        .text_size(px(13.))
                        .text_color(theme.warning)
                        .child(error.clone()),
                );
            }

            let subscribe = view.clone();
            let mut subscribe_button =
                Button::new("upgrade-subscribe")
                    .primary()
                    .label(if checkout_pending {
                        "WAITING FOR PAYMENT…"
                    } else {
                        "SUBSCRIBE — $5/MONTH"
                    });
            if checkout_pending {
                subscribe_button = subscribe_button.loading(true);
            }
            body = body.child(subscribe_button.on_click(move |_, _, cx| {
                subscribe.update(cx, start_checkout);
            }));

            if checkout_pending {
                body = body.child(
                    div()
                        .text_size(px(12.))
                        .text_color(theme.muted_foreground)
                        .child(
                            "Complete the checkout in your browser — this screen updates itself.",
                        ),
                );
            }

            // Manual check: the escape hatch for a browser tab the poll
            // cannot see, and the only refresh control on the screen.
            let refresh = view.clone();
            body = body.child(
                Button::new("upgrade-refresh")
                    .secondary()
                    .label("I'VE PAID — REFRESH")
                    .on_click(move |_, _, cx| spawn_refresh(refresh.clone(), cx)),
            );
        }

        if has_billing {
            let portal = view.clone();
            body = body.child(
                Button::new("upgrade-portal")
                    .ghost()
                    .text_color(theme.muted_foreground)
                    .label("MANAGE BILLING")
                    .on_click(move |_, _, cx| {
                        portal.update(cx, start_portal);
                    }),
            );
        }

        self.shell(theme, view, "Premium", "", None, body)
    }

    /// Signed-in identity: who this is, what they have, and the door out.
    fn account_screen(&self, theme: &Theme, view: Entity<AppView>, cx: &App) -> Div {
        let Some(session) = self.session.as_ref() else {
            return self.login_screen(theme, view, cx);
        };

        let plan = match &session.entitlement {
            Some(entitlement) if entitlement.premium => match entitlement.reason.as_str() {
                "grace" | "paid_through" => entitlement
                    .access_ends_at
                    .map(|ends| format!("Premium — ends {}", in_words(ends, auth::now_ms())))
                    .unwrap_or_else(|| "Premium".to_string()),
                _ => "Premium".to_string(),
            },
            Some(entitlement) if entitlement.reason == "expired" => {
                "Free — subscription ended".to_string()
            }
            _ => "Free".to_string(),
        };

        // The face: the user's own picture when they have chosen one, the
        // provider's otherwise, and initials underneath both — never a hole.
        let mut avatar = Avatar::new().with_size(Size::Size(px(56.)));
        avatar = match &session.avatar_url {
            Some(url) => avatar.src(url.clone()),
            None => avatar.name(
                session
                    .name
                    .clone()
                    .unwrap_or_else(|| session.email.clone()),
            ),
        };

        let identity = div()
            .rounded_md()
            .border_1()
            .border_color(theme.border)
            .bg(theme.secondary)
            .p_5()
            .flex()
            .flex_row()
            .gap_4()
            .items_center()
            .child(avatar)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_size(px(13.))
                            .text_color(theme.muted_foreground)
                            .child("SIGNED IN AS"),
                    )
                    .child(
                        div()
                            .text_size(px(17.))
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.foreground)
                            .child(session.email.clone()),
                    )
                    .child(
                        div()
                            .text_size(px(13.))
                            .text_color(theme.primary)
                            .font_weight(FontWeight::BOLD)
                            .child(plan),
                    ),
            );

        let pick = view.clone();
        let change_picture = if self.auth.avatar_pending {
            Button::new("change-picture")
                .secondary()
                .label("UPLOADING\u{2026}")
                .loading(true)
        } else {
            Button::new("change-picture")
                .secondary()
                .label("CHANGE PICTURE")
                .on_click(move |_, _, cx| {
                    // The native picker: a modal by design, run by the OS.
                    // Cancel is a no-op, not an error.
                    let Some(path) = rfd::FileDialog::new()
                        .add_filter("Images", &["png", "jpg", "jpeg", "webp", "gif"])
                        .pick_file()
                    else {
                        return;
                    };
                    start_avatar_upload(pick.clone(), path, cx);
                })
        };

        let mut body = div()
            .flex()
            .flex_col()
            .gap_5()
            .child(identity)
            .child(change_picture);

        if let Some(error) = &self.auth.avatar_error {
            body = body.child(
                div()
                    .text_size(px(13.))
                    .text_color(theme.warning)
                    .child(error.clone()),
            );
        }

        if session.has_billing {
            let portal = view.clone();
            body = body.child(
                Button::new("account-portal")
                    .secondary()
                    .label("MANAGE BILLING")
                    .on_click(move |_, _, cx| {
                        portal.update(cx, start_portal);
                    }),
            );
        }

        if let Some(error) = &self.auth.billing_error {
            body = body.child(
                div()
                    .text_size(px(13.))
                    .text_color(theme.warning)
                    .child(error.clone()),
            );
        }

        let out = view.clone();
        body = body.child(
            Button::new("account-signout")
                .ghost()
                .text_color(theme.warning)
                .label("SIGN OUT")
                .on_click(move |_, _, cx| {
                    out.update(cx, sign_out);
                }),
        );

        self.shell(theme, view, "Account", "", None, body)
    }

    /// The how-to tour — the answer to "I opened the app, now what?"
    ///
    /// Every line here is a promise the app keeps; when the behaviour
    /// changes, this screen changes with it.
    fn help_screen(&self, theme: &Theme, view: Entity<AppView>, _cx: &App) -> Div {
        /// One term and what it does: bold term, muted explanation.
        fn row(theme: &Theme, term: &str, explanation: &str) -> Div {
            div()
                .flex()
                .flex_row()
                .gap_3()
                .items_baseline()
                .child(
                    div()
                        .w(px(150.))
                        .min_w(px(150.))
                        .text_size(px(13.))
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.primary)
                        .child(term.to_string()),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w(px(0.))
                        .text_size(px(13.))
                        .text_color(theme.foreground)
                        .child(explanation.to_string()),
                )
        }

        fn section(theme: &Theme, title: &str, rows: &[(&str, &str)]) -> Div {
            let mut list = div().flex().flex_col().gap_2();
            for (term, explanation) in rows {
                list = list.child(row(theme, term, explanation));
            }
            div()
                .flex()
                .flex_col()
                .gap_3()
                .child(label(theme, title))
                .child(list)
        }

        let body = div()
            .flex()
            .flex_col()
            .gap_6()
            .child(
                div()
                    .text_size(px(13.))
                    .text_color(theme.muted_foreground)
                    .child("BORROW_ teaches Rust by having you write it. This is every control, in the order you will meet it."),
            )
            .child(section(theme, "GETTING AROUND", &[
                ("BORROW_ (top left)", "Back to the home screen from anywhere."),
                ("\u{2190} BACK", "Returns to the screen you were on — a lesson you sent to the Playground, an exercise you came from. It only appears once you have been somewhere."),
                ("DASHBOARD / PROGRESS", "The curriculum with per-module bars, and your totals across the course."),
                ("PLAYGROUND / HELP / ACCOUNT", "The editor, this page, and your plan, profile and sign-out."),
            ]))
            .child(section(theme, "LESSONS & EXERCISES", &[
                ("MARK COMPLETE", "Records the lesson as read — lessons award no XP; exercises are the work that does."),
                ("NEXT \u{2192}", "Walks forward through the course in order."),
                ("CHECK ANSWER", "Grades an exercise the moment you submit it."),
                ("TRY AGAIN", "After a wrong answer, re-attempt in place — your pick stays selected, and XP can only be earned once per exercise."),
                ("Open-ended exercises", "Have no single right answer: submitting records them for self-review."),
            ]))
            .child(section(theme, "PRACTICE", &[
                ("PRACTICE THIS MODULE", "Builds a question set from one module's exercises."),
                ("Practice never touches XP", "It is revision: answers are recorded for review, not scored."),
            ]))
            .child(section(theme, "THE PLAYGROUND", &[
                ("RUN / STOP", "Compiles and runs the buffer. STOP (or Esc) ends a run early; there is a 5-second timeout and output is capped."),
                ("\u{2318}/CTRL + ENTER", "Runs from the keyboard while the editor has focus."),
                ("\u{25B6} RUN IN PLAYGROUND", "Under every snippet in lessons and exercises: loads the code into the editor and runs it immediately. \u{2190} BACK returns you to where you were reading."),
            ]))
            .child(section(theme, "MODULES & PROGRESSION", &[
                ("Sequential unlocks", "Module N+1 opens when module N is finished — every lesson and every exercise."),
                ("CONTINUE \u{2014} N EXERCISES LEFT", "Shown when the lessons are done but exercises remain: it jumps to the first one you have not answered."),
                ("NEXT MODULE", "Appears when the module is genuinely complete."),
            ]))
            .child(section(theme, "PREMIUM", &[
                ("Modules 1\u{2013}3", "Free with an account."),
                ("Modules 4\u{2013}11", "Included with Premium — $5/month, cancelled any time from the billing portal."),
                ("After cancelling", "Access runs to the end of the paid period, then a 7-day grace window before the modules lock."),
            ]))
            .child(section(theme, "ACCOUNT & SYNC", &[
                ("Gmail or GitHub", "Either signs you in; both point at the same account when the emails match."),
                ("Profile picture", "Your provider's picture is the default — change it from the Account screen."),
                ("Progress sync", "Completions follow your account: sign in anywhere and your work is there. Lessons work offline; the account catches up when you are back online."),
            ]));

        self.shell(theme, view, "How to use BORROW_", "", None, body)
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
        // The advertised ⌘/CTRL+ENTER lives in the editor's key context, so it
        // only fires while the editor holds focus — and nothing else in the
        // app focuses it. A new entity means a fresh visit (the keyed state is
        // dropped while the screen is not drawn), so focus is taken once on
        // entry and left alone for the rest of the visit.
        if self.playground.editor.as_ref() != Some(&editor) {
            self.playground.editor = Some(editor.clone());
            editor.update(cx, |state, cx| state.focus(window, cx));
        }

        // The shortcut listens to what the editor *emits*, not to the key
        // itself. `secondary-enter` — ⌘ on macOS, CTRL elsewhere — is the
        // editor's own binding: where that binding matches, the keystroke is
        // consumed as an action and never reaches a raw key listener, which
        // is exactly why a key handler passed on Linux and failed on macOS.
        // PressEnter with `secondary` set is the same combination everywhere.
        if self.playground.shortcut.is_none() {
            let subscription = cx.subscribe(&editor, move |view, entity, event, cx| {
                if matches!(
                    event,
                    crate::component::input::InputEvent::PressEnter {
                        secondary: true,
                        ..
                    }
                ) {
                    let code = entity.read(cx).value().to_string();
                    begin_run(view, &code, cx);
                }
            });
            self.playground.shortcut = Some(subscription);
        }

        // A lesson or exercise's "RUN IN PLAYGROUND" left its snippet
        // here. Applied after the keyed state above, because that is the
        // first moment the editor exists to receive it — and "run" means
        // run: the snippet is loaded *and* started, so a learner arrives at
        // the output. Snippets that do not compile show their diagnostics;
        // the timeout and STOP still bound the ones that do.
        if let Some(code) = self.playground.pending_code.take() {
            editor.update(cx, |state, cx| state.set_value(&code, window, cx));
            begin_run(self, &code, cx);
        }

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

        // An empty title means the body carries its own heading (the landing
        // hero), so no shell heading is added — an empty heading div would
        // still consume a flex gap.
        let mut column = div().mx_auto().max_w(px(820.)).flex().flex_col().gap_5();
        if !title.is_empty() {
            column = column.child(heading(theme, title, subtitle));
        }

        div()
            .size_full()
            .bg(theme.background)
            .text_color(theme.foreground)
            .flex()
            .flex_col()
            .child(navbar(
                theme,
                to_home,
                module_name,
                !self.history.is_empty(),
            ))
            .child(
                div()
                    // Keyed by screen: gpui keeps a scrollable div's offset in
                    // element state under its id, and a constant id carried
                    // the old lesson's position onto the next one — the reader
                    // landed partway down instead of at the top. A new id per
                    // screen (lesson index included) starts at zero on every
                    // navigation, whichever button did the navigating.
                    .id(format!("screen-scroll-{:?}", self.screen))
                    .flex_1()
                    .min_h(px(0.))
                    .overflow_y_scroll()
                    .px_6()
                    .py_8()
                    .child(column.child(body)),
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
                // Same fixed-height clip as the option rows: an open hint is
                // multi-line text inside Medium's h_8. Size::Size lets the
                // disclosure grow with the hint behind it.
                .with_size(Size::Size(px(200.)))
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
                        .whitespace_normal()
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
            .child(exercise_view::question(theme, record, Some(view.clone())))
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
                                // New row (if any) goes to the account;
                                // a failed push is caught by the next sync.
                                spawn_push_now(reported.read(cx), cx);
                            }
                        }),
                );
            }
        }

        if let Some(result) = &self.result {
            body = body.child(self.feedback(theme, record, &options, result));

            // Wrong on a graded exercise: offer the attempt again in place.
            // Leaving and re-entering used to be the only reset — "start
            // the whole process over" for one wrong tap. Clearing `result`
            // is the whole button: CHECK ANSWER returns with the previous
            // pick still selected, and XP cannot double, because the
            // completion row is written once.
            if !record.is_open_ended() && !result.correct {
                let retry = view.clone();
                body = body.child(
                    Button::new("retry")
                        .secondary()
                        .label("TRY AGAIN")
                        .on_click(move |_, _, cx| {
                            retry.update(cx, |view, cx| {
                                view.result = None;
                                cx.notify();
                            });
                        }),
                );
            }
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
                    // Button's default Medium height is a fixed h_8, and with
                    // our p_3 that leaves an 8px band for the text — the
                    // bottom half of every option was clipped. Size::Size
                    // drops the fixed height and keeps the keyboard reach,
                    // which is the same fix the module cards needed.
                    .with_size(Size::Size(px(200.)))
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
                            // Button's label wrapper is whitespace_nowrap and
                            // overflow_hidden; without this a long option is
                            // cut at the right edge with no wrap and no
                            // ellipsis.
                            .whitespace_normal()
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
            .child(exercise_view::question(theme, exercise, Some(view.clone())))
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

// ── Auth & billing flows ────────────────────────────────────────────────────
//
// Every network call below is spawned onto the database's tokio runtime
// (`app_db::Db::runtime`) and awaited from a gpui task: sqlx and reqwest
// need a reactor, gpui's executors are not one, and a `JoinHandle` is a
// completion to wait on rather than a future bound to the wrong runtime.

/// The fetch-and-apply half of a refresh, as a future both the launch path
/// (inside an entity update, which already holds a `Context`) and the
/// button paths (which build a task first) can share.
async fn refresh_and_apply(
    cx: &mut AsyncApp,
    runner: Entity<AppView>,
    site: String,
    token: String,
    runtime: std::sync::Arc<tokio::runtime::Runtime>,
) {
    let result = runtime
        .spawn(async move { api::me(&site, &token).await })
        .await;
    cx.update(|cx| {
        runner.update(cx, |view, cx| {
            apply_refresh(view, result);
            cx.notify();
        });
    });
}

/// Builds the one-shot refresh task.
///
/// Deliberately takes the *token*, not the entity: callers arm this from
/// inside an entity update, where touching the entity again (even a read)
/// is a lease violation and panics. The strong `runner` clone the task
/// holds is just a handle — it leases nothing until the task runs, long
/// after the update that spawned it has ended.
pub(crate) fn refresh_task(runner: Entity<AppView>, cx: &App, token: String) -> Option<Task<()>> {
    let site = crate::config::convex_site_url()?;
    let runtime = cx.try_global::<Db>()?.runtime();
    Some(cx.spawn(async move |cx: &mut AsyncApp| {
        refresh_and_apply(cx, runner, site, token, runtime).await;
    }))
}

/// Pulls a fresh entitlement into the cached session. For button handlers:
/// read, build, store — three steps that never overlap a lease.
///
/// The only failures that change anything: a `401` means the session row is
/// gone server-side, so this device signs out; everything else (offline,
/// server hiccup) keeps the cache — no offline lockout.
pub(crate) fn spawn_refresh(runner: Entity<AppView>, cx: &mut App) {
    let Some(token) = runner
        .read(cx)
        .session
        .as_ref()
        .map(|session| session.token.clone())
    else {
        return;
    };
    let Some(task) = refresh_task(runner.clone(), cx, token) else {
        return;
    };
    runner.update(cx, |view, _| view.auth.refresh_task = Some(task));
}

fn apply_refresh(
    view: &mut AppView,
    result: Result<Result<Me, FetchError>, tokio::task::JoinError>,
) {
    match result {
        Ok(Ok(me)) => {
            if let Some(session) = view.session.as_mut() {
                session.apply_me(&me);
                let _ = auth::save(session);
            }
        }
        Ok(Err(FetchError::Unauthorized)) => {
            auth::clear();
            view.adopt_session(None);
        }
        Ok(Err(err)) => eprintln!("entitlement refresh failed: {err}"),
        Err(err) => eprintln!("entitlement refresh crashed: {err}"),
    }
}

/// The session's daily re-check: grace windows end while the app is open,
/// and the cache must notice. Stops itself if the user signs out or the
/// build is unconfigured — there is nothing left to refresh for.
///
/// Like [`refresh_task`], it takes only a handle, never a lease: arming it
/// inside an entity update must not touch the entity.
pub(crate) fn daily_task(runner: Entity<AppView>, cx: &App) -> Option<Task<()>> {
    Some(cx.spawn(async move |cx: &mut AsyncApp| loop {
        cx.background_executor()
            .timer(Duration::from_secs(24 * 60 * 60))
            .await;

        let cycle = {
            let site = crate::config::convex_site_url();
            let token = cx.update(|cx| {
                runner
                    .read(cx)
                    .session
                    .as_ref()
                    .map(|session| session.token.clone())
            });
            match (site, token) {
                (Some(site), Some(token)) => (site, token),
                _ => return,
            }
        };
        let (site, token) = cycle;
        let Some(db) = cx.update(|cx| cx.try_global::<Db>().map(|db| db.runtime())) else {
            return;
        };

        let result = db.spawn(async move { api::me(&site, &token).await }).await;
        cx.update(|cx| {
            runner.update(cx, |view, cx| {
                apply_refresh(view, result);
                cx.notify();
            });
        });
    }))
}

/// Sign-in button handler: binds the loopback, opens the browser, and
/// finishes on the backend. Config problems are a message on the login
/// screen, not a crash.
fn start_login(view: &mut AppView, cx: &mut Context<AppView>, provider: &'static str) {
    if view.auth.login_pending {
        return;
    }
    view.auth.login_error = None;

    let Some(site) = crate::config::convex_site_url() else {
        view.auth.login_error =
            Some("Sign-in is not configured for this build. See SETUP.md.".into());
        cx.notify();
        return;
    };
    let client_id = match provider {
        "google" => crate::config::google_client_id(),
        _ => crate::config::github_client_id(),
    };
    let Some(client_id) = client_id else {
        view.auth.login_error = Some(format!(
            "Sign-in with {provider} is not configured for this build."
        ));
        cx.notify();
        return;
    };
    let Some(db) = cx.try_global::<Db>() else {
        view.auth.login_error = Some("The local database is not open.".into());
        cx.notify();
        return;
    };
    let runtime = db.runtime();

    view.auth.login_pending = true;
    cx.notify();

    let entity = cx.entity();
    let task = cx.spawn(async move |_this, cx| {
        let started = runtime
            .spawn(async move { auth::begin(provider, &client_id).await })
            .await;
        let flow = match started {
            Ok(Ok(flow)) => flow,
            Ok(Err(err)) => {
                fail_login(&entity, cx, format!("Could not start sign-in: {err:#}"));
                return;
            }
            Err(err) => {
                fail_login(&entity, cx, format!("Could not start sign-in: {err}"));
                return;
            }
        };

        cx.update(|cx| cx.open_url(&flow.authorize_url));

        let exchanged = runtime
            .spawn(async move {
                let callback = flow.wait_for_code(auth::LOGIN_TIMEOUT).await?;
                api::exchange(
                    &site,
                    provider,
                    &callback.code,
                    &callback.code_verifier,
                    &callback.redirect_uri,
                )
                .await
            })
            .await;

        cx.update(|cx| {
            entity.update(cx, |view, cx| {
                view.auth.login_pending = false;
                match exchanged {
                    Ok(Ok(response)) => match auth::StoredSession::from_exchange(response) {
                        Ok(session) => {
                            if let Err(err) = auth::save(&session) {
                                eprintln!("could not persist the session: {err:#}");
                            }
                            view.adopt_session(Some(session));
                            // Entitlement lands a beat later; the login
                            // already opened the door to the free modules.
                            // Armed on `view` directly — we are inside its
                            // update, and re-entering the entity here is
                            // exactly the lease violation that crashed the
                            // first post-login build.
                            if let Some(token) =
                                view.session.as_ref().map(|session| session.token.clone())
                            {
                                view.auth.refresh_task = refresh_task(entity.clone(), cx, token);
                                view.auth.daily_task = daily_task(entity.clone(), cx);
                            }
                            crate::sync::spawn_initial(view, cx);
                        }
                        Err(err) => {
                            view.auth.login_error = Some(format!("{err:#}"));
                        }
                    },
                    Ok(Err(err)) => {
                        view.auth.login_error = Some(format!("{err}"));
                    }
                    Err(err) => {
                        view.auth.login_error = Some(format!("Sign-in failed: {err}"));
                    }
                }
                cx.notify();
            });
        });
    });
    view.auth.login_task = Some(task);
}

fn fail_login(entity: &Entity<AppView>, cx: &mut AsyncApp, message: String) {
    cx.update(|cx| {
        entity.update(cx, |view, cx| {
            view.auth.login_pending = false;
            view.auth.login_error = Some(message);
            cx.notify();
        });
    });
}

/// SUBSCRIBE: mints hosted checkout, opens it in the browser, then polls
/// entitlement until the webhook flips it (or the window gives up — the
/// "REFRESH" button is the manual escape hatch for a slow tab).
fn start_checkout(view: &mut AppView, cx: &mut Context<AppView>) {
    if view.auth.checkout_pending {
        return;
    }
    view.auth.billing_error = None;

    let Some(site) = crate::config::convex_site_url() else {
        view.auth.billing_error = Some("Billing is not configured for this build.".into());
        cx.notify();
        return;
    };
    let Some(token) = view.session.as_ref().map(|session| session.token.clone()) else {
        return;
    };
    let Some(db) = cx.try_global::<Db>() else {
        return;
    };
    let runtime = db.runtime();

    view.auth.checkout_pending = true;
    cx.notify();

    let entity = cx.entity();
    let task = cx.spawn(async move |_this, cx| {
        let started = runtime
            .spawn({
                let site = site.clone();
                let token = token.clone();
                async move { api::checkout_url(&site, &token).await }
            })
            .await;

        let url = match started {
            Ok(Ok(url)) => url,
            Ok(Err(err)) => {
                billing_fail(&entity, cx, format!("{err}"));
                return;
            }
            Err(err) => {
                billing_fail(&entity, cx, format!("Checkout failed: {err}"));
                return;
            }
        };

        cx.update(|cx| cx.open_url(&url));

        // The webhook is the source of truth; this loop only notices.
        let deadline = Instant::now() + Duration::from_secs(15 * 60);
        let mut unlocked = false;
        let mut dead = false;
        while !unlocked && !dead && Instant::now() < deadline {
            cx.background_executor().timer(Duration::from_secs(2)).await;

            let result = runtime
                .spawn({
                    let site = site.clone();
                    let token = token.clone();
                    async move { api::me(&site, &token).await }
                })
                .await;

            cx.update(|cx| {
                entity.update(cx, |view, cx| match &result {
                    Ok(Ok(me)) if me.entitlement.premium => {
                        if let Some(session) = view.session.as_mut() {
                            session.apply_me(me);
                            let _ = auth::save(session);
                        }
                        cx.notify();
                    }
                    _ => {}
                });
            });

            match &result {
                Ok(Ok(me)) if me.entitlement.premium => unlocked = true,
                Ok(Err(FetchError::Unauthorized)) => dead = true,
                _ => {}
            }
        }

        cx.update(|cx| {
            entity.update(cx, |view, cx| {
                view.auth.checkout_pending = false;
                if dead {
                    auth::clear();
                    view.adopt_session(None);
                } else if !unlocked {
                    view.auth.billing_error =
                        Some("Payment not detected yet — if you have paid, press REFRESH.".into());
                }
                cx.notify();
            });
        });
    });
    view.auth.checkout_task = Some(task);
}

fn billing_fail(entity: &Entity<AppView>, cx: &mut AsyncApp, message: String) {
    cx.update(|cx| {
        entity.update(cx, |view, cx| {
            view.auth.checkout_pending = false;
            view.auth.billing_error = Some(message);
            cx.notify();
        });
    });
}

/// MANAGE BILLING: a portal session, opened in the browser. Cancelling and
/// card updates happen there — none of it is built in-app.
fn start_portal(view: &mut AppView, cx: &mut Context<AppView>) {
    view.auth.billing_error = None;

    let Some(site) = crate::config::convex_site_url() else {
        view.auth.billing_error = Some("Billing is not configured for this build.".into());
        cx.notify();
        return;
    };
    let Some(token) = view.session.as_ref().map(|session| session.token.clone()) else {
        return;
    };
    let Some(db) = cx.try_global::<Db>() else {
        return;
    };
    let runtime = db.runtime();

    let entity = cx.entity();
    let task = cx.spawn(async move |_this, cx| {
        let opened = runtime
            .spawn(async move { api::portal_url(&site, &token).await })
            .await;
        cx.update(|cx| {
            entity.update(cx, |view, cx| {
                match opened {
                    Ok(Ok(url)) => cx.open_url(&url),
                    Ok(Err(err)) => view.auth.billing_error = Some(format!("{err}")),
                    Err(err) => view.auth.billing_error = Some(format!("Portal failed: {err}")),
                }
                cx.notify();
            });
        });
    });
    view.auth.checkout_task = Some(task);
}

/// CHANGE PICTURE: pick → upload → the account now wears it.
///
/// The dialog blocks (it is the OS's), the upload does not: bytes go to
/// the backend on the tokio runtime, and the view is leased for the
/// write-back only after the round trip, from a context holding nothing
/// else.
fn start_avatar_upload(runner: Entity<AppView>, path: std::path::PathBuf, cx: &mut App) {
    let Some(site) = crate::config::convex_site_url() else {
        runner.update(cx, |view, cx| {
            view.auth.avatar_error = Some("Uploads are not configured for this build.".into());
            cx.notify();
        });
        return;
    };
    let Some(token) = runner
        .read(cx)
        .session
        .as_ref()
        .map(|session| session.token.clone())
    else {
        return;
    };
    let Some(db) = cx.try_global::<Db>() else {
        return;
    };
    let runtime = db.runtime();

    runner.update(cx, |view, cx| {
        view.auth.avatar_pending = true;
        view.auth.avatar_error = None;
        cx.notify();
    });

    let writer = runner.clone();
    let task = cx.spawn(async move |cx: &mut AsyncApp| {
        let uploaded = runtime
            .spawn(async move {
                let bytes = tokio::fs::read(&path)
                    .await
                    .map_err(|err| anyhow::anyhow!("could not read the file: {err}"))?;
                let mime = image_mime(&path);
                api::upload_avatar(&site, &token, bytes, mime).await
            })
            .await;

        cx.update(|cx| {
            writer.update(cx, |view, cx| {
                view.auth.avatar_pending = false;
                match uploaded {
                    Ok(Ok(url)) => {
                        if let Some(session) = view.session.as_mut() {
                            session.avatar_url = Some(url);
                            let _ = auth::save(session);
                        }
                    }
                    Ok(Err(err)) => view.auth.avatar_error = Some(format!("{err}")),
                    Err(err) => view.auth.avatar_error = Some(format!("Upload failed: {err}")),
                }
                cx.notify();
            });
        });
    });
    runner.update(cx, |view, _| view.auth.avatar_task = Some(task));
}

/// The content type the backend stores the bytes under, from the extension.
fn image_mime(path: &std::path::Path) -> &'static str {
    match path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("webp") => "image/webp",
        Some("gif") => "image/gif",
        _ => "image/png",
    }
}

/// SIGN OUT: revoke server-side (best-effort), drop the file, land on the
/// gate. The local clear alone would already sign this device out.
fn sign_out(view: &mut AppView, cx: &mut Context<AppView>) {
    if let Some(session) = view.session.take() {
        if let (Some(site), Some(db)) = (crate::config::convex_site_url(), cx.try_global::<Db>()) {
            let runtime = db.runtime();
            runtime.spawn(async move {
                let _ = api::logout(&site, &session.token).await;
            });
        }
    }
    auth::clear();
    view.adopt_session(None);
    cx.notify();
}

/// Fire-and-forget push after a completion: the local row already exists,
/// so a failed push just means the next launch's sync catches up.
pub(crate) fn spawn_push_now(view: &AppView, cx: &App) {
    let Some(site) = crate::config::convex_site_url() else {
        return;
    };
    let Some(token) = view.session.as_ref().map(|session| session.token.clone()) else {
        return;
    };
    let Some(db) = cx.try_global::<Db>() else {
        return;
    };
    let runtime = db.runtime();
    let pool = db.pool().clone();
    runtime.spawn(async move {
        if let Err(err) = crate::sync::push(&pool, &site, &token).await {
            eprintln!("sync: push failed: {err:#}");
        }
    });
}

/// `1791600000000` → "in 6 days" / "3 days ago" — enough for a paywall
/// status line, without a calendar library for one string.
fn in_words(epoch_ms: u64, now_ms: u64) -> String {
    let delta = epoch_ms as i64 - now_ms as i64;
    let past = delta < 0;
    let seconds = delta.unsigned_abs() / 1000;
    let (value, unit) = if seconds >= 2 * 24 * 3600 {
        (seconds / (24 * 3600), "days")
    } else if seconds >= 24 * 3600 {
        (1, "day")
    } else if seconds >= 2 * 3600 {
        (seconds / 3600, "hours")
    } else if seconds >= 3600 {
        (1, "hour")
    } else if seconds >= 60 {
        (seconds / 60, "minutes")
    } else {
        (1, "minute")
    };
    let plural = if value == 1 { "" } else { "s" };
    if past {
        format!("{value} {unit}{plural} ago")
    } else {
        format!("in {value} {unit}{plural}")
    }
}

/// The next unfinished exercise in a module, and how many are left — the
/// door that is actually open when the lessons are done but the exercises
/// are not, so "the module is finished" never dead-ends in a hidden button.
fn next_open_exercise(seed: &ModuleSeed, done: &[i64]) -> Option<(usize, usize)> {
    let position = seed
        .exercises
        .iter()
        .position(|exercise| !done.contains(&exercise.id))?;
    let remaining = seed
        .exercises
        .iter()
        .filter(|exercise| !done.contains(&exercise.id))
        .count();
    Some((position, remaining))
}

/// The top strip: wordmark back to home, and where you are.
///
/// `can_go_back` shows the `← BACK` door — the answer to "I sent this
/// snippet to the Playground, how do I get back to the lesson". It appears
/// only when there *is* a previous screen, because a dead back button is
/// worse than none.
fn navbar(theme: &Theme, to_home: Entity<AppView>, module_name: String, can_go_back: bool) -> Div {
    let to_dashboard = to_home.clone();
    let to_progress = to_home.clone();
    let to_playground = to_home.clone();
    let to_help = to_home.clone();
    let to_account = to_home.clone();
    let to_back = to_home.clone();

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
        .child(if can_go_back {
            let back = to_back;
            div().child(
                Button::new("nav-back")
                    .ghost()
                    .text_size(px(13.))
                    .text_color(theme.muted_foreground)
                    .label("\u{2190} BACK")
                    .on_click(move |_, _, cx| {
                        back.update(cx, |view, cx| {
                            view.go_back();
                            cx.notify();
                        });
                    }),
            )
        } else {
            div()
        })
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
        )
        .child(
            Button::new("nav-help")
                .ghost()
                .text_size(px(13.))
                .text_color(theme.muted_foreground)
                .child("HELP")
                .on_click(move |_, _, cx| {
                    to_help.update(cx, |view, cx| {
                        view.navigate(Screen::Help);
                        cx.notify();
                    });
                }),
        )
        .child(
            Button::new("nav-account")
                .ghost()
                .text_size(px(13.))
                .text_color(theme.muted_foreground)
                .child("ACCOUNT")
                .on_click(move |_, _, cx| {
                    to_account.update(cx, |view, cx| {
                        view.navigate(Screen::Account);
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
        self.record_history();

        match self.screen.clone() {
            Screen::Login => self.login_screen(theme, view, cx),
            Screen::Help => self.help_screen(theme, view, cx),
            Screen::Upgrade => self.upgrade_screen(theme, view, cx),
            Screen::Account => self.account_screen(theme, view, cx),
            // A premium-locked module hands every one of its screens to the
            // paywall — the guard sits above navigation so a direct jump
            // (a restored screen, a test) still hits the gate.
            Screen::Module(index) if self.needs_premium(index) => {
                self.upgrade_screen(theme, view, cx)
            }
            Screen::Lesson { module, .. } if self.needs_premium(module) => {
                self.upgrade_screen(theme, view, cx)
            }
            Screen::Exercise { module, .. } if self.needs_premium(module) => {
                self.upgrade_screen(theme, view, cx)
            }
            Screen::Practice { module } if self.needs_premium(module) => {
                self.upgrade_screen(theme, view, cx)
            }
            Screen::PracticeRun { module } if self.needs_premium(module) => {
                self.upgrade_screen(theme, view, cx)
            }
            Screen::PracticeResults { module } if self.needs_premium(module) => {
                self.upgrade_screen(theme, view, cx)
            }
            Screen::Home => self.home(theme, view, cx),
            Screen::Module(index) => self.module_screen(theme, view, index, cx),
            Screen::Lesson { module, lesson } => {
                self.lesson_screen(view, window, cx, module, lesson)
            }
            Screen::Exercise { module, exercise } => {
                self.exercise_screen(theme, view, module, exercise)
            }
            Screen::Practice { module } => self.practice_screen(theme, view, module),
            Screen::PracticeRun { module } => self.practice_run_screen(theme, view, module),
            Screen::PracticeResults { module } => self.practice_results_screen(theme, view, module),
            Screen::Dashboard => self.dashboard_screen(theme, view, cx),
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

/// Completed lesson and exercise ids — the two lists progressive gating
/// reads. Two empty lists when the `Db` global is absent (headless tests),
/// which locks every module after the first.
fn completed_progress(cx: &App) -> (Vec<i64>, Vec<i64>) {
    cx.try_global::<Db>()
        .map(|db| {
            let lessons = db
                .block_on(progress::completed_lesson_ids(db.pool()))
                .unwrap_or_default();
            let exercises = db
                .block_on(progress::completed_exercise_ids(db.pool()))
                .unwrap_or_default();
            (lessons, exercises)
        })
        .unwrap_or_default()
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
    let code = editor.read(cx).value().to_string();
    view.update(cx, |view, cx| begin_run(view, &code, cx));
}

/// The state half of a run, with the view already open.
///
/// The Enter subscription hands us `&mut AppView`, so nothing here may read
/// or update the entity again — a second lease panics at runtime rather than
/// failing politely.
fn begin_run(view: &mut AppView, code: &str, cx: &mut Context<AppView>) {
    // A second press while one is in flight cancels it — the same toggle the
    // web's RUN button has, and the Esc shortcut routes here too.
    if view.playground.running {
        cancel_run(view, cx);
        return;
    }

    if code.trim().is_empty() {
        return;
    }

    let code = code.to_string();
    let cancel = sandbox::CancelFlag::default();
    let flag = cancel.clone();
    let executor = cx.background_executor().clone();
    let runner = cx.entity();
    let task = cx.spawn(async move |_this, cx| {
        let started = std::time::Instant::now();
        let result = executor
            .spawn(async move { sandbox::run_cancellable(&code, &flag) })
            .await;
        let elapsed = started.elapsed().as_millis();

        cx.update(|cx| {
            runner.update(cx, |view, cx| {
                view.playground.result = Some(result.clone());
                view.playground.elapsed_ms = Some(elapsed);
                view.playground.running = false;
                view.playground.cancel = None;
                cx.notify();
            });
        });

        result
    });

    view.playground.result = None;
    view.playground.elapsed_ms = None;
    view.playground.explanation_open = false;
    view.playground.running = true;
    view.playground.cancel = Some(cancel);
    view.playground.run = Some(task);
    cx.notify();
}

/// Stops the run in flight.
///
/// Raising the flag kills the child in the sandbox within one 8 ms poll tick;
/// dropping the task only abandons the result, so the flag is what actually
/// ends the run. The stop is reported immediately rather than waiting for the
/// kill to land — the learner pressed a button and the UI must answer now.
pub(crate) fn cancel_run(view: &mut AppView, cx: &mut Context<AppView>) {
    if let Some(flag) = view.playground.cancel.take() {
        flag.store(true, Ordering::Relaxed);
    }
    view.playground.run.take();
    view.playground.result = Some(sandbox::RunResult::cancelled());
    view.playground.elapsed_ms = None;
    view.playground.running = false;
    view.playground.explanation_open = false;
    cx.notify();
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
        if let Some(flag) = view.playground.cancel.take() {
            flag.store(true, Ordering::Relaxed);
        }
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

/// A bullet of the upgrade screen's value list.
fn premium_line(theme: &Theme, text: &str) -> Div {
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap_3()
        .child(
            div()
                .text_size(px(14.))
                .text_color(theme.primary)
                .font_weight(FontWeight::BOLD)
                .child("\u{2713}"),
        )
        .child(
            div()
                .text_size(px(14.))
                .text_color(theme.foreground)
                .child(text.to_string()),
        )
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
        answer_question, begin_run, cancel_run, exercise_view, finish_run, load_builder,
        load_dashboard, load_progress, next_open_exercise, run_program, start_run, AppView,
        LessonContent, LessonSection, ModuleLock, Screen,
    };
    use crate::services::remote::Entitlement;
    use gpui_kit::{TestAppContext, TestDispatcher, VisualTestContext};

    /// Progressive gating: module 0 is always open; module N opens only when
    /// module N-1 is complete (all lessons AND all exercises).
    #[test]
    fn modules_unlock_progressively() {
        let view = AppView::new(crate::content::all());
        let none: Vec<i64> = vec![];

        // Fresh account: only module 0 is open.
        assert!(view.module_is_unlocked(0, &none, &none));
        assert!(!view.module_is_unlocked(1, &none, &none));

        // Completing module 0's lessons alone is not enough — exercises count.
        let seed0 = &view.modules[0];
        let lessons: Vec<i64> = seed0.lessons.iter().map(|l| l.id).collect();
        let exercises: Vec<i64> = seed0.exercises.iter().map(|e| e.id).collect();
        assert!(
            !view.module_is_complete(0, &lessons, &none),
            "exercises still outstanding"
        );
        assert!(!view.module_is_unlocked(1, &lessons, &none));

        // Lessons + exercises: module 0 complete, module 1 unlocks.
        assert!(view.module_is_complete(0, &lessons, &exercises));
        assert!(view.module_is_unlocked(1, &lessons, &exercises));
        assert!(
            !view.module_is_unlocked(2, &lessons, &exercises),
            "module 2 waits on module 1"
        );
    }

    /// Premium *adds* to the sequential gate: with modules 1–3 finished,
    /// module 4 is a paywall without a subscription and open with one —
    /// while the free modules never need one either way.
    #[test]
    fn premium_locks_modules_four_onward() {
        let mut view = AppView::new(crate::content::all());

        let mut lessons = Vec::new();
        let mut exercises = Vec::new();
        for seed in view.modules.iter().take(3) {
            lessons.extend(seed.lessons.iter().map(|lesson| lesson.id));
            exercises.extend(seed.exercises.iter().map(|exercise| exercise.id));
        }

        // Prerequisites met, no subscription: the paywall, not the
        // progression lock — that is the distinction the dashboard shows.
        assert_eq!(
            view.module_lock(3, &lessons, &exercises),
            ModuleLock::Premium,
            "module 4 is behind the paywall"
        );
        assert_eq!(view.module_lock(2, &lessons, &exercises), ModuleLock::Open);
        // And the same progress, without prerequisites at all, reads as
        // progression on a premium module — the course still sequences.
        assert_eq!(
            view.module_lock(3, &[], &[]),
            ModuleLock::Progression,
            "prerequisites come first"
        );

        // A cached premium entitlement is all the paywall asked for.
        view.session = Some(crate::auth::StoredSession {
            token: "t".into(),
            expires_at: 0,
            email: "ada@example.com".into(),
            name: None,
            avatar_url: None,
            entitlement: Some(Entitlement {
                premium: true,
                reason: "active".into(),
                access_ends_at: None,
            }),
            has_billing: false,
            subscription: None,
            fetched_at: 0,
        });
        assert_eq!(view.module_lock(3, &lessons, &exercises), ModuleLock::Open);

        // Free modules are unaffected by the missing subscription.
        view.session = None;
        assert_eq!(view.module_lock(0, &[], &[]), ModuleLock::Open);
    }

    /// The trail the `← BACK` button walks: recorded on render (so the
    /// direct `view.screen = …` writes count too), one pop per press, and
    /// nothing to pop at the start — a dead back button is worse than none.
    #[test]
    fn back_returns_to_the_previous_screen() {
        let mut cx = TestAppContext::build(TestDispatcher::new(0), Some("back_trail"));
        let (view, cx) = open(&mut cx);

        assert!(
            cx.update(|_, cx| view.read(cx).history.is_empty()),
            "nothing has been visited yet"
        );

        cx.update(|_, cx| {
            view.update(cx, |view, cx| {
                view.navigate(Screen::Dashboard);
                cx.notify();
            });
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));

        cx.update(|_, cx| {
            view.update(cx, |view, cx| {
                view.navigate(Screen::Playground);
                cx.notify();
            });
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));

        cx.update(|_, cx| {
            view.update(cx, |view, cx| {
                view.go_back();
                cx.notify();
            });
        });
        assert_eq!(screen_of(&view, cx), "Dashboard", "one pop back");

        cx.update(|_, cx| {
            view.update(cx, |view, cx| {
                view.go_back();
                cx.notify();
            });
        });
        assert_eq!(
            screen_of(&view, cx),
            "Home",
            "two pops: Home is where the walk started, and it is on the trail"
        );

        cx.update(|_, cx| {
            view.update(cx, |view, cx| {
                view.go_back();
                cx.notify();
            });
        });
        assert_eq!(
            screen_of(&view, cx),
            "Home",
            "the trail is empty now: back is a no-op, not a jump somewhere invented"
        );
    }

    /// The continue-door counts what is actually left, and disappears when
    /// nothing is — the difference between "module finished" and "lessons
    /// finished" that the module screen used to collapse into a hidden
    /// button.
    #[test]
    fn the_next_open_exercise_counts_what_is_left() {
        let view = AppView::new(crate::content::all());
        let seed = &view.modules[0];
        let all: Vec<i64> = seed.exercises.iter().map(|exercise| exercise.id).collect();

        assert_eq!(
            next_open_exercise(seed, &[]),
            Some((0, all.len())),
            "nothing done: the first exercise, and the whole count"
        );

        let done = all[..all.len() - 1].to_vec();
        assert_eq!(
            next_open_exercise(seed, &done),
            Some((all.len() - 1, 1)),
            "one left: the last position, count of one"
        );

        assert_eq!(next_open_exercise(seed, &all), None, "all done: no door");
    }

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
    fn tab_walks_from_the_wordmark_to_start_learning() {
        let mut cx = TestAppContext::build(TestDispatcher::new(0), Some("tab_walks"));
        let (view, cx) = open(&mut cx);

        assert_eq!(screen_of(&view, cx), "Home");

        // Wordmark, five nav entries (dashboard, progress, playground,
        // help, account), then START LEARNING — the landing screen's
        // primary CTA. Enter on it resumes the first lesson.
        for _ in 0..7 {
            cx.update(|window, cx| window.focus_next(cx));
        }
        press(cx, "enter");
        cx.run_until_parked();
        cx.update(|window, cx| window.draw(cx).clear(cx));

        assert_eq!(
            screen_of(&view, cx),
            "Lesson { module: 0, lesson: 0 }",
            "Enter on START LEARNING should open the first lesson"
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

        // `secondary` is ⌘ on macOS and CTRL elsewhere — the same word the
        // editor's binding uses, so the test presses the real combination.
        press(cx, "secondary-enter");
        cx.run_until_parked();

        let ran = cx.update(|_, cx| view.read(cx).playground.result.is_some());
        assert!(ran, "the shortcut should have started a run");
    }

    /// Entering the Playground focuses the editor, so the advertised shortcut
    /// works without a click first — the state issue #6 reported.
    #[test]
    fn the_shortcut_works_on_entry_without_a_click() {
        let mut cx = TestAppContext::build(
            TestDispatcher::new(0),
            Some("playground_shortcut_autofocus"),
        );
        let (view, cx) = open(&mut cx);

        cx.update(|_, cx| {
            view.update(cx, |view, cx| {
                view.navigate(Screen::Playground);
                cx.notify();
            });
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));

        // No manual focus here — that is the whole point. The draw above ran
        // playground_screen, which takes focus on a fresh editor.
        press(cx, "secondary-enter");
        cx.run_until_parked();

        let ran = cx.update(|_, cx| view.read(cx).playground.result.is_some());
        assert!(
            ran,
            "the shortcut should fire on entry, with the editor focused for the learner"
        );
    }

    /// Issue #8, extended: a snippet handed to the Playground lands in the
    /// editor *and starts running* — the click cannot set the value itself
    /// (that needs `&mut Window`), so it leaves the code behind for the
    /// next draw to pick up and launch.
    #[test]
    fn a_lesson_snippet_reaches_the_playground_editor_and_runs() {
        let mut cx = TestAppContext::build(TestDispatcher::new(0), Some("pending_snippet"));
        let (view, cx) = open(&mut cx);

        cx.update(|_, cx| {
            view.update(cx, |view, cx| {
                view.playground.pending_code = Some("fn main() { let x = 40 + 2; }".to_string());
                view.navigate(Screen::Playground);
                cx.notify();
            });
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));

        let value = cx.update(|_, cx| {
            let editor = view
                .read(cx)
                .playground
                .editor
                .clone()
                .expect("the draw creates the editor");
            editor.read(cx).value().to_string()
        });
        assert_eq!(value, "fn main() { let x = 40 + 2; }");
        assert!(
            cx.update(|_, cx| view.read(cx).playground.pending_code.is_none()),
            "the snippet is consumed by the draw, not left to overwrite the next one"
        );

        // "Run in playground" means run: the arrival both loads the buffer
        // and starts the compile, so the learner lands on output.
        cx.run_until_parked();
        let result = cx.update(|_, cx| view.read(cx).playground.result.clone());
        assert!(
            result.is_some(),
            "the snippet should have been run on arrival, not just loaded"
        );
    }

    /// Cancelling mid-run: the flag is raised, the task is dropped, and the
    /// console reports the stop rather than waiting out the wall clock.
    #[test]
    fn a_run_can_be_cancelled_and_the_console_reports_it() {
        use crate::services::sandbox::Status;

        let mut cx = TestAppContext::build(TestDispatcher::new(0), Some("playground_cancel"));
        let (view, cx) = open(&mut cx);

        cx.update(|_, cx| {
            view.update(cx, |view, cx| {
                view.navigate(Screen::Playground);
                cx.notify();
            });
        });
        cx.update(|window, cx| window.draw(cx).clear(cx));

        cx.update(|_, cx| {
            view.update(cx, |view, cx| {
                begin_run(
                    view,
                    "fn main() { let mut i = 0u64; loop { i = i.wrapping_add(1); } }",
                    cx,
                );
            });
        });

        let started = cx.update(|_, cx| view.read(cx).playground.running);
        assert!(started, "the run should be in flight");

        cx.update(|_, cx| {
            view.update(cx, |view, cx| {
                cancel_run(view, cx);
            });
        });

        let (running, status) = cx.update(|_, cx| {
            let playground = &view.read(cx).playground;
            let status = playground.result.as_ref().map(|result| result.status);
            (playground.running, status)
        });
        assert!(!running, "cancel must clear the running state at once");
        assert_eq!(
            status,
            Some(Status::Cancelled),
            "the console reports the stop"
        );

        cx.run_until_parked();
        cx.update(|window, cx| window.draw(cx).clear(cx));
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
