//! Module 10 — Generics & Traits, lesson 32.
//!
//! The capstone model: generics name the shape, traits name the behaviour,
//! bounds connect them, and dispatch chooses the cost.

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
                "Four ideas, each answering one question. Together they are the whole of generics and \
                 traits, and you have already met every one of them under another name.",
            ),
            table(
                &["Idea", "Answers", "Looked like, earlier"],
                &[
                    &["generics", "what shape of data does this code work on?", "`Vec<T>`, `Option<T>`, `Result<T, E>`"],
                    &["traits", "what behaviour can these types share?", "`Display`, `From`, `Iterator`"],
                    &["bounds", "which types are allowed here?", "`T: PartialOrd + Copy`"],
                    &["dispatch", "known type, or chosen later?", "`notify<T>` vs `notify(&dyn Trait)`"],
                ],
            ),
            diagram(
                "How the four fit together",
                "generic code  ──needs──▶  a trait (behaviour)\n      │                          │\n      │                       a bound\n      │                       narrows it\n      ▼                          ▼\n   one function,          \"types that can X\"\n   many types\n                                │\n               static      OR   │   dynamic\n        (type known)             (type erased)\n              impl Trait / <T>    dyn Trait + Box",
            ),
            checklist(
                "You can now do this",
                &[
                    "Replace duplicated functions with one generic function",
                    "Define a generic struct or enum, and read `Option`/`Result` as generic types",
                    "Explain monomorphisation and why generics are zero-cost",
                    "Move long bounds into a `where` clause",
                    "Define a trait, implement it for a type, and add a default method",
                    "Bound a generic function or `impl` block to \"types that can X\"",
                    "Read a \"trait bound not satisfied\" error and fix it",
                    "Use `#[derive]` knowingly, and hand-write `Display` and your own traits",
                    "Implement `From` and get `Into` for free",
                    "Tell `impl Trait` from `dyn Trait` and choose deliberately",
                ],
            ),
            callout(
                "The one sentence to carry out of here",
                "A **generic** names the shape; a **trait** names the behaviour; a **bound** connects \
                 them; **dispatch** chooses the cost. When you see `<T: Something>`, read it as 'any \
                 type that can be used here, as long as it can do `Something`'.",
            ),
            labelled(
                "Where this goes next",
                "One trait has been standing just out of focus the whole module: `Iterator`. It is the \
                 trait behind `.map`, `.filter`, `.sum`, and every `for` loop, and it is the last major \
                 abstraction the course teaches. The next module makes it the subject.",
            ),
        ],
    )
}
