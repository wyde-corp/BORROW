//! Module 06 — Borrowing, exercises for lessons 8–10.
//!
//! Duration and exclusivity. These questions turn on *where* the borrows are
//! used, so the learner has to trace regions rather than count references.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // ── L8 Borrowing and Scope ─────────────────────────────────────────
        SeedExercise::new(
            Some(7),
            ExerciseKind::PredictCompile,
            "Two mutable borrows in the same function, with a print between them. Predict the result.",
            &[
                "It compiles — the first borrow ends at its last use, before the second begins",
                "It does not compile — two mutable borrows in one scope are always rejected",
                "It does not compile — `s` is borrowed twice at the same time",
            ],
            0,
            "`r`'s last use is the `println!`, and `r2` is created after that. The regions do not \
             overlap, so exclusivity is never violated. A borrow ends at its last use, not at the \
             closing brace — the old 'lives to end of scope' rule is an oversimplification.",
        )
        .with_code(
            "let mut s = String::from(\"hello\");\n\nlet r = &mut s;\nprintln!(\"{r}\");\n\nlet r2 = &mut s;\nr2.push('!');\n\nprintln!(\"{s}\");",
        )
        .with_hints(&["When is `r` last used?", "Does that point come before `r2` is created?"]),

        // ── L9 Multiple Immutable Borrows ──────────────────────────────────
        SeedExercise::new(
            Some(8),
            ExerciseKind::PredictCompile,
            "A mutation is attempted while two immutable borrows are still going to be used. Predict.",
            &[
                "It compiles — mutation is allowed as long as the borrows are immutable",
                "It does not compile — `s` cannot be mutated while `a` and `b` are still used later",
                "It does not compile — `a` and `b` may not both borrow the same value",
            ],
            1,
            "The problem is not that `a` and `b` exist — read-only views are allowed in any number. The \
             problem is that they are used *after* the mutation, so their regions overlap the write. \
             Move the two prints above the push and the same program is accepted.",
        )
        .with_code(
            "let mut s = String::from(\"hello\");\n\nlet a = &s;\nlet b = &s;\n\ns.push_str(\"!\");\n\nprintln!(\"{a} {b}\");",
        )
        .with_hints(&["It is about use, not existence.", "Where is each borrow last used?"]),

        // ── L10 Mutable Borrowing and Exclusivity ──────────────────────────
        SeedExercise::new(
            Some(9),
            ExerciseKind::FixCode,
            "Two mutable borrows are used at the same time. Which change makes this compile with only \
             one mutable borrow active at any moment?",
            &[
                "Change both `&mut s` to `&s`",
                "Wrap the first borrow in its own `{ }` block so it ends before the second is created",
                "Add `.clone()` to `s` before each borrow",
                "Declare `s` as `let mut mut s`",
            ],
            1,
            "A scope is the direct way to make the borrows sequential instead of simultaneous: the \
             inner brace ends `r1` before `r2` exists. Cloning would create a second value and lose \
             the point; changing to `&s` would remove the write ability entirely. Exclusivity is about \
             overlap, and a block removes the overlap.",
        )
        .with_code(
            "let mut s = String::from(\"hello\");\n\nlet r1 = &mut s;\nlet r2 = &mut s;\n\nr1.push('!');\nr2.push('?');\n\nprintln!(\"{s}\");",
        )
        .with_hints(&["The two borrows currently exist at the same time.", "How do you end a borrow early?"]),
    ]
}
