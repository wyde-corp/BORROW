//! Module 11 — Iterators, exercises for lessons 26–27.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L26 The Log Analyzer, Assembled
        SeedExercise::new(
            Some(25),
            ExerciseKind::Mcq,
            "In the assembled analyzer, grouping by level uses a `HashMap` loop rather than a chain. Why \
             is that not a mistake?",
            &[
                "Grouping must always use a loop",
                "Filtering and mapping read well as adapters, but grouping into a map reads better as a loop",
                "Iterators cannot build a `HashMap`",
            ],
            1,
            "`filter` and `map` express 'keep and transform' clearly. Grouping needs a mutable map with \
             the `entry` API, which reads better as a plain loop. Choosing per task — not chaining for \
             its own sake — is the skill.",
        ),

        // L27 Filtering, Counting, Top N
        SeedExercise::new(
            Some(26),
            ExerciseKind::PredictOutput,
            "Filter, then count. Predict the output.",
            &["2", "5", "3"],
            0,
            "The filter keeps the two numbers greater than three (`4, 9`), and `count` returns 2. No \
             collection is built — counting drives the pipeline directly.",
        )
        .with_code(
            "fn main() {\n    let nums = vec![4, 9, 2, 7, 5];\n    let big = nums.iter().filter(|n| **n > 3).count();\n    println!(\"{big}\");\n}",
        ),
    ]
}
