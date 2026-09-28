use gpui::*;
use crate::modules::Lesson;
use crate::theme::Theme;
use crate::runner::{run_code, RunnerOutput};
use crate::components::sidebar::Sidebar;
use crate::components::lesson_view::LessonView;
use crate::components::code_editor::CodeEditor;
use crate::components::output_panel::OutputPanel;

pub struct AppState {
    pub modules: Vec<crate::modules::Module>,
    pub selected_module: usize,
    pub selected_lesson: usize,
    pub editor_content: String,
    pub runner_output: Entity<RunnerOutput>,
    pub completed_lessons: Vec<(usize, usize)>,
}

impl AppState {
    pub fn current_lesson(&self) -> Option<&Lesson> {
        self.modules
            .get(self.selected_module)
            .and_then(|m| m.lessons.get(self.selected_lesson))
    }

    pub fn select_lesson(&mut self, module: usize, lesson: usize, cx: &mut Context<Self>) {
        self.selected_module = module;
        self.selected_lesson = lesson;
        if let Some(l) = self.current_lesson() {
            self.editor_content = l.starter_code.clone();
        }
        cx.notify();
    }

    pub fn run_code(&mut self, cx: &mut Context<Self>) {
        let code = self.editor_content.clone();
        self.runner_output = run_code(&code, cx);
        cx.notify();
    }

    pub fn mark_lesson_complete(&mut self, module: usize, lesson: usize) {
        if !self.completed_lessons.contains(&(module, lesson)) {
            self.completed_lessons.push((module, lesson));
        }
    }

    pub fn is_lesson_complete(&self, module: usize, lesson: usize) -> bool {
        self.completed_lessons.contains(&(module, lesson))
    }
}

impl Render for AppState {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>().clone();
        let entity = cx.entity();
        let sidebar = cx.new(|_| Sidebar::new(entity));
        let lesson_entity = self.current_lesson().map(|l| cx.new(|_| LessonView::new(l.clone())));
        let editor = cx.new(|_| CodeEditor::new(self.editor_content.clone()));
        let output = cx.new(|_| OutputPanel::new(self.runner_output.clone()));

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.bg)
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_8()
                    .py_4()
                    .border_b_1()
                    .border_color(theme.border)
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.text)
                            .child("BORROW")
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_dim)
                            .child(format!("MODULE {} / 10", self.modules.get(self.selected_module).map(|m| m.number).unwrap_or("??")))
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_1()
                    .overflow_hidden()
                    .child(sidebar)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .overflow_hidden()
                            .child(
                                div()
                                    .flex()
                                    .flex_1()
                                    .overflow_hidden()
                                    .child(lesson_entity.unwrap_or_else(|| cx.new(|_| LessonView::new(Lesson {
                                        title: "No lesson selected".into(),
                                        content: vec![],
                                        starter_code: String::new(),
                                        project: None,
                                    }))))
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .h(px(220.))
                                    .border_t_1()
                                    .border_color(theme.border)
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .px_5()
                                            .py_3()
                                            .border_b_1()
                                            .border_color(theme.border)
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.text_dim)
                                                    .child("main.rs")
                                            )
                                            .child(
                                                div()
                                                    .px_5()
                                                    .py_1()
                                                    .bg(theme.accent)
                                                    .rounded_xs()
                                                    .cursor_pointer()
                                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                                        this.run_code(cx);
                                                    }))
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .font_weight(FontWeight::BOLD)
                                                            .text_color(theme.bg)
                                                            .child("RUN")
                                                    )
                                            )
                                    )
                                    .child(editor)
                            )
                    )
            )
            .child(output)
    }
}
