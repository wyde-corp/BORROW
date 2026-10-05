//! Module 09 — Option & Result, exercises for lessons 9–16.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L9 The Failing Operation
        SeedExercise::new(
            Some(8),
            ExerciseKind::Mcq,
            "Why is marking a task done better modelled as a `Result` than an `Option`?",
            &[
                "`Result` is shorter to write",
                "The operation can fail for a reason the caller should know, not just be absent",
                "`Option` cannot hold an error",
            ],
            1,
            "Finding a task is absence, so `Option` fits. Marking it done is an operation that can \
             fail, and the caller should learn *why* — the id did not exist. That reason is the `Err`.",
        ),

        // L10 Ok and Err
        SeedExercise::new(
            Some(9),
            ExerciseKind::PredictOutput,
            "Predict what this prints.",
            &["added, length 8", "could not add: title cannot be empty", "It does not compile"],
            0,
            "`\"buy milk\"` is not empty, so the function returns `Ok(8)` — the length of the title — \
             and the `Ok` arm prints `added, length 8`.",
        )
        .with_code(
            "fn add(title: &str) -> Result<u32, String> {\n    if title.is_empty() {\n        return Err(String::from(\"title cannot be empty\"));\n    }\n    Ok(title.len() as u32)\n}\n\nfn main() {\n    match add(\"buy milk\") {\n        Ok(n) => println!(\"added, length {n}\"),\n        Err(e) => println!(\"could not add: {e}\"),\n    }\n}",
        ),

        // L11 Reading the Error
        SeedExercise::new(
            Some(10),
            ExerciseKind::PredictOutput,
            "The error carries the missing id. Predict what this prints.",
            &["no task with id 7", "no task with id 0", "It does not compile"],
            0,
            "`TodoError::NotFound(7)` matches the `NotFound(id)` pattern, binding `id` to `7`, so the \
             message names it. Matching the error is how the program reacts to *which* failure \
             happened.",
        )
        .with_code(
            "#[derive(Debug)]\nenum TodoError { NotFound(u32), EmptyTitle }\n\nfn main() {\n    let err = TodoError::NotFound(7);\n    match err {\n        TodoError::NotFound(id) => println!(\"no task with id {id}\"),\n        TodoError::EmptyTitle => println!(\"a task needs a title\"),\n    }\n}",
        ),

        // L12 ? on Result
        SeedExercise::new(
            Some(11),
            ExerciseKind::PredictCompile,
            "The function returns `Result` and uses `?` on a `Result`. Predict the result.",
            &[
                "It compiles — `?` propagates the `Err` on failure and continues on `Ok`",
                "It does not compile — `?` only works in `main`",
            ],
            0,
            "`?` works in any function whose return type matches: `Result` for `?` on a `Result`. On \
             failure it returns the `Err` immediately; on success it hands back the value.",
        )
        .with_code(
            "#[derive(Debug)]\nenum E { NotFound }\n\nfn get(ok: bool) -> Result<u32, E> {\n    if !ok {\n        return Err(E::NotFound);\n    }\n    Ok(1)\n}\n\nfn run(ok: bool) -> Result<u32, E> {\n    let n = get(ok)?;\n    Ok(n + 1)\n}\n\nfn main() {\n    println!(\"{:?}\", run(true));\n    println!(\"{:?}\", run(false));\n}",
        ),

        // L13 map and map_err
        SeedExercise::new(
            Some(12),
            ExerciseKind::PredictOutput,
            "`map_err` rewrites the error. Predict what this prints.",
            &["Ok(6)", "Err(\"parse error: bad input\")", "Err(\"bad input\")"],
            1,
            "`map_err` touches the error only, so `Err(\"bad input\")` becomes \
             `Err(\"parse error: bad input\")`. The `Ok` value would be left alone, but there is none \
             here.",
        )
        .with_code(
            "fn main() {\n    let err: Result<u32, String> = Err(String::from(\"bad input\"));\n    let labelled = err.map_err(|e| format!(\"parse error: {e}\"));\n    println!(\"{:?}\", labelled);\n}",
        ),

        // L14 and_then and Chaining
        SeedExercise::new(
            Some(13),
            ExerciseKind::PredictOutput,
            "The first step fails. Predict what `and_then` returns.",
            &[
                "Ok(1) — the second step still runs",
                "Err(\"first failed\") — the chain stops at the first failure",
                "It does not compile",
            ],
            1,
            "`and_then` runs the next step only on `Ok`. With an `Err`, the closure is never called and \
             the original error is returned — the short-circuit that makes a chain safe.",
        )
        .with_code(
            "fn main() {\n    let r: Result<u32, String> = Err(String::from(\"first failed\"));\n    let out = r.and_then(|n| Ok(n + 1));\n    println!(\"{out:?}\");\n}",
        ),

        // L15 Defaults on Result
        SeedExercise::new(
            Some(14),
            ExerciseKind::PredictOutput,
            "A failed `Result` is given a default. Predict the output.",
            &["5", "0", "It panics"],
            1,
            "`err.unwrap_or(0)` returns the success value if present, else `0`. There is no success \
             here, so it prints `0`. Unlike `unwrap`, this does not panic.",
        )
        .with_code(
            "fn main() {\n    let err: Result<u32, String> = Err(String::from(\"bad\"));\n    println!(\"{}\", err.unwrap_or(0));\n}",
        ),

        // L16 Combining Option and Result
        SeedExercise::new(
            Some(15),
            ExerciseKind::FillGap,
            "A lookup returns an `Option` inside a fallible function. Fill the gap to turn the missing \
             case into the function's error.",
            &["ok_or", "unwrap_or", "ok"],
            0,
            "`ok_or(TodoError::NotFound(id))` bridges `Option` to `Result`, so the `?` can propagate it. \
             `ok` goes the other way (Result to Option); `unwrap_or` supplies a default.",
        )
        .with_code(
            "fn complete(tasks: &mut [Task], id: u32) -> Result<(), TodoError> {\n    let task = tasks.iter_mut().find(|t| t.id == id).____(TodoError::NotFound(id))?;\n    task.done = true;\n    Ok(())\n}",
        ),
    ]
}
