//! Module 07 — Structs & Enums, exercises for lessons 21–26.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L21 The Magic-String Problem
        SeedExercise::new(
            Some(20),
            ExerciseKind::Mcq,
            "Why is storing an order status as a `String` a problem?",
            &[
                "Strings cannot be compared",
                "The type allows infinitely many values, so a typo or nonsense status is never caught",
                "Strings are slower than enums",
            ],
            1,
            "The legal statuses are a fixed set, but `String` accepts anything, and a typo compares \
             unequal without complaint. An enum makes the set the type, so a wrong value will not \
             compile.",
        ),

        // L22 Defining an Enum
        SeedExercise::new(
            Some(21),
            ExerciseKind::PredictCompile,
            "A variant is misspelled. Predict what the compiler does.",
            &[
                "It compiles and treats the typo as a new variant",
                "It does not compile — there is no such variant, and the error suggests the closest one",
            ],
            1,
            "Only declared variants exist. A misspelling is a compile error, which is exactly the \
             protection a `String` status could not give. The error suggests the intended variant.",
        )
        .with_code(
            "#[derive(Debug)]\nenum Size { Small, Medium, Large }\n\nfn main() {\n    let s = Size::Meduim;\n    println!(\"{s:?}\");\n}",
        ),

        // L23 Variants Can Carry Data
        SeedExercise::new(
            Some(22),
            ExerciseKind::Mcq,
            "What makes `enum OrderStatus { Placed, Preparing { since_minutes: u32 } }` more than a \
             set of constants?",
            &[
                "It stores the status as an integer",
                "Each variant can carry exactly the data that variant needs",
                "It allows two variants at once",
            ],
            1,
            "A value is still exactly one variant, but a variant may hold data — so `Preparing` has a \
             time and `Placed` does not. Illegal combinations, like a `Placed` status with a \
             preparation time, cannot be constructed.",
        ),

        // L24 Enums vs Structs
        SeedExercise::new(
            Some(23),
            ExerciseKind::Mcq,
            "A payment is either Cash or Card { last_four: String }. Struct or enum?",
            &[
                "Struct — it has several fields",
                "Enum — it is exactly one of a fixed set, and one variant carries data",
                "Tuple — two values",
            ],
            1,
            "It is one of two possibilities, and `Card` carries data while `Cash` does not — the \
             definition of a good enum. 'All of these together' would be a struct; 'one of these' is \
             an enum.",
        ),

        // L25 Option Is an Enum
        SeedExercise::new(
            Some(24),
            ExerciseKind::Mcq,
            "What is `Option<T>` actually?",
            &[
                "A special language feature with no equivalent",
                "An ordinary enum with two variants, `None` and `Some(T)`",
                "A pointer that may be null",
            ],
            1,
            "`Option` is defined in the standard library as an enum with `None` and `Some(T)`. Every \
             skill you have for enums applies to it directly, which is why Module 09 can build on it \
             immediately.",
        ),

        // L26 Methods on Enums
        SeedExercise::new(
            Some(25),
            ExerciseKind::PredictOutput,
            "A method matches the variant and ignores its data with `..`. Predict what this prints.",
            &[
                "preparing",
                "Preparing { since_minutes: 10 }",
                "It does not compile — `match` needs the data",
            ],
            0,
            "`OrderStatus::Preparing { .. }` matches the variant and ignores `since_minutes`, so the \
             arm returns `\"preparing\"`. Methods on enums are ordinary `impl` blocks with a `match` \
             that asks which variant the value is.",
        )
        .with_code(
            "#[derive(Debug)]\nenum OrderStatus {\n    Placed,\n    Preparing { since_minutes: u32 },\n}\n\nimpl OrderStatus {\n    fn label(&self) -> &str {\n        match self {\n            OrderStatus::Placed => \"placed\",\n            OrderStatus::Preparing { .. } => \"preparing\",\n        }\n    }\n}\n\nfn main() {\n    let s = OrderStatus::Preparing { since_minutes: 10 };\n    println!(\"{}\", s.label());\n}",
        ),
    ]
}
