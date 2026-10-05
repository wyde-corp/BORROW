//! Module 11 — Iterators.
//!
//! The trait behind every `for` loop and every `.map`, `.filter`, and
//! `.collect`. Adapters describe work, consumers run it, and nothing happens
//! until a consumer asks — the module's whole idea. The running Log Analyzer
//! grows through every lesson, and this is the final module of the course.

mod challenge;
mod exercises;
mod lessons;
mod projects;

use crate::content::types::{SeedExercise, SeedModule};

pub fn content() -> SeedModule {
    SeedModule::new(
        "iterators",
        "Iterators",
        "Move through sequences of data with Rust's iterator pattern: lazy adapters, driving consumers, and pipelines from the standard library.",
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
