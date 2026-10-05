//! Module 09 — Option & Result.
//!
//! Absence and failure, handled honestly. `Option` for a value that may be
//! missing, `Result` for an operation that may fail, `?` to carry failure up
//! the call stack, and a small error enum with `From` so the two compose. The
//! running Todo CLI grows through every lesson.

mod challenge;
mod exercises;
mod lessons;
mod projects;

use crate::content::types::{SeedExercise, SeedModule};

pub fn content() -> SeedModule {
    SeedModule::new(
        "option-and-result",
        "Option & Result",
        "Handle absence and failure honestly: Option when a value may be missing, Result when an operation may fail, and ? to carry failure up.",
        lessons::lessons(),
        full_exercises(),
        projects::projects(),
    )
}

/// Lesson exercises plus the module challenges.
fn full_exercises() -> Vec<SeedExercise> {
    let mut all = exercises::exercises();
    all.extend(challenge::challenges());
    all
}
