//! Module 11 — Iterators, exercise for lesson 28.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![SeedExercise::new(
        Some(27),
        ExerciseKind::Mcq,
        "Which sentence captures the whole module?",
        &[
            "Iterators are faster than loops",
            "An adapter describes work, a consumer runs it, and nothing happens until the consumer asks",
            "`for` loops are obsolete once you know iterators",
        ],
        1,
        "Adapters like `map` and `filter` build a description; consumers like `collect` and `sum` run \
         it. Laziness means the description does nothing until a consumer pulls. And `for` is built on \
         the same trait — not obsolete, just one consumer among many.",
    )]
}
