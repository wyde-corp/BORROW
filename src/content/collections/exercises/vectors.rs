//! Module 08 — Collections, exercises for lessons 1–10.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L1 From One Value to Many
        SeedExercise::new(
            Some(0),
            ExerciseKind::Mcq,
            "Why is describing a playlist with separate variables (`title1`, `title2`, …) a problem?",
            &[
                "It uses too much memory",
                "There is no way to loop over, count, or pass the whole list as one value",
                "Rust does not allow more than four variables",
            ],
            1,
            "Separate variables cannot be iterated, counted, or passed as one thing. A collection is \
             a single value that holds many, which is what the rest of the module is about.",
        ),

        // L2 Creating a Vec
        SeedExercise::new(
            Some(1),
            ExerciseKind::FillGap,
            "Fill the gap with the type of an ordered list of owned strings.",
            &[
                "Vec<String>",
                "HashMap<String>",
                "String",
            ],
            0,
            "`Vec<String>` is an ordered list of owned strings. `HashMap` needs a key and a value; a \
             bare `String` is a single value, not a list.",
        )
        .with_code("let mut playlist: ____ = Vec::new();"),

        // L3 push and pop
        SeedExercise::new(
            Some(2),
            ExerciseKind::PredictOutput,
            "One push and one pop. Predict what this prints.",
            &["Some(\"Nude\") — 1 left", "Some(\"Blackbird\") — 1 left", "None — 1 left"],
            0,
            "`push` adds `Nude` to the end; `pop` removes and returns the last item, `Some(\"Nude\")`. \
             The playlist is back to one song. `pop` returns `Option` because the vector might be \
             empty.",
        )
        .with_code(
            "fn main() {\n    let mut p = vec![String::from(\"Blackbird\")];\n    p.push(String::from(\"Nude\"));\n    let last = p.pop();\n    println!(\"{:?} — {} left\", last, p.len());\n}",
        ),

        // L4 Indexing vs get
        SeedExercise::new(
            Some(3),
            ExerciseKind::PredictOutput,
            "The playlist has one song. Predict what `playlist[3]` does.",
            &[
                "Prints an empty string",
                "Panics at runtime — index out of bounds",
                "Prints nothing and returns `None`",
            ],
            1,
            "Square-bracket indexing panics when the index is past the end. It compiles because the \
             compiler does not track the length; that is why `get` exists for when a position might \
             not exist.",
        )
        .with_code(
            "fn main() {\n    let playlist = vec![String::from(\"Blackbird\")];\n    println!(\"{}\", playlist[3]);\n}",
        ),

        // L5 Iterating by Reference
        SeedExercise::new(
            Some(4),
            ExerciseKind::PredictOutput,
            "The loop borrows the playlist. Predict the output.",
            &[
                "Two titles, then `still 2 songs`",
                "It does not compile — the loop moves the playlist",
                "Two titles, then `still 0 songs`",
            ],
            0,
            "`&playlist` borrows, so each `song` is a reference and the playlist is untouched. The \
             count after the loop is still 2. Iterating by reference is the default for reading.",
        )
        .with_code(
            "fn main() {\n    let p = vec![String::from(\"Blackbird\"), String::from(\"Nude\")];\n    for song in &p {\n        println!(\"{song}\");\n    }\n    println!(\"still {} songs\", p.len());\n}",
        ),

        // L6 Iterating Mutably
        SeedExercise::new(
            Some(5),
            ExerciseKind::PredictCompile,
            "The loop holds an exclusive borrow and the body pushes. Predict the result.",
            &[
                "It compiles and adds a song each time",
                "It does not compile — the vector is borrowed while `push` needs a mutable borrow",
            ],
            1,
            "`&mut playlist` borrows the vector exclusively for the loop, and `push` needs another \
             mutable borrow at the same time. The same collection conflict as before — collect the \
             additions and push after the loop.",
        )
        .with_code(
            "fn main() {\n    let mut p = vec![String::from(\"Blackbird\")];\n    for song in &mut p {\n        p.push(String::from(\"Nude\"));\n        println!(\"{song}\");\n    }\n}",
        ),

        // L7 Iterating by Value
        SeedExercise::new(
            Some(6),
            ExerciseKind::PredictCompile,
            "The loop has no `&`, and the playlist is used after it. Predict the result.",
            &[
                "It compiles and prints the count",
                "It does not compile — the loop consumed the vector",
            ],
            1,
            "`for song in playlist` moves the vector and each item out, so using `playlist` afterwards \
             is a use-after-move. Iterate by value only when you mean to consume the collection.",
        )
        .with_code(
            "fn main() {\n    let p = vec![String::from(\"Blackbird\")];\n    for song in p {\n        println!(\"{song}\");\n    }\n    println!(\"{} songs\", p.len());\n}",
        ),

        // L8 Inserting and Removing in the Middle
        SeedExercise::new(
            Some(7),
            ExerciseKind::PredictOutput,
            "A song is inserted at position 1, then removed from position 1. Predict the output.",
            &[
                "removed: Some(\"Karma Police\")",
                "removed: Some(\"Nude\")",
                "removed: None",
            ],
            0,
            "`insert(1, ...)` puts the song at position 1 and shifts `Nude` right. `remove(1)` takes \
             back the item at position 1, which is the one just inserted. Both shift the items around \
             them.",
        )
        .with_code(
            "fn main() {\n    let mut p = vec![String::from(\"Blackbird\"), String::from(\"Nude\")];\n    p.insert(1, String::from(\"Karma Police\"));\n    let removed = p.remove(1);\n    println!(\"removed: {:?}\", removed);\n}",
        ),

        // L9 A Vec of Structs
        SeedExercise::new(
            Some(8),
            ExerciseKind::PredictOutput,
            "A vector of songs is summed by duration. Predict the output.",
            &["2 songs, 463 seconds", "2 songs, 208 seconds", "It does not compile"],
            0,
            "The loop borrows the vector and sums `duration_secs`: 208 + 255 = 463. A vector of \
             structs iterated to compute a total is one of the most common Rust shapes.",
        )
        .with_code(
            "struct Song { title: String, duration_secs: u32 }\n\nfn main() {\n    let p = vec![\n        Song { title: String::from(\"Blackbird\"), duration_secs: 208 },\n        Song { title: String::from(\"Nude\"), duration_secs: 255 },\n    ];\n\n    let mut total = 0;\n    for song in &p {\n        total += song.duration_secs;\n    }\n    println!(\"{} songs, {total} seconds\", p.len());\n}",
        ),

        // L10 Capacity and Reallocation
        SeedExercise::new(
            Some(9),
            ExerciseKind::PredictCompile,
            "A reference into the vector is held while it is pushed to. Predict the result.",
            &[
                "It compiles — the reference stays valid",
                "It does not compile — a `push` may reallocate, invalidating the reference",
            ],
            1,
            "A `push` may reallocate the buffer and move every item, so a live reference into the \
             vector could dangle. The compiler treats any `push` as potentially invalidating a \
             borrow, and rejects this. End the borrow first.",
        )
        .with_code(
            "fn main() {\n    let mut v = vec![1, 2];\n    let first = &v[0];\n    v.push(3);\n    println!(\"{first}\");\n}",
        ),
    ]
}
