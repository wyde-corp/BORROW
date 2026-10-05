//! Module 08 — Collections, exercises for lessons 31–33.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L31 The Playlist, Assembled
        SeedExercise::new(
            Some(30),
            ExerciseKind::Mcq,
            "In the assembled playlist, `songs` is a `Vec<Song>`, `genres()` returns a \
             `HashSet<&str>`, and `by_artist()` returns a `HashMap<&str, Vec<&str>>`. Why is `songs` \
             a `Vec`?",
            &[
                "A `Vec` is faster than a `HashMap` for everything",
                "The playlist is an ordered list — position and order matter",
                "A `HashMap` cannot store structs",
            ],
            1,
            "The playlist is a sequence: song 1, song 2, and the order you added them. That is what a \
             `Vec` is for. The genre and artist structures answer different questions, which is why \
             they are different containers.",
        ),

        // L32 Searching and Reporting
        SeedExercise::new(
            Some(31),
            ExerciseKind::PredictOutput,
            "Two identical songs are indexed by artist. Predict the length of that artist's list.",
            &["1", "2", "It does not compile"],
            1,
            "A `HashMap<&str, Vec<&str>>` keeps one list per artist, and `push` appends both titles \
             even when they are identical. If duplicates should collapse, the value type would be a \
             `HashSet<&str>` instead — a different question.",
        )
        .with_code(
            "use std::collections::HashMap;\n\nfn main() {\n    let songs = vec![\"Nude\", \"Nude\"];\n    let mut index: HashMap<&str, Vec<&str>> = HashMap::new();\n    for title in songs {\n        index.entry(\"Radiohead\").or_insert_with(Vec::new).push(title);\n    }\n    println!(\"{}\", index.get(\"Radiohead\").unwrap().len());\n}",
        ),

        // L33 Change the Design
        SeedExercise::new(
            Some(32),
            ExerciseKind::ApiDesign,
            "You want to merge two playlists and report the combined *distinct* genres. Which \
             container operation fits?",
            &[
                "Chain the two `Vec`s and scan for duplicates",
                "Build a `HashSet` from each and use `union`",
                "Use a `BTreeMap` keyed by genre",
            ],
            1,
            "Distinct genres is membership over two sets, so building a `HashSet` from each and \
             taking their `union` expresses it directly. Chaining `Vec`s and scanning is the manual \
             version of the same thing.",
        ),
    ]
}
