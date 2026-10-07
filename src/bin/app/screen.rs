//! Screens and navigation: home → module → lesson.
//!
//! One `AppView` owns the screen, and buttons capture a handle to the view so a
//! click can update that state and request a re-render. Nothing here is async —
//! `content::all()` is compiled in, so switching screens costs a render rather
//! than a query.

use crate::components::label;
use crate::content::ModuleSeed;
use crate::lesson::render_section;
use crate::models::LessonContent;
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
    Lesson { module: usize, lesson: usize },
}

pub struct AppView {
    modules: Vec<ModuleSeed>,
    screen: Screen,
}

impl AppView {
    pub fn new(modules: Vec<ModuleSeed>) -> Self {
        Self {
            modules,
            screen: Screen::Home,
        }
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

            list = list.child(
                div()
                    .id(format!("module-{index}"))
                    .p_4()
                    .rounded_md()
                    .border_1()
                    .border_color(theme.border)
                    .bg(theme.secondary)
                    .cursor_pointer()
                    .hover(|d| d.bg(theme.muted))
                    .on_click(move |_, _, cx| {
                        jump_to.update(cx, |view, cx| {
                            view.screen = Screen::Module(index);
                            cx.notify();
                        });
                    })
                    .flex()
                    .flex_col()
                    .gap_1()
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
                    ),
            );
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
                div()
                    .id(format!("lesson-{id}"))
                    .p_3()
                    .rounded_md()
                    .border_1()
                    .border_color(theme.border)
                    .bg(theme.secondary)
                    .cursor_pointer()
                    .hover(|d| d.bg(theme.muted))
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

        let body = div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .text_size(px(13.))
                    .text_color(theme.muted_foreground)
                    .child(description),
            )
            .child(list);

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
            div()
                .id("nav-home")
                .cursor_pointer()
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
        }
    }
}

/// The page heading: title, plus a subtitle only when the screen has one.
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
