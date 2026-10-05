//! Module 11 — Iterators lessons.

mod adapters;
mod basics;
mod consumers;
mod custom;
mod laziness;
mod model;
mod real_world;

use crate::content::types::SeedLesson;

pub fn lessons() -> Vec<SeedLesson> {
    let mut all = basics::lessons();
    all.extend(adapters::lessons());
    all.extend(consumers::lessons());
    all.extend(laziness::lessons());
    all.extend(custom::lessons());
    all.extend(real_world::lessons());
    all.extend(model::lessons());
    all
}
