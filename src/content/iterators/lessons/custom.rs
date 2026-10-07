//! Module 11 — Iterators, lessons 23–25.
//!
//! Custom. Returning `impl Iterator`, writing an iterator by hand, and the ten
//! common mistakes.

use crate::content::types::{
    callout, code, gap_fill, guess_run, labelled, step_through, table, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        returning_impl_iterator(),
        writing_an_iterator(),
        common_mistakes(),
    ]
}

fn returning_impl_iterator() -> SeedLesson {
    SeedLesson::new(
        "Returning `impl Iterator`",
        vec![
            labelled(
                "CONCEPT",
                "A function can return an iterator instead of a `Vec`, so the caller drives it and no \
                 intermediate collection is built. The return type is `impl Iterator<Item = ...>`, read \
                 as 'some type that yields these items'.",
            ),
            gap_fill(
                "Return a lazy iterator of the error messages. Fill in the return type and the consumer \
                 the caller uses.",
                r#"fn error_messages(entries: &[LogEntry]) -> ____ Iterator<Item = &str> {
    entries.iter().filter(|e| e.is_error).map(|e| e.message.as_str())
}

let messages: Vec<&str> = error_messages(&entries).____();"#,
                &[
                    ("i1", "impl", "The anonymous-iterator return type."),
                    ("c1", "collect", "The caller's consumer."),
                ],
            ),
            code(
                "rust",
                r#"struct LogEntry {
    is_error: bool,
    message: String,
}

fn error_messages<'a>(entries: &'a [LogEntry]) -> impl Iterator<Item = &'a str> {
    entries
        .iter()
        .filter(|e| e.is_error)
        .map(|e| e.message.as_str())
}

fn main() {
    let entries = vec![
        LogEntry { is_error: true, message: String::from("db refused") },
        LogEntry { is_error: false, message: String::from("cache warm") },
    ];

    // The caller decided how to consume it.
    let messages: Vec<&str> = error_messages(&entries).collect();
    let count = error_messages(&entries).count();

    println!("{messages:?} ({count})");
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`impl Iterator<Item = &str>` means 'a type I will not name, that yields `&str` items'. \
                 The concrete type is a filter wrapper around a map wrapper around a slice iterator — a \
                 mouthful nobody wants to write, which is exactly why `impl Trait` exists. The caller \
                 gets a lazy stream and chooses `collect`, `count`, or a `for` loop.",
            ),
            callout(
                "Return the pipeline, not a `Vec`",
                "Returning `Vec` forces the work and allocates, even if the caller only wanted to count. \
                 Returning `impl Iterator` postpones all of it. It is the iterator equivalent of \
                 borrowing instead of cloning: hand back the ability, let the caller decide.",
            ),
            guess_run(
                "The error list is consumed twice. Predict what happens.",
                r#"struct LogEntry { is_error: bool, message: String }

fn error_messages<'a>(entries: &'a [LogEntry]) -> impl Iterator<Item = &'a str> {
    entries.iter().filter(|e| e.is_error).map(|e| e.message.as_str())
}

fn main() {
    let entries = vec![LogEntry { is_error: true, message: String::from("db") }];

    let a: Vec<&str> = error_messages(&entries).collect();
    let b: Vec<&str> = error_messages(&entries).collect();
    println!("{a:?} {b:?}");
}"#,
                &[
                    "[db] []",
                    "[db] [db]",
                    "It does not compile",
                ],
                1,
                "Each call to `error_messages` returns a *fresh* iterator built from `entries`, so both \
                 collect `[db]`. The exhaustion rule from the laziness lessons applies to a single \
                 iterator value, not to a function that builds a new one each call.",
            ),
        ],
    )
}

fn writing_an_iterator() -> SeedLesson {
    SeedLesson::new(
        "Writing an Iterator by Hand",
        vec![
            labelled(
                "CONCEPT",
                "You rarely need to, but you can implement `Iterator` yourself by providing `next`. The \
                 trait needs one method and one associated type, `Item` — the type each call yields.",
            ),
            step_through(
                "A countdown that is an iterator",
                r#"struct Countdown(u32);

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.0 == 0 {
            None
        } else {
            let current = self.0;
            self.0 -= 1;
            Some(current)
        }
    }
}"#,
                &[
                    (&[1][..], "The type holds the state — here, the next number to yield."),
                    (&[4][..], "`Item` says each call yields a `u32`."),
                    (&[6][..], "`next` takes `&mut self`, because it advances the state."),
                    (&[7][..], "At zero, there is nothing left: return `None`."),
                    (&[10][..], "Otherwise yield the current value and step down."),
                ],
            ),
            code(
                "rust",
                r#"struct Countdown(u32);

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.0 == 0 {
            None
        } else {
            let current = self.0;
            self.0 -= 1;
            Some(current)
        }
    }
}

fn main() {
    for n in Countdown(3) {
        println!("{n}");
    }

    let doubled: Vec<u32> = Countdown(3).map(|n| n * 2).collect();
    println!("{doubled:?}");
}"#,
            ),
            labelled(
                "EXPLANATION",
                "Because `Countdown` implements `Iterator`, every adapter and consumer works on it for \
                 free. `map`, `collect`, `filter`, `for` — all built from the one `next` method you \
                 wrote. This is the payoff of the trait being small: one method to implement, the whole \
                 toolbox to gain.",
            ),
            callout(
                "The state lives in `self`",
                "An iterator is just a struct that remembers where it is. `Countdown` stores the next \
                 number; a slice iterator stores a pointer and a remaining length. `next` reads the \
                 state, advances it, and returns an `Option`. Nothing else is required.",
            ),
        ],
    )
}

fn common_mistakes() -> SeedLesson {
    SeedLesson::new(
        "The Ten Common Mistakes",
        vec![
            labelled(
                "CONCEPT",
                "These are the mistakes that survive after the syntax is learned. Read them as a \
                 checklist against your own understanding.",
            ),
            table(
                &["#", "Mistake", "The correction"],
                &[
                    &["1", "expecting `.map` to run immediately", "nothing runs until a consumer drives it"],
                    &["2", "forgetting the consumer", "a chain with no `collect`/`for`/`sum` does nothing"],
                    &["3", "reusing an exhausted iterator", "call `.iter()` again for a fresh pass"],
                    &["4", "confusing `iter`, `iter_mut`, `into_iter`", "borrow, borrow mutably, or move"],
                    &["5", "letting `collect` infer nothing", "annotate the target type, e.g. `Vec<_>`"],
                    &["6", "using `filter` then `map` when `filter_map` fits", "one call, one pass"],
                    &["7", "building a `Vec` only to iterate it once", "return `impl Iterator` instead"],
                    &["8", "collecting an infinite iterator", "bound it with `take` or `find`"],
                    &["9", "chaining for its own sake", "a plain `for` loop is sometimes clearer"],
                    &["10", "`.clone()` inside a pipeline to dodge a borrow", "borrow, or collect owned values"],
                ],
            ),
            guess_run(
                "A chain is written but never consumed. Predict what prints.",
                r#"fn main() {
    let nums = vec![1, 2, 3];

    nums.iter().map(|n| {
        println!("mapping {n}");
        n * 2
    });

    println!("done");
}"#,
                &[
                    "mapping 1 / mapping 2 / mapping 3 / done",
                    "done",
                    "It does not compile",
                ],
                1,
                "The `map` builds an adapter and returns it, but nothing consumes it, so the closure \
                 never runs. Only `done` prints. Rust even warns that the returned iterator is unused. \
                 Add `.collect::<Vec<_>>()` or a `for` loop and the closure runs. This is laziness in \
                 its purest form.",
            ),
            callout(
                "Read the errors as instructions",
                "Two errors dominate: a `collect` that cannot infer its type (annotate it) and a value \
                 used after being moved into `into_iter` (borrow with `iter`). Both are mechanical once \
                 you have seen the shape.",
            ),
        ],
    )
}
