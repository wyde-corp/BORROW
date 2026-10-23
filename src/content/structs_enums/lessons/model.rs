//! Module 07 — Structs & Enums, lesson 36.
//!
//! The capstone model. One compact way to decide between a struct and an enum,
//! and a checklist of what the learner can now do.

use crate::content::types::{callout, checklist, diagram, labelled, table, SeedLesson};

pub fn lessons() -> Vec<SeedLesson> {
    vec![final_model()]
}

fn final_model() -> SeedLesson {
    SeedLesson::new(
        "Final Mental Model",
        vec![
            labelled(
                "THE IDEA",
                "Two questions decide almost every modelling choice in this module. Ask them in order, \
                 and the type you need is usually obvious.",
            ),
            table(
                &["Ask", "If yes", "If it is a set"],
                &[
                    &["Is every one of these facts true at once?", "struct", "enum"],
                    &["Can it be A now and B later, but never both?", "enum", "struct"],
                    &["Does the data have a fixed list of possible states?", "enum", "struct"],
                    &["Does the data describe one thing made of several?", "struct", "enum"],
                ],
            ),
            diagram(
                "The decision, in one line",
                "NAME THE DATA\n     ↓\nIS IT 'ALL OF THESE'?  ──yes──→  STRUCT\n     │ no\n     ↓\nIS IT 'ONE OF THESE'?  ──yes──→  ENUM\n     │ no\n     ↓\nTRY A TUPLE, OR SPLIT IT",
            ),
            checklist(
                "You can now do this",
                &[
                    "Define and construct a struct, and say why a struct is one value",
                    "Use field-init shorthand and struct update syntax",
                    "Derive `Debug`, `Clone`, `PartialEq`, and `Default`, and say what each gives you",
                    "Nest structs and hold structs in a `Vec`",
                    "Write methods, and choose between `&self`, `&mut self`, and `self`",
                    "Write associated functions such as `Pizza::new`",
                    "Define an enum whose variants carry data",
                    "Explain why `Option` is an ordinary enum",
                    "Use `match` exhaustively, with patterns, wildcards, `if let`, and `let else`",
                    "Destructure a struct or a variant inside a pattern",
                    "Model a small real domain with named, honest types",
                ],
            ),
            callout(
                "The one sentence to carry out of here",
                "A struct is **one thing made of several**. An enum is **one thing that is one of \
                 several**. When you meet a new domain, ask which one each piece is — the answer is \
                 usually the whole design.",
            ),
            labelled(
                "Where this goes next",
                "The enums you just met are the ones Rust builds its most important types from. \
                 `Option` is the first you have seen; `Result`, for operations that can fail, is the \
                 next. Both are just enums with variants that carry data, and both are matched and \
                 destructured exactly as you have practised here.",
            ),
        ],
    )
}
