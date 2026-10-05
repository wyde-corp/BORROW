//! Module 11 — the three module challenges.
//!
//! Choosing between a pipeline and a loop, grouping with a map, and proving
//! laziness. The walkthrough is the last hint.

use crate::content::types::{ExerciseKind, SeedExercise};

const LOOP_OR_PIPELINE: &str = r#"// Write each of these both as an iterator chain and as a `for`
// loop, then say which you would ship and why.
//
// 1. The messages of all error entries.
// 2. The number of entries longer than 100 bytes.
// 3. A `HashMap` from source to error count.
// 4. The first error entry, if any.
// 5. Whether every entry is under 1000 bytes.
"#;

const TOP_N: &str = r#"use std::collections::HashMap;

#[derive(Debug)]
struct LogEntry {
    source: String,
    is_error: bool,
}

// Find the three sources with the most error entries.
// The count needs a `HashMap`; the selection can be an iterator.
fn top_error_sources(entries: &[LogEntry]) -> Vec<(String, usize)> {
    todo!()
}

fn main() {
    let entries = vec![
        LogEntry { source: String::from("db"), is_error: true },
        LogEntry { source: String::from("db"), is_error: true },
        LogEntry { source: String::from("cache"), is_error: true },
        LogEntry { source: String::from("db"), is_error: false },
        LogEntry { source: String::from("auth"), is_error: true },
    ];

    println!("{:?}", top_error_sources(&entries));
}"#;

const LAZY_PROOF: &str = r#"// Write a function returning `impl Iterator` that filters and maps a
// slice. Then:
//   1. call it, and print a line before consuming it;
//   2. consume it with `collect`, and then with `count`;
//   3. put a `println!` inside the closure and use it to prove WHEN the
//      closure runs.
fn error_messages<'a>(entries: &'a [LogEntry]) -> impl Iterator<Item = &'a str> {
    todo!()
}

struct LogEntry {
    is_error: bool,
    message: String,
}

fn main() {
    // Build `entries`, then experiment.
}
"#;

pub const LOOP_WALKTHROUGH: &str = r#"The five, as chains and as loops:

  1. messages of errors
     chain: entries.iter().filter(|e| e.is_error).map(|e| e.message.as_str()).collect()
     loop:  let mut v = Vec::new();
            for e in &entries { if e.is_error { v.push(e.message.as_str()); } }
     ship:  the chain. "filter then map" is exactly what the adapters name.

  2. count over 100 bytes
     chain: entries.iter().filter(|e| e.bytes > 100).count()
     loop:  a counter and an if
     ship:  the chain. `count` is clearer than a mutable counter.

  3. HashMap from source to error count
     chain: build a Vec of pairs, then collect into a HashMap, then... this gets awkward.
     loop:  a map and `*map.entry(source).or_insert(0) += 1` — clear and direct.
     ship:  the loop. Grouping is a mutation per element; the `entry` API reads better.

  4. first error
     chain: entries.iter().find(|e| e.is_error)
     loop:  a loop that `break`s on the first match
     ship:  the chain. `find` names the intent and short-circuits for free.

  5. all under 1000 bytes
     chain: entries.iter().all(|e| e.bytes < 1000)
     loop:  a loop that returns `false` early
     ship:  the chain. `all` is a single word for the question.

The pattern: chains win when the task is 'filter / map / find / count / all' — a
nouns-and-verbs sentence. Loops win when the task is 'build a map by mutating,
one element at a time'. Neither is wrong; the deliverable is the judgment."#;

pub const TOP_N_WALKTHROUGH: &str = r#"Count first, then select:

  fn top_error_sources(entries: &[LogEntry]) -> Vec<(String, usize)> {
      let mut counts: HashMap<&str, usize> = HashMap::new();
      for e in entries.iter().filter(|e| e.is_error) {
          *counts.entry(e.source.as_str()).or_insert(0) += 1;
      }

      let mut ranked: Vec<(&str, usize)> = counts.into_iter().collect();
      ranked.sort_by(|a, b| b.1.cmp(&a.1));
      ranked.into_iter().take(3).map(|(s, n)| (s.to_string(), n)).collect()
  }

Notice the shape: a `for` loop driven by an iterator adapters, building a
`HashMap` (grouping is a loop), then `into_iter().collect()` to move the map's
pairs into a vector, `sort_by` to rank, and `take(3)` to limit. The pipeline
starts again once grouping is done.

There is no single adapter for 'the N largest by count'. Reaching for
`sort` then `take` is the honest answer; the alternative — a full scan keeping a
top-three list — is an optimisation, not a clarity win for a log tool."#;

pub const LAZY_WALKTHROUGH: &str = r#"The function:

  fn error_messages<'a>(entries: &'a [LogEntry]) -> impl Iterator<Item = &'a str> {
      entries.iter().filter(|e| e.is_error).map(|e| e.message.as_str())
  }

Calling it builds a fresh, lazy iterator; it does not run anything. If you put a
`println!("checking")` inside the filter closure, you will see *nothing* print
until you call `.collect()` or `.count()`. Then you will see one line per element
that the pipeline pulls.

Two things this proves:

  1. The adapters are lazy — `error_messages(...)` on its own does no work.
  2. Each call returns a new iterator from the slice, so you can consume it as
     many times as you call the function, and each call starts fresh.

That is the difference between returning `impl Iterator` and returning a `Vec`:
the `Vec` version would run and allocate eagerly, and the caller could not choose
to stop early."#;

pub fn challenges() -> Vec<SeedExercise> {
    vec![
        SeedExercise::new(
            None,
            ExerciseKind::Challenge,
            "PIPELINE OR LOOP? — For each of five data-processing tasks, write it both as an iterator \
             chain and as a `for` loop, then say which you would ship and why. The judgment is the \
             deliverable.",
            &[
                "Write each task both ways",
                "For each, name the consumer or the loop that does the work",
                "Decide which you would ship, and give a reason in one sentence",
                "Call out any task where the two are genuinely close",
                "Do not chain for its own sake — a loop is allowed to win",
            ],
            0,
            LOOP_WALKTHROUGH,
        )
        .code(LOOP_OR_PIPELINE)
        .xp(170)
        .hints(&[
            "Filter, map, find, count and all read as a sentence; grouping does not.",
            "Grouping into a map is a mutation per element — the `entry` API.",
            "`find` and `all` short-circuit; a loop can too, with `break`.",
            "The walkthrough decides all five and names the pattern.",
        ]),
        SeedExercise::new(
            None,
            ExerciseKind::Challenge,
            "TOP N BY SOURCE — Given log entries, find the three sources with the most error entries. \
             The count needs a `HashMap`; the ranking can be an iterator. Decide where the loop ends and \
             the pipeline begins.",
            &[
                "Filter to the errors, then count per source with the `entry` API",
                "Move the counts into a vector with `into_iter().collect()`",
                "Sort by count, descending",
                "Take the top three",
                "Say why there is no single adapter for this",
            ],
            0,
            TOP_N_WALKTHROUGH,
        )
        .code(TOP_N)
        .xp(180)
        .hints(&[
            "Counting is a loop over `entry(...).or_insert(0)`.",
            "A `HashMap` has no order, so ranking needs a vector.",
            "`sort_by` then `take(3)` is the selection.",
            "The walkthrough shows the mixed shape: loop to group, pipeline to rank.",
        ]),
        SeedExercise::new(
            None,
            ExerciseKind::Challenge,
            "BUILD A LAZY PIPELINE — Write a function returning `impl Iterator` that filters and maps a \
             slice, then prove it is lazy by putting a `println!` in the closure and observing when it \
             runs.",
            &[
                "Write the function with an `impl Iterator` return type",
                "Call it without consuming the result and print a marker",
                "Then consume it with `collect` and observe when the closure runs",
                "Consume it again with `count` and note that the function rebuilt the iterator",
                "Write one sentence on what the experiment proved",
            ],
            0,
            LAZY_WALKTHROUGH,
        )
        .code(LAZY_PROOF)
        .xp(160)
        .hints(&[
            "Nothing in the chain runs until a consumer pulls.",
            "A `println!` inside the closure is your probe.",
            "Each call to the function builds a new iterator from the slice.",
            "The walkthrough shows the proof and what it means.",
        ]),
    ]
}
