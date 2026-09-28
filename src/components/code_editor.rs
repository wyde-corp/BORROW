use gpui::*;
use crate::theme::Theme;

pub struct CodeEditor {
    content: String,
}

impl CodeEditor {
    pub fn new(initial: String) -> Self {
        Self { content: initial }
    }
}

impl Render for CodeEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.bg_editor)
            .border_1()
            .border_color(theme.border)
            .rounded_sm()
            .child(
                div()
                    .flex_1()
                    .px_6()
                    .py_4()
                    .child(
                        div()
                            .text_sm()
                            .font_family("IBM Plex Mono")
                            .text_color(theme.text)
                            .whitespace_normal()
                            .child(self.content.clone())
                    )
            )
    }
}
