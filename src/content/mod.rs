pub mod borrowing;
pub mod collections;
pub mod control_flow;
mod curriculum;
pub mod functions;
pub mod getting_started;
pub mod ownership;
pub mod option_result;
pub mod structs_enums;
mod types;
#[cfg(test)]
mod tests;
pub mod variables_and_types;

pub use curriculum::{generics_traits, iterators};

pub use types::SeedModule;

use crate::models::{Exercise, Lesson, Module, Project};

pub struct ModuleSeed {
    pub module: Module,
    pub lessons: Vec<Lesson>,
    pub exercises: Vec<Exercise>,
    pub projects: Vec<Project>,
}

pub fn all() -> Vec<ModuleSeed> {
    let seeds: Vec<SeedModule> = vec![
        getting_started::content(),
        variables_and_types::content(),
        control_flow::content(),
        functions::content(),
        ownership::content(),
        borrowing::content(),
        structs_enums::content(),
        collections::content(),
        option_result::content(),
        generics_traits(),
        iterators(),
    ];

    seeds.into_iter().flat_map(|m| m.into_seeds()).collect()
}

impl SeedModule {
    pub fn into_seeds(self) -> Vec<ModuleSeed> {
        let module_id = id_for_slug(self.slug);
        let module = self.to_module(module_id, module_id);

        let lesson_base = lesson_id_base(module_id);
        let exercise_base = exercise_id_base(module_id);
        let project_base = project_id_base(module_id);

        vec![ModuleSeed {
            module,
            lessons: self.to_lessons(module_id, lesson_base),
            exercises: self.to_exercises(module_id, lesson_base, exercise_base),
            projects: self.to_projects(module_id, project_base),
        }]
    }
}

pub fn id_for_slug(slug: &str) -> i64 {
    crate::content::registry::MODULES
        .iter()
        .position(|m| *m == slug)
        .map(|i| i as i64 + 1)
        .unwrap_or(1)
}

/// The per-module ID block. Each module gets `SLOTS` consecutive ids for each
/// kind, so a module's lessons, exercises, and projects never collide with the
/// next module's.
///
/// This has to stay larger than the biggest module in the curriculum. It was
/// originally 100, which silently collided the moment Ownership grew past it:
/// Ownership's exercises run into the 500–600 range and Borrowing's begin at
/// 601, so seeding a fresh database failed on `UNIQUE constraint failed:
/// exercises.id`. A module in this course can reasonably carry a few hundred
/// exercises, so the block is generous on purpose.
const SLOTS: i64 = 1000;

pub fn lesson_id_base(module_id: i64) -> i64 {
    module_id * SLOTS
}

pub fn exercise_id_base(module_id: i64) -> i64 {
    module_id * SLOTS
}

pub fn project_id_base(module_id: i64) -> i64 {
    module_id * SLOTS
}

pub mod registry {
    pub const MODULES: [&str; 11] = [
        "getting-started",
        "variables-and-types",
        "control-flow",
        "functions",
        "ownership",
        "borrowing",
        "structs-and-enums",
        "collections",
        "option-and-result",
        "generics-and-traits",
        "iterators",
    ];
}
