//! Module 09 — Option & Result exercises.

mod errors;
mod model;
mod option;
mod patterns;
mod real_world;
mod result;

use crate::content::types::SeedExercise;

pub fn exercises() -> Vec<SeedExercise> {
    let mut all = option::exercises();
    all.extend(result::exercises());
    all.extend(errors::exercises());
    all.extend(patterns::exercises());
    all.extend(real_world::exercises());
    all.extend(model::exercises());
    all
}
