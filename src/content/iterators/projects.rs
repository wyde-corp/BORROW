//! Module 11 — Iterators projects.
//!
//! Six projects escalating from deliberate repetition to the Log Analyzer
//! capstone. Each asks the learner to choose a consumer, or to notice when a
//! loop reads better than a chain.

use crate::content::types::{Difficulty, SeedProject};

pub fn projects() -> Vec<SeedProject> {
    vec![
        SeedProject::new(
            "Iterator Playground",
            "Deliberately repetitive. Drive the same list with a `for` loop, with `.map`, and with `.filter()`, and record what each produces.",
            Difficulty::Beginner,
            "30 min",
            &[
                "Build a `Vec` of five numbers",
                "Print it with a `for` loop",
                "Print each number doubled with `.map` and a consumer",
                "Print only the even numbers with `.filter`",
                "Count the evens with `.count`",
                "Write one sentence per approach on what it is for",
            ],
            &[
                "`iter`, `map`, `filter`, `count`",
                "laziness",
                "consumers drive adapters",
            ],
            &[
                "Every chain must end in a consumer",
                "No index loops in this project",
                "Each of the four approaches gets a sentence",
                "The `map`/`filter` versions must not build intermediate `Vec`s unless collecting",
            ],
            "let doubled: Vec<i32> = nums.iter().map(|n| n * 2).collect();\nlet evens: Vec<i32> = nums.iter().filter(|n| *n % 2 == 0).copied().collect();\nlet count = nums.iter().filter(|n| *n % 2 == 0).count();",
            &[
                "Print the same list sorted, reversed, and taken to the first three, all as chains",
                "Explain in a comment why the `.count()` version allocates nothing",
            ],
        ),

        SeedProject::new(
            "Filter and Map",
            "Pull the errors out of log entries and map them to their messages — the two adapters that carry the module.",
            Difficulty::Beginner,
            "45 min",
            &[
                "Define `LogEntry { is_error: bool, message: String }`",
                "Collect the error messages with `.filter(...).map(...).collect()`",
                "Count the errors without collecting",
                "Find the first error with `.find`",
                "Build the same results with a `for` loop and compare",
                "Say which version you prefer and why",
            ],
            &[
                "`filter`, `map`, `find`",
                "collect versus count",
                "adapters versus a loop",
            ],
            &[
                "At least one result must use `find` and handle `Option`",
                "The loop version must produce identical results",
                "No cloning messages into owned strings on the read path",
                "A one-sentence comparison is required",
            ],
            "let messages: Vec<&str> = entries.iter().filter(|e| e.is_error).map(|e| e.message.as_str()).collect();\nlet first = entries.iter().find(|e| e.is_error);",
            &[
                "Add a `sources()` method returning `impl Iterator` over distinct sources",
                "Explain why `find` stops early but `filter` does not",
            ],
        ),

        SeedProject::new(
            "Count and Summarise",
            "Counting consumers and a `fold`: how many by level, how many bytes in total, and the biggest entry.",
            Difficulty::Intermediate,
            "1 h",
            &[
                "Define `Level` and `LogEntry` with a byte count",
                "Count the entries per level with a `HashMap` and `entry`",
                "Sum the bytes with `.map(...).sum()`",
                "Find the biggest entry with `.max_by`",
                "Compute the average bytes with `fold` or `sum`/`count`",
                "Say which consumer each question needed",
            ],
            &[
                "`sum`, `max_by`, `fold`",
                "`HashMap` counting",
                "choosing the right consumer",
            ],
            &[
                "Summing uses `.sum()`, not a manual accumulator",
                "The average handles the empty case without dividing by zero",
                "The largest returns an `Option` and is handled",
                "Comment each consumer with why it fits",
            ],
            "let total: u32 = entries.iter().map(|e| e.bytes).sum();\nlet biggest = entries.iter().max_by(|a, b| a.bytes.cmp(&b.bytes));",
            &[
                "Compute the average with `fold` and compare it to `sum` then divide",
                "Add the noisiest source using the count map and `max_by_key`",
            ],
        ),

        SeedProject::new(
            "Top N",
            "A `HashMap` count fed by an iterator, then ranked and limited. The mixed loop-and-pipeline shape.",
            Difficulty::Intermediate,
            "1 h",
            &[
                "Count entries per source with a `HashMap` and the `entry` API",
                "Move the counts into a `Vec` with `into_iter().collect()`",
                "Sort by count, descending",
                "Take the top three with `.take(3)`",
                "Print the result",
                "Say where the loop ends and the pipeline begins",
            ],
            &[
                "grouping with a map",
                "`into_iter().collect()`",
                "`sort_by` and `take`",
                "mixing loops and pipelines",
            ],
            &[
                "The count must use `entry(...).or_insert(0)`",
                "The ranking must use `take`, not a manual break at index 3",
                "A comment must name the loop section and the pipeline section",
                "No cloning of source strings on the count path",
            ],
            "let mut counts: HashMap<&str, usize> = HashMap::new();\nfor e in &entries {\n    *counts.entry(e.source.as_str()).or_insert(0) += 1;\n}\nlet mut ranked: Vec<(&str, usize)> = counts.into_iter().collect();\nranked.sort_by(|a, b| b.1.cmp(&a.1));\nlet top: Vec<_> = ranked.into_iter().take(3).collect();",
            &[
                "Make a version that keeps a top-three list without sorting and explain the trade-off",
                "Add a case-insensitive count by lowercasing sources",
            ],
        ),

        SeedProject::new(
            "A Lazy Report",
            "A function returning `impl Iterator` that the caller drives with whichever consumer fits — selecting, counting, or collecting.",
            Difficulty::Advanced,
            "1.5 h",
            &[
                "Write `fn errors<'a>(entries: &'a [LogEntry]) -> impl Iterator<Item = &'a LogEntry>`",
                "Write `fn messages<'a>(...) -> impl Iterator<Item = &'a str>` on top of it",
                "Consume the first with `count`, the second with `collect`",
                "Drive the second with a `for` loop as well",
                "Prove laziness with a `println!` in a closure",
                "Say what returning `impl Iterator` saves over returning a `Vec`",
            ],
            &[
                "returning `impl Iterator`",
                "composing lazy pipelines",
                "proving laziness",
            ],
            &[
                "No function may return a `Vec`",
                "The filter closure must carry the laziness probe at least once",
                "The functions must be usable by at least two different consumers",
                "The comment on what is saved is required",
            ],
            "fn errors<'a>(entries: &'a [LogEntry]) -> impl Iterator<Item = &'a LogEntry> {\n    entries.iter().filter(|e| e.is_error)\n}\n\nlet n = errors(&entries).count();\nlet all: Vec<_> = errors(&entries).collect();",
            &[
                "Add a `.take(n)` on the returned iterator and explain what changed",
                "Compare the binary's work when the caller only counts versus collects",
            ],
        ),

        SeedProject::new(
            "Log Analyzer",
            "The capstone: the analyzer built across the module, finished and justified. Parse, filter, map, count, group, and rank — with a written note on every place a pipeline or a loop was chosen.",
            Difficulty::Advanced,
            "3 h",
            &[
                "Parse log lines into `LogEntry` values, dropping bad lines with `filter_map`",
                "Answer: which entries are errors, and what messages did they carry?",
                "Count the entries per level",
                "Find the noisiest source and the top three messages",
                "Return at least one function's results as `impl Iterator`",
                "A note: for each question, say whether you used a chain or a loop, and why",
            ],
            &[
                "parsing with `filter_map`",
                "adapters and consumers",
                "`HashMap` grouping",
                "`impl Iterator` returns",
                "API design and justification",
            ],
            &[
                "No index loops anywhere",
                "Parsing drops bad lines rather than panicking",
                "Every chain ends in a consumer",
                "The written note is required — working code alone is not the deliverable",
            ],
            "let entries: Vec<LogEntry> = raw.lines().filter_map(|l| parse_line(l).ok()).collect();\n\nlet errors: Vec<&str> = entries.iter().filter(|e| e.is_error).map(|e| e.message.as_str()).collect();\n\nlet noisy = source_counts.iter().max_by_key(|(_, n)| **n);",
            &[
                "Add a `--stats` mode printing counts per level, per source, and the average message length",
                "Return the top-three messages as `impl Iterator` and drive it from `main`",
                "Write the justification note a second time, after the code, and compare",
            ],
        ),
    ]
}
