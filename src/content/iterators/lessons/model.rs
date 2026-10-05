//! Module 11 — Iterators, lesson 28.
//!
//! The capstone model: adapters describe, consumers run, nothing happens until
//! the consumer asks.

use crate::content::types::{callout, checklist, diagram, labelled, table, SeedLesson};

pub fn lessons() -> Vec<SeedLesson> {
    vec![final_model()]
}

fn final_model() -> SeedLesson {
    SeedLesson::new(
        "Final Mental Model",
        vec![
            labelled(
                "CONCEPT",
                "One trait, one required method, and a toolbox built on top of it. That is the whole \
                 module, and the last abstraction in the course.",
            ),
            table(
                &["Piece", "What it is", "What it does"],
                &[
                    &["`next`", "the one required method", "yields the next element, or `None`"],
                    &["adapter", "`map`, `filter`, `take`, …", "describes work, returns another iterator"],
                    &["consumer", "`collect`, `sum`, `fold`, …", "runs the pipeline, produces a value"],
                    &["laziness", "the property", "nothing runs until a consumer pulls"],
                ],
            ),
            diagram(
                "The shape of every pipeline",
                "source        adapters                  consumer\n  │              │                          │\n  ▼              ▼                          ▼\niter()  →  filter(...)  →  map(...)  →   collect()\n                                              │\n                                     pulls until done",
            ),
            checklist(
                "You can now do this",
                &[
                    "Read a chain and say what each adapter and the consumer do",
                    "Choose `iter`, `iter_mut`, or `into_iter` from the body's needs",
                    "Reach for the right adapter: `map`, `filter`, `filter_map`, `flat_map`, `take`, `enumerate`",
                    "Reach for the right consumer: `collect`, `sum`, `count`, `find`, `fold`",
                    "Explain laziness and prove it with a `println!` in a closure",
                    "Bound an infinite iterator with `take` or `find`",
                    "Drive an iterator by hand with `next`, and explain exhaustion",
                    "Return `impl Iterator` instead of an intermediate `Vec`",
                    "Write `impl Iterator` by hand for a simple type",
                    "Choose a `for` loop when it reads better than a chain",
                ],
            ),
            callout(
                "The one sentence to carry out of here",
                "An **adapter** describes work; a **consumer** runs it; nothing happens until the \
                 consumer asks. When you see a chain, find the consumer first — it is the thing doing \
                 all the work.",
            ),
            labelled(
                "Where this goes next",
                "This was the last abstraction the course teaches. Everything the standard library does \
                 with data — reading files line by line, parsing, summarising, transforming — is the \
                 iterator trait plus the types you learned along the way: structs and enums to hold \
                 data, `Option` and `Result` for what might be missing or fail, traits to name \
                 behaviour, and iterators to move through it. You now have the whole set.",
            ),
        ],
    )
}
