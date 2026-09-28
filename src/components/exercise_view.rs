use gpui::*;
use crate::modules::Exercise;
use crate::theme::Theme;

pub struct ExerciseView {
    exercise: Exercise,
    selected_option: Option<usize>,
    fix_input: String,
    completed: bool,
}

impl ExerciseView {
    pub fn new(exercise: Exercise) -> Self {
        Self {
            exercise,
            selected_option: None,
            fix_input: String::new(),
            completed: false,
        }
    }
}

impl Render for ExerciseView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>().clone();
        let entity = cx.entity();

        match &self.exercise {
            Exercise::MultipleChoice { question, options, correct } => {
                let is_correct = self.selected_option == Some(*correct);
                let show_result = self.selected_option.is_some();
                let question = question.clone();
                let options = options.clone();
                let correct = *correct;

                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .bg(theme.bg_panel)
                    .border_1()
                    .border_color(theme.border)
                    .rounded_sm()
                    .px_6()
                    .py_5()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_dim)
                            .child("CHECK")
                    )
                    .child(
                        div()
                            .text_base()
                            .text_color(theme.text)
                            .child(question)
                    )
                    .children(
                        options.into_iter().enumerate().map(|(i, option)| {
                            let is_selected = self.selected_option == Some(i);
                            let entity = entity.clone();

                            div()
                                .flex()
                                .items_center()
                                .gap_3()
                                .px_4()
                                .py_3()
                                .border_1()
                                .border_color(if is_selected { theme.accent } else { theme.border })
                                .rounded_sm()
                                .cursor_pointer()
                                .on_mouse_down(MouseButton::Left, move |_event, _window, cx| {
                                    entity.update(cx, |this, cx| {
                                        this.selected_option = Some(i);
                                        if i == correct {
                                            this.completed = true;
                                        }
                                        cx.notify();
                                    });
                                })
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(if is_selected { theme.accent } else { theme.text })
                                        .child(option)
                                )
                        })
                    )
                    .children(if show_result {
                        if is_correct {
                            Some(
                                div()
                                    .text_sm()
                                    .text_color(theme.success)
                                    .child("Correct!")
                            )
                        } else {
                            Some(
                                div()
                                    .text_sm()
                                    .text_color(theme.error)
                                    .child("Not quite. Try again.")
                            )
                        }
                    } else {
                        None
                    })
            }
            Exercise::PredictOutput { code, options, correct } => {
                let is_correct = self.selected_option == Some(*correct);
                let show_result = self.selected_option.is_some();
                let code = code.clone();
                let options = options.clone();
                let correct = *correct;

                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .bg(theme.bg_panel)
                    .border_1()
                    .border_color(theme.border)
                    .rounded_sm()
                    .px_6()
                    .py_5()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_dim)
                            .child("PREDICT")
                    )
                    .child(
                        div()
                            .bg(theme.bg)
                            .border_1()
                            .border_color(theme.border)
                            .rounded_sm()
                            .px_4()
                            .py_3()
                            .child(
                                div()
                                    .text_sm()
                                    .font_family("IBM Plex Mono")
                                    .text_color(theme.text)
                                    .whitespace_normal()
                                    .child(code)
                            )
                    )
                    .child(
                        div()
                            .text_base()
                            .text_color(theme.text)
                            .child("What does this program print?")
                    )
                    .children(
                        options.into_iter().enumerate().map(|(i, option)| {
                            let is_selected = self.selected_option == Some(i);
                            let entity = entity.clone();

                            div()
                                .flex()
                                .items_center()
                                .gap_3()
                                .px_4()
                                .py_3()
                                .border_1()
                                .border_color(if is_selected { theme.accent } else { theme.border })
                                .rounded_sm()
                                .cursor_pointer()
                                .on_mouse_down(MouseButton::Left, move |_event, _window, cx| {
                                    entity.update(cx, |this, cx| {
                                        this.selected_option = Some(i);
                                        if i == correct {
                                            this.completed = true;
                                        }
                                        cx.notify();
                                    });
                                })
                                .child(
                                    div()
                                        .text_sm()
                                        .font_family("IBM Plex Mono")
                                        .text_color(if is_selected { theme.accent } else { theme.text })
                                        .child(option)
                                )
                        })
                    )
                    .children(if show_result {
                        if is_correct {
                            Some(
                                div()
                                    .text_sm()
                                    .text_color(theme.success)
                                    .child("Correct!")
                            )
                        } else {
                            Some(
                                div()
                                    .text_sm()
                                    .text_color(theme.error)
                                    .child("Not quite. Try again.")
                            )
                        }
                    } else {
                        None
                    })
            }
            Exercise::FixCode { broken_code, fixed_code, hint } => {
                let broken_code = broken_code.clone();
                let fixed_code = fixed_code.clone();
                let hint = hint.clone();

                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .bg(theme.bg_panel)
                    .border_1()
                    .border_color(theme.border)
                    .rounded_sm()
                    .px_6()
                    .py_5()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_dim)
                            .child("FIX")
                    )
                    .child(
                        div()
                            .text_base()
                            .text_color(theme.text)
                            .child("This code has a problem. Fix it:")
                    )
                    .child(
                        div()
                            .bg(theme.bg)
                            .border_1()
                            .border_color(theme.border)
                            .rounded_sm()
                            .px_4()
                            .py_3()
                            .child(
                                div()
                                    .text_sm()
                                    .font_family("IBM Plex Mono")
                                    .text_color(theme.error)
                                    .whitespace_normal()
                                    .child(broken_code)
                            )
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.text_muted)
                            .child(hint)
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .px_4()
                            .py_3()
                            .border_1()
                            .border_color(theme.border)
                            .rounded_sm()
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_dim)
                                    .child("YOUR FIX:")
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .text_sm()
                                    .font_family("IBM Plex Mono")
                                    .text_color(theme.text)
                                    .child(self.fix_input.clone())
                            )
                    )
                    .child(
                        div()
                            .px_4()
                            .py_2()
                            .bg(theme.accent)
                            .rounded_xs()
                            .cursor_pointer()
                            .on_mouse_down(MouseButton::Left, {
                                let entity = entity.clone();
                                move |_event, _window, cx| {
                                    entity.update(cx, |this, cx| {
                                        if this.fix_input.trim() == fixed_code.trim() {
                                            this.completed = true;
                                        }
                                        cx.notify();
                                    });
                                }
                            })
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.bg)
                                    .child("CHECK")
                            )
                    )
            }
        }
    }
}
