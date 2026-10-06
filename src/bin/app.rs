//! Phase 1 spike — `cargo run --bin app`
//!
//! Exists only to answer one question before any real porting work: does the
//! gpui-kit stack compile, open a window, and render BORROW_'s palette? Every
//! screen after this depends on that being true.
//!
//! It is deliberately a separate binary rather than a rewrite of `main.rs`. The
//! web app must keep building and its 78 tests must stay green while the desktop
//! side is explored, so this spike shares `Cargo.toml` with it and nothing else.
//!
//! Out of scope for the spike (tracked as later phases):
//! - bundled fonts — `assets/fonts/` is empty and the web app already falls
//!   back to system fonts, so this matches current behaviour for now.
//! - the `app_id` used by Wayland/portal window placement.

use gpui_kit::component::button::*;
use gpui_kit::*;

// ── BORROW_ design tokens ─────────────────────────────────────────────────
//
// Mirrors the `:root` block in static/style.css and BORROW_UI_DESIGN.md.
// Duplicated here rather than shared because the CSS is not parseable from
// Rust; the two are reconciled when the real theme lands in Phase 2, after
// which this file goes away.
const BACKGROUND: u32 = 0x0B0A08;
const SURFACE: u32 = 0x14120F;
const ELEVATED: u32 = 0x1C1915;
const BORDER: u32 = 0x34302A;
const BORDER_STRONG: u32 = 0x6D6558;
const ACCENT: u32 = 0xFF6B35;
const ACCENT_SECONDARY: u32 = 0xF2C14E;
const SUCCESS: u32 = 0x39FFB6;
const TEXT: u32 = 0xF4EFE6;
const TEXT_MUTED: u32 = 0xA9A197;

/// The tokens as one swatch strip, so a single screenshot proves the whole
/// palette survived the port — not just the two colours the layout happens to
/// use.
const SWATCHES: [(&str, u32); 9] = [
    ("background", BACKGROUND),
    ("surface", SURFACE),
    ("elevated", ELEVATED),
    ("border", BORDER),
    ("border-strong", BORDER_STRONG),
    ("accent", ACCENT),
    ("accent-2", ACCENT_SECONDARY),
    ("success", SUCCESS),
    ("text", TEXT),
];

struct Spike;

impl Render for Spike {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .bg(rgb(BACKGROUND))
            .text_color(rgb(TEXT))
            // Wordmark.
            .child(
                div()
                    .text_size(px(40.))
                    .text_color(rgb(ACCENT))
                    .child("BORROW_"),
            )
            .child(
                div()
                    .text_size(px(13.))
                    .text_color(rgb(TEXT_MUTED))
                    .child("gpui-kit spike — BORROW_ palette on a native window"),
            )
            // A real component, so the spike proves gpui-component initialised
            // and is restyled, not just that div() painted.
            .child(Button::new("run").primary().label("TRY THIS"))
            // Palette strip.
            .child(div().flex().flex_row().gap_1().mt_4().child(swatch_strip()))
    }
}

/// One block per token, labelled, so mismatches are visible at a glance.
fn swatch_strip() -> impl IntoElement {
    let mut row = div()
        .flex()
        .flex_row()
        .gap_1()
        .border_1()
        .border_color(rgb(BORDER_STRONG));

    for (name, colour) in SWATCHES {
        row = row.child(
            div()
                .w(px(80.))
                .bg(rgb(colour))
                .p_1()
                .flex()
                .flex_col()
                .justify_end()
                .child(
                    div()
                        .text_size(px(9.))
                        .text_color(if colour == ACCENT_SECONDARY || colour == SUCCESS {
                            rgb(BACKGROUND)
                        } else {
                            rgb(TEXT)
                        })
                        .child(name),
                ),
        );
    }

    row
}

fn main() {
    gpui_kit::application().run(|cx| {
        // Required before anything else uses the component system.
        gpui_kit::init(cx);

        let options = WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some("BORROW_".into()),
                ..Default::default()
            }),
            window_bounds: Some(WindowBounds::Windowed(bounds(
                point(px(120.), px(120.)),
                size(px(1000.), px(640.)),
            ))),
            ..Default::default()
        };

        gpui_kit::open_window(options, cx, |_, cx| cx.new(|_| Spike))
            .expect("failed to open window");
    });
}
