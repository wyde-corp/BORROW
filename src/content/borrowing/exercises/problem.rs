//! Module 06 — Borrowing, exercises for lessons 1–4.
//!
//! One exercise per lesson, opening with prediction: does it compile, and why?
//! The first four install the problem and the mechanism, so the questions are
//! about ownership outcomes rather than rule details.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // ── L1 The Problem Borrowing Solves ────────────────────────────────
        SeedExercise::new(
            Some(0),
            ExerciseKind::PredictCompile,
            "A function takes the whole `String` just to measure it. Predict whether this compiles.",
            &[
                "It compiles — `String` is passed by reference automatically",
                "It does not compile — `name` was moved into the function",
                "It does not compile — `length` needs to be `mut`",
                "It compiles, but `name` is empty afterwards",
            ],
            1,
            "`calculate_length` takes `s: String` by value, so the call moves `name` into it. When the \
             function returns, the `String` is dropped with it, and the second `println!` uses a name \
             that no longer owns anything. The function only needed to read, but the signature asked \
             for the whole value — which is exactly the friction borrowing removes.",
        )
        .with_code(
            "fn calculate_length(s: String) -> usize {\n    s.len()\n}\n\nfn main() {\n    let name = String::from(\"Gus\");\n\n    let length = calculate_length(name);\n\n    println!(\"{length}\");\n    println!(\"{name}\");\n}",
        )
        .with_hints(&[
            "Look at the parameter type, not the function body.",
            "Who owns the String after the call returns?",
        ]),

        // ── L2 References ──────────────────────────────────────────────────
        SeedExercise::new(
            Some(1),
            ExerciseKind::Mcq,
            "After `let r = &s;`, which statement is true?",
            &[
                "`r` and `s` now both own the value",
                "`r` refers to the value owned by `s`, and `s` is still usable",
                "The value moved from `s` to `r`",
                "`r` owns the value and `s` is disarmed",
            ],
            1,
            "A reference never owns. `s` remains the sole owner and stays usable; `r` is a borrowed \
             view. If `r` were a second owner, there would be two frees, which is the double-free bug \
             ownership exists to prevent.",
        )
        .with_code(
            "let s = String::from(\"hello\");\nlet r = &s;\n\nprintln!(\"{s}\");",
        ),

        // ── L3 Borrowing Without Ownership Transfer ────────────────────────
        SeedExercise::new(
            Some(2),
            ExerciseKind::PredictCompile,
            "Same setup, but the second binding borrows instead of moving. Predict the result.",
            &[
                "It compiles — `s1` still owns the value and is usable",
                "It does not compile — `s1` was moved",
                "It does not compile — two owners are not allowed",
            ],
            0,
            "`let s2 = &s1;` creates a reference; ownership does not move. `s1` remains the owner and \
             both prints are legal. Contrast this with `let s2 = s1;`, where `s1` would be disarmed.",
        )
        .with_code(
            "let s1 = String::from(\"hello\");\nlet s2 = &s1;\n\nprintln!(\"{s2}\");\nprintln!(\"{s1}\");",
        )
        .with_hints(&["`&` asks to borrow, it does not take.", "Which name is responsible for freeing the buffer?"]),

        // ── L4 Dereferencing ───────────────────────────────────────────────
        SeedExercise::new(
            Some(3),
            ExerciseKind::Mcq,
            "What does the `*` in `*r` do?",
            &[
                "It creates a second reference to the same value",
                "It follows the reference back to the value it points at",
                "It removes the borrow and transfers ownership",
                "It makes the reference mutable",
            ],
            1,
            "`&` borrows; `*` dereferences — it reaches through the reference to the value. \
             `println!(\"{r}\")` and `println!(\"{}\", *r)` print the same text because formatting \
             reaches through a reference automatically, which is why the explicit `*` is usually \
             optional.",
        )
        .with_code(
            "let s = String::from(\"hello\");\nlet r = &s;\n\nprintln!(\"{r}\");\nprintln!(\"{}\", *r);",
        ),
    ]
}
