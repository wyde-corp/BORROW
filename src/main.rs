mod app;
mod theme;
mod runner;
mod modules;
mod components;

use gpui::*;
use theme::Theme;
use app::AppState;
use runner::RunnerOutput;
use modules::all_modules;

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.set_global(Theme::new());

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1200.0), px(800.0)),
                    cx,
                ))),
                ..Default::default()
            },
            |_, cx| {
                let runner_output = cx.new(|_| RunnerOutput {
                    stdout: String::new(),
                    stderr: String::new(),
                    success: false,
                    running: false,
                });

                let modules = all_modules();
                let initial_code = modules
                    .first()
                    .and_then(|m| m.lessons.first())
                    .map(|l| l.starter_code.clone())
                    .unwrap_or_default();

                cx.new(|_| AppState {
                    modules,
                    selected_module: 0,
                    selected_lesson: 0,
                    editor_content: initial_code,
                    runner_output,
                    completed_lessons: Vec::new(),
                })
            },
        )
        .unwrap();
    });
}
