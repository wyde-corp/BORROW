use gpui::*;

#[derive(Clone)]
pub struct Theme {
    pub bg: Hsla,
    pub bg_panel: Hsla,
    pub bg_sidebar: Hsla,
    pub bg_editor: Hsla,
    pub bg_output: Hsla,
    pub border: Hsla,
    pub text: Hsla,
    pub text_muted: Hsla,
    pub text_dim: Hsla,
    pub accent: Hsla,
    pub accent_dim: Hsla,
    pub success: Hsla,
    pub error: Hsla,
    pub warning: Hsla,
}

impl Theme {
    pub fn new() -> Self {
        Self {
            bg: rgb(0xF4F1EA).into(),
            bg_panel: rgb(0xFAF8F4).into(),
            bg_sidebar: rgb(0xF0EDE6).into(),
            bg_editor: rgb(0xFAF8F4).into(),
            bg_output: rgb(0xF0EDE6).into(),
            border: rgb(0xD4CFC5).into(),
            text: rgb(0x1A1815).into(),
            text_muted: rgb(0x6B6560).into(),
            text_dim: rgb(0x9A948C).into(),
            accent: rgb(0x8B6914).into(),
            accent_dim: rgb(0xA07D1E).into(),
            success: rgb(0x4A7A5A).into(),
            error: rgb(0xA04030).into(),
            warning: rgb(0xA06820).into(),
        }
    }
}

impl Global for Theme {}
