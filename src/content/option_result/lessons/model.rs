//! Module 09 — Option & Result, lesson 30.
//!
//! The capstone model: absence, failure, and how failure travels.

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
                "Two types, one question each, and one operator that carries failure up the call \
                 stack. That is the whole module.",
            ),
            table(
                &["Type", "Means", "Carries", "Reach for it when"],
                &[
                    &["`Option<T>`", "a `T`, or nothing", "the value, or nothing", "a lookup may simply miss"],
                    &["`Result<T, E>`", "a `T`, or a failure", "the value, or a reason", "an operation may fail and the reason matters"],
                ],
            ),
            diagram(
                "Absence, failure, and the bridge",
                "lookup may miss ──→ Option ──ok_or──→ Result ←── operation may fail\n                                            │\n                                           `?`\n                                            │\n                              unwinds to the caller",
            ),
            checklist(
                "You can now do this",
                &[
                    "Tell absence (`Option`) from failure (`Result`) and choose correctly",
                    "Open either with `match`, `if let`, or `let else`",
                    "Replace most `.unwrap()` calls with `?`, `unwrap_or`, `map`, or `ok_or`",
                    "Propagate an error with `?` and explain the `From` conversion it performs",
                    "Write a small error enum with data, plus `Display` and a `From` impl",
                    "Return `Result` from `main` and let a failure end the program with a message",
                    "Cross between `Option` and `Result` with `ok_or` and `ok`",
                    "Collect a list of results into one `Result` that stops at the first failure",
                    "Say why `.unwrap()` panics, and the narrow cases where that is honest",
                    "Name `Box<dyn Error>` and its trade-off",
                ],
            ),
            callout(
                "The one sentence to carry out of here",
                "`Option` is for **absence**. `Result` is for **failure**. `?` is how failure travels. \
                 When you meet a fallible step, ask which of the two it is — and if it is absence that \
                 the user needs to understand, bridge it with `ok_or`.",
            ),
            labelled(
                "Where this goes next",
                "You have now implemented a trait on purpose — `Display` and `Error` — and used `From` \
                 as a conversion. Those are the first real uses of Rust's trait system, which is the \
                 subject of the next module: defining shared behaviour and writing code that works for \
                 many types at once.",
            ),
        ],
    )
}
