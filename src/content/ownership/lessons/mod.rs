//! Module 05 — Ownership.
//!
//! Assembles the lesson set from its parts. The lessons are grouped by the
//! argument they are making rather than by topic, because ownership only
//! becomes coherent once move, copy, clone, scope, and function calls are
//! already in place.

mod foundations;
mod in_code;
mod mental_models;
mod moving;

use crate::content::types::SeedLesson;

pub fn lessons() -> Vec<SeedLesson> {
    let mut all = foundations::lessons();
    all.extend(moving::lessons());
    all.extend(in_code::lessons());
    all.extend(mental_models::lessons());
    all
}
