pub mod getting_started;
pub mod control_flow;
pub mod functions;
pub mod ownership;
pub mod borrowing;
pub mod structs_enums;
pub mod collections;
pub mod error_handling;
pub mod generics_traits;
pub mod iterators;

#[derive(Clone)]
pub struct Lesson {
    pub title: String,
    pub content: Vec<LessonBlock>,
    pub starter_code: String,
    pub project: Option<Project>,
}

#[derive(Clone)]
pub enum LessonBlock {
    Text(String),
    Code(String),
    Heading(String),
    Tip(String),
    Warning(String),
    Exercise(Exercise),
}

#[derive(Clone)]
pub enum Exercise {
    MultipleChoice {
        question: String,
        options: Vec<String>,
        correct: usize,
    },
    PredictOutput {
        code: String,
        options: Vec<String>,
        correct: usize,
    },
    FixCode {
        broken_code: String,
        fixed_code: String,
        hint: String,
    },
}

#[derive(Clone)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub requirements: Vec<String>,
}

#[derive(Clone)]
pub struct Module {
    pub number: &'static str,
    pub title: &'static str,
    pub lessons: Vec<Lesson>,
}

pub fn all_modules() -> Vec<Module> {
    vec![
        getting_started::module(),
        control_flow::module(),
        functions::module(),
        ownership::module(),
        borrowing::module(),
        structs_enums::module(),
        collections::module(),
        error_handling::module(),
        generics_traits::module(),
        iterators::module(),
    ]
}
