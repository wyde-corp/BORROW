//! Module 09 — Option & Result lessons.

mod errors;
mod model;
mod option;
mod patterns;
mod real_world;
mod result;

use crate::content::types::SeedLesson;

pub fn lessons() -> Vec<SeedLesson> {
    let mut all = option::lessons();
    all.extend(result::lessons());
    all.extend(errors::lessons());
    all.extend(patterns::lessons());
    all.extend(real_world::lessons());
    all.extend(model::lessons());
    all
}
