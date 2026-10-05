//! Module 08 — Collections.
//!
//! Working with many values: an ordered list (`Vec`), a key-value lookup
//! (`HashMap`), and a set of unique items (`HashSet`). The running Music
//! Playlist program grows through every lesson, and the whole module turns on
//! one idea — position, lookup, membership.

mod challenge;
mod exercises;
mod lessons;
mod projects;

use crate::content::types::{SeedExercise, SeedModule};

pub fn content() -> SeedModule {
    SeedModule::new(
        "collections",
        "Collections",
        "Work with many values at once: ordered lists, key-value lookups, and sets of unique items.",
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
