use super::layout::escape;
use crate::models::{CodeStatus, DispatchStep, GapBlank, PipelineStep, Step};
use std::sync::atomic::{AtomicUsize, Ordering};

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

/// Any serializable value as an escaped JSON attribute value. Same reasoning as
/// [`snippet_attribute`], generalised for the interactive widgets whose state is
/// a small object rather than a bare string.
fn json_attribute<T: serde::Serialize + ?Sized>(value: &T) -> String {
    let json = serde_json::to_string(value).unwrap_or_else(|_| "null".to_string());

    escape(&json)
}

/// Hands out unique radio-group names. Two `guess_run` widgets on one page must
/// not share a radio group, or selecting an option in one clears the other.
static WIDGET_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// A code panel with inline inputs, one per `____` marker, checked per blank.
pub fn gap_fill_block(
    intro: &str,
    language: &str,
    code: &str,
    blanks: &[GapBlank],
    after: &str,
) -> String {
    let parts: Vec<&str> = code.split("____").collect();
    let mut body = String::new();

    for (i, part) in parts.iter().enumerate() {
        body.push_str(&escape_html(part));

        if i + 1 == parts.len() {
            continue;
        }

        match blanks.get(i) {
            Some(blank) => {
                // Size the field to its answer so the gap reads as "this long",
                // rather than a uniform box that hides how much is missing.
                let width = blank.answer.chars().count().max(6);
                body.push_str(&format!(
                    r##"<input class="gap-input" type="text" data-gap data-blank-id="{id}" data-answer="{answer}" data-hint="{hint}" size="{width}" autocomplete="off" autocapitalize="off" spellcheck="false" aria-label="blank {n}">"##,
                    id = escape(&blank.id),
                    answer = escape(&blank.answer),
                    hint = escape(&blank.hint),
                    width = width,
                    n = i + 1
                ));
            }
            None => body.push_str("____"),
        }
    }

    let answers: String = blanks
        .iter()
        .map(|b| {
            let hint = if b.hint.trim().is_empty() {
                String::new()
            } else {
                format!(" — {}", escape(&b.hint))
            };
            format!("<li><code>{}</code>{}</li>", escape(&b.answer), hint)
        })
        .collect();

    let after = if after.trim().is_empty() {
        String::new()
    } else {
        format!(
            r##"<p class="widget-after">{}</p>"##,
            escape_html(after)
        )
    };

    format!(
        r##"<figure class="widget gap-fill-widget" data-gap-fill>
    <figcaption class="widget-intro">{intro}</figcaption>
    <div class="code-block widget-code">
        <div class="code-header"><span class="code-lang">{lang}</span><span class="widget-badge">FILL THE GAPS</span></div>
        <pre><code>{body}</code></pre>
    </div>
    <div class="widget-actions">
        <button type="button" class="btn btn-primary" data-gap-check>CHECK</button>
        <button type="button" class="btn btn-ghost" data-gap-reset>RESET</button>
        <span class="widget-status" data-gap-status role="status" aria-live="polite"></span>
    </div>
    <details class="widget-fallback">
        <summary>Show answers</summary>
        <ul class="gap-answers">{answers}</ul>
    </details>
    {after}
</figure>"##,
        intro = escape(intro),
        lang = escape(language).to_uppercase(),
        body = body,
        answers = answers,
        after = after
    )
}

/// A snippet stepped through one line-group at a time.
pub fn step_through_block(
    title: &str,
    language: &str,
    code: &str,
    steps: &[Step],
) -> String {
    let lines: String = code
        .lines()
        .enumerate()
        .map(|(i, line)| {
            format!(
                r##"<span class="st-line" data-line="{}">{}</span>"##,
                i + 1,
                escape_html(line)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let fallback: String = steps
        .iter()
        .map(|s| {
            let refs: String = s
                .lines
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                r##"<li><span class="st-ref">line {}</span> {}</li>"##,
                escape(&refs),
                escape(&s.note)
            )
        })
        .collect();

    format!(
        r##"<figure class="widget step-through" data-step-through data-steps="{steps}">
    <figcaption class="widget-title">{title}</figcaption>
    <div class="code-block widget-code">
        <div class="code-header"><span class="code-lang">{lang}</span><span class="widget-badge">STEP THROUGH</span></div>
        <pre><code>{lines}</code></pre>
    </div>
    <div class="st-controls" hidden>
        <button type="button" class="btn btn-ghost" data-st-prev aria-label="Previous step">← PREV</button>
        <span class="st-counter" data-st-counter role="status" aria-live="polite"></span>
        <button type="button" class="btn btn-ghost" data-st-next aria-label="Next step">NEXT →</button>
    </div>
    <p class="st-note" data-st-note hidden></p>
    <ol class="st-fallback">{fallback}</ol>
</figure>"##,
        title = escape(title),
        lang = escape(language).to_uppercase(),
        lines = lines,
        steps = json_attribute(steps),
        fallback = fallback
    )
}

/// Predict what a program does, then run it and compare.
pub fn guess_run_block(
    prompt: &str,
    language: &str,
    code: &str,
    options: &[String],
    correct: usize,
    explanation: &str,
) -> String {
    let group = format!("guess-{}", WIDGET_COUNTER.fetch_add(1, Ordering::Relaxed));

    let rows: String = options
        .iter()
        .enumerate()
        .map(|(i, text)| {
            format!(
                r##"<label class="option guess-option">
    <input type="radio" name="{group}" value="{i}">
    <span class="option-text">{text}</span>
</label>"##,
                group = group,
                i = i,
                text = escape(text)
            )
        })
        .collect();

    let config = json_attribute(&serde_json::json!({
        "code": code,
        "correct": correct,
        "explanation": explanation,
    }));

    format!(
        r##"<figure class="widget guess-run" data-guess-run data-config="{config}">
    <figcaption class="widget-title">{prompt}</figcaption>
    <div class="code-block widget-code">
        <div class="code-header"><span class="code-lang">{lang}</span><span class="widget-badge">GUESS, THEN RUN</span></div>
        <pre><code>{code}</code></pre>
    </div>
    <fieldset class="options guess-options"><legend>What will happen?</legend>{rows}</fieldset>
    <div class="widget-actions">
        <button type="button" class="btn btn-primary" data-guess-submit disabled>GUESS, THEN RUN</button>
        <span class="widget-status" data-guess-status role="status" aria-live="polite"></span>
    </div>
    <div class="guess-result console" data-guess-result role="status" hidden></div>
    <details class="widget-fallback">
        <summary>Reveal the answer</summary>
        <p>{explanation}</p>
    </details>
</figure>"##,
        prompt = escape(prompt),
        lang = escape(language).to_uppercase(),
        code = escape_html(code),
        rows = rows,
        explanation = escape(explanation),
        config = config
    )
}

/// Static versus dynamic dispatch over the same call, advanced in tandem.
///
/// Two columns narrate the same steps under each dispatch form; the learner
/// steps both at once and sees where they diverge. The columns are rendered
/// server-side, so with no script the fallback list still tells the whole
/// story.
pub fn dispatch_block(
    title: &str,
    language: &str,
    code: &str,
    steps: &[DispatchStep],
) -> String {
    let lines: String = code
        .lines()
        .enumerate()
        .map(|(i, line)| {
            format!(
                r##"<span class="dsp-line" data-line="{}">{}</span>"##,
                i + 1,
                escape_html(line)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let fallback: String = steps
        .iter()
        .map(|s| {
            let refs: String = s
                .lines
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                r##"<li>
    <span class="dsp-ref">line {refs}</span>
    <p class="dsp-fb dsp-fb-static"><strong>static:</strong> {static_note}</p>
    <p class="dsp-fb dsp-fb-dynamic"><strong>dynamic:</strong> {dynamic_note}</p>
</li>"##,
                refs = escape(&refs),
                static_note = escape(&s.static_note),
                dynamic_note = escape(&s.dynamic_note)
            )
        })
        .collect();

    format!(
        r##"<figure class="widget dispatch" data-dispatch data-steps="{steps}">
    <figcaption class="widget-title">{title}</figcaption>
    <div class="code-block widget-code">
        <div class="code-header"><span class="code-lang">{lang}</span><span class="widget-badge">STATIC vs DYNAMIC</span></div>
        <pre><code>{lines}</code></pre>
    </div>
    <div class="dsp-controls" hidden>
        <button type="button" class="btn btn-ghost" data-dsp-prev aria-label="Previous step">← PREV</button>
        <span class="dsp-counter" data-dsp-counter role="status" aria-live="polite"></span>
        <button type="button" class="btn btn-ghost" data-dsp-next aria-label="Next step">NEXT →</button>
    </div>
    <div class="dsp-columns">
        <div class="dsp-col dsp-static">
            <h4 class="dsp-heading">STATIC DISPATCH · generics</h4>
            <p class="dsp-note" data-dsp-static></p>
        </div>
        <div class="dsp-col dsp-dynamic">
            <h4 class="dsp-heading">DYNAMIC DISPATCH · dyn Trait</h4>
            <p class="dsp-note" data-dsp-dynamic></p>
        </div>
    </div>
    <ol class="dsp-fallback">{fallback}</ol>
</figure>"##,
        title = escape(title),
        lang = escape(language).to_uppercase(),
        lines = lines,
        steps = json_attribute(steps),
        fallback = fallback
    )
}

/// A pipeline of adapters, shown first as lazy and then driven by a consumer.
///
/// The phase badge (LAZY/EAGER) is the whole lesson: the adapters are described
/// first and do nothing, until a consumer pulls elements through. With no
/// script the fallback list carries the same two phases.
pub fn pipeline_block(
    title: &str,
    language: &str,
    code: &str,
    steps: &[PipelineStep],
) -> String {
    let lines: String = code
        .lines()
        .enumerate()
        .map(|(i, line)| {
            format!(
                r##"<span class="pipe-line" data-line="{}">{}</span>"##,
                i + 1,
                escape_html(line)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let fallback: String = steps
        .iter()
        .map(|s| {
            let refs: String = s
                .lines
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                r##"<li>
    <span class="pipe-phase-tag pipe-tag-{phase}">{phase}</span>
    <span class="pipe-ref">line {refs}</span>
    <p class="pipe-fb">{note}</p>
</li>"##,
                phase = escape(&s.phase),
                refs = escape(&refs),
                note = escape(&s.note)
            )
        })
        .collect();

    format!(
        r##"<figure class="widget pipeline" data-pipeline data-steps="{steps}">
    <figcaption class="widget-title">{title}</figcaption>
    <div class="code-block widget-code">
        <div class="code-header"><span class="code-lang">{lang}</span><span class="widget-badge">LAZY → EAGER</span></div>
        <pre><code>{lines}</code></pre>
    </div>
    <div class="pipe-controls" hidden>
        <span class="pipe-phase" data-pipe-phase>LAZY</span>
        <button type="button" class="btn btn-ghost" data-pipe-prev aria-label="Previous step">← PREV</button>
        <span class="pipe-counter" data-pipe-counter role="status" aria-live="polite"></span>
        <button type="button" class="btn btn-ghost" data-pipe-next aria-label="Next step">NEXT →</button>
    </div>
    <p class="pipe-note" data-pipe-note hidden></p>
    <ol class="pipe-fallback">{fallback}</ol>
</figure>"##,
        title = escape(title),
        lang = escape(language).to_uppercase(),
        lines = lines,
        steps = json_attribute(steps),
        fallback = fallback
    )
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
