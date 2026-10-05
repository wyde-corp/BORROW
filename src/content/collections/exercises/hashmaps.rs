//! Module 08 — Collections, exercises for lessons 11–20.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L11 The Scan Problem
        SeedExercise::new(
            Some(10),
            ExerciseKind::Mcq,
            "You look a song up by title by looping over a `Vec` every time. What is the real issue?",
            &[
                "The loop is syntactically wrong",
                "The data is keyed by title, so the container should be a map, not a list",
                "`Vec` cannot hold strings",
            ],
            1,
            "A scan works, but the question is a lookup, and the shape does not match it. A \
             `HashMap<String, V>` answers 'what is under this key' directly, which is why the \
             container matters.",
        ),

        // L12 Creating a HashMap and Inserting
        SeedExercise::new(
            Some(11),
            ExerciseKind::FillGap,
            "Fill the gap with the method that adds an entry to a map.",
            &[
                "insert",
                "push",
                "append",
            ],
            0,
            "A `HashMap` uses `insert(key, value)`. `push` is for a `Vec`; a map is not a list. \
             `append` is a different `Vec` method.",
        )
        .with_code("durations.____(String::from(\"Nude\"), 255);"),

        // L13 Looking Up with get
        SeedExercise::new(
            Some(12),
            ExerciseKind::PredictOutput,
            "The key is missing. Predict what this prints.",
            &["0 seconds", "not in the map", "It panics"],
            1,
            "`get` returns `None` when the key is absent — no panic, no default. You decide what \
             'missing' means, which is why the return type is `Option` rather than a bare value.",
        )
        .with_code(
            "use std::collections::HashMap;\n\nfn main() {\n    let map: HashMap<String, u32> = HashMap::new();\n    match map.get(\"Nude\") {\n        Some(d) => println!(\"{d} seconds\"),\n        None => println!(\"not in the map\"),\n    }\n}",
        ),

        // L14 Who Owns the Keys and Values
        SeedExercise::new(
            Some(13),
            ExerciseKind::PredictCompile,
            "A `String` is inserted into a map and then used. Predict the result.",
            &[
                "It compiles — `insert` borrows the key",
                "It does not compile — `insert` moved the key into the map",
            ],
            1,
            "`insert` takes the key and value by value, so the map owns them. Using `key` afterwards \
             is a use-after-move. Look up with a `&str` if you need the value later.",
        )
        .with_code(
            "use std::collections::HashMap;\n\nfn main() {\n    let key = String::from(\"Nude\");\n    let mut durations: HashMap<String, u32> = HashMap::new();\n    durations.insert(key, 255);\n    println!(\"{key}\");\n}",
        ),

        // L15 The entry API
        SeedExercise::new(
            Some(14),
            ExerciseKind::PredictOutput,
            "One entry exists; `entry` is used on a missing key. Predict the printed value.",
            &["0", "1", "It panics"],
            0,
            "`entry(\"b\").or_insert(0)` inserts the default `0` and returns a mutable reference to \
             it, so `*v` is `0`. This is the three-line counting idiom.",
        )
        .with_code(
            "use std::collections::HashMap;\n\nfn main() {\n    let mut m: HashMap<&str, u32> = HashMap::new();\n    m.insert(\"a\", 1);\n    let v = m.entry(\"b\").or_insert(0);\n    println!(\"{}\", *v);\n}",
        ),

        // L16 Iterating a HashMap
        SeedExercise::new(
            Some(15),
            ExerciseKind::Mcq,
            "Two entries are inserted in a known order. What order does iterating a `HashMap` give?",
            &[
                "Always insertion order",
                "No guaranteed order — it depends on the hashes of the keys",
                "Always reverse insertion order",
            ],
            1,
            "A `HashMap` stores entries by hash, not by position, so iteration order is not \
             promised. If order matters, use a `Vec` or a `BTreeMap`.",
        ),

        // L17 Counting with a Map
        SeedExercise::new(
            Some(16),
            ExerciseKind::PredictOutput,
            "Genres are counted with the `entry` API. Predict the count for \"Folk\".",
            &["1", "2", "3"],
            2,
            "Folk appears three times in the list, so `entry(\"Folk\").or_insert(0)` is incremented \
             three times. The entry API counts without a separate `contains_key`",
        )
        .with_code(
            "use std::collections::HashMap;\n\nfn main() {\n    let genres = vec![\"Folk\", \"Alternative\", \"Folk\", \"Folk\"];\n    let mut counts: HashMap<&str, u32> = HashMap::new();\n    for g in genres {\n        *counts.entry(g).or_insert(0) += 1;\n    }\n    println!(\"{}\", counts.get(\"Folk\").unwrap());\n}",
        ),

        // L18 A Map of Vectors
        SeedExercise::new(
            Some(17),
            ExerciseKind::PredictOutput,
            "Two Radiohead songs are indexed by artist. Predict the length of the artist's list.",
            &["1", "2", "It does not compile"],
            1,
            "`or_insert_with(Vec::new).push(...)` appends both titles to the same artist's vector, so \
             Radiohead has two songs. The value type `Vec<&str>` is what makes a key hold a list.",
        )
        .with_code(
            "use std::collections::HashMap;\n\nfn main() {\n    let songs = vec![(\"Radiohead\", \"Nude\"), (\"Radiohead\", \"Karma Police\")];\n    let mut index: HashMap<&str, Vec<&str>> = HashMap::new();\n    for (artist, title) in songs {\n        index.entry(artist).or_insert_with(Vec::new).push(title);\n    }\n    println!(\"{}\", index.get(\"Radiohead\").unwrap().len());\n}",
        ),

        // L19 Updating and Removing Entries
        SeedExercise::new(
            Some(18),
            ExerciseKind::PredictOutput,
            "A key is inserted twice, then removed. Predict the output.",
            &["removed: Some(260), contains: false", "removed: Some(255), contains: false", "removed: None, contains: true"],
            0,
            "The second `insert` replaces the first value, so `remove` returns the latest, `260`. \
             After removal the key is gone, so `contains_key` is `false`. Inserting over a key is \
             silent replacement.",
        )
        .with_code(
            "use std::collections::HashMap;\n\nfn main() {\n    let mut d: HashMap<&str, u32> = HashMap::new();\n    d.insert(\"Nude\", 255);\n    d.insert(\"Nude\", 260);\n    let removed = d.remove(\"Nude\");\n    println!(\"removed: {:?}, contains: {}\", removed, d.contains_key(\"Nude\"));\n}",
        ),

        // L20 When Order Matters
        SeedExercise::new(
            Some(19),
            ExerciseKind::Mcq,
            "You need to look up songs by title and always iterate them in alphabetical order. Which \
             container?",
            &[
                "`HashMap`",
                "`BTreeMap`",
                "`Vec`",
            ],
            1,
            "`BTreeMap` keeps its keys sorted, so iteration is always in key order. A `HashMap` has \
             the same methods but no ordering; a `Vec` has no key lookup.",
        ),
    ]
}
