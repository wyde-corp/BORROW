//! Module 08 — Collections, exercise for lesson 34.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![SeedExercise::new(
        Some(33),
        ExerciseKind::Mcq,
        "Match each container to its question. Which pairing is correct?",
        &[
            "`Vec` is for lookup, `HashMap` is for position, `HashSet` is for membership",
            "`Vec` is for position, `HashMap` is for lookup, `HashSet` is for membership",
            "`Vec` is for membership, `HashMap` is for position, `HashSet` is for lookup",
        ],
        1,
        "`Vec` answers 'what is at this position', `HashMap` answers 'what is under this key', and \
         `HashSet` answers 'is this present'. Say your question out loud and the container follows.",
    )]
}
