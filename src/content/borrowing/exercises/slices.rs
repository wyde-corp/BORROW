//! Module 06 — Borrowing, exercises for lessons 17–20.
//!
//! Containers and views. Collections show the model does not change with the
//! type; slices show a borrow can cover part of a value, and where byte indices
//! stop being safe.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // ── L17 Borrowing and Collections ──────────────────────────────────
        SeedExercise::new(
            Some(16),
            ExerciseKind::PredictCompile,
            "A reference into a vector is still live when the vector is pushed to. Predict.",
            &[
                "It compiles — vectors can always be mutated",
                "It does not compile — a push may reallocate, invalidating the live element reference",
                "It does not compile — you may not index a vector that is borrowed",
            ],
            1,
            "`first` borrows element 0. A `push` may need to grow the buffer, moving every element to \
             new memory; the reference would then point into the old, freed allocation. This is the \
             dangling-reference problem from Part 16 caused by a collection operation. The model does \
             not change with the type.",
        )
        .with_code(
            "let mut names = vec![String::from(\"Gus\")];\n\nlet first = &names[0];\n\nnames.push(String::from(\"Alice\"));\n\nprintln!(\"{first}\");",
        )
        .with_hints(&["What can a `push` do to the vector's buffer?", "Where would `first` point afterwards?"]),

        // ── L18 Slices: Borrowing Part of a Value ──────────────────────────
        SeedExercise::new(
            Some(17),
            ExerciseKind::TraceBorrow,
            "Trace the borrow in `let hello = &s[0..5];`. What owns what, and can `s` be changed while \
             `hello` is used?",
            &[
                "`s` owns the text; `hello` borrows a range of it; `s` may not be mutated while `hello` lives",
                "`hello` owns a copy of the first five characters",
                "`s` and `hello` both own the text, so either may mutate it",
                "`hello` borrows the whole string, not a range",
            ],
            0,
            "The slice is a borrow of a contiguous range, not a copy — no characters were allocated. \
             `s` stays the owner, `hello` borrows bytes 0..5, and while `hello` is used `s` may not be \
             mutated. A slice is a reference with a shape: which part, in addition to which value.",
        )
        .with_code(
            "let s = String::from(\"hello world\");\nlet hello = &s[0..5];\n\nprintln!(\"{hello}\");",
        )
        .with_hints(&["Is a slice a copy or a view?", "Who is responsible for the characters?"]),

        // ── L19 `String` vs `&str` ─────────────────────────────────────────
        SeedExercise::new(
            Some(18),
            ExerciseKind::Mcq,
            "Why do Rust functions usually accept `&str` rather than `&String` for read-only text?",
            &[
                "`&str` is faster because it copies less",
                "`&str` is more flexible: a `&String` coerces to it, and a literal already is one",
                "`&String` cannot be printed",
                "`&str` owns the text, so it is safer",
            ],
            1,
            "`&str` accepts more callers without losing safety: pass `&owned` and it coerces, or pass \
             `\"literal\"` directly. Neither copies the text — both are borrowed views. Choosing `&str` \
             is the same least-powerful-parameter principle as everywhere else in the module.",
        )
        .with_code(
            "fn first_word(s: &str) -> &str {\n    s.split_whitespace().next().unwrap_or(\"\")\n}",
        ),

        // ── L20 Slice Ranges and Borrowing ─────────────────────────────────
        SeedExercise::new(
            Some(19),
            ExerciseKind::FixCode,
            "Slicing this string panics because the range cuts through a multi-byte character. What is \
             the right way to think about the fix?",
            &[
                "Slice on character boundaries, e.g. by finding an index with `char_indices` or `find`",
                "Add 1 to the end index so it clears the character",
                "Use `&text[0..]` and let Rust trim the invalid bytes",
                "Convert the string to a `Vec<u8>` and slice that",
            ],
            0,
            "Ranges are byte indices, and a `char` can occupy several bytes, so a range must land on \
             boundaries. The fix is to derive the index from character-aware operations (`find`, \
             `char_indices`) rather than guessing a number. Slicing bytes and hoping is what panics.",
        )
        .with_code(
            "let text = String::from(\"héllo\");\nlet bad = &text[0..2];\nprintln!(\"{bad}\");",
        )
        .with_hints(&["Are ranges counted in bytes or characters?", "How do you find a safe boundary?"]),
    ]
}
