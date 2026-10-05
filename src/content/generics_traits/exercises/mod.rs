//! Module 10 — Generics & Traits exercises.

mod derives;
mod dispatch;
mod generics;
mod model;
mod real_world;
mod traits;

use crate::content::types::SeedExercise;

pub fn exercises() -> Vec<SeedExercise> {
    let mut all = generics::exercises();
    all.extend(traits::exercises());
    all.extend(derives::exercises());
    all.extend(dispatch::exercises());
    all.extend(real_world::exercises());
    all.extend(model::exercises());
    all
}
