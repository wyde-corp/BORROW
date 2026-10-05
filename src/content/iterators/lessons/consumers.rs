//! Module 11 — Iterators, lessons 13–18.
//!
//! Consumers. `collect`, `sum`, `product`, `count`, `any`, `all`, `find`,
//! `position`, `fold`, and the picking consumers — the methods that run a
//! pipeline and produce a value.

use crate::content::types::{
    callout, code, gap_fill, guess_run, labelled, step_through, table, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        collect(),
        sum_and_product(),
        count_any_all(),
        find_and_position(),
        fold(),
        picking(),
    ]
}

fn collect() -> SeedLesson {
    SeedLesson::new(
        "`collect`",
        vec![
            labelled(
                "CONCEPT",
                "`collect` runs a pipeline and gathers the results into a collection. Which collection \
                 is decided by the type you ask for, so the target usually appears in an annotation.",
            ),
            gap_fill(
                "Gather the error messages into a `Vec`. Fill in the consumer and the target type.",
                r#"let messages: ____<&str> = entries
    .iter()
    .map(|e| e.message.as_str())
    .____();"#,
                &[
                    ("t1", "Vec", "The collection to gather into."),
                    ("c1", "collect", "The consumer that drives the pipeline."),
                ],
            ),
            code(
                "rust",
                r#"use std::collections::HashSet;

struct LogEntry {
    source: String,
}

fn main() {
    let entries = vec![
        LogEntry { source: String::from("db") },
        LogEntry { source: String::from("db") },
        LogEntry { source: String::from("cache") },
    ];

    let sources: Vec<&str> = entries.iter().map(|e| e.source.as_str()).collect();
    let unique: HashSet<&str> = entries.iter().map(|e| e.source.as_str()).collect();

    println!("{sources:?}");
    println!("{unique:?}");
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`collect` is one consumer with many targets. Ask for a `Vec` and it builds a vector; ask \
                 for a `HashSet` and it de-duplicates; ask for a `String` or a `HashMap` and it builds \
                 those. The type on the binding is what tells `collect` which constructor to call — the \
                 same trick you used when de-duplicating a list in Module 08.",
            ),
            callout(
                "The type annotation is the instruction",
                "If `collect` complains that the type cannot be inferred, it is because nothing said \
                 what to build. Writing `let v: Vec<_> = ...` is usually the whole fix. `_` lets the \
                 compiler fill in the element type from the pipeline.",
            ),
        ],
    )
}

fn sum_and_product() -> SeedLesson {
    SeedLesson::new(
        "`sum` and `product`",
        vec![
            labelled(
                "CONCEPT",
                "`sum` adds every element; `product` multiplies them. Both consume the iterator and give \
                 one number, and both work for any numeric type.",
            ),
            gap_fill(
                "Total the bytes across the entries. Fill in the consumer.",
                r#"let total: u32 = entries
    .iter()
    .map(|entry| entry.bytes)
    .____();"#,
                &[("s1", "sum", "Add every element.")],
            ),
            code(
                "rust",
                r#"struct LogEntry {
    bytes: u32,
}

fn main() {
    let entries = vec![
        LogEntry { bytes: 120 },
        LogEntry { bytes: 80 },
        LogEntry { bytes: 200 },
    ];

    let total: u32 = entries.iter().map(|e| e.bytes).sum();
    println!("{total} bytes");

    let product: u32 = vec![2, 3, 4].into_iter().product();
    println!("{product}");
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`sum` is exactly the `total += ...` you would have written in a loop, but the accumulator \
                 is handled for you. It needs a starting value, which comes from the type — hence the \
                 annotation on `total`. `product` is the multiplicative twin, used far less often but \
                 free once `sum` is understood.",
            ),
            callout(
                "A fold you do not have to write",
                "Both are specialisations of `fold`, the general reduce two lessons away. Reach for \
                 `sum` when you are adding and `product` when you are multiplying; reach for `fold` when \
                 the combination is anything else.",
            ),
        ],
    )
}

fn count_any_all() -> SeedLesson {
    SeedLesson::new(
        "`count`, `any`, `all`",
        vec![
            labelled(
                "CONCEPT",
                "Three consumers answer questions about a whole stream. `count` says how many elements \
                 there are; `any` says whether at least one passes; `all` says whether every one does.",
            ),
            guess_run(
                "Predict both booleans, given levels `[Error, Info, Error]`.",
                r#"#[derive(PartialEq)]
enum Level { Error, Info }

fn main() {
    let levels = [Level::Error, Level::Info, Level::Error];

    println!("{}", levels.iter().any(|l| *l == Level::Error));
    println!("{}", levels.iter().all(|l| *l == Level::Error));
}"#,
                &[
                    "true and true",
                    "true and false",
                    "false and false",
                ],
                1,
                "`any` is true because at least one level is `Error`. `all` is false because the `Info` \
                 entry is not. Both stop as soon as the answer is known: `any` short-circuits on the \
                 first `true`, `all` on the first `false` — so they may not look at the whole stream.",
            ),
            code(
                "rust",
                r#"fn main() {
    let numbers = [1, 2, 3, 4];

    println!("count: {}", numbers.iter().count());
    println!("any even: {}", numbers.iter().any(|n| n % 2 == 0));
    println!("all positive: {}", numbers.iter().all(|n| *n > 0));
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`count` consumes the whole iterator. `any` and `all` take a predicate and stop early \
                 when the outcome is settled. All three turn a pipeline into a single answer — a number \
                 or a `bool` — and none of them build a collection.",
            ),
            table(
                &["Consumer", "Answers", "Stops early?"],
                &[
                    &["`count()`", "how many", "no"],
                    &["`any(p)`", "does at least one pass?", "on the first `true`"],
                    &["`all(p)`", "do all pass?", "on the first `false`"],
                ],
            ),
            callout(
                "Predicates are just functions",
                "`any` and `all` take the same kind of closure as `filter` — something that returns a \
                 `bool`. `filter` keeps the passing elements; `any`/`all` answer yes or no and stop.",
            ),
        ],
    )
}

fn find_and_position() -> SeedLesson {
    SeedLesson::new(
        "`find` and `position`",
        vec![
            labelled(
                "CONCEPT",
                "`find` returns the first element that passes, wrapped in `Option`. `position` returns \
                 its index instead. Both stop the moment they find it.",
            ),
            gap_fill(
                "Fetch the first error entry, if any. Fill in the consumer.",
                r#"let first_error: Option<&LogEntry> = entries
    .iter()
    .____(|entry| entry.is_error());"#,
                &[("f1", "find", "First element that passes the predicate.")],
            ),
            code(
                "rust",
                r#"struct LogEntry {
    message: String,
    is_error: bool,
}

fn main() {
    let entries = vec![
        LogEntry { message: String::from("cache warm"), is_error: false },
        LogEntry { message: String::from("db refused"), is_error: true },
    ];

    if let Some(entry) = entries.iter().find(|e| e.is_error) {
        println!("first error: {}", entry.message);
    }

    let at = entries.iter().position(|e| e.is_error);
    println!("at index {at:?}");
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`find` returns `Option<&LogEntry>` — the same 'might be missing' type from Module 09 — \
                 so you handle the none case with `if let`. `position` returns `Option<usize>`: the index \
                 of the first match, or `None`. Both short-circuit, so on a huge log you stop at the \
                 first hit.",
            ),
            callout(
                "The pipeline ends in an `Option`",
                "`find` and `position` are where iterators meet `Option`. Everything before them is a \
                 description; these consume it and hand back a value that might not exist, which is why \
                 the next line is usually `if let Some(...)`.",
            ),
        ],
    )
}

fn fold() -> SeedLesson {
    SeedLesson::new(
        "`fold`",
        vec![
            labelled(
                "CONCEPT",
                "`fold` is the general consumer behind `sum`, `count`, and almost everything else. It \
                 carries an accumulator from element to element: start with a value, and for each \
                 element produce the next accumulator.",
            ),
            step_through(
                "Counting errors with fold",
                r#"let count = entries.iter().fold(0, |acc, entry| {
    if entry.is_error {
        acc + 1
    } else {
        acc
    }
});"#,
                &[
                    (&[1][..], "`fold` starts with `0` — the initial accumulator."),
                    (&[1][..], "For the first entry it computes the next accumulator."),
                    (&[2][..], "If the entry is an error, the accumulator increases."),
                    (&[4][..], "If not, the accumulator is unchanged."),
                    (&[6][..], "After the last entry, the final accumulator is the result."),
                ],
            ),
            code(
                "rust",
                r#"struct LogEntry {
    is_error: bool,
}

fn main() {
    let entries = vec![
        LogEntry { is_error: true },
        LogEntry { is_error: false },
        LogEntry { is_error: true },
    ];

    let count = entries.iter().fold(0, |acc, entry| {
        if entry.is_error { acc + 1 } else { acc }
    });

    println!("{count} errors");

    // The same fold can build a String.
    let joined = vec!["db", "cache", "auth"]
        .into_iter()
        .fold(String::new(), |acc, name| acc + name + ",");

    println!("{joined}");
}"#,
            ),
            labelled(
                "EXPLANATION",
                "Read `fold` as two arguments: the starting accumulator (`0`) and a closure that takes \
                 the accumulator-so-far and the current element and returns the new accumulator. The \
                 second example folds strings into one — joining with a comma — which no built-in \
                 consumer does. That flexibility is why `fold` exists.",
            ),
            table(
                &["You want", "Consumer"],
                &[
                    &["to add numbers", "`sum`"],
                    &["how many", "`count`"],
                    &["to combine any other way", "`fold`"],
                ],
            ),
            callout(
                "`fold` is the raw power tool",
                "Every other consumer could be written as a `fold`, but they are clearer as themselves. \
                 Reach for `fold` when the combination is not one of the built-ins — concatenating, \
                 building a map, or carrying two values at once in a tuple accumulator.",
            ),
        ],
    )
}

fn picking() -> SeedLesson {
    SeedLesson::new(
        "`max_by`, `min_by`, `last`",
        vec![
            labelled(
                "CONCEPT",
                "Some consumers pick one element out. `max_by` and `min_by` take a comparison and return \
                 the largest or smallest, wrapped in `Option`. `last` returns the final element, also as \
                 an `Option`.",
            ),
            gap_fill(
                "Find the entry with the most bytes. Fill in the consumer.",
                r#"let biggest = entries
    .iter()
    .____(|a, b| a.bytes.cmp(&b.bytes));"#,
                &[("m1", "max_by", "Pick the greatest by the comparison.")],
            ),
            code(
                "rust",
                r#"struct LogEntry {
    message: String,
    bytes: u32,
}

fn main() {
    let entries = vec![
        LogEntry { message: String::from("cache warm"), bytes: 80 },
        LogEntry { message: String::from("db refused"), bytes: 220 },
        LogEntry { message: String::from("timeout"), bytes: 140 },
    ];

    if let Some(biggest) = entries.iter().max_by(|a, b| a.bytes.cmp(&b.bytes)) {
        println!("biggest: {} ({})", biggest.message, biggest.bytes);
    }

    let last = entries.iter().last();
    println!("{:?}", last.map(|e| e.message.as_str()));
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`max_by` takes a comparator — `a.cmp(b)` — and returns `Option<&LogEntry>`. Use `Option` \
                 because an empty iterator has no maximum. `last` consumes the iterator and returns the \
                 final element, or `None` if it was empty. Both finish with the `Option` handling you \
                 know.",
            ),
            callout(
                "Comparators, not `>`",
                "`max_by` needs an ordering function rather than the `>` operator, because the elements \
                 are references and the comparison is on a field. `a.bytes.cmp(&b.bytes)` is the idiom; \
                 `Ord::cmp` returns an `Ordering`, which is what the consumer wants.",
            ),
        ],
    )
}
