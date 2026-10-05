//! Module 11 — Iterators, lessons 1–6.
//!
//! Basics. The manual loop, what an iterator is, the three ways to iterate, the
//! `for` loop desugared, adapters versus consumers, and the sources you can
//! iterate.

use crate::content::types::{
    callout, code, diagram, gap_fill, labelled, paragraphs, step_through, table, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        the_manual_loop(),
        what_an_iterator_is(),
        three_ways(),
        for_desugared(),
        adapters_versus_consumers(),
        ranges_and_sources(),
    ]
}

fn the_manual_loop() -> SeedLesson {
    SeedLesson::new(
        "The Loop You Keep Writing",
        vec![
            labelled(
                "CONCEPT",
                "The Log Analyzer starts with a list of entries and a question: which ones are errors? \
                 Written by hand, that is an index, a check, a `push`, and a `Vec` you built yourself.",
            ),
            code(
                "rust",
                r#"fn main() {
    let logs = vec![
        (true, "db connection refused"),
        (false, "cache warm"),
        (true, "request timed out"),
    ];

    let mut errors: Vec<&str> = Vec::new();
    for i in 0..logs.len() {
        if logs[i].0 {
            errors.push(logs[i].1);
        }
    }

    println!("{} errors: {:?}", errors.len(), errors);
}"#,
            ),
            labelled(
                "EXPLANATION",
                "This works. But notice how much of it is bookkeeping: the index `i`, the bounds check, \
                 the mutable result, the `push`. The actual intent is one sentence — 'take the messages \
                 of the error entries' — and none of that sentence is visible in the loop.",
            ),
            callout(
                "The intent is a pipeline",
                "You want to walk the list, keep the errors, and take their messages. That is three \
                 operations: *iterate*, *filter*, *map*. Rust names each one, and chains them so the \
                 code reads the way the sentence does. That is what the next five lessons build to.",
            ),
            paragraphs(&[
                "Keep the manual loop in your head, though. Once you have seen the pipeline, the last \
                 lessons of the module will ask when the loop is the *better* choice — and sometimes it \
                 is.",
            ]),
        ],
    )
}

fn what_an_iterator_is() -> SeedLesson {
    SeedLesson::new(
        "What an Iterator Is",
        vec![
            labelled(
                "CONCEPT",
                "Anything you can call `.next()` on until it runs out is an *iterator*. `next` returns \
                 `Option`: `Some(item)` while there is a next element, `None` when there is not.",
            ),
            step_through(
                "Draining an iterator by hand",
                r#"let mut it = vec![10, 20, 30].into_iter();

println!("{:?}", it.next());
println!("{:?}", it.next());
println!("{:?}", it.next());
println!("{:?}", it.next());"#,
                &[
                    (&[1][..], "`into_iter()` turns the vector into an iterator."),
                    (&[3][..], "First call: `Some(10)`."),
                    (&[5][..], "Third call: `Some(30)`."),
                    (&[6][..], "Fourth call: `None` — the iterator is exhausted."),
                ],
            ),
            code(
                "rust",
                r#"fn main() {
    let mut it = vec![10, 20, 30].into_iter();

    while let Some(n) = it.next() {
        println!("{n}");
    }
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`Iterator` is a trait, and `next` is its one required method. Everything else you have \
                 seen — `map`, `filter`, `sum` — is a *provided* method built on `next`. That is why a \
                 `for` loop and a long chain do the same thing: they both end up calling `next` until \
                 they get `None`.",
            ),
            callout(
                "One method is the whole contract",
                "Implementing `next` gives you the entire iterator toolbox for free — exactly like \
                 implementing `Display` gave you `ToString` in Module 10. The loop above is the manual \
                 version the compiler writes for `for x in it`.",
            ),
        ],
    )
}

fn three_ways() -> SeedLesson {
    SeedLesson::new(
        "Three Ways to Iterate",
        vec![
            labelled(
                "CONCEPT",
                "A collection gives you three iterators, and they yield different things: `iter()` \
                 borrows, `iter_mut()` borrows mutably, and `into_iter()` takes ownership.",
            ),
            gap_fill(
                "Name the method that lets the loop change each entry, and the method that consumes the \
                 collection.",
                r#"for entry in logs.____() {
    entry.visited = true;
}

for entry in logs.____() {
    println!("{}", entry.message);
}"#,
                &[
                    ("m1", "iter_mut", "Yields `&mut T`, so the body may change it."),
                    ("m2", "into_iter", "Yields owned `T` and consumes the collection."),
                ],
            ),
            code(
                "rust",
                r#"struct LogEntry {
    message: String,
    visited: bool,
}

fn main() {
    let mut logs = vec![
        LogEntry { message: String::from("db refused"), visited: false },
    ];

    for entry in logs.iter_mut() {
        entry.visited = true;
    }

    for entry in logs.iter() {
        println!("{} ({})", entry.message, entry.visited);
    }
}"#,
            ),
            table(
                &["Method", "Yields", "Collection after"],
                &[
                    &["`.iter()`", "`&T`", "still yours, unchanged"],
                    &["`.iter_mut()`", "`&mut T`", "still yours, possibly changed"],
                    &["`.into_iter()`", "`T`", "consumed"],
                ],
            ),
            labelled(
                "EXPLANATION",
                "Choose from what the body needs. Reading → `iter`. Changing → `iter_mut`. Taking the \
                 elements out for good → `into_iter`. This is exactly the borrow-versus-move decision \
                 from Modules 05 and 06, now spelled as three method names.",
            ),
            callout(
                "`for` picks one by default",
                "`for entry in &logs` uses `iter`; `for entry in &mut logs` uses `iter_mut`; `for entry \
                 in logs` uses `into_iter`. The explicit methods are for when you are building a chain \
                 rather than writing a loop.",
            ),
        ],
    )
}

fn for_desugared() -> SeedLesson {
    SeedLesson::new(
        "The `for` Loop Desugared",
        vec![
            labelled(
                "CONCEPT",
                "A `for` loop is not special syntax. It calls `into_iter` (or a trait method on the \
                 reference) and then calls `next` until it gets `None`. You can write it out by hand.",
            ),
            diagram(
                "Two spellings, one mechanism",
                "for entry in logs { body }\n\n            is the same as\n\nlet mut it = logs.into_iter();\nwhile let Some(entry) = it.next() { body }",
            ),
            code(
                "rust",
                r#"fn main() {
    let names = vec!["a", "b", "c"];

    for name in &names {
        println!("{name}");
    }

    // The same loop, driven by hand.
    let mut it = names.iter();
    while let Some(name) = it.next() {
        println!("{name}");
    }
}"#,
            ),
            labelled(
                "EXPLANATION",
                "Seeing this once is what makes iterator chains stop looking like magic. A chain such as \
                 `.filter(...).map(...).collect()` is also just `next` being called in a loop — filter \
                 calls `next` and skips, map calls `next` and transforms, collect calls `next` and \
                 stores. The `for` loop is the plainest member of the family.",
            ),
            callout(
                "`IntoIterator` is what `for` actually needs",
                "The trait that a `for` loop consumes is `IntoIterator`, and every collection implements \
                 it. You do not need to remember the name today; you need the fact that `for` is built \
                 from `next`, which is why the rest of the module works the way it does.",
            ),
        ],
    )
}

fn adapters_versus_consumers() -> SeedLesson {
    SeedLesson::new(
        "Adapters versus Consumers",
        vec![
            labelled(
                "CONCEPT",
                "Iterator methods come in two kinds. **Adapters** (`map`, `filter`, `take`) take an \
                 iterator and return another, describing work. **Consumers** (`collect`, `sum`, `fold`) \
                 take an iterator and produce a final value, running the work.",
            ),
            table(
                &["Kind", "Examples", "Returns", "Runs the pipeline?"],
                &[
                    &["adapter", "`map`, `filter`, `take`, `enumerate`", "another iterator", "no"],
                    &["consumer", "`collect`, `sum`, `count`, `fold`", "a value", "yes"],
                ],
            ),
            code(
                "rust",
                r#"fn main() {
    let logs = vec![(true, "db refused"), (false, "cache warm"), (true, "timeout")];

    // Adapters only: this describes work, but does nothing yet.
    let described = logs.iter().filter(|(is_err, _)| *is_err);

    // The consumer drives it and produces a value.
    let messages: Vec<&str> = described.map(|(_, msg)| *msg).collect();

    println!("{messages:?}");
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`logs.iter()` gives an iterator. `.filter(...)` wraps it in another iterator that skips \
                 non-errors. `.map(...)` wraps that one to transform each remaining item. Only `.collect()` \
                 actually pulls elements through — it calls `next` until the source is exhausted, storing \
                 what comes out. Every chain is some adapters followed by one consumer.",
            ),
            callout(
                "Adapters stack; the consumer runs them",
                "This split is the key to the whole module, and to laziness: building a chain of adapters \
                 costs nothing until a consumer asks for an element. That is why a chain can be built, \
                 passed around, and only then run.",
            ),
        ],
    )
}

fn ranges_and_sources() -> SeedLesson {
    SeedLesson::new(
        "Ranges and Sources",
        vec![
            labelled(
                "CONCEPT",
                "Collections are not the only iterators. A range `0..n` is an iterator, and strings and \
                 vectors have adapters like `.chars()`, `.lines()`, and `.split()` that produce them.",
            ),
            gap_fill(
                "Split the text into lines, then each line into words. Fill in the two source methods.",
                r#"let text = "ERROR db refused\nINFO cache warm";

for line in text.____() {
    for word in line.split_whitespace() {
        println!("{word}");
    }
}"#,
                &[("m1", "lines", "Yields each line as a `&str`.")],
            ),
            code(
                "rust",
                r#"fn main() {
    for n in 0..3 {
        println!("{n}");
    }

    let text = "ERROR db refused";

    for word in text.split_whitespace() {
        println!("[{word}]");
    }
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`0..3` is an iterator of `0, 1, 2`. `.split_whitespace()` on a `&str` yields borrowed \
                 word slices — the same call you used to count words in Module 08, now understood as an \
                 iterator. Text is one of the richest sources, because parsing log lines is exactly \
                 splitting strings.",
            ),
            callout(
                "The Log Analyzer begins here",
                "Before you can filter errors, you have to get from raw text to entries. Splitting lines \
                 and words is the first stage of that pipeline — and it is an iterator, so it composes \
                 with everything that follows.",
            ),
        ],
    )
}
