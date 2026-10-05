use crate::views::components::nav_link;

// The practice and projects links are anchors rather than pages. `/practice`
// takes you to module 1's run builder, which is the entry point to every
// module's questions; previously this pointed at `/dashboard#practice`, an
// anchor that only ever existed on a module page, so the nav link went nowhere.
pub const NAV_ITEMS: [(&str, &str, &str); 3] = [
    ("Learn", "/dashboard", "dashboard"),
    ("Practice", "/practice/1", "practice"),
    ("Projects", "/module/1#projects", "projects"),
];

pub fn render_page(title: &str, content: &str) -> String {
    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>BORROW_ — {title}</title>
    <meta name="description" content="Learn Rust fundamentals with short lessons, real practice, and small projects.">
    <link rel="stylesheet" href="/static/style.css">
    <script src="/static/app.js" defer></script>
    <script src="/static/editor.js" defer></script>
    <script src="/static/interactive.js" defer></script>
</head>
<body>
{content}
{playground}
</body>
</html>"##,
        title = escape(title),
        content = content,
        playground = playground_dialog()
    )
}

pub fn page(active: &str, title: &str, body: &str) -> String {
    let content = format!(
        "{nav}\n    <main class=\"content\">\n        {body}\n    </main>",
        nav = navbar(active),
        body = body
    );
    render_page(title, &content)
}

pub fn navbar(active: &str) -> String {
    let links: String = NAV_ITEMS
        .iter()
        .map(|(label, href, key)| nav_link(label, href, *key == active))
        .collect();

    format!(
        r##"<nav class="navbar">
        <a href="/" class="logo">BORROW_</a>
        <div class="nav-links">{links}</div>
    </nav>"##,
        links = links
    )
}

pub fn page_with_progress(active: &str, title: &str, body: &str, percent: i64, xp: i64) -> String {
    let content = format!(
        "{nav}\n    <main class=\"content\">\n        {body}\n    </main>",
        nav = navbar_with_progress(active, percent, xp),
        body = body
    );
    render_page(title, &content)
}

pub fn navbar_with_progress(active: &str, percent: i64, xp: i64) -> String {
    let mut nav = navbar(active);

    let progress = format!(
        r##"<div class="nav-progress"><span>{pct}% · {xp} XP</span><div class="progress-bar"><div class="progress-fill" style="width:{pct}%"></div></div></div>"##,
        pct = percent.clamp(0, 100),
        xp = xp
    );

    nav = nav.replace("</nav>", &format!("{progress}</nav>"));
    nav
}

pub fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// The playground, rendered once per page.
///
/// It lives in the layout rather than in a lesson view because the editor has to
/// survive navigation within a lesson — clicking TRY THIS on panel 6, reading
/// the error, scrolling to panel 2 — and re-rendering a server-side editor on
/// every step would throw away the learner's edits. A single dialog shared by
/// every panel is also why `editor.js` can assume exactly one instance exists.
pub fn playground_dialog() -> String {
    r##"<dialog class="playground" id="playground" aria-labelledby="playground-title">
    <div class="playground-panel">
        <header class="playground-header">
            <h2 id="playground-title" class="playground-title">TRY THIS</h2>
            <div class="playground-header-actions">
                <button type="button" class="btn btn-ghost playground-reset" data-pg-reset>RESET</button>
                <button type="button" class="playground-close" data-pg-close aria-label="Close the playground">✕</button>
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
    </div>
</dialog>"##
    .to_string()
}
