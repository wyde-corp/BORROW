//! Module 06 — Borrowing exercises.
//!
//! One exercise per lesson, in lesson order. The mix follows the spec: predict
//! the compiler, fix the borrow, trace the borrow, and decide an API — with the
//! two new formats (`trace_borrow`, `api_design`) carrying the module's
//! reasoning questions.

mod checker;
mod functions;
mod model;
mod problem;
mod rules;
mod scope;
mod slices;

use crate::content::types::SeedExercise;

pub fn exercises() -> Vec<SeedExercise> {
    let mut all = problem::exercises();
    all.extend(rules::exercises());
    all.extend(scope::exercises());
    all.extend(functions::exercises());
    all.extend(slices::exercises());
    all.extend(checker::exercises());
    all.extend(model::exercises());
    all
}
