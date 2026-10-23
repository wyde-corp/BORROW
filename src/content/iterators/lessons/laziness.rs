//! Module 11 — Iterators, lessons 19–22.
//!
//! Laziness. Nothing runs until a consumer, infinite iterators, driving with
//! `next`, and the fact that an iterator is exhausted once.

use crate::content::types::{
    callout, code, gap_fill, guess_run, labelled, pipeline, step_through, table, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        nothing_until_a_consumer(),
        infinite_iterators(),
        driving_with_next(),
        exhausted_once(),
    ]
}

fn nothing_until_a_consumer() -> SeedLesson {
    SeedLesson::new(
        "Nothing Runs Until a Consumer",
        vec![
            labelled(
                "THE IDEA",
                "Adapters are lazy. Calling `.filter(...)` or `.map(...)` builds a description; not one \
                 element moves until a consumer asks for the first value. This is the single most \
                 important thing to understand about iterators.",
            ),
            pipeline(
                "The adapters are set up, and then collect runs the whole pipeline",
                r#"let errors: Vec<&str> = entries
    .iter()
    .filter(|e| e.is_error)
    .map(|e| e.message.as_str())
    .collect();"#,
                &[
                    (
                        &[3][..],
                        "lazy",
                        "`filter` is set up: an iterator that will skip non-errors. Nothing has run.",
                    ),
                    (
                        &[4][..],
                        "lazy",
                        "`map` is set up too. Still nothing has run — this is just a description.",
                    ),
                    (
                        &[5][..],
                        "eager",
                        "`collect` asks for the first item, and the pipeline springs to life.",
                    ),
                    (
                        &[5][..],
                        "eager",
                        "`collect` pulls until the source is empty, then returns the vector.",
                    ),
                ],
            ),
            code(
                "rust",
                r#"#[derive(Clone, Copy)]
struct LogEntry {
    is_error: bool,
    message: &'static str,
}

fn main() {
    let entries = vec![
        LogEntry { is_error: true, message: "db refused" },
        LogEntry { is_error: false, message: "cache warm" },
        LogEntry { is_error: true, message: "timeout" },
    ];

    let errors: Vec<&str> = entries
        .iter()
        .filter(|e| e.is_error)
        .map(|e| e.message)
        .collect();

    println!("{errors:?}");
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "You can prove the laziness yourself: put a `println!` inside the filter closure. Build \
                 the chain and print 'built' — the closure has not run. Not until `.collect()` (or \
                 `.count()`, or a `for` loop) does it run, once per element. The pipeline is a recipe \
                 until a consumer cooks it.",
            ),
            callout(
                "Laziness is why chains are cheap",
                "Building a chain of five adapters allocates nothing and does no work. The cost is paid \
                 once, when the consumer runs, and only for the elements actually needed — which is why \
                 `take(3)` after a filtering chain only filters enough to find three.",
            ),
        ],
    )
}

fn infinite_iterators() -> SeedLesson {
    SeedLesson::new(
        "Infinite Iterators",
        vec![
            labelled(
                "THE IDEA",
                "Because adapters are lazy, an infinite iterator is safe — as long as you combine it with \
                 something that stops. `0..` goes on forever, `repeat(x)` never ends, `cycle()` repeats a \
                 list endlessly; `take` bounds them.",
            ),
            gap_fill(
                "Take the first five even numbers from an endless source. Fill in the bound.",
                r#"let first_five: Vec<u32> = (0..)
    .filter(|n| n % 2 == 0)
    .____(5)
    .collect();"#,
                &[("t1", "take", "Stop after five.")],
            ),
            code(
                "rust",
                r#"fn main() {
    let first_five: Vec<u32> = (0..)
        .filter(|n| n % 2 == 0)
        .take(5)
        .collect();

    println!("{first_five:?}");
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`0..` is an infinite range — it would run forever on its own. `filter` is lazy, so it \
                 does nothing yet. `take(5)` is the bound: it pulls only until it has five values, then \
                 stops, and the source is never asked for a sixth. Infinite sources are useful when the \
                 number you need is decided at the end of the chain.",
            ),
            table(
                &["Source", "Yields", "Bound it with"],
                &[
                    &["`0..`", "0, 1, 2, … forever", "`take(n)` or `find`"],
                    &["`std::iter::repeat(x)`", "`x` forever", "`take(n)`"],
                    &["`list.iter().cycle()`", "the list, over and over", "`take(n)`"],
                ],
            ),
            callout(
                "The bound must be a consumer or a short-circuit",
                "`take` bounds by count; `find` and `any` stop early by condition; `collect` on its own \
                 would hang on an infinite source. The rule is simple: an infinite iterator needs \
                 something downstream that stops pulling.",
            ),
        ],
    )
}

fn driving_with_next() -> SeedLesson {
    SeedLesson::new(
        "Driving with `next`",
        vec![
            labelled(
                "THE IDEA",
                "When you want elements one at a time — not a whole collection — call `.next()` in a \
                 `while let` loop. This is the manual form of `for`, and it is what makes `next` the \
                 heart of the trait.",
            ),
            step_through(
                "Pulling elements until the iterator is done",
                r#"let mut it = (1..).map(|n| n * n);

while let Some(square) = it.next() {
    if square > 30 {
        break;
    }
    println!("{square}");
}"#,
                &[
                    (&[1][..], "An infinite source, mapped to squares — still lazy."),
                    (&[3][..], "Each `next()` pulls one value through the map and gives it back."),
                    (&[4][..], "The loop decides when to stop, not the iterator."),
                    (&[5][..], "Prints 1, 4, 9, 16, 25; at 36 it breaks."),
                ],
            ),
            code(
                "rust",
                r#"fn main() {
    let mut it = (1..).map(|n| n * n);

    while let Some(square) = it.next() {
        if square > 30 {
            break;
        }
        println!("{square}");
    }
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`next()` returns `Option`, so `while let Some(x) = it.next()` runs until it is `None` — \
                 or until your own `break`. Because the source is infinite, the termination is in the \
                 body, which is exactly the control you sometimes need and a `for` loop does not offer.",
            ),
            callout(
                "The `for` loop is this, tidied",
                "You are unlikely to write `while let Some(...) = it.next()` often — `for` is clearer \
                 when you want everything. But knowing it is the same mechanism is what makes the whole \
                 trait click: `next` is the primitive, and every comfort is built on it.",
            ),
        ],
    )
}

fn exhausted_once() -> SeedLesson {
    SeedLesson::new(
        "An Iterator Is Exhausted Once",
        vec![
            labelled(
                "THE IDEA",
                "An iterator is a *stream with a cursor*. Once a consumer has pulled it to `None`, it \
                 stays `None`. A second loop over the same iterator finds nothing — you need a fresh \
                 one from the collection.",
            ),
            guess_run(
                "The same iterator is drained twice. Predict what the second loop prints.",
                r#"fn main() {
    let mut it = vec![1, 2, 3].into_iter();

    let first: Vec<i32> = it.by_ref().collect();
    let second: Vec<i32> = it.collect();

    println!("{first:?} {second:?}");
}"#,
                &[
                    "[1, 2, 3] [1, 2, 3]",
                    "[1, 2, 3] []",
                    "It does not compile",
                ],
                1,
                "The first `collect` drains the iterator, so the second finds it exhausted and produces \
                 an empty vector. `by_ref` lets the first consumer borrow the iterator without consuming \
                 the variable, but the cursor has still moved to the end. A second pass needs a new \
                 iterator from the collection.",
            ),
            code(
                "rust",
                r#"fn main() {
    let numbers = vec![1, 2, 3];

    let first: Vec<i32> = numbers.iter().copied().collect();
    let second: Vec<i32> = numbers.iter().copied().collect();

    println!("{first:?} {second:?}");
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The fix is to call `.iter()` again — each call returns a fresh iterator positioned at \
                 the start. The exhausted iterator in the previous example was a single cursor over the \
                 vector, and a cursor only moves forward. `by_ref` was the tool for handing the same \
                 iterator to a consumer without giving it up, but it does not rewind.",
            ),
            callout(
                "Iterators are values, not the collection",
                "This is why `for x in numbers.iter()` can appear twice on the same vector: each \
                 `.iter()` is a new iterator. It is also why storing an iterator in a variable and \
                 looping over it twice does not work. The collection is unchanged; the *cursor* was \
                 spent.",
            ),
        ],
    )
}
