//! Module 10 — Generics & Traits, exercise for lesson 32.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![SeedExercise::new(
        Some(31),
        ExerciseKind::Mcq,
        "Which sentence captures the whole module?",
        &[
            "Generics and traits are advanced features for library authors",
            "A generic names the shape, a trait names the behaviour, a bound connects them, and dispatch chooses the cost",
            "Traits are interfaces, and generics are templates",
        ],
        1,
        "Generics describe the shape of data, traits name shared behaviour, bounds say which types are \
         allowed, and dispatch decides whether the type is known at compile time. You have used \
         `Vec<T>`, `Option<T>`, `From`, and `Display` since early modules — none of it was advanced \
         trivia.",
    )]
}
