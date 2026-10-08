//! The fonts the theme names, embedded in the binary.
//!
//! gpui resolves a family from the platform's font database, and an explicitly
//! named family that was never registered renders no glyphs at all — which is
//! how `borrow.theme.json` shipped two phases without a `font.family`. These
//! are registered once at startup, before the theme is applied.
//!
//! The files are IBM Plex Mono, SIL OFL 1.1 (`assets/fonts/OFL.txt`).

use std::borrow::Cow;

const REGULAR: &[u8] = include_bytes!("../../../assets/fonts/IBMPlexMono-Regular.ttf");
const SEMIBOLD: &[u8] = include_bytes!("../../../assets/fonts/IBMPlexMono-SemiBold.ttf");
const BOLD: &[u8] = include_bytes!("../../../assets/fonts/IBMPlexMono-Bold.ttf");
const ITALIC: &[u8] = include_bytes!("../../../assets/fonts/IBMPlexMono-Italic.ttf");

/// The family these files provide. `borrow.theme.json` must ask for exactly
/// this; the test in `theme.rs` is what keeps the two in step.
pub const FAMILY: &str = "IBM Plex Mono";

/// The embedded files, ready for `text_system().add_fonts`.
///
/// Weights the design asks for and this list does not carry (300, 500) are
/// matched to the nearest embedded face rather than synthesised, which is
/// what the web does with its own fallback chain.
pub fn all() -> Vec<Cow<'static, [u8]>> {
    vec![
        Cow::Borrowed(REGULAR),
        Cow::Borrowed(SEMIBOLD),
        Cow::Borrowed(BOLD),
        Cow::Borrowed(ITALIC),
    ]
}

#[cfg(test)]
mod tests {
    // No `use super::*`: the parent module is compiled into a crate that does
    // `use gpui_kit::*`, whose `test` macro would replace Rust's (HANDOFF 1).
    use super::{all, FAMILY};

    /// Every embedded file is a real TrueType font — `include_bytes!` happily
    /// embeds a 404 page, and a font that is not a font registers as nothing.
    #[test]
    fn every_embedded_file_is_a_true_type_font() {
        let fonts = all();
        assert_eq!(fonts.len(), 4, "regular, semibold, bold and italic");

        for bytes in &fonts {
            let magic = bytes.get(..4).unwrap_or_default();
            let true_type = magic == [0x00, 0x01, 0x00, 0x00] || magic == b"OTTO";
            assert!(true_type, "not a font: {magic:?}");
            assert!(
                bytes.len() > 10_000,
                "suspiciously small: {} bytes",
                bytes.len()
            );
        }
    }

    /// The family the theme asks for is the family embedded here.
    ///
    /// Two sources of truth — a JSON string and a constant — and naming the
    /// wrong one is invisible until text fails to shape, so they are compared
    /// rather than trusted.
    #[test]
    fn the_theme_asks_for_the_embedded_family() {
        let json: serde_json::Value =
            serde_json::from_str(include_str!("borrow.theme.json")).expect("theme parses");
        let theme = &json["themes"][0];

        for key in ["font.family", "mono_font.family"] {
            assert_eq!(
                theme[key].as_str(),
                Some(FAMILY),
                "{key} must name the embedded family"
            );
        }
    }
}
