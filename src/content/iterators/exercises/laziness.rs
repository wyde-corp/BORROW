//! Module 11 — Iterators, exercises for lessons 19–22.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L19 Nothing Runs Until a Consumer
        SeedExercise::new(
            Some(18),
            ExerciseKind::Mcq,
            "A chain is built but never consumed. What happens to the `map` closure?",
            &[
                "It runs once for each element",
                "It never runs — adapters are lazy until a consumer pulls",
                "It runs once when the chain is built",
            ],
            1,
            "Adapters describe work and do nothing. Without a consumer — `collect`, `sum`, a `for` loop \
             — no element moves and the closure never runs. Rust even warns that the iterator is unused.",
        ),

        // L20 Infinite Iterators
        SeedExercise::new(
            Some(19),
            ExerciseKind::PredictOutput,
            "An infinite range is bounded by `take`. Predict the output.",
            &["[0, 1, 2]", "[1, 2, 3]", "It never finishes"],
            0,
            "`0..` is infinite, but `take(3)` pulls only three values and stops, so it prints `[0, 1, 2]`. \
             Laziness is what makes an infinite source safe to use.",
        )
        .with_code(
            "fn main() {\n    let first: Vec<u32> = (0..).take(3).collect();\n    println!(\"{first:?}\");\n}",
        ),

        // L21 Driving with next
        SeedExercise::new(
            Some(20),
            ExerciseKind::PredictOutput,
            "The loop breaks when the square exceeds 30. Predict the last value printed.",
            &["25", "36", "16"],
            0,
            "Squares are 1, 4, 9, 16, 25, 36. At 36 the condition breaks before printing, so the last \
             value printed is 25. Driving with `next` lets the loop choose when to stop.",
        )
        .with_code(
            "fn main() {\n    let mut it = (1..).map(|n| n * n);\n    while let Some(sq) = it.next() {\n        if sq > 30 {\n            break;\n        }\n        println!(\"{sq}\");\n    }\n}",
        ),

        // L22 An Iterator Is Exhausted Once
        SeedExercise::new(
            Some(21),
            ExerciseKind::PredictOutput,
            "The same iterator is collected twice. Predict the second result.",
            &["[1, 2, 3]", "[]", "[1]"],
            1,
            "The first `collect` drains the iterator, so the second finds it exhausted and produces an \
             empty vector. A fresh pass needs a new iterator from the collection.",
        )
        .with_code(
            "fn main() {\n    let mut it = vec![1, 2, 3].into_iter();\n    let a: Vec<i32> = it.by_ref().collect();\n    let b: Vec<i32> = it.collect();\n    println!(\"{a:?} {b:?}\");\n}",
        ),
    ]
}
