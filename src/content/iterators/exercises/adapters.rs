//! Module 11 — Iterators, exercises for lessons 7–12.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L7 map
        SeedExercise::new(
            Some(6),
            ExerciseKind::FillGap,
            "Fill the gap with the adapter that transforms each element.",
            &["map", "filter", "take"],
            0,
            "`map` transforms each element one-for-one. `filter` keeps or drops; `take` bounds the count.",
        )
        .with_code("let doubled: Vec<i32> = nums.iter().____(|n| n * 2).collect();"),

        // L8 filter
        SeedExercise::new(
            Some(7),
            ExerciseKind::PredictOutput,
            "Only even numbers pass the predicate. Predict the output.",
            &["[2, 4]", "[1, 2, 3, 4]", "[1, 3]"],
            0,
            "`filter` keeps the elements where the closure is `true` — the evens — so the result is \
             `[2, 4]`. Unlike `map`, it can shrink the stream.",
        )
        .with_code(
            "fn main() {\n    let nums = [1, 2, 3, 4];\n    let evens: Vec<i32> = nums.iter().filter(|n| *n % 2 == 0).copied().collect();\n    println!(\"{evens:?}\");\n}",
        ),

        // L9 take and skip
        SeedExercise::new(
            Some(8),
            ExerciseKind::PredictOutput,
            "`skip(1)` then `take(2)` is applied. Predict the output.",
            &["[20, 30]", "[10, 20]", "[10, 40]"],
            0,
            "`skip(1)` drops `10`, then `take(2)` yields the next two: `20, 30`. Together they select a \
             window in the middle of the stream.",
        )
        .with_code(
            "fn main() {\n    let nums = vec![10, 20, 30, 40, 50];\n    let window: Vec<i32> = nums.iter().skip(1).take(2).copied().collect();\n    println!(\"{window:?}\");\n}",
        ),

        // L10 enumerate
        SeedExercise::new(
            Some(9),
            ExerciseKind::FillGap,
            "Fill the gaps: the adapter and the index binding.",
            &["enumerate, i", "zip, i", "count, n"],
            0,
            "`enumerate` pairs each element with its index, yielding `(i, item)`, so the destructuring \
             binds `i` and `item`. `zip` pairs two different sources.",
        )
        .with_code("for (____, item) in items.iter().____() {\n    println!(\"{i}: {}\", item.name);\n}"),

        // L11 zip and chain
        SeedExercise::new(
            Some(10),
            ExerciseKind::PredictOutput,
            "`zip` stops at the shorter side. Predict the output.",
            &["[(1, \"a\"), (2, \"b\")]", "[(1, \"a\"), (2, \"b\"), (3, \"\")]", "It does not compile"],
            0,
            "`zip` yields pairs until the shorter iterator ends. The left side has two elements, so the \
             third number is never paired — the result is two pairs.",
        )
        .with_code(
            "fn main() {\n    let nums = [1, 2, 3];\n    let names = [\"a\", \"b\"];\n    let pairs: Vec<_> = nums.iter().zip(names.iter()).collect();\n    println!(\"{pairs:?}\");\n}",
        ),

        // L12 filter_map and flat_map
        SeedExercise::new(
            Some(11),
            ExerciseKind::PredictOutput,
            "`filter_map` keeps only the `Some`s. Predict the output.",
            &["[10, 20]", "[1, 2, 3]", "[10, 11, 20]"],
            0,
            "The closure returns `Some(n)` only for `1` and `2`, mapping them to `10` and `20`; the rest \
             are `None` and dropped. `filter_map` did a filter and a map in one call.",
        )
        .with_code(
            "fn main() {\n    let nums = [1, 2, 3];\n    let out: Vec<i32> = nums.iter().filter_map(|n| if *n < 3 { Some(n * 10) } else { None }).collect();\n    println!(\"{out:?}\");\n}",
        ),
    ]
}
