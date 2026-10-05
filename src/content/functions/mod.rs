//! Module 04 — Functions.

mod challenge;
mod exercises;
mod lessons;
mod projects;

use crate::content::types::SeedModule;

pub fn content() -> SeedModule {
    SeedModule::new(
        "functions",
        "Functions",
        "How to name behaviour, pass information in, get values back, and structure a program around it.",
        lessons::lessons(),
        full_exercises(),
        projects::projects(),
    )
}

/// Lesson exercises plus the two module challenges.
fn full_exercises() -> Vec<crate::content::types::SeedExercise> {
    let mut all = exercises::exercises();
    all.extend(challenge::challenges());
    all
}
