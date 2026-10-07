//! Phase 2 — the design system on native: `cargo run --bin app`
//!
//! Phase 1 proved the stack opens a window. This proves BORROW_'s tokens
//! actually reach gpui-component's components, which is the whole content of
//! "gpui-component, restyled": components are not restyled individually, they
//! all read one global theme, so applying our config restyles all of them.

// An explicit path, because `src/bin/app.rs` is a crate root: `mod theme;`
// would resolve against `src/bin/`, and placing the file there would make
// cargo autobin-discover it as a second binary called `theme`.
#[path = "app/theme.rs"]
mod theme;

use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::*;
use gpui_kit::*;

/// A swatch: a design token's name and how to read that colour from the
/// live theme.
type Swatch = (&'static str, fn(&Theme) -> Hsla);

/// One swatch per token, so a single view shows whether the palette that
/// reached native matches the stylesheet.
const SWATCHES: &[Swatch] = &[
    ("background", |t| t.background),
    ("surface", |t| t.secondary),
    ("elevated", |t| t.muted),
    ("border", |t| t.border),
    ("border-strong", |t| t.input),
    ("accent", |t| t.primary),
    ("accent-2", |t| t.warning),
    ("success", |t| t.success),
    ("info", |t| t.info),
    ("text", |t| t.foreground),
    ("text-muted", |t| t.muted_foreground),
];

struct DesignSystem;

impl Render for DesignSystem {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_5()
            .p_8()
            .bg(theme.background)
            .text_color(theme.foreground)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_size(px(34.))
                            .text_color(theme.primary)
                            .font_weight(FontWeight::BOLD)
                            .child("BORROW_"),
                    )
                    .child(
                        div()
                            .text_size(px(13.))
                            .text_color(theme.muted_foreground)
                            .child("phase 2 — BORROW_ tokens applied to gpui-component"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_3()
                    .child(
                        Button::new("primary").primary().label("TRY THIS"),
                    )
                    .child(
                        Button::new("secondary").secondary().label("RESET"),
                    )
                    .child(
                        div()
                            .text_size(px(12.))
                            .text_color(theme.muted_foreground)
                            .child("⌘/CTRL + ENTER"),
                    ),
            )
            .child(swatch_row(theme))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap_4()
                            .child(
                                div()
                                    .text_size(px(20.))
                                    .font_weight(FontWeight::BOLD)
                                    .child("Section heading"),
                            )
                            .child(
                                div()
                                    .text_size(px(11.))
                                    .text_color(theme.muted_foreground)
                                    .child("LESSON 01 / 03"),
                            ),
                    )
                    .child(
                        div()
                            .max_w(px(620.))
                            .text_size(px(13.))
                            .text_color(theme.foreground)
                            .child("A variable is a name attached to a value so later code can refer to that value by something meaningful — not to save typing, but so the code says what it means."),
                    )
                    .child(
                        // Callout, for the tone accent the site uses on idea boxes.
                        div()
                            .w(px(620.))
                            .mt_2()
                            .p_3()
                            .rounded_md()
                            .border_l_1()
                            .border_color(theme.info)
                            .bg(theme.muted)
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_size(px(10.))
                                    .text_color(theme.info)
                                    .font_weight(FontWeight::BOLD)
                                    .child("CONCEPT"),
                            )
                            .child(
                                div()
                                    .text_size(px(12.5))
                                    .text_color(theme.muted_foreground)
                                    .child("Every running program needs memory, and that somewhere has to be handed back when you are finished."),
                            ),
                    ),
            )
    }
}

/// Each swatch shows the token name over its colour, so a mismatch between
/// this and `style.css` is visible rather than merely detectable.
fn swatch_row(theme: &Theme) -> impl IntoElement {
    let mut row = div()
        .flex()
        .flex_row()
        .gap_1()
        .border_1()
        .border_color(theme.border);

    for (name, getter) in SWATCHES {
        let colour = getter(theme);

        row = row.child(
            div()
                .w(px(74.))
                .h(px(58.))
                .bg(colour)
                .flex()
                .flex_col()
                .items_start()
                .justify_end()
                .p_1()
                .child(
                    div()
                        .text_size(px(8.5))
                        .text_color(label_on(colour, theme))
                        .child(*name),
                ),
        );
    }

    row
}

/// Which of the theme's two text colours a swatch label needs.
///
/// Labels are only readable on roughly half the palette, so choosing by the
/// swatch's own lightness keeps every name legible without inventing a third
/// colour outside the design tokens.
fn label_on(colour: Hsla, theme: &Theme) -> Hsla {
    if colour.l > 0.6 {
        theme.background
    } else {
        theme.foreground
    }
}

fn main() {
    gpui_kit::application().run(|cx| {
        gpui_kit::init(cx);

        // After init: gpui_kit::init installs its own defaults and flips to
        // light mode, so applying before it would be thrown away.
        if let Err(err) = theme::apply(cx) {
            panic!("could not apply the BORROW_ theme: {err:#}");
        }

        let options = WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some("BORROW_".into()),
                ..Default::default()
            }),
            window_bounds: Some(WindowBounds::Windowed(bounds(
                point(px(120.), px(120.)),
                size(px(1000.), px(760.)),
            ))),
            ..Default::default()
        };

        gpui_kit::open_window(options, cx, |_, cx| cx.new(|_| DesignSystem))
            .expect("failed to open window");
    });
}
