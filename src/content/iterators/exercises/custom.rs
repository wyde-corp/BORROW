//! Module 11 — Iterators, exercises for lessons 23–25.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L23 Returning impl Iterator
        SeedExercise::new(
            Some(22),
            ExerciseKind::FillGap,
            "Fill the return type so the function hands back a lazy iterator of the error messages.",
            &["impl Iterator<Item = &'a str>", "Vec<&'a str>", "&'a str"],
            0,
            "`impl Iterator<Item = &'a str>` means 'some unnamed type that yields `&'a str`'. Returning a \
             `Vec` would force the work; returning `&str` would be a single value.",
        )
        .with_code(
            "fn error_messages<'a>(entries: &'a [LogEntry]) -> ____ {\n    entries.iter().filter(|e| e.is_error).map(|e| e.message.as_str())\n}",
        ),

        // L24 Writing an Iterator by Hand
        SeedExercise::new(
            Some(23),
            ExerciseKind::PredictOutput,
            "A hand-written countdown starts at 3. Predict the output.",
            &["3 2 1", "2 1 0", "3 2 1 0"],
            0,
            "`next` yields 3, then 2, then 1, and stops at 0. The state lives in `self`, and `next` \
             advances it each call.",
        )
        .with_code(
            "struct Countdown(u32);\n\nimpl Iterator for Countdown {\n    type Item = u32;\n    fn next(&mut self) -> Option<u32> {\n        if self.0 == 0 {\n            None\n        } else {\n            let current = self.0;\n            self.0 -= 1;\n            Some(current)\n        }\n    }\n}\n\nfn main() {\n    for n in Countdown(3) {\n        print!(\"{n} \");\n    }\n}",
        ),

        // L25 Common Mistakes
        SeedExercise::new(
            Some(24),
            ExerciseKind::PredictCompile,
            "A chain is built and discarded, with a `println!` inside the closure. Predict what runs.",
            &[
                "The closure runs for each element",
                "Only the outer print runs; the chain is never consumed",
            ],
            1,
            "The chain has no consumer, so it is lazy and the closure never runs. Only the outer \
             `println!` prints. Adding `.collect::<Vec<_>>()` or a `for` loop would drive it.",
        )
        .with_code(
            "fn main() {\n    let nums = vec![1, 2, 3];\n    nums.iter().map(|n| {\n        println!(\"mapping {n}\");\n        n * 2\n    });\n    println!(\"done\");\n}",
        ),
    ]
}
