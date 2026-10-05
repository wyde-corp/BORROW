//! Module 11 — Iterators exercises.

mod adapters;
mod basics;
mod consumers;
mod custom;
mod laziness;
mod model;
mod real_world;

use crate::content::types::SeedExercise;

pub fn exercises() -> Vec<SeedExercise> {
    let mut all = basics::exercises();
    all.extend(adapters::exercises());
    all.extend(consumers::exercises());
    all.extend(laziness::exercises());
    all.extend(custom::exercises());
    all.extend(real_world::exercises());
    all.extend(model::exercises());
    all
}
