//! Module 09 — Option & Result, exercise for lesson 30.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![SeedExercise::new(
        Some(29),
        ExerciseKind::Mcq,
        "Which pairing of type and meaning is correct?",
        &[
            "`Option` is for failure, `Result` is for absence",
            "`Option` is for absence, `Result` is for failure",
            "Both are for failure; `Option` is the shorter one",
        ],
        1,
        "`Option` says 'a value, or nothing' — absence. `Result` says 'a value, or a reason it \
         failed' — failure. `?` is how either travels up the call stack.",
    )]
}
