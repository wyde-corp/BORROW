//! Module 08 — Collections exercises.

mod choosing;
mod hashmaps;
mod model;
mod real_world;
mod sets;
mod vectors;

use crate::content::types::SeedExercise;

pub fn exercises() -> Vec<SeedExercise> {
    let mut all = vectors::exercises();
    all.extend(hashmaps::exercises());
    all.extend(sets::exercises());
    all.extend(choosing::exercises());
    all.extend(real_world::exercises());
    all.extend(model::exercises());
    all
}
