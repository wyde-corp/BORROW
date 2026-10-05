//! Module 08 — Collections lessons.

mod choosing;
mod hashmaps;
mod model;
mod real_world;
mod sets;
mod vectors;

use crate::content::types::SeedLesson;

pub fn lessons() -> Vec<SeedLesson> {
    let mut all = vectors::lessons();
    all.extend(hashmaps::lessons());
    all.extend(sets::lessons());
    all.extend(choosing::lessons());
    all.extend(real_world::lessons());
    all.extend(model::lessons());
    all
}
