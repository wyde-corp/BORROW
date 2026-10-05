//! Module 06 — Borrowing, exercises for lessons 5–7.
//!
//! The two borrow flavours and the core rule. Prediction and repair: does this
//! compile, and if not, which single change makes it compile for the right
//! reason?

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // ── L5 Immutable Borrowing ─────────────────────────────────────────
        SeedExercise::new(
            Some(4),
            ExerciseKind::PredictCompile,
            "Two immutable borrows of the same value. Predict whether this compiles.",
            &[
                "It compiles — any number of immutable borrows may coexist",
                "It does not compile — only one borrow is allowed at a time",
                "It does not compile — `s` is not declared `mut`",
            ],
            0,
            "Read-only views cannot conflict, so any number of them may coexist. This is not a special \
             allowance for two references; it is the consequence of none of them being able to write. \
             The rule being protected is the absence of mutation, not a count.",
        )
        .with_code(
            "let s = String::from(\"hello\");\n\nlet r1 = &s;\nlet r2 = &s;\n\nprintln!(\"{r1} {r2}\");",
        ),

        // ── L6 Mutable Borrowing ───────────────────────────────────────────
        SeedExercise::new(
            Some(5),
            ExerciseKind::FixCode,
            "This program will not compile. Which single change fixes it without cloning?",
            &[
                "Change `r.push_str(\" world\");` to `s.push_str(\" world\");`",
                "Change `let r = &s;` to `let r = &mut s;` and `let s` to `let mut s`",
                "Change `let r = &s;` to `let r = s;`",
                "Add `.clone()` before `push_str`",
            ],
            1,
            "Two permissions are missing. The owner must be `let mut s` for its value to be mutable, \
             and the borrow must be `&mut s` to ask for write access. Cloning would create a second \
             value and change the design; removing the borrow would not make the string mutable.",
        )
        .with_code(
            "let s = String::from(\"hello\");\nlet r = &s;\n\nr.push_str(\" world\");\nprintln!(\"{r}\");",
        )
        .with_hints(&[
            "There are two separate `mut` permissions involved.",
            "One is on the owner, one is on the borrow.",
        ]),

        // ── L7 The Core Borrowing Rule ─────────────────────────────────────
        SeedExercise::new(
            Some(6),
            ExerciseKind::PredictCompile,
            "A reader and a writer over the same value, both used at the end. Predict the result.",
            &[
                "It compiles — Rust picks the most recent borrow",
                "It does not compile — a mutable borrow cannot coexist with a live immutable borrow",
                "It does not compile — `s` is not declared `mut`",
                "It compiles, but the printed values are undefined",
            ],
            1,
            "`r1` is an immutable borrow whose last use is the `println!` at the end, so its region \
             overlaps `r2 = &mut s`. That puts a reader and a writer over the same value at once, \
             which the core rule forbids. This is 'many readers OR one writer', never both.",
        )
        .with_code(
            "let mut s = String::from(\"hello\");\n\nlet r1 = &s;\nlet r2 = &mut s;\n\nprintln!(\"{r1} {r2}\");",
        )
        .with_hints(&[
            "Find the last use of each borrow.",
            "Do the two regions overlap?",
        ]),
    ]
}
