//! Module 08 — Collections, lesson 34.
//!
//! The capstone model: position, lookup, membership, and the decision procedure
//! that turns a question into a container.

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
                "Three containers, three questions. Ask the question first and the container follows; \
                 the syntax is the small part.",
            ),
            table(
                &["The container", "The question", "Keeps order?", "Duplicates?"],
                &[
                    &["`Vec<T>`", "what is at this position / in what order?", "yes", "yes"],
                    &["`HashMap<K, V>`", "what value is under this key?", "no", "keys unique"],
                    &["`HashSet<T>`", "is this value present?", "no", "no"],
                ],
            ),
            diagram(
                "The decision, in one line",
                "WHAT IS YOUR QUESTION?\n     │\n     ├── \"what is at position N?\" ──→ Vec\n     │\n     ├── \"what is under key K?\"  ──→ HashMap\n     │\n     └── \"is X present?\"         ──→ HashSet\n\n(need sorted keys? BTreeMap.  need both ends? VecDeque.)",
            ),
            checklist(
                "You can now do this",
                &[
                    "Create, grow, index, and iterate a `Vec`, by reference, mutably, and by value",
                    "Choose between `v[i]` and `v.get(i)` and explain the difference",
                    "Build and query a `HashMap`, and use `entry(...).or_insert(...)` to count",
                    "Explain why `HashMap` and `HashSet` iteration order is not guaranteed",
                    "Build a map of vectors for an artist index",
                    "Use a `HashSet` for membership and do union, intersection, and difference",
                    "De-duplicate a list by collecting through a set",
                    "Choose between `Vec`, `HashMap`, and `HashSet` from the shape of the question",
                    "Name `BTreeMap` and `VecDeque` for sorted keys and fast front operations",
                    "Recognise the borrow conflict of mutating a collection while iterating it",
                ],
            ),
            callout(
                "The one sentence to carry out of here",
                "A `Vec` is for **position**, a `HashMap` is for **lookup**, and a `HashSet` is for \
                 **membership**. When you meet a new problem, ask what question you are really \
                 asking — the container is usually the answer.",
            ),
            labelled(
                "Where this goes next",
                "Every collection method that can miss — `get`, `pop`, `find`, `remove` — returned an \
                 `Option`. Writing code that handles 'might be missing' gracefully, and its sibling \
                 'might fail', is the next module.",
            ),
        ],
    )
}
