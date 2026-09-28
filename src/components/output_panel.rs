use gpui::*;
use crate::theme::Theme;
use crate::runner::RunnerOutput;

pub struct OutputPanel {
    output: Entity<RunnerOutput>,
}

impl OutputPanel {
    pub fn new(output: Entity<RunnerOutput>) -> Self {
        Self { output }
    }
}

impl Render for OutputPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let output = self.output.read(cx);

        let output_text = if output.running {
            "Compiling and running...".to_string()
        } else if output.stdout.is_empty() && output.stderr.is_empty() {
            "Awaiting execution...".to_string()
        } else {
            let mut text = String::new();
            if !output.stdout.is_empty() {
                text.push_str(&output.stdout);
            }
            if !output.stderr.is_empty() {
                text.push_str(&output.stderr);
            }
            text
        };

        div()
            .flex()
            .flex_col()
            .h(px(140.))
            .bg(theme.bg_output)
            .border_t_1()
            .border_color(theme.border)
            .child(
                div()
                    .flex()
                    .items_center()
                    .px_5()
                    .py_3()
                    .border_b_1()
                    .border_color(theme.border)
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_dim)
                            .child("OUTPUT")
                    )
            )
            .child(
                div()
                    .flex_1()
                    .overflow_y_hidden()
                    .px_5()
                    .py_4()
                    .child(
                        div()
                            .text_sm()
                            .font_family("IBM Plex Mono")
                            .text_color(theme.text_muted)
                            .whitespace_normal()
                            .child(output_text)
                    )
            )
    }
}
