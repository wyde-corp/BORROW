//! Module 11 — Iterators, exercises for lessons 13–18.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L13 collect
        SeedExercise::new(
            Some(12),
            ExerciseKind::FillGap,
            "Fill the gap with the target type that makes `collect` build a de-duplicated set.",
            &["HashSet<&str>", "Vec<&str>", "String"],
            0,
            "The type on the binding tells `collect` what to build. `HashSet<&str>` de-duplicates; \
             `Vec<&str>` keeps every element; `String` would need the items to be characters or \
             strings to concatenate.",
        )
        .with_code(
            "let unique: ____ = sources.iter().map(|s| s.as_str()).collect();",
        ),

        // L14 sum and product
        SeedExercise::new(
            Some(13),
            ExerciseKind::PredictOutput,
            "Predict the sum.",
            &["60", "6", "123"],
            0,
            "`sum()` adds `10 + 20 + 30 = 60`. It is the accumulator loop you would have written, with \
             the total handled for you.",
        )
        .with_code(
            "fn main() {\n    let total: i32 = vec![10, 20, 30].into_iter().sum();\n    println!(\"{total}\");\n}",
        ),

        // L15 count, any, all
        SeedExercise::new(
            Some(14),
            ExerciseKind::PredictOutput,
            "Predict both booleans.",
            &["true and true", "true and false", "false and false"],
            1,
            "`any` is `true` because at least one number is even. `all` is `false` because `1` and `3` \
             are not. Both short-circuit as soon as the answer is known.",
        )
        .with_code(
            "fn main() {\n    let nums = [1, 2, 3];\n    println!(\"{} {}\", nums.iter().any(|n| n % 2 == 0), nums.iter().all(|n| n % 2 == 0));\n}",
        ),

        // L16 find and position
        SeedExercise::new(
            Some(15),
            ExerciseKind::FillGap,
            "Fill the gap with the consumer that returns the first match as an `Option`.",
            &["find", "position", "filter"],
            0,
            "`find` returns the first matching element, wrapped in `Option`. `position` returns an \
             index; `filter` is an adapter that keeps all matches.",
        )
        .with_code(
            "let first: Option<&LogEntry> = entries.iter().____(|e| e.is_error);",
        ),

        // L17 fold
        SeedExercise::new(
            Some(16),
            ExerciseKind::PredictOutput,
            "`fold` starts at 0 and adds each element. Predict the output.",
            &["6", "0", "10"],
            0,
            "`fold(0, |acc, n| acc + n)` adds `1 + 2 + 3 = 6`. It is exactly what `sum` would produce, \
             spelled out as the general reduce.",
        )
        .with_code(
            "fn main() {\n    let total = vec![1, 2, 3].into_iter().fold(0, |acc, n| acc + n);\n    println!(\"{total}\");\n}",
        ),

        // L18 max_by, min_by, last
        SeedExercise::new(
            Some(17),
            ExerciseKind::PredictOutput,
            "`max_by` compares by the value. Predict the picked number.",
            &["30", "10", "20"],
            0,
            "`max_by` returns the greatest element per the comparator, which here compares the numbers \
             themselves, so it picks `30`. The result is an `Option`.",
        )
        .with_code(
            "fn main() {\n    let nums = [10, 30, 20];\n    let biggest = nums.iter().max_by(|a, b| a.cmp(b));\n    println!(\"{biggest:?}\");\n}",
        ),
    ]
}
