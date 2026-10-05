//! Module 10 — Generics & Traits lessons.

mod derives;
mod dispatch;
mod generics;
mod model;
mod real_world;
mod traits;

use crate::content::types::SeedLesson;

pub fn lessons() -> Vec<SeedLesson> {
    let mut all = generics::lessons();
    all.extend(traits::lessons());
    all.extend(derives::lessons());
    all.extend(dispatch::lessons());
    all.extend(real_world::lessons());
    all.extend(model::lessons());
    all
}
