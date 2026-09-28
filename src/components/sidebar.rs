use gpui::*;
use crate::modules::{all_modules, Module};
use crate::theme::Theme;
use crate::app::AppState;

pub struct Sidebar {
    modules: Vec<Module>,
    app_state: Entity<AppState>,
}

impl Sidebar {
    pub fn new(app_state: Entity<AppState>) -> Self {
        Self {
            modules: all_modules(),
            app_state,
        }
    }
}

impl Render for Sidebar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let app_state = self.app_state.read(cx);

        div()
            .flex()
            .flex_col()
            .w(px(250.))
            .h_full()
            .bg(theme.bg_sidebar)
            .border_r_1()
            .border_color(theme.border)
            .overflow_y_hidden()
            .child(
                div()
                    .px_5()
                    .py_6()
                    .border_b_1()
                    .border_color(theme.border)
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.text)
                            .child("BORROW")
                    )
            )
            .child(
                div()
                    .px_5()
                    .py_3()
                    .border_b_1()
                    .border_color(theme.border)
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_dim)
                            .child("CURRICULUM")
                    )
            )
            .children(
                self.modules.iter().enumerate().map(|(mi, module)| {
                    let is_module_selected = mi == app_state.selected_module;
                    div()
                        .flex()
                        .flex_col()
                        .py_2()
                        .child(
                            div()
                                .px_5()
                                .py_2()
                                .flex()
                                .items_baseline()
                                .gap_3()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(if is_module_selected { theme.accent } else { theme.text_dim })
                                        .child(module.number)
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(if is_module_selected { theme.text } else { theme.text_dim })
                                        .child(module.title)
                                )
                        )
                        .children(
                            module.lessons.iter().enumerate().map(|(li, lesson)| {
                                let selected = is_module_selected && li == app_state.selected_lesson;
                                div()
                                    .pl_12()
                                    .pr_5()
                                    .py_2()
                                    .ml_4()
                                    .text_sm()
                                    .text_color(if selected { theme.accent } else { theme.text_muted })
                                    .bg(if selected { theme.bg_panel } else { theme.bg_sidebar })
                                    .border_l_2()
                                    .border_color(if selected { theme.accent } else { theme.border })
                                    .rounded_r_sm()
                                    .px_3()
                                    .cursor_pointer()
                                    .on_mouse_down(MouseButton::Left, {
                                        let app_state = self.app_state.clone();
                                        move |_event, _window, cx| {
                                            app_state.update(cx, |state, cx| {
                                                state.select_lesson(mi, li, cx);
                                            });
                                        }
                                    })
                                    .child(lesson.title.clone())
                            })
                        )
                })
            )
    }
}
