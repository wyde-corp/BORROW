use super::layout::escape;
use crate::models::CodeStatus;

pub fn nav_link(label: &str, href: &str, active: bool) -> String {
    let class = if active { " class=\"active\"" } else { "" };
    format!(r##"<a href="{href}"{class}>{label}</a>"##)
}

pub fn escape_html(text: &str) -> String {
    escape(text)
}

pub fn progress_bar(percent: i64, large: bool) -> String {
    let clamped = percent.clamp(0, 100);
    let size = if large { " progress-bar-large" } else { "" };

    format!(
        r##"<div class="progress-bar{size}" role="progressbar" aria-valuenow="{clamped}" aria-valuemin="0" aria-valuemax="100"><div class="progress-fill" style="width:{clamped}%"></div></div>"##,
        size = size,
        clamped = clamped
    )
}

pub fn segmented_progress(percent: i64, segments: usize) -> String {
    let clamped = percent.clamp(0, 100);
    let filled = ((clamped * segments as i64) as f64 / 100.0).round() as usize;

    let cells: String = (0..segments)
        .map(|i| {
            let class = if i < filled { "segment filled" } else { "segment" };
            format!(r##"<span class="{class}"></span>"##)
        })
        .collect();

    format!(r##"<div class="segmented-progress" role="progressbar" aria-valuenow="{clamped}" aria-valuemin="0" aria-valuemax="100">{cells}</div>"##, clamped = clamped, cells = cells)
}

pub fn code_block(language: &str, code: &str, copyable: bool) -> String {
    code_panel(language, code, copyable, None)
}

/// A code panel that is deliberately invalid, flagged so the learner is not confused.
pub fn broken_code_block(language: &str, code: &str) -> String {
    code_panel(language, code, true, Some(CodeStatus::Broken))
}

/// The corrected version of a panel previously marked broken.
pub fn corrected_code_block(language: &str, code: &str) -> String {
    code_panel(language, code, true, Some(CodeStatus::Corrected))
}

fn code_panel(language: &str, code: &str, copyable: bool, status: Option<CodeStatus>) -> String {
    let flag = match status {
        Some(CodeStatus::Broken) => {
            r##"<span class="code-flag flag-broken">✗ THIS DOES NOT COMPILE</span>"##
        }
        Some(CodeStatus::Corrected) => {
            r##"<span class="code-flag flag-corrected">✓ CORRECTED</span>"##
        }
        _ => "",
    };

    let class = match status {
        Some(CodeStatus::Broken) => "code-block broken",
        Some(CodeStatus::Corrected) => "code-block corrected",
        Some(CodeStatus::Valid) | None => "code-block",
    };

    let copy_button = if copyable {
        r##"<button type="button" class="copy-btn" data-copy>COPY</button>"##
    } else {
        ""
    };

    // Every Rust panel is runnable. The snippet travels to the browser as a
    // JSON value in a data attribute rather than being re-parsed out of the
    // rendered `<code>`, which would fight the HTML escaping and the
    // whitespace the gutter depends on.
    let try_this = if language.eq_ignore_ascii_case("rust") {
        format!(
            r##"<button type="button" class="try-this" data-try-this="{}">TRY THIS</button>"##,
            snippet_attribute(code)
        )
    } else {
        String::new()
    };

    format!(
        r##"<div class="{class}">
        <div class="code-header"><span class="code-lang">{lang}</span>{flag}<span class="code-header-actions">{copy_button}{try_this}</span></div>
        <pre><code>{code}</code></pre>
    </div>"##,
        class = class,
        lang = escape(language).to_uppercase(),
        flag = flag,
        copy_button = copy_button,
        try_this = try_this,
        code = escape_html(code)
    )
}

/// Source on the left, per-line explanation on the right.
pub fn breakdown(title: &str, code: &str, items: &[String]) -> String {
    let lines: Vec<&str> = code.lines().collect();

    let numbered: String = lines
        .iter()
        .enumerate()
        .map(|(i, l)| {
            format!(
                r##"<span class="bd-code-line"><span class="bd-num">{}</span>{}</span>"##,
                i + 1,
                escape_html(l).replace('\t', "    ")
            )
        })
        .collect();

    let notes: String = items
        .iter()
        .map(|it| format!(r##"<li>{}</li>"##, escape(it)))
        .collect();

    format!(
        r##"<figure class="breakdown">
        <figcaption>{title}</figcaption>
        <div class="bd-body">
            <pre class="bd-code">{code}</pre>
            <ul class="bd-notes">{notes}</ul>
        </div>
    </figure>"##,
        title = escape(title),
        code = numbered,
        notes = notes
    )
}

/// A ticked list of achieved abilities.
pub fn checklist(title: &str, items: &[String]) -> String {
    let rows: String = items
        .iter()
        .map(|it| {
            format!(
                r##"<li><span class="check" aria-hidden="true">✓</span><span>{}</span></li>"##,
                escape(it)
            )
        })
        .collect();

    format!(
        r##"<section class="checklist-block">
        <h3 class="section-heading">{title}</h3>
        <ul class="checklist">{rows}</ul>
    </section>"##,
        title = escape(title),
        rows = rows
    )
}

/// A monospace panel for ASCII diagrams and visualisations.
pub fn diagram(title: &str, content: &str) -> String {
    format!(
        r##"<figure class="diagram">
        <figcaption>{title}</figcaption>
        <pre>{content}</pre>
    </figure>"##,
        title = escape(title),
        content = escape_html(content)
    )
}

/// A data table with a header row.
pub fn table(headers: &[String], rows: &[Vec<String>]) -> String {
    let head: String = headers
        .iter()
        .map(|h| format!("<th>{}</th>", escape(h)))
        .collect();

    let body: String = rows
        .iter()
        .map(|r| {
            let cells: String = r
                .iter()
                .map(|c| {
                    let cls = if c == "→" || c == "✓" { " class=\"tick\"" } else { "" };
                    format!("<td{cls}>{}</td>", escape(c))
                })
                .collect();
            format!("<tr>{cells}</tr>")
        })
        .collect();

    format!(
        r##"<div class="table-wrap"><table><thead><tr>{head}</tr></thead><tbody>{body}</tbody></table></div>"##,
        head = head,
        body = body
    )
}

/// An ordered list of instructions or explanation steps.
pub fn steps_block(items: &[String]) -> String {
    let list: String = items
        .iter()
        .map(|i| format!("<li>{}</li>", escape(i)))
        .collect();

    format!(r##"<ol class="steps">{list}</ol>"##, list = list)
}

pub fn callout(title: &str, text: &str, tone: CalloutTone) -> String {
    let tone_class = match tone {
        CalloutTone::Idea => "callout",
        CalloutTone::Practice => "callout callout-practice",
        CalloutTone::Challenge => "callout callout-challenge",
        CalloutTone::Project => "callout callout-project",
        CalloutTone::Error => "callout callout-error",
    };

    format!(
        r##"<aside class="{tone}"><span class="callout-title">{title}</span><p>{text}</p></aside>"##,
        tone = tone_class,
        title = escape(title),
        text = escape_html(text)
    )
}

pub enum CalloutTone {
    Idea,
    Practice,
    Challenge,
    Project,
    Error,
}

pub fn status_icon(state: StatusState) -> &'static str {
    match state {
        StatusState::Locked => r##"<span class="status-icon locked" aria-label="Locked">○</span>"##,
        StatusState::Available => r##"<span class="status-icon available" aria-label="Available">○</span>"##,
        StatusState::InProgress => r##"<span class="status-icon progress" aria-label="In progress">●</span>"##,
        StatusState::Completed => r##"<span class="status-icon done" aria-label="Completed">✓</span>"##,
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum StatusState {
    Locked,
    Available,
    InProgress,
    Completed,
}

/// A code snippet as an HTML attribute value, safe to hand to `JSON.parse`.
///
/// Two separate encodings, and the order matters. `serde_json` produces valid
/// JSON but escapes quotes as `\"`, which is not valid inside a
/// double-quoted HTML attribute — the browser ends the value at the first quote
/// and the JSON is left truncated. So the JSON is produced first, then escaped
/// for the attribute; escaping first would turn the JSON's own backslashes into
/// literal backslashes and corrupt the string.
fn snippet_attribute(code: &str) -> String {
    let json = serde_json::to_string(code).unwrap_or_else(|_| "\"\"".to_string());

    escape(&json)
}

#[cfg(test)]
mod snippet_tests {
    use super::snippet_attribute;

    /// A Rust program is full of quotes, and a lesson panel is pasted verbatim
    /// into an attribute. If the encoding is wrong the browser ends the
    /// attribute at the first quote and `JSON.parse` throws on an empty string
    /// — which is exactly what a real lesson panel does.
    #[test]
    fn a_program_with_quotes_survives_the_attribute_encoding() {
        let code = r#"fn main() { println!("hello"); }"#;
        let attr = snippet_attribute(code);

        // The property that matters is the round trip, through the same two
        // steps the browser performs: decode the attribute, then parse the JSON.
        let decoded: String = serde_json::from_str(&unescape(&attr)).expect("round trips");
        assert_eq!(decoded, code);
    }

    #[test]
    fn a_multiline_program_round_trips() {
        let code = r#"fn main() {
    let s1 = String::from("hi");
    println!("{s1}");
}
"#;
        let decoded: String = serde_json::from_str(&unescape(&snippet_attribute(code))).unwrap();
        assert_eq!(decoded, code);
    }

    /// `&` and `<` have to be escaped too. A raw `<` in an attribute is a
    /// parse error, and a raw `&` is quietly decoded, so both corrupt the value
    /// even though they look harmless.
    #[test]
    fn markup_characters_are_escaped() {
        let code = r#"fn main() { let v = vec![1, 2]; if v.len() > 1 && v[0] < v[1] { } }"#;
        let attr = snippet_attribute(code);

        assert!(!attr.contains('<'), "raw < in an attribute: {attr}");
        assert!(!attr.contains('>'), "raw > in an attribute: {attr}");

        let decoded: String = serde_json::from_str(&unescape(&attr)).unwrap();
        assert_eq!(decoded, code);
    }

    /// The inverse of `escape`, standing in for what the browser does when it
    /// parses the attribute. Only needs to handle what `escape` produces.
    fn unescape(text: &str) -> String {
        text.replace("&quot;", "\"")
            .replace("&gt;", ">")
            .replace("&lt;", "<")
            .replace("&amp;", "&")
    }
}
