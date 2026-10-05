//! Module 10 — Generics & Traits, exercises for lessons 16–18.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L16 What derive Does
        SeedExercise::new(
            Some(15),
            ExerciseKind::Mcq,
            "What does `#[derive(Debug)]` actually do?",
            &[
                "It adds run-time reflection",
                "It asks the compiler to generate an `impl Debug for ...`",
                "It makes the type printable with `{}`",
            ],
            1,
            "Derive writes an `impl Debug` for you, using the obvious 'print every field' body. It is \
             not reflection, and it produces `{:?}`, not `{}` — `{}` needs `Display`, which is never \
             derived.",
        ),

        // L17 The Common Derives
        SeedExercise::new(
            Some(16),
            ExerciseKind::FillGap,
            "Fill the gap with the derive that gives the type `==`.",
            &["PartialEq", "Debug", "Clone"],
            0,
            "`PartialEq` gives `==` and `!=`. `Debug` gives printing; `Clone` gives `.clone()`.",
        )
        .with_code("#[derive(Debug, ____)]\nstruct Book {\n    title: String,\n    pages: u32,\n}"),

        // L18 When Derive Is Not Enough
        SeedExercise::new(
            Some(17),
            ExerciseKind::Mcq,
            "Why is there no `#[derive(Display)]`?",
            &[
                "It is an oversight in the standard library",
                "There is no single obvious user-facing format, so `Display` is always written by hand",
                "`Display` is not a trait",
            ],
            1,
            "`Debug` has one obvious meaning — show every field. `Display` is about how a *user* should \
             see the type, which only you know, so it is always a manual `impl`. `Debug` and `Display` \
             are both ordinary traits.",
        ),
    ]
}
