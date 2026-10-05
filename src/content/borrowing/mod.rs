//! Module 06 — Borrowing.
//!
//! The natural follow-on to ownership: if I do not want to take a value, how do
//! I use it temporarily? Lessons are one per PART of the module spec, grouped
//! by the argument they make — the problem and the mechanism, the rule and its
//! duration, borrowing at function boundaries, views into values, and finally
//! the reasoning that turns the rules into prediction.

mod challenge;
mod exercises;
mod lessons;
mod projects;

use crate::content::types::{SeedExercise, SeedModule};

pub fn content() -> SeedModule {
    SeedModule::new(
        "borrowing",
        "Borrowing",
        "Use a value without taking responsibility for it. References, immutable and mutable borrows, and the rules that keep them from conflicting.",
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
