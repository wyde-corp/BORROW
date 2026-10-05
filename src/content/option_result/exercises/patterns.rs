//! Module 09 — Option & Result, exercises for lessons 23–26.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L23 Choosing the Shape
        SeedExercise::new(
            Some(22),
            ExerciseKind::FillGap,
            "The missing case is an early exit and the success path should stay flat. Fill in the two \
             keywords.",
            &["Some … else", "None … else", "Ok … match"],
            0,
            "`let Some(task) = ... else { return ... };` binds on success and bails out on failure, so \
             the rest of the function reads at one indent level. This is the `let else` shape.",
        )
        .with_code(
            "let ____(task) = tasks.iter().find(|t| t.id == id) ____ {\n    return String::from(\"(unknown)\");\n};",
        ),

        // L24 Early Return versus Nesting
        SeedExercise::new(
            Some(23),
            ExerciseKind::PredictCompile,
            "Two fallible steps are chained with `?`. Predict the result.",
            &[
                "It compiles — each `?` returns early on failure, else continues",
                "It does not compile — only one `?` is allowed per function",
            ],
            0,
            "A function may use `?` as many times as it has fallible steps. Each exits early on \
             failure; execution continues only when the step succeeded. This is what keeps the chain \
             flat.",
        )
        .with_code(
            "#[derive(Debug)]\nenum E { Bad }\n\nfn step(n: u32) -> Result<u32, E> {\n    if n == 0 { return Err(E::Bad); }\n    Ok(n)\n}\n\nfn two(a: u32, b: u32) -> Result<u32, E> {\n    let x = step(a)?;\n    let y = step(b)?;\n    Ok(x + y)\n}\n\nfn main() {\n    println!(\"{:?}\", two(1, 2));\n    println!(\"{:?}\", two(1, 0));\n}",
        ),

        // L25 Several Errors in One Function
        SeedExercise::new(
            Some(24),
            ExerciseKind::Mcq,
            "A function reads a file (IO error) and parses a number (parse error). How do both travel \
             out?",
            &[
                "Each function needs its own error type",
                "Both convert into the function's one error type — IO via `From`, parse via `map_err`",
                "The parse error must be a `String`",
            ],
            1,
            "The IO error converts automatically at `?` because of `From`; the parse error is converted \
             on the spot with `map_err`. Both leave as the same `TodoError`, so one `match` at the top \
             handles everything.",
        ),

        // L26 Stopping at the First Failure
        SeedExercise::new(
            Some(25),
            ExerciseKind::PredictOutput,
            "A list is collected into a `Result`. Predict what `parse_all` returns.",
            &[
                "Ok([1, 2, 4]) — the bad line is skipped",
                "Err(\"bad: x\") — it stops at the first failure",
                "Ok([1, 2, 0, 4]) — the bad line becomes 0",
            ],
            1,
            "Collecting into `Result<Vec<u32>, String>` stops at the first `Err`, so the whole call is \
             `Err(\"bad: x\")` and the `4` is never parsed. All-or-nothing is the point.",
        )
        .with_code(
            "fn parse_all(lines: &[&str]) -> Result<Vec<u32>, String> {\n    lines\n        .iter()\n        .map(|l| l.parse::<u32>().map_err(|_| format!(\"bad: {l}\")))\n        .collect()\n}\n\nfn main() {\n    println!(\"{:?}\", parse_all(&[\"1\", \"2\", \"x\", \"4\"]));\n}",
        ),
    ]
}
