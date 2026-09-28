use gpui::*;
use crate::modules::Lesson;
use crate::theme::Theme;
use crate::components::exercise_view::ExerciseView;

pub struct LessonView {
    lesson: Lesson,
}

impl LessonView {
    pub fn new(lesson: Lesson) -> Self {
        Self { lesson }
    }
}

impl Render for LessonView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>().clone();

        div()
            .flex()
            .flex_col()
            .flex_1()
            .overflow_y_hidden()
            .px_12()
            .py_10()
            .gap_8()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_dim)
                            .child("LESSON")
                    )
                    .child(
                        div()
                            .text_3xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.text)
                            .child(self.lesson.title.clone())
                    )
            )
            .children(
                self.lesson.content.iter().map(|block| {
                    match block {
                        crate::modules::LessonBlock::Text(text) => {
                            div()
                                .text_base()
                                .text_color(theme.text_muted)
                                .child(text.clone())
                        }
                        crate::modules::LessonBlock::Code(code) => {
                            div()
                                .bg(theme.bg_panel)
                                .border_1()
                                .border_color(theme.border)
                                .rounded_sm()
                                .px_6()
                                .py_5()
                                .child(
                                    div()
                                        .text_sm()
                                        .font_family("IBM Plex Mono")
                                        .text_color(theme.text)
                                        .whitespace_normal()
                                        .child(code.clone())
                                )
                        }
                        crate::modules::LessonBlock::Heading(heading) => {
                            div()
                                .pt_6()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.accent)
                                        .child("SECTION")
                                )
                                .child(
                                    div()
                                        .pt_1()
                                        .text_lg()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.text)
                                        .child(heading.clone())
                                )
                        }
                        crate::modules::LessonBlock::Tip(tip) => {
                            div()
                                .bg(theme.bg_panel)
                                .border_1()
                                .border_color(theme.border)
                                .rounded_sm()
                                .px_5()
                                .py_4()
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(theme.text_muted)
                                        .child(tip.clone())
                                )
                        }
                        crate::modules::LessonBlock::Warning(warning) => {
                            div()
                                .bg(theme.bg_panel)
                                .border_1()
                                .border_color(theme.warning)
                                .rounded_sm()
                                .px_5()
                                .py_4()
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(theme.warning)
                                        .child(warning.clone())
                                )
                        }
                        crate::modules::LessonBlock::Exercise(exercise) => {
                            div().child(cx.new(|_| ExerciseView::new(exercise.clone())))
                        }
                    }
                })
            )
    }
}
