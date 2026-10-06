use crate::views::components::nav_link;

// The practice and projects links are anchors rather than pages. `/practice`
// takes you to module 1's run builder, which is the entry point to every
// module's questions; previously this pointed at `/dashboard#practice`, an
// anchor that only ever existed on a module page, so the nav link went nowhere.
pub const NAV_ITEMS: [(&str, &str, &str); 4] = [
    ("Learn", "/dashboard", "dashboard"),
    ("Practice", "/practice/1", "practice"),
    ("Projects", "/module/1#projects", "projects"),
    ("Playground", "/playground", "playground"),
];

pub fn render_page(title: &str, content: &str) -> String {
    render_page_inner(title, content, true)
}

/// Same as [`render_page`], minus the global TRY THIS dialog.
///
/// `/playground` renders the editor as its own page content, so it must not also
/// get the modal copy — two elements would share `id="playground-code"` and the
/// editor script, which looks the input up by id, would bind to whichever came
/// first in the document.
pub fn render_page_without_playground(title: &str, content: &str) -> String {
    render_page_inner(title, content, false)
}

fn render_page_inner(title: &str, content: &str, with_playground: bool) -> String {
    let playground = if with_playground {
        super::playground::dialog()
    } else {
        String::new()
    };

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
        playground = playground
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

/// Like [`page`], but for a screen that owns the whole viewport's editor and so
/// must not also carry the modal copy of it.
pub fn page_without_playground(active: &str, title: &str, body: &str) -> String {
    let content = format!(
        "{nav}\n    <main class=\"content\">\n        {body}\n    </main>",
        nav = navbar(active),
        body = body
    );
    render_page_without_playground(title, &content)
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

