//! Module 08 — Collections, exercises for lessons 27–30.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L27 The Decision Table
        SeedExercise::new(
            Some(26),
            ExerciseKind::ApiDesign,
            "You must answer 'is this genre already in the playlist?' Which container should hold the \
             genres?",
            &[
                "`Vec<String>` — a list",
                "`HashSet<String>` — a set",
                "`HashMap<String, String>` — a map",
            ],
            1,
            "The question is membership — 'is this present?' — which is exactly what a `HashSet` \
             answers. A `Vec` would need a scan; a `HashMap` would give every genre a value it does \
             not need.",
        ),

        // L28 BTreeMap and VecDeque
        SeedExercise::new(
            Some(27),
            ExerciseKind::Mcq,
            "You add and remove items at the *front* of a list as often as the back. Which container?",
            &[
                "`Vec`",
                "`VecDeque`",
                "`HashSet`",
            ],
            1,
            "A `Vec` only does the back cheaply; front operations shift everything. `VecDeque` is fast \
             at both ends, which is exactly a queue's shape.",
        ),

        // L29 A Little Performance Intuition
        SeedExercise::new(
            Some(28),
            ExerciseKind::Mcq,
            "Which of these does *not* get slower as the collection grows large?",
            &[
                "Scanning a `Vec` for a value",
                "Looking a key up in a `HashMap`",
                "Inserting at the front of a `Vec`",
            ],
            1,
            "A `HashMap` lookup is roughly constant: it does not care how many entries there are. \
             Scanning a `Vec` and front-inserting both grow with the number of items.",
        ),

        // L30 Common Mistakes
        SeedExercise::new(
            Some(29),
            ExerciseKind::PredictCompile,
            "A map is mutated while a loop borrows it. Predict the result.",
            &[
                "It compiles and adds an entry each time",
                "It does not compile — a mutable borrow is requested while the loop holds a shared one",
            ],
            1,
            "`&map` borrows for the loop, and `insert` needs a mutable borrow at the same time. Gather \
             the new entries and insert them after the loop — the same rule for every collection.",
        )
        .with_code(
            "use std::collections::HashMap;\n\nfn main() {\n    let mut m: HashMap<&str, u32> = HashMap::new();\n    m.insert(\"a\", 1);\n    for (k, v) in &m {\n        println!(\"{k} {v}\");\n        m.insert(\"b\", 2);\n    }\n}",
        ),
    ]
}
