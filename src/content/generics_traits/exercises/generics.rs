//! Module 10 — Generics & Traits, exercises for lessons 1–7.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L1 The Duplicated Function
        SeedExercise::new(
            Some(0),
            ExerciseKind::Mcq,
            "Two functions are identical except for the type (`i32` vs `f64`). What does that tell you?",
            &[
                "Rust needs one function per type",
                "The body does not depend on the type — a generic function would remove the duplication",
                "You should use macros instead",
            ],
            1,
            "Identical bodies over different types are the exact case generics exist for. The body only \
             needs comparison and copying, so a bound expresses what it requires.",
        ),

        // L2 Generic Functions
        SeedExercise::new(
            Some(1),
            ExerciseKind::FillGap,
            "Fill the gap with what the body needs `T` to be able to do.",
            &["PartialOrd + Copy", "Debug", "Clone + Display"],
            0,
            "The body uses `>` (from `PartialOrd`) and copies elements with `for &item` (from `Copy`), \
             so both bounds are required. `Debug` and `Display` are about printing, which this body \
             does not do.",
        )
        .with_code("fn largest<T: ____>(list: &[T]) -> T {\n    let mut largest = list[0];\n    for &item in list {\n        if item > largest {\n            largest = item;\n        }\n    }\n    largest\n}"),

        // L3 Generic Structs and Enums
        SeedExercise::new(
            Some(2),
            ExerciseKind::PredictOutput,
            "A generic `Point<T>` is built with integers. Predict what prints.",
            &["1 2", "Point { x: 1, y: 2 }", "It does not compile"],
            0,
            "`Point { x: 1, y: 2 }` is a `Point<i32>`; the compiler infers `T` from the values. The \
             fields print as `1 2`. Generics do not change what field access does.",
        )
        .with_code(
            "struct Point<T> { x: T, y: T }\n\nfn main() {\n    let p = Point { x: 1, y: 2 };\n    println!(\"{} {}\", p.x, p.y);\n}",
        ),

        // L4 More Than One Parameter
        SeedExercise::new(
            Some(3),
            ExerciseKind::FillGap,
            "Fill the gap so the two halves of the pair may be different types.",
            &["T, U", "T", "T, T"],
            0,
            "`Pair<T, U>` declares two independent type parameters, so `first` and `second` can differ. \
             `Pair<T>` would force them to be the same type.",
        )
        .with_code("struct Pair<____> {\n    first: T,\n    second: U,\n}"),

        // L5 Monomorphisation
        SeedExercise::new(
            Some(4),
            ExerciseKind::Mcq,
            "What does monomorphisation do to a generic function?",
            &[
                "Adds a run-time type check on every call",
                "Produces a concrete copy of the function for each type used",
                "Stores the function once and looks it up in a table",
            ],
            1,
            "The compiler emits one concrete function per type — `largest::<i32>`, `largest::<f64>`, and \
             so on. The calls are direct and there is no run-time lookup, which is why generics are \
             called zero-cost. A table lookup is dynamic dispatch, the `dyn Trait` path.",
        ),

        // L6 where Clauses
        SeedExercise::new(
            Some(5),
            ExerciseKind::FillGap,
            "Fill the gap with the keyword that introduces the bounds below the signature.",
            &["where", "impl", "for"],
            0,
            "`where` moves the bounds below the signature so they line up. `impl` introduces an \
             implementation; `for` connects a trait to a type.",
        )
        .with_code("fn largest<T>(list: &[T]) -> T\n____\n    T: PartialOrd + Copy,\n{\n    list[0]\n}"),

        // L7 Generics in Methods
        SeedExercise::new(
            Some(6),
            ExerciseKind::PredictCompile,
            "A method is defined only on `Point<f64>` and called on a `Point<i32>`. Predict the result.",
            &[
                "It compiles — the method works on any `Point`",
                "It does not compile — `distance_from_origin` is not implemented for `Point<i32>`",
            ],
            1,
            "`impl Point<f64>` attaches the method only to `Point<f64>`. A `Point<i32>` does not have it, \
             so the call is an error. A generic `impl<T> Point<T>` would provide it for all `T`.",
        )
        .with_code(
            "struct Point<T> { x: T, y: T }\n\nimpl Point<f64> {\n    fn distance_from_origin(&self) -> f64 {\n        (self.x * self.x + self.y * self.y).sqrt()\n    }\n}\n\nfn main() {\n    let p = Point { x: 3, y: 4 };\n    println!(\"{}\", p.distance_from_origin());\n}",
        ),
    ]
}
