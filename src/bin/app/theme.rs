//! The BORROW_ theme: one JSON config, applied on top of gpui-component's.
//!
//! gpui-component ships a complete shadcn theme and every component reads from
//! it, so restyling is a matter of registering a config of our own and telling
//! the global theme to apply it — not of restyling components one by one.
//!
//! The JSON is generated from `static/style.css` by `tools/gen_theme.py`, and
//! the test below asserts the two still agree. That is deliberate: two palettes
//! living in two files is exactly how a desktop app ends up quietly off-brand.

use gpui_kit::component::{Theme, ThemeRegistry};
use gpui_kit::App;

/// BORROW_'s palette, as a gpui-component theme config.
const BORROW_THEME: &str = include_str!("borrow.theme.json");

/// The theme's name inside the config, looked up after registration.
const THEME_NAME: &str = "BORROW_";

/// Registers BORROW_ and makes it the active theme.
///
/// Must be called **after** `gpui_kit::init(cx)`, because that call installs
/// gpui-component's defaults and switches to its light theme — applying ours
/// first would be overwritten. Once applied this sets the active mode to dark,
/// the resolved colours, the radii, and the syntax highlighting for the code
/// panel.
pub fn apply(cx: &mut App) -> anyhow::Result<()> {
    let registered = {
        let registry = ThemeRegistry::global_mut(cx);
        registry.load_themes_from_str(BORROW_THEME)?;
        registry.themes().get(THEME_NAME).cloned()
    };

    let config = registered.ok_or_else(|| {
        anyhow::anyhow!("the {THEME_NAME} theme did not register — check borrow.theme.json")
    })?;

    Theme::global_mut(cx).apply_config(&config);
    Ok(())
}

/// Builds BORROW_'s theme in memory, for tests that need real colours without
/// opening a window. The same path `apply` takes, minus the registry.
#[cfg(test)]
pub fn test_theme() -> gpui_kit::component::Theme {
    let set: gpui_kit::component::ThemeSet =
        serde_json::from_str(BORROW_THEME).expect("borrows theme must parse");
    let config = set
        .themes
        .into_iter()
        .find(|t| t.name == THEME_NAME)
        .expect("BORROW_ theme present");

    let mut theme = gpui_kit::component::Theme::default();
    theme.apply_config(&std::rc::Rc::new(config));
    theme
}

#[cfg(test)]
mod tests {
    use super::BORROW_THEME;
    use std::collections::HashMap;

    /// The theme must deserialize with gpui-component's own types. serde here
    /// is the real validator: `ThemeConfigColors` renames most of its fields,
    /// and an unknown key is silently dropped rather than erroring, so a typo
    /// would otherwise ship as a component left on shadcn's palette.
    #[test]
    fn the_theme_deserializes_as_a_complete_gpui_config() {
        let set: gpui_kit::component::ThemeSet =
            serde_json::from_str(BORROW_THEME).expect("borrows theme must parse");

        let theme = set
            .themes
            .iter()
            .find(|t| t.name == "BORROW_")
            .expect("BORROW_ theme present");

        assert!(theme.mode.is_dark(), "BORROW_ is a dark design");
        assert!(theme.colors.background.is_some(), "background token kept");
        assert!(theme.colors.primary.is_some(), "primary token kept");
        assert!(
            theme.highlight.is_some(),
            "syntax highlighting is part of the design"
        );
    }

    /// Applies the config the way `apply` does, and checks the resolved
    /// palette.
    ///
    /// This is the runtime half of the drift guard: the test above proves the
    /// file agrees with `style.css`, and this proves gpui-component's
    /// `apply_config` actually carries those values into the live theme. It
    /// runs headless on purpose — it needs no window and no compositor, so it
    /// can gate every change in CI.
    #[test]
    fn applying_the_config_resolves_the_borrow_palette() {
        use gpui_kit::component::Theme;
        use gpui_kit::rgb;
        use std::rc::Rc;

        let set: gpui_kit::component::ThemeSet =
            serde_json::from_str(BORROW_THEME).expect("borrows theme must parse");
        let config = set
            .themes
            .into_iter()
            .find(|t| t.name == "BORROW_")
            .expect("BORROW_ theme present");

        let mut theme = Theme::default();
        theme.apply_config(&Rc::new(config));

        assert_eq!(theme.background, rgb(0x0B0A08).into(), "page background");
        assert_eq!(theme.foreground, rgb(0xF4EFE6).into(), "body text");
        assert_eq!(theme.border, rgb(0x34302A).into(), "hairline border");
        assert_eq!(theme.primary, rgb(0xFF6B35).into(), "accent / buttons");
        assert_eq!(
            theme.primary_foreground,
            rgb(0x0B0A08).into(),
            "text on accent"
        );
        assert_eq!(
            theme.muted_foreground,
            rgb(0xA9A197).into(),
            "secondary text"
        );
        assert_eq!(theme.success, rgb(0x39FFB6).into(), "success state");
        assert_eq!(theme.info, rgb(0x5CC8FF).into(), "info state");
        assert_eq!(theme.warning, rgb(0xF2C14E).into(), "warning state");

        // BORROW_ is a dark design; gpui-component's default theme is dark
        // too, so this only proves the mode survived, not that ours applied.
        assert!(theme.mode.is_dark());
        assert_eq!(theme.radius, gpui_kit::px(4.), "the design's small radius");
        assert_eq!(
            theme.radius_lg,
            gpui_kit::px(8.),
            "the design's large radius"
        );
    }

    fn css_tokens() -> HashMap<String, String> {
        let css = include_str!("../../../static/style.css");
        let start = css.find(":root {").expect("style.css has a :root block");
        let block = &css[start..css[start..].find('}').map(|i| start + i).unwrap()];

        block
            .lines()
            .filter_map(|line| {
                let line = line.split("/*").next().unwrap_or("");
                let (key, value) = line.split_once(':')?;
                let key = key.trim().trim_start_matches("--");
                if key.is_empty() {
                    return None;
                }
                Some((
                    key.to_string(),
                    value.trim().trim_end_matches(';').trim().to_string(),
                ))
            })
            .collect()
    }

    /// The desktop palette must equal the web palette.
    ///
    /// Catches the failure mode this arrangement invites: someone tunes a
    /// colour in `style.css`, ships it to the site, and never regenerates the
    /// JSON — so the app drifts from the design it is supposed to share. The
    /// generator's output is checked here rather than trusted.
    #[test]
    fn every_borrow_token_comes_from_the_stylesheet() {
        let css = css_tokens();

        // Left column is the token as it appears in the generated file, right
        // column the custom property it must carry through.
        let carried: &[(&str, &str)] = &[
            ("background", "background"),
            ("foreground", "text"),
            ("border", "border"),
            ("ring", "focus"),
            ("caret", "text"),
            ("primary.background", "accent"),
            ("success.background", "success"),
            ("info.background", "info"),
            ("warning.background", "accent-secondary"),
            ("muted.foreground", "text-muted"),
            ("input.border", "border-strong"),
        ];

        let mut checked = 0;
        for (json_key, css_key) in carried {
            let expected = css
                .get(*css_key)
                .unwrap_or_else(|| panic!("style.css must define --{css_key}"));
            let actual = json_token(json_key);
            assert_eq!(
                actual.as_deref().map(str::to_lowercase),
                Some(expected.to_lowercase()),
                "--{css_key} is {expected} in style.css but {json_key} is {actual:?} in the desktop theme"
            );
            checked += 1;
        }

        assert!(checked >= 10, "checked {checked} tokens");
    }

    /// Reads a colour straight out of the generated file.
    ///
    /// The raw JSON rather than the deserialized struct, because serde renames
    /// these fields on the way out too (`primary` <-> `primary.background`).
    /// Asserting against the wrong spelling reports every token missing while
    /// the app itself renders perfectly.
    fn json_token(key: &str) -> Option<String> {
        let value: serde_json::Value = serde_json::from_str(BORROW_THEME).ok()?;
        value
            .get("themes")?
            .as_array()?
            .iter()
            .find(|t| t.get("name") == Some(&serde_json::Value::String("BORROW_".into())))?
            .get("colors")?
            .get(key)?
            .as_str()
            .map(str::to_string)
    }

    /// The radii are design tokens, not gpui-component's defaults —
    /// BORROW_ is a 4/6/8 scale where shadcn is 6/8.
    #[test]
    fn the_radius_scale_is_borrows() {
        let css = css_tokens();
        assert_eq!(css.get("radius-md"), Some(&"6px".to_string()));

        let set: gpui_kit::component::ThemeSet =
            serde_json::from_str(BORROW_THEME).expect("borrows theme must parse");
        let theme = set.themes.iter().find(|t| t.name == "BORROW_").unwrap();

        assert_eq!(theme.radius, Some(4));
        assert_eq!(theme.radius_lg, Some(8));
    }
}
