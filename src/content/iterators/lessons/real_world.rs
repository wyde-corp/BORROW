//! Module 11 — Iterators, lessons 26–27.
//!
//! The Log Analyzer, assembled, and its three core questions answered with
//! pipelines.

use crate::content::types::{callout, code, labelled, paragraphs, pipeline, table, SeedLesson};

pub fn lessons() -> Vec<SeedLesson> {
    vec![assembled(), filtering_counting_top_n()]
}

fn assembled() -> SeedLesson {
    SeedLesson::new(
        "The Log Analyzer, Assembled",
        vec![
            labelled(
                "CONCEPT",
                "Here is the whole analyzer. Raw text is split into lines and parsed into entries; the \
                 entries are filtered, mapped, counted, and grouped — every question answered by an \
                 iterator rather than a hand-written index loop.",
            ),
            code(
                "rust",
                r#"use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum Level {
    Error,
    Warn,
    Info,
}

#[derive(Debug, Clone, PartialEq)]
struct LogEntry {
    level: Level,
    source: String,
    message: String,
}

#[derive(Debug)]
struct ParseError(String);

fn parse_line(line: &str) -> Result<LogEntry, ParseError> {
    let mut parts = line.splitn(3, ' ');

    let level = match parts.next() {
        Some("ERROR") => Level::Error,
        Some("WARN") => Level::Warn,
        Some("INFO") => Level::Info,
        _ => return Err(ParseError(line.to_string())),
    };

    let source = parts.next().ok_or_else(|| ParseError(line.to_string()))?;
    let message = parts.next().ok_or_else(|| ParseError(line.to_string()))?;

    Ok(LogEntry {
        level,
        source: source.to_string(),
        message: message.to_string(),
    })
}

fn main() {
    let raw = "ERROR db connection refused\nINFO cache warm\nWARN db slow query\nERROR db timeout";

    let entries: Vec<LogEntry> = raw
        .lines()
        .filter_map(|line| parse_line(line).ok())
        .collect();

    let errors: Vec<&str> = entries
        .iter()
        .filter(|e| e.level == Level::Error)
        .map(|e| e.message.as_str())
        .collect();
    println!("errors: {errors:?}");

    let mut by_level: HashMap<&str, usize> = HashMap::new();
    for entry in &entries {
        let key = match entry.level {
            Level::Error => "error",
            Level::Warn => "warn",
            Level::Info => "info",
        };
        *by_level.entry(key).or_insert(0) += 1;
    }
    println!("by level: {by_level:?}");

    let mut source_counts: HashMap<&str, usize> = HashMap::new();
    for entry in &entries {
        *source_counts.entry(entry.source.as_str()).or_insert(0) += 1;
    }

    if let Some((source, count)) = source_counts.iter().max_by_key(|(_, n)| **n) {
        println!("noisiest source: {source} ({count})");
    }
}"#,
            ),
            labelled(
                "EXPLANATION",
                "Trace the pipeline, not the lines. `raw.lines()` is an iterator; `.filter_map(parse)` \
                 parses each line and drops the failures; `.collect()` runs it into a `Vec<LogEntry>`. \
                 Then `.filter(...).map(...).collect()` answers the first question. The two counting \
                 loops use a `HashMap` and the `entry` API from Module 08 — and it is honest that they \
                 are loops, because grouping is clearer as one.",
            ),
            table(
                &["Question", "Shape", "Kind"],
                &[
                    &["which entries are errors?", "`filter`", "adapter"],
                    &["what messages did they carry?", "`map`", "adapter"],
                    &["how many of each level?", "a `HashMap` count", "explicit loop"],
                    &["which source logs most?", "`max_by_key`", "consumer"],
                ],
            ),
            callout(
                "Iterators where they help, loops where they do not",
                "The final design is not 'all chains'. Filtering and mapping read best as adapters; \
                 grouping into a `HashMap` reads best as a loop. Choosing is the skill this module has \
                 been building toward.",
            ),
        ],
    )
}

fn filtering_counting_top_n() -> SeedLesson {
    SeedLesson::new(
        "Filtering, Counting, Top N",
        vec![
            labelled(
                "CONCEPT",
                "The three questions the analyzer exists to answer, each as a pipeline. Notice that the \
                 chain describes the work and the consumer runs it.",
            ),
            pipeline(
                "Nothing runs until collect",
                r#"let errors: Vec<&LogEntry> = entries
    .iter()
    .filter(|e| e.level == Level::Error)
    .collect();"#,
                &[
                    (
                        &[3][..],
                        "lazy",
                        "`filter` is set up. No entry has been examined.",
                    ),
                    (
                        &[4][..],
                        "eager",
                        "`collect` asks for the first item; the filter begins checking entries.",
                    ),
                    (
                        &[4][..],
                        "eager",
                        "It keeps the errors and drops the rest, until the source runs out.",
                    ),
                ],
            ),
            code(
                "rust",
                r#"fn main() {
    let numbers = vec![4, 9, 2, 7, 5];

    // Count the numbers greater than three.
    let big = numbers.iter().filter(|n| **n > 3).count();
    println!("{big} big numbers");

    // The two largest, in order.
    let mut sorted = numbers.clone();
    sorted.sort_unstable_by(|a, b| b.cmp(a));
    let top_two: Vec<i32> = sorted.into_iter().take(2).collect();
    println!("{top_two:?}");
}"#,
            ),
            labelled(
                "EXPLANATION",
                "Counting matches is `filter(...).count()` — no collection is built. Top N needs the \
                 values ordered first, so the code sorts and then `take(2)`; there is no single adapter \
                 for 'the N largest', and reaching for `sort` then `take` is the honest answer. The \
                 `take` is still an iterator adapter, and the `collect` still drives it.",
            ),
            paragraphs(&[
                "For the real analyzer, 'top source' is a `HashMap` count followed by \
                 `max_by_key` — a grouping loop and one consumer. That mixture of a loop for grouping \
                 and an iterator for the selection is typical of real Rust, and the last lesson's \
                 checklist of mistakes includes reaching for a chain where a loop was clearer.",
            ]),
            callout(
                "The pipeline is a sentence",
                "Filter the errors, collect them. Take the biggest, count the rest. When the code reads \
                 like that sentence, the iterator has earned its place; when it does not, a `for` loop \
                 still exists, and using it is not a defeat.",
            ),
        ],
    )
}
