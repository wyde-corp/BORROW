//! Module 08 — Collections, exercises for lessons 21–26.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L21 Uniqueness
        SeedExercise::new(
            Some(20),
            ExerciseKind::PredictOutput,
            "Three values are inserted, one of them twice. Predict the length.",
            &["2", "3", "1"],
            0,
            "A `HashSet` ignores a value it already holds, so the repeated \"Folk\" changes nothing. \
             Two distinct genres remain. Uniqueness is enforced on insert, not on read.",
        )
        .with_code(
            "use std::collections::HashSet;\n\nfn main() {\n    let mut genres: HashSet<String> = HashSet::new();\n    genres.insert(String::from(\"Folk\"));\n    genres.insert(String::from(\"Folk\"));\n    genres.insert(String::from(\"Alternative\"));\n    println!(\"{}\", genres.len());\n}",
        ),

        // L22 insert and contains
        SeedExercise::new(
            Some(21),
            ExerciseKind::PredictOutput,
            "The same value is inserted twice. Predict what the two booleans are.",
            &["true false", "false true", "true true"],
            0,
            "`insert` returns `true` when the value was new and `false` when it was already present. \
             The first returns `true`, the second `false`. That is how you detect a duplicate without \
             a separate `contains`.",
        )
        .with_code(
            "use std::collections::HashSet;\n\nfn main() {\n    let mut s: HashSet<&str> = HashSet::new();\n    let a = s.insert(\"Folk\");\n    let b = s.insert(\"Folk\");\n    println!(\"{a} {b}\");\n}",
        ),

        // L23 Iterating a Set
        SeedExercise::new(
            Some(22),
            ExerciseKind::Mcq,
            "In what order does iterating a `HashSet` produce its items?",
            &[
                "Insertion order",
                "No guaranteed order",
                "Alphabetical order",
            ],
            1,
            "A `HashSet`, like a `HashMap`, is unordered. Items come out in a hash-dependent order. \
             If you need a stable order, collect into a `Vec` and sort, or use a `BTreeSet`.",
        ),

        // L24 Set Operations
        SeedExercise::new(
            Some(23),
            ExerciseKind::PredictOutput,
            "Two sets share one genre. Predict how many items the intersection has.",
            &["1", "2", "3"],
            0,
            "`a.intersection(&b)` yields only the items present in both sets. `{Folk, Rock}` and \
             `{Rock, Jazz}` share `Rock`, so the intersection has one item.",
        )
        .with_code(
            "use std::collections::HashSet;\n\nfn main() {\n    let a: HashSet<&str> = [\"Folk\", \"Rock\"].into_iter().collect();\n    let b: HashSet<&str> = [\"Rock\", \"Jazz\"].into_iter().collect();\n    let shared: Vec<&&str> = a.intersection(&b).collect();\n    println!(\"{}\", shared.len());\n}",
        ),

        // L25 De-duplicating a Vec
        SeedExercise::new(
            Some(24),
            ExerciseKind::PredictOutput,
            "A list with repeats is collected through a set and back. Predict the unique count.",
            &["3", "5", "2"],
            0,
            "The list has three distinct values: `Folk`, `Rock`, `Jazz`. Collecting through a \
             `HashSet` drops the repeats, so the unique list has three. Note the order is not \
             preserved.",
        )
        .with_code(
            "use std::collections::HashSet;\n\nfn main() {\n    let raw = vec![\"Folk\", \"Rock\", \"Folk\", \"Rock\", \"Jazz\"];\n    let set: HashSet<&str> = raw.into_iter().collect();\n    let unique: Vec<&str> = set.into_iter().collect();\n    println!(\"{}\", unique.len());\n}",
        ),

        // L26 Sets and Ownership
        SeedExercise::new(
            Some(25),
            ExerciseKind::PredictCompile,
            "A `String` is inserted into a set and then used. Predict the result.",
            &[
                "It compiles — `insert` borrows the value",
                "It does not compile — `insert` moved the value into the set",
            ],
            1,
            "The set owns what you insert, exactly as a `Vec` and a `HashMap` do. Using `genre` \
             afterwards is a use-after-move; ask the set with `contains` instead.",
        )
        .with_code(
            "use std::collections::HashSet;\n\nfn main() {\n    let mut genres: HashSet<String> = HashSet::new();\n    let genre = String::from(\"Folk\");\n    genres.insert(genre);\n    println!(\"{genre}\");\n}",
        ),
    ]
}
