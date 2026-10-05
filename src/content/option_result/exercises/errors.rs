//! Module 09 — Option & Result, exercises for lessons 17–22.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L17 One Error Type for the Program
        SeedExercise::new(
            Some(16),
            ExerciseKind::Mcq,
            "Why define one `TodoError` enum instead of using a `String` for every failure?",
            &[
                "A `String` cannot be printed",
                "Callers can match the enum and react to different failures",
                "An enum is shorter to write",
            ],
            1,
            "A `String` can be printed but not matched. An enum lets a caller tell 'not found' from \
             'could not read the file' and respond differently — retry one, report the other.",
        ),

        // L18 From and the ? Conversion
        SeedExercise::new(
            Some(17),
            ExerciseKind::FillGap,
            "Teach `?` how to convert an IO error. Fill in the trait and its method.",
            &[
                "From, from",
                "Into, into",
                "Display, fmt",
            ],
            0,
            "`impl From<io::Error> for TodoError { fn from(err: io::Error) -> TodoError { ... } }` is \
             exactly what `?` looks for. If it is missing, `?` refuses to convert and the function \
             will not compile.",
        )
        .with_code(
            "impl ____<io::Error> for TodoError {\n    fn ____(err: io::Error) -> TodoError {\n        TodoError::Io(err)\n    }\n}",
        ),

        // L19 Printing an Error
        SeedExercise::new(
            Some(18),
            ExerciseKind::Mcq,
            "Which format and trait show an error to a *user* as readable text?",
            &[
                "`{err}` using `Display`",
                "`{err:?}` using `Debug`",
                "`{err:#?}` using `Debug`",
            ],
            0,
            "`{}` uses `Display`, the trait for user-facing text. `{:?}` is `Debug`, meant for \
             developers and full of implementation detail.",
        ),

        // L20 Errors That Carry Data
        SeedExercise::new(
            Some(19),
            ExerciseKind::Mcq,
            "Why is `NotFound(u32)` better than a bare `NotFound`?",
            &[
                "It uses less memory",
                "It carries the id that was missing, so the message can name it",
                "It is required by the compiler",
            ],
            1,
            "The carried id turns 'no task found' into 'no task with id 7' — a message the user can \
             act on. Add data when it makes the message specific.",
        ),

        // L21 Box<dyn Error>
        SeedExercise::new(
            Some(20),
            ExerciseKind::Mcq,
            "What is the trade-off of `Box<dyn Error>`?",
            &[
                "It holds any error, but you cannot match on the specific kind",
                "It is faster but holds only one error type",
                "It requires implementing `Error` for every type by hand",
            ],
            0,
            "`Box<dyn Error>` can hold anything implementing `Error`, and `?` converts into it. The \
             cost is that at compile time it is just 'an error', so a caller cannot match the specific \
             failure — which is why the Todo CLI uses its own enum.",
        ),

        // L22 When to Panic
        SeedExercise::new(
            Some(21),
            ExerciseKind::Mcq,
            "The user types an id that is not in the list. Return or panic?",
            &[
                "Panic — it is an invalid id",
                "Return a `Result` — it is expected, and the user can fix it",
            ],
            1,
            "Expected failures caused by outside input are returned, not panicked. Panics are for \
             broken invariants — states the code itself promised could never happen.",
        ),
    ]
}
