//! Module 10 — Generics & Traits.
//!
//! Writing code once for many types (generics) and naming the behaviour types
//! share (traits). The running Media Library grows through every lesson: a
//! `Summary` trait, a generic `Library<T: Summary>`, and a mixed shelf of
//! `Box<dyn Summary>`. The module turns on four ideas — generics name the
//! shape, traits name the behaviour, bounds connect them, dispatch chooses the
//! cost.

mod challenge;
mod exercises;
mod lessons;
mod projects;

use crate::content::types::{SeedExercise, SeedModule};

pub fn content() -> SeedModule {
    SeedModule::new(
        "generics-and-traits",
        "Generics & Traits",
        "Write code that works for many types, and define the shared behaviour those types must provide.",
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
