//! Module 05 — Ownership.
//!
//! The module that justifies every other module. Lessons are grouped by the
//! argument they make rather than by topic, because ownership only becomes
//! coherent once move, copy, clone, scope, and function calls are all in
//! place — the order the spec asks for, with the myths and the debugging
//! material folded in as their own lessons rather than appended as an appendix.

mod challenge;
mod exercises;
mod lessons;
mod projects;

use crate::content::types::SeedModule;

pub fn content() -> SeedModule {
    SeedModule::new(
        "ownership",
        "Ownership",
        "Why Rust has a memory model at all, how a value gets an owner, and what actually happens to that value when you assign it to something else.",
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
