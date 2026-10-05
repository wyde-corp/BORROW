//! Module 07 — Structs & Enums.
//!
//! How to define your own data. A struct says "these values are one value"; an
//! enum says "this value is exactly one of these". The running Pizza Order
//! program grows through every lesson, from five loose variables to a small
//! program with named types, methods, and exhaustive matching.

mod challenge;
mod exercises;
mod lessons;
mod projects;

use crate::content::types::{SeedExercise, SeedModule};

pub fn content() -> SeedModule {
    SeedModule::new(
        "structs-and-enums",
        "Structs & Enums",
        "Define your own data: name the groups that belong together, and the fixed sets that are one of several.",
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
