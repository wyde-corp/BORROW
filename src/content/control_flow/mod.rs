//! Module 03 — Control Flow.

mod challenge;
mod exercises;
mod lessons;
mod projects;

use crate::content::types::SeedModule;

pub fn content() -> SeedModule {
    SeedModule::new(
        "control-flow",
        "Control Flow",
        "How Rust programs make decisions, repeat work, and produce values.",
        lessons::lessons(),
        full_exercises(),
        projects::projects(),
    )
}

/// Lessons exercises plus the module challenge and the open-ended build task.
fn full_exercises() -> Vec<crate::content::types::SeedExercise> {
    let mut all = exercises::exercises();
    all.extend(challenge::challenges());
    all
}
