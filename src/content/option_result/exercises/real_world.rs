//! Module 09 — Option & Result, exercises for lessons 27–29.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L27 Parsing a Todo Line
        SeedExercise::new(
            Some(26),
            ExerciseKind::PredictOutput,
            "A line has the wrong number of parts. Predict what `parse_line` returns.",
            &[
                "Err(Parse(\"oops\")) — it carries the offending line",
                "It panics",
                "Ok with an empty task",
            ],
            0,
            "The parts count is not 3, so it returns `Err(TodoError::Parse(\"oops\"))`. Carrying the \
             original line in the error lets the message quote exactly what failed.",
        )
        .with_code(
            "#[derive(Debug)]\nenum TodoError { Parse(String) }\n\n#[derive(Debug)]\nstruct Task { id: u32, title: String, done: bool }\n\nfn parse_line(line: &str) -> Result<Task, TodoError> {\n    let parts: Vec<&str> = line.split('|').collect();\n    if parts.len() != 3 {\n        return Err(TodoError::Parse(line.to_string()));\n    }\n    let id = parts[0].trim().parse::<u32>().map_err(|_| TodoError::Parse(line.to_string()))?;\n    Ok(Task { id, title: parts[1].trim().to_string(), done: parts[2].trim() == \"done\" })\n}\n\nfn main() {\n    println!(\"{:?}\", parse_line(\"oops\"));\n}",
        ),

        // L28 Loading and Saving
        SeedExercise::new(
            Some(27),
            ExerciseKind::Mcq,
            "What does `?` do with an `io::Error` when the function returns `Result<_, TodoError>`?",
            &[
                "It panics on the IO error",
                "It converts the `io::Error` into a `TodoError` via `From`, then returns it",
                "It requires you to convert it manually with `map_err`",
            ],
            1,
            "`?` performs the early return *and* the conversion. With `impl From<io::Error> for \
             TodoError`, the conversion is automatic, which is why file operations can use `?` \
             directly.",
        ),

        // L29 The Todo CLI, Assembled
        SeedExercise::new(
            Some(28),
            ExerciseKind::Mcq,
            "In the assembled CLI, `find` returns `Option` while `mark_done` returns `Result`. Why the \
             difference?",
            &[
                "It is a mistake — both should return `Result`",
                "A lookup missing is absence; marking done is a failure with a reason, and the reason \
                 matters to the user",
                "`Option` is faster than `Result`",
            ],
            1,
            "The two types describe two different situations. 'Is this task here?' is absence. 'Mark \
             this task done' is an operation that can fail, and the user deserves to know the id was \
             not found. Placing the two correctly is the skill.",
        ),
    ]
}
