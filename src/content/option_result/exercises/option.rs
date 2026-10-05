//! Module 09 — Option & Result, exercises for lessons 1–8.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L1 The Missing Task
        SeedExercise::new(
            Some(0),
            ExerciseKind::Mcq,
            "`find` returns `Option<&Task>` and the id is not present. What is the right response in a \
             to-do list?",
            &[
                "Panic, because a missing id is a bug",
                "Handle the `None` case and tell the user",
                "Return an empty `Task`",
            ],
            1,
            "A missing id is an ordinary outcome a user can cause, not a bug. The type tells you it can \
             happen, and the program should report it rather than crash.",
        ),

        // L2 Option Is a Value
        SeedExercise::new(
            Some(1),
            ExerciseKind::PredictOutput,
            "Predict what this prints.",
            &["Some(3)", "3", "None"],
            0,
            "`Some(3)` is a value of type `Option<i32>` and prints as `Some(3)`. An `Option` always \
             exists; it just may not contain a value.",
        )
        .with_code("fn main() {\n    let x: Option<i32> = Some(3);\n    println!(\"{:?}\", x);\n}"),

        // L3 Getting the Value Out
        SeedExercise::new(
            Some(2),
            ExerciseKind::FillGap,
            "Fill the gap with the pattern that matches a present value.",
            &["Some(title)", "None", "Ok(title)"],
            0,
            "`if let Some(title) = found` matches the present variant and binds its value. `None` \
             matches the absent one; `Ok` is for `Result`.",
        )
        .with_code("if let ____ = found {\n    println!(\"found: {title}\");\n}"),

        // L4 unwrap and expect
        SeedExercise::new(
            Some(3),
            ExerciseKind::PredictOutput,
            "An `Option` is `None` and `unwrap` is called. Predict what happens.",
            &[
                "Prints nothing",
                "Panics at runtime and ends the program",
                "Does not compile",
            ],
            1,
            "`unwrap` compiles and panics on `None`. The panic ends the program — the exact outcome the \
             `Option` type was warning about. `expect` does the same with a custom message.",
        )
        .with_code(
            "fn main() {\n    let found: Option<&str> = None;\n    println!(\"{}\", found.unwrap());\n}",
        ),

        // L5 Defaults
        SeedExercise::new(
            Some(4),
            ExerciseKind::FillGap,
            "Fill the gap with the method that gives the value or a substitute, without panicking.",
            &["unwrap_or", "unwrap", "expect"],
            0,
            "`unwrap_or(\"(no title)\")` gives the value if present and the substitute if not. `unwrap` \
             and `expect` panic instead — the opposite of what a default is for.",
        )
        .with_code("let title = found.____(\"(no title)\");"),

        // L6 map and and_then
        SeedExercise::new(
            Some(5),
            ExerciseKind::PredictOutput,
            "`map` transforms the value inside a `None`. Predict the output.",
            &["None", "0", "It panics"],
            0,
            "`map` on `None` applies nothing and stays `None`. The transformation runs only for a \
             `Some`, which is why you can chain `map` calls without handling the missing case between \
             them.",
        )
        .with_code(
            "fn main() {\n    let x: Option<i32> = None;\n    println!(\"{:?}\", x.map(|n| n * 2));\n}",
        ),

        // L7 ? on Option
        SeedExercise::new(
            Some(6),
            ExerciseKind::PredictCompile,
            "The function returns a plain `&str` and uses `?` on an `Option`. Predict the result.",
            &[
                "It compiles and returns an empty string on `None`",
                "It does not compile — `?` needs the function to return `Option` or `Result`",
            ],
            1,
            "`?` has to return something on the missing case, and a plain `&str` has nothing to return. \
             The function's return type must be `Option` (or `Result`) for `?` to be allowed.",
        )
        .with_code(
            "struct Task { id: u32, title: String }\n\nfn title(tasks: &[Task], id: u32) -> &str {\n    let task = tasks.iter().find(|t| t.id == id)?;\n    task.title.as_str()\n}",
        ),

        // L8 Turning None into an Error
        SeedExercise::new(
            Some(7),
            ExerciseKind::FillGap,
            "Fill the gap with the method that turns a missing value into a specific error.",
            &["ok_or", "unwrap_or", "map"],
            0,
            "`ok_or(TodoError::NotFound(id))` converts `Some` into `Ok` and `None` into \
             `Err(NotFound(id))`. `unwrap_or` supplies a default instead of an error; `map` transforms \
             the value.",
        )
        .with_code(
            "let task = tasks.iter().find(|t| t.id == id).____(TodoError::NotFound(id))?;",
        ),
    ]
}
