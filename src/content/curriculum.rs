use super::types::{SeedModule, SeedProject};

macro_rules! placeholder_module {
    ($fn_name:ident, $slug:literal, $title:literal, $description:literal) => {
        pub fn $fn_name() -> SeedModule {
            SeedModule::new(
                $slug,
                $title,
                $description,
                vec![],
                vec![],
                vec![SeedProject {
                    title: "Coming soon".to_string(),
                    description: "This project will be added when the module content is written.".to_string(),
                    difficulty: super::types::Difficulty::Beginner,
                    estimated_time: "—".to_string(),
                    requirements: vec![],
                    concepts: vec![],
                    constraints: vec![],
                    example_interaction: String::new(),
                    stretch_goals: vec![],
                }],
            )
        }
    };
}

placeholder_module!(
    iterators,
    "iterators",
    "Iterators",
    "Process sequences of data with Rust's iterator pattern."
);
