//! Module 07 — Structs & Enums, exercise for lesson 36.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![SeedExercise::new(
        Some(35),
        ExerciseKind::Mcq,
        "An order's `status` is one of five stages. A `Pizza` has a size and a list of toppings. \
         Which is which?",
        &[
            "Both are structs",
            "`status` is an enum; `Pizza` is a struct",
            "Both are enums",
        ],
        1,
        "The status is exactly one of a fixed set, so it is an enum. A pizza is several facts at \
         once — a size and a list — so it is a struct. Ask 'all of these?' or 'one of these?' and the \
         answer is immediate.",
    )]
}
