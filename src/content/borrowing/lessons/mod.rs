//! Module 06 — Borrowing lessons.
//!
//! One lesson per PART of the module spec, grouped by the argument each cluster
//! makes: the problem and the mechanism first, then the rule, its duration, its
//! use at function boundaries, containers, and finally the reasoning that turns
//! the rules into prediction.

mod checker;
mod functions;
mod model;
mod problem;
mod rules;
mod scope;
mod slices;

use crate::content::types::SeedLesson;

pub fn lessons() -> Vec<SeedLesson> {
    let mut all = problem::lessons();
    all.extend(rules::lessons());
    all.extend(scope::lessons());
    all.extend(functions::lessons());
    all.extend(slices::lessons());
    all.extend(checker::lessons());
    all.extend(model::lessons());
    all
}
