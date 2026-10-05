//! Module 05 — Ownership exercises.
//!
//! Split by the part of the argument they serve: foundations and the move,
//! then the three outcomes, then ownership in real code, then the consolidation
//! material that turns recognition into prediction.

mod detective;
mod foundations;
mod in_code;
mod moving;

use crate::content::types::SeedExercise;

pub fn exercises() -> Vec<SeedExercise> {
    let mut all = foundations::exercises();
    all.extend(moving::exercises());
    all.extend(in_code::exercises());
    all.extend(detective::exercises());
    all
}
