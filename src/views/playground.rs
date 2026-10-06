//! The standalone playground page, and the editor markup shared with the
//! TRY THIS dialog.
//!
//! There is one editor per page, and it is the same editor either way — a lesson
//! opens it in a modal over the panel you clicked, `/playground` renders it as
//! the page itself. Keeping the interior in one function is what stops the two
//! from drifting apart, and it is why `editor.js` can look up its controls by
//! `data-pg-*` without caring which wrapper it is in.

use super::layout::{escape, page_without_playground};

/// The editor itself: header, gutter, textarea, RUN, console.
///
/// `close` is false on the standalone page, where there is nothing to close.
pub fn editor_panel(title: &str, close: bool) -> String {
    let close_button = if close {
        r##"<button type="button" class="playground-close" data-pg-close aria-label="Close the playground">✕</button>"##
    } else {
        ""
    };

    format!(
        r##"<div class="playground-panel">
        <header class="playground-header">
            <h2 id="playground-title" class="playground-title">{title}</h2>
            <div class="playground-header-actions">
                <button type="button" class="btn btn-ghost playground-reset" data-pg-reset>RESET</button>
                {close_button}
            </div>
        </header>

        <div class="playground-body">
            <div class="editor-shell">
                <div class="editor-gutter" data-pg-gutter aria-hidden="true"></div>
                <label class="visually-hidden" for="playground-code">Your Rust program</label>
                <textarea id="playground-code" class="editor-input" data-pg-input
                    spellcheck="false" autocomplete="off" autocapitalize="off"
                    autocorrect="off" wrap="off"
                    aria-describedby="playground-help"></textarea>
            </div>

            <div class="playground-actions">
                <button type="button" class="btn btn-primary run-btn" data-pg-run>
                    <span class="run-btn-label" data-pg-run-label>RUN</span>
                </button>
                <span class="playground-hint" id="playground-help">⌘/CTRL + ENTER</span>
                <span class="playground-status" data-pg-status role="status" aria-live="polite"></span>
            </div>

            <section class="console" data-pg-console aria-label="Output and errors">
                <div class="console-empty" data-pg-empty>
                    <span class="console-empty-mark">&gt;_</span>
                    <p>Nothing has run yet. Press RUN to compile and execute this program.</p>
                </div>
                <div class="console-body" data-pg-results hidden></div>
            </section>
        </div>
    </div>"##,
        title = escape(title),
        close_button = close_button,
    )
}

/// The modal wrapper, rendered into every page so a lesson's TRY THIS button has
/// somewhere to open.
pub fn dialog() -> String {
    format!(
        r##"<dialog class="playground" data-playground id="playground" aria-labelledby="playground-title">{panel}</dialog>"##,
        panel = editor_panel("TRY THIS", true)
    )
}

/// The `/playground` page.
///
/// When the server is not on loopback the playground is refused at the endpoint
/// too, but saying so here means a visitor understands why before they type
/// anything and press RUN into a 403.
pub fn show(enabled: bool) -> String {
    let notice = if enabled {
        r##"<p class="playground-page-intro">Write Rust and run it right here. Every code panel in a lesson also has a <strong>TRY THIS</strong> button that opens this editor with that example loaded. Your work is saved in this browser.</p>"##
            .to_string()
    } else {
        r##"<div class="playground-disabled">
            <span class="playground-disabled-label">PLAYGROUND DISABLED</span>
            <p>This server is not bound to localhost, so running learner code would let anyone who can reach it execute programs with the server's privileges. The editor still works, but RUN is turned off. See the note on <code>BORROW_ALLOW_PLAYGROUND</code> if you intend to expose it.</p>
        </div>"##
            .to_string()
    };

    let body = format!(
        r##"<header class="playground-page-header">
    <span class="module-label">PLAYGROUND</span>
    <h1>Run Rust</h1>
</header>

{notice}

<section class="playground-inline" data-playground aria-labelledby="playground-title"{disabled}>
    {panel}
</section>"##,
        notice = notice,
        disabled = if enabled { "" } else { " data-pg-disabled=\"true\"" },
        panel = editor_panel("PLAYGROUND", false)
    );

    page_without_playground("playground", "Playground", &body)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The page must not carry a second copy of the editor. The global dialog is
    /// rendered on every other page, so if it appeared here too there would be
    /// two elements with `id="playground-code"` and `editor.js` — which queries
    /// the first match — could wire up the wrong one.
    #[test]
    fn the_playground_page_has_exactly_one_editor() {
        let page = show(true);

        assert_eq!(page.matches("id=\"playground-code\"").count(), 1);
        assert_eq!(page.matches("data-playground").count(), 1);
        assert!(
            !page.contains("<dialog"),
            "the modal wrapper should not be on its own page"
        );
    }

    /// The dialog variant is the one every other page embeds.
    #[test]
    fn the_dialog_is_a_single_modal_editor() {
        let dialog = dialog();

        assert_eq!(dialog.matches("id=\"playground-code\"").count(), 1);
        assert!(dialog.contains("<dialog"));
        assert!(dialog.contains("data-pg-close"), "a modal needs a way out");
    }

    #[test]
    fn the_page_reports_a_disabled_playground() {
        let page = show(false);

        assert!(page.contains("PLAYGROUND DISABLED"));
        assert!(page.contains("data-pg-disabled=\"true\""));
    }
}
