//! Phase 3 — the screens: `cargo run --bin app`
//!
//! Phase 1 proved the stack opens a window; phase 2 proved BORROW_'s tokens
//! reach gpui-component's components. This renders real course content — the
//! 14 lesson block types — driven by `content::all()` rather than the
//! database, so the desktop app reads the same material the site does with no
//! async and no seeding step.
//!
//! Progress (completions, XP) stays in SQLite later; content is compiled in.

// `src/bin/app.rs` is a crate root, so its children resolve against `src/bin/`.
// The shared modules are pulled in by explicit path instead of living under
// `src/bin/`, where cargo's autobin discovery would pick them up as extra
// binaries.
#[path = "app/components.rs"]
mod components;
#[path = "../content/mod.rs"]
mod content;
#[path = "app/lesson.rs"]
mod lesson;
#[path = "../models/mod.rs"]
mod models;
#[path = "app/screen.rs"]
mod screen;
#[path = "app/theme.rs"]
mod theme;

use gpui_kit::*;
use screen::AppView;

fn main() {
    gpui_kit::application().run(|cx| {
        gpui_kit::init(cx);

        // After init: gpui_kit::init installs gpui-component's own defaults and
        // flips to light mode, so applying ours first would be thrown away.
        if let Err(err) = theme::apply(cx) {
            panic!("could not apply the BORROW_ theme: {err:#}");
        }

        let modules = content::all();

        let options = WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some("BORROW_".into()),
                ..Default::default()
            }),
            window_bounds: Some(WindowBounds::Windowed(bounds(
                point(px(120.), px(120.)),
                size(px(1040.), px(760.)),
            ))),
            ..Default::default()
        };

        gpui_kit::open_window(options, cx, |_, cx| cx.new(|_| AppView::new(modules)))
            .expect("failed to open window");
    });
}

#[cfg(test)]
mod sweep_tests {
    use crate::lesson::render_section;
    use crate::models::{LessonContent, LessonSection};
    use crate::theme::test_theme;

    /// Every block in every lesson renders.
    ///
    /// The match in `render_section` is exhaustive, so each variant has a
    /// renderer by construction — this sweep proves the *data* holds up: that
    /// all 284 lessons parse, and that the rendering path handles their real
    /// shapes rather than a fixture's.
    #[test]
    fn every_block_in_the_course_renders() {
        let modules = crate::content::all();
        let theme = test_theme();

        let mut lessons = 0;
        let mut blocks = 0;
        let mut parse_failures = Vec::new();

        for seed in &modules {
            for record in &seed.lessons {
                lessons += 1;
                match serde_json::from_str::<LessonContent>(&record.content) {
                    Ok(content) => {
                        for section in &content.sections {
                            let _ = render_section(&theme, section);
                            blocks += 1;
                        }
                    }
                    Err(err) => parse_failures
                        .push(format!("lesson {} ({}): {err}", record.id, record.title)),
                }
            }
        }

        assert!(
            parse_failures.is_empty(),
            "{} lessons failed to parse:\n{}",
            parse_failures.len(),
            parse_failures.join("\n")
        );
        assert_eq!(lessons, 284, "the seeded course has 284 lessons");
        // 2005 as of this commit. A floor rather than an exact count: content
        // edits move the number every day, and a regression to exactly 2005
        // plus a few would be indistinguishable from normal drift — but losing
        // whole lesson bodies would show up below here.
        assert!(
            blocks > 1800,
            "rendered only {blocks} blocks across {lessons} lessons"
        );
    }

    /// Every block type in the model actually occurs in the course.
    ///
    /// Without this, a renderer could exist for a variant that no lesson uses
    /// and never be exercised — the sweep above would stay green while a whole
    /// block type sat untested.
    #[test]
    fn every_block_type_is_exercised_by_real_content() {
        let theme = test_theme();
        let mut seen = [false; 14];

        for seed in &crate::content::all() {
            for record in &seed.lessons {
                let Ok(content) = serde_json::from_str::<LessonContent>(&record.content) else {
                    continue;
                };
                for section in &content.sections {
                    let index = variant_index(section);
                    seen[index] = true;
                    let _ = render_section(&theme, section);
                }
            }
        }

        let missing: Vec<usize> = seen
            .iter()
            .enumerate()
            .filter(|(_, s)| !**s)
            .map(|(i, _)| i)
            .collect();
        assert!(
            missing.is_empty(),
            "block types {missing:?} never appear in the seeded course, so they are untested"
        );
    }

    /// Maps a section to a stable index, so the test above can report which
    /// types were absent. Rust's match here is exhaustive: adding a variant
    /// without updating this is a compile error.
    fn variant_index(section: &LessonSection) -> usize {
        match section {
            LessonSection::Text { .. } => 0,
            LessonSection::Callout { .. } => 1,
            LessonSection::Code { .. } => 2,
            LessonSection::Breakdown { .. } => 3,
            LessonSection::Checklist { .. } => 4,
            LessonSection::Diagram { .. } => 5,
            LessonSection::Table { .. } => 6,
            LessonSection::Steps { .. } => 7,
            LessonSection::Paragraphs { .. } => 8,
            LessonSection::GapFill { .. } => 9,
            LessonSection::StepThrough { .. } => 10,
            LessonSection::GuessRun { .. } => 11,
            LessonSection::Dispatch { .. } => 12,
            LessonSection::Pipeline { .. } => 13,
        }
    }
}
