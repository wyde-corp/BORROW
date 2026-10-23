//! Module 11 — Iterators, lessons 7–12.
//!
//! Adapters. `map`, `filter`, `take`, `skip`, `enumerate`, `zip`, `chain`,
//! `filter_map`, and `flat_map` — the methods that describe work without
//! running it.

use crate::content::types::{
    callout, code, gap_fill, guess_run, labelled, step_through, table, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        map(),
        filter(),
        take_and_skip(),
        enumerate(),
        zip_and_chain(),
        filter_map_and_flat_map(),
    ]
}

fn map() -> SeedLesson {
    SeedLesson::new(
        "`map`",
        vec![
            labelled(
                "THE IDEA",
                "`map` transforms each element. Give it a function or closure, and it returns a new \
                 iterator that yields the transformed values — one for one.",
            ),
            gap_fill(
                "Take each entry's message. Fill in the adapter.",
                r#"let messages = entries
    .iter()
    .____(|entry| entry.message.as_str())
    .collect::<Vec<_>>();"#,
                &[("m1", "map", "Transform each element.")],
            ),
            code(
                "rust",
                r#"struct LogEntry {
    message: String,
}

fn main() {
    let entries = vec![
        LogEntry { message: String::from("db refused") },
        LogEntry { message: String::from("timeout") },
    ];

    let messages: Vec<&str> = entries.iter().map(|e| e.message.as_str()).collect();
    println!("{messages:?}");
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`map` reads the closure once per element and yields the result. Here it turns each \
                 `&LogEntry` into a `&str` — the message. The count of elements is unchanged: `map` is \
                 one-in, one-out. To *remove* elements, that is `filter`.",
            ),
            callout(
                "`map` says nothing about why it runs",
                "Calling `.map(...)` builds an adapter and does nothing. The closure runs only when a \
                 consumer pulls a value through. That is why you can chain `map` freely without paying \
                 for work you might not need.",
            ),
        ],
    )
}

fn filter() -> SeedLesson {
    SeedLesson::new(
        "`filter`",
        vec![
            labelled(
                "THE IDEA",
                "`filter` keeps only the elements for which a closure returns `true`. It takes a \
                 predicate and returns an iterator that skips the rest.",
            ),
            gap_fill(
                "Keep only the error entries. Fill in the adapter.",
                r#"let errors: Vec<&LogEntry> = entries
    .iter()
    .____(|entry| entry.is_error())
    .collect();"#,
                &[("f1", "filter", "Keep the elements that pass.")],
            ),
            code(
                "rust",
                r#"#[derive(PartialEq, Debug)]
enum Level {
    Error,
    Info,
}

struct LogEntry {
    level: Level,
    message: String,
}

fn main() {
    let entries = vec![
        LogEntry { level: Level::Error, message: String::from("db refused") },
        LogEntry { level: Level::Info, message: String::from("cache warm") },
        LogEntry { level: Level::Error, message: String::from("timeout") },
    ];

    let errors: Vec<&LogEntry> = entries.iter().filter(|e| e.level == Level::Error).collect();
    println!("{}", errors.len());
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The closure receives a reference to each element and answers a yes/no question. `filter` \
                 keeps the `true`s and drops the `false`s. Unlike `map`, the number of elements can \
                 shrink — here from three entries to two errors.",
            ),
            table(
                &["Adapter", "Keeps", "Count"],
                &[
                    &["`map(f)`", "every element, transformed", "unchanged"],
                    &["`filter(p)`", "the elements where `p` is `true`", "same or fewer"],
                ],
            ),
            callout(
                "`filter` then `map` is the bread and butter",
                "Keep the errors, then take their messages: `.filter(...).map(...)`. That pair — select, \
                 then transform — is the most common two-step pipeline there is, and it is the heart of \
                 the Log Analyzer.",
            ),
        ],
    )
}

fn take_and_skip() -> SeedLesson {
    SeedLesson::new(
        "`take` and `skip`",
        vec![
            labelled(
                "THE IDEA",
                "`take(n)` yields at most the first `n` elements. `skip(n)` drops the first `n` and \
                 yields the rest. Together they slice a stream without building a `Vec` first.",
            ),
            gap_fill(
                "Show the three most recent entries, assuming the newest are first. Fill in the two \
                 adapters.",
                r#"let recent: Vec<&LogEntry> = entries
    .iter()
    .____(3)
    .collect();

let after_first: Vec<&LogEntry> = entries
    .iter()
    .____(1)
    .collect();"#,
                &[
                    ("t1", "take", "At most the first n."),
                    ("s1", "skip", "Drop the first n."),
                ],
            ),
            code(
                "rust",
                r#"fn main() {
    let numbers = vec![10, 20, 30, 40, 50];

    let first_three: Vec<i32> = numbers.iter().take(3).copied().collect();
    let after_one: Vec<i32> = numbers.iter().skip(1).copied().collect();

    println!("{first_three:?}");
    println!("{after_one:?}");
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`take(3)` stops after three elements; it does not even look at the rest. `skip(1)` drops \
                 the first and yields the rest. Chaining `skip(1).take(2)` gives a window in the middle. \
                 These are the cheapest way to say 'the first N' or 'everything after the header'.",
            ),
            callout(
                "`take` is how you tame an infinite source",
                "Because adapters are lazy, `take` is also the tool that makes an endless iterator safe: \
                 `(0..).filter(...).take(5)` pulls exactly five values and stops. The laziness lesson \
                 returns to this.",
            ),
        ],
    )
}

fn enumerate() -> SeedLesson {
    SeedLesson::new(
        "`enumerate`",
        vec![
            labelled(
                "THE IDEA",
                "`enumerate` pairs each element with its index, yielding `(0, first), (1, second), …`. It \
                 replaces the index loop you would otherwise write by hand.",
            ),
            gap_fill(
                "Number the messages as you list them. Fill in the adapter and the loop pattern.",
                r#"for (____, entry) in entries.iter().____() {
    println!("{i}: {}", entry.message);
}"#,
                &[
                    ("i1", "i", "The index binding."),
                    ("e1", "enumerate", "Pair each element with its position."),
                ],
            ),
            code(
                "rust",
                r#"struct LogEntry {
    message: String,
}

fn main() {
    let entries = vec![
        LogEntry { message: String::from("db refused") },
        LogEntry { message: String::from("timeout") },
    ];

    for (i, entry) in entries.iter().enumerate() {
        println!("{i}: {}", entry.message);
    }
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`enumerate` yields a tuple `(usize, Item)`, so destructuring in the loop header gives \
                 both. The index starts at 0. This is the concise replacement for `for i in 0..v.len()` \
                 when the loop body needs the position as well as the value.",
            ),
            callout(
                "A tuple is a single iterator item",
                "After `enumerate`, each item is one `(index, value)` pair, not two items. `map`, \
                 `filter`, and the rest see that pair as a unit — which is why you often destructure \
                 with `|(i, e)|` inside a later adapter.",
            ),
        ],
    )
}

fn zip_and_chain() -> SeedLesson {
    SeedLesson::new(
        "`zip` and `chain`",
        vec![
            labelled(
                "THE IDEA",
                "`zip` walks two iterators together, yielding pairs until the shorter runs out. `chain` \
                 glues two iterators end to end, yielding all of the first, then all of the second.",
            ),
            step_through(
                "Two ways to combine streams",
                r#"let names = ["db", "cache"];
let codes = [500, 200];

let pairs: Vec<_> = names.iter().zip(codes.iter()).collect();
let joined: Vec<i32> = vec![1, 2].into_iter().chain(vec![3, 4]).collect();"#,
                &[
                    (&[1][..], "Two independent sources."),
                    (&[4][..], "`zip` yields `(\"db\", 500), (\"cache\", 200)` — pairs, side by side."),
                    (&[5][..], "`chain` yields `1, 2, 3, 4` — one after the other."),
                ],
            ),
            code(
                "rust",
                r#"fn main() {
    let names = ["db", "cache"];
    let codes = [500, 200];

    let pairs: Vec<_> = names.iter().zip(codes.iter()).collect();
    println!("{pairs:?}");

    let joined: Vec<i32> = vec![1, 2].into_iter().chain(vec![3, 4]).collect();
    println!("{joined:?}");
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`zip` pairs element 0 with element 0, element 1 with element 1, and stops when either \
                 side is exhausted — so a longer side is truncated. `chain` runs the second iterator \
                 after the first. One combines corresponding positions; the other concatenates.",
            ),
            table(
                &["Adapter", "Yields", "When one side ends"],
                &[
                    &["`zip`", "pairs `(a, b)`", "stops at the shorter"],
                    &["`chain`", "all of `a`, then all of `b`", "no dependency between them"],
                ],
            ),
            callout(
                "Combining without a temporary `Vec`",
                "Both let you relate two streams directly. `zip` is how you attach labels to values; \
                 `chain` is how you treat two batches as one. Neither builds an intermediate collection \
                 unless a consumer asks it to.",
            ),
        ],
    )
}

fn filter_map_and_flat_map() -> SeedLesson {
    SeedLesson::new(
        "`filter_map` and `flat_map`",
        vec![
            labelled(
                "THE IDEA",
                "`filter_map` filters and transforms in one step: the closure returns an `Option`, and \
                 only the `Some`s are kept. `flat_map` maps each element to an iterator and flattens \
                 the results into one stream.",
            ),
            gap_fill(
                "Parse each line, keeping only the ones that parsed. Fill in the two adapters.",
                r#"// Keep the parsed entries, dropping the failures.
let parsed: Vec<LogEntry> = lines.iter().____(|line| parse_line(line).ok()).collect();

// Every word of every line, in one flat list.
let words: Vec<&str> = lines.iter().____(|line| line.split_whitespace()).collect();"#,
                &[
                    ("f1", "filter_map", "Filter and transform at once, via an `Option`."),
                    ("f2", "flat_map", "Map to an iterator, then flatten."),
                ],
            ),
            code(
                "rust",
                r#"fn main() {
    let lines = ["db:refused", "garbage", "cache:warm"];

    // `filter_map`: parse each line, keep the successes.
    let parsed: Vec<&str> = lines
        .iter()
        .filter_map(|line| line.split_once(':'))
        .map(|(source, _)| source)
        .collect();
    println!("{parsed:?}");

    // `flat_map`: every word of every line, flattened.
    let words: Vec<&str> = lines
        .iter()
        .flat_map(|line| line.split(':'))
        .collect();
    println!("{words:?}");
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`filter_map` is the common 'parse and drop failures' shape: return `Option` from the \
                 closure and let the adapter discard the `None`s. `flat_map` takes a closure returning \
                 an iterator and splices those iterators together — so a list of lines becomes a single \
                 list of words. Without `flat_map` you would get a `Vec<Vec<&str>>` and flatten it by \
                 hand.",
            ),
            guess_run(
                "Predict what this collects.",
                r#"fn main() {
    let nums = [1, 2, 3, 4];
    let out: Vec<i32> = nums
        .iter()
        .filter_map(|n| if n % 2 == 0 { Some(n * 10) } else { None })
        .collect();
    println!("{out:?}");
}"#,
                &[
                    "[10, 20, 30, 40]",
                    "[20, 40]",
                    "[1, 3]",
                ],
                1,
                "The closure returns `Some(n * 10)` only for even numbers, and `filter_map` drops the \
                 `None`s. Even numbers are 2 and 4, so the result is `[20, 40]`. That single call did \
                 the work of a `filter` and a `map`.",
            ),
            callout(
                "One pass, two jobs",
                "`filter_map` and `flat_map` exist because 'filter then map' and 'map then flatten' are \
                 so common that naming them removes a step. They are still adapters — lazy, and only \
                 run when a consumer drives them.",
            ),
        ],
    )
}
