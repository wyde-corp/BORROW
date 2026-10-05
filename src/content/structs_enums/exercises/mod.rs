//! Module 07 — Structs & Enums exercises.

mod construction;
mod enums;
mod foundations;
mod matching;
mod methods;
mod model;
mod patterns;

use crate::content::types::SeedExercise;

pub fn exercises() -> Vec<SeedExercise> {
    let mut all = foundations::exercises();
    all.extend(construction::exercises());
    all.extend(methods::exercises());
    all.extend(enums::exercises());
    all.extend(matching::exercises());
    all.extend(patterns::exercises());
    all.extend(model::exercises());
    all
}
