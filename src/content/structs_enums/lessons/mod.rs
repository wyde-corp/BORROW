//! Module 07 — Structs & Enums lessons.

mod construction;
mod enums;
mod foundations;
mod matching;
mod methods;
mod model;
mod patterns;

use crate::content::types::SeedLesson;

pub fn lessons() -> Vec<SeedLesson> {
    let mut all = foundations::lessons();
    all.extend(construction::lessons());
    all.extend(methods::lessons());
    all.extend(enums::lessons());
    all.extend(matching::lessons());
    all.extend(patterns::lessons());
    all.extend(model::lessons());
    all
}
