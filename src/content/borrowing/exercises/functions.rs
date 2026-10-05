//! Module 06 — Borrowing, exercises for lessons 11–16.
//!
//! Function boundaries. The API-design exercise and the move/borrow questions
//! are the practical core: choosing `T`, `&T`, or `&mut T` deliberately, and
//! spotting the one reference that cannot outlive its value.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // ── L11 Borrowing in Functions ─────────────────────────────────────
        SeedExercise::new(
            Some(10),
            ExerciseKind::ApiDesign,
            "A function prints a name and the caller needs the name afterwards. Which signature should \
             it have?",
            &[
                "`fn show(name: String)` — take ownership",
                "`fn show(name: &String)` or, better, `fn show(name: &str)` — borrow",
                "`fn show(name: &mut String)` — borrow mutably",
            ],
            1,
            "The function only reads, and the caller keeps the value, so it should borrow. `&str` is the \
             better borrow because a `&String` coerces to it and a literal already is one. Taking \
             ownership would force the caller to give up a value they still need; `&mut` would claim \
             write access the function does not use.",
        )
        .with_code(
            "fn show(name: ???) {\n    println!(\"{name}\");\n}\n\nfn main() {\n    let name = String::from(\"Gus\");\n    show(???);\n    println!(\"still here: {name}\");\n}",
        )
        .with_hints(&["Does the function keep the value?", "Does it need to modify it?"]),

        // ── L12 `&T` vs `&mut T` ───────────────────────────────────────────
        SeedExercise::new(
            Some(11),
            ExerciseKind::Mcq,
            "What does `&mut T` actually mean?",
            &[
                "The reference variable itself can be reassigned",
                "This reference grants mutable access to the borrowed value",
                "The value is cloned before being borrowed",
                "The owner is transferred to the borrower",
            ],
            1,
            "The `mut` belongs to the borrow, not the reference variable. `&mut T` says 'this reference \
             grants write access to the value'. Reassigning the reference itself would be `let mut r = \
             ...`, a separate idea, and ownership never moves when borrowing.",
        )
        .with_code(
            "fn add_suffix(s: &mut String) {\n    s.push_str(\"!\");\n}",
        ),

        // ── L13 Mutable Owner vs Mutable Reference ─────────────────────────
        SeedExercise::new(
            Some(12),
            ExerciseKind::Mcq,
            "`s` is declared mutable, yet `r.push_str` is rejected. Why?",
            &[
                "`mut` on the owner does not make a shared borrow writable",
                "The value is moved out by the borrow",
                "`push_str` only works on `&mut str`, never `String`",
            ],
            0,
            "`let mut s` permits the owner's value to change. But `let r = &s;` is a shared, read-only \
             borrow, and its permissions win while it is active. To write through the reference it must \
             be `&mut s`. The owner's `mut` is a capability, not a guarantee that every borrow gets it.",
        )
        .with_code(
            "let mut s = String::from(\"hello\");\nlet r = &s;\n\nr.push_str(\"!\");",
        ),

        // ── L14 Borrowing vs Moving ────────────────────────────────────────
        SeedExercise::new(
            Some(13),
            ExerciseKind::PredictOwnership,
            "`inspect` takes `&String`. After the call, who owns the value and can it be modified?",
            &[
                "The function owns it; the caller cannot use it, and it may be modified",
                "The caller owns it; the function could not modify it",
                "Both own it; either may modify it",
            ],
            1,
            "A `&T` parameter never takes ownership, so the caller still owns the value after the call. \
             A shared borrow also cannot modify: the function had read-only access. If it needed to \
             change the value it would have to take `&mut T`, and the caller would write \
             `inspect(&mut s)`.",
        )
        .with_code(
            "fn inspect(s: &String) {\n    println!(\"{s}\");\n}\n\nfn main() {\n    let s = String::from(\"hello\");\n    inspect(&s);\n    println!(\"{s}\");\n}",
        )
        .with_hints(&["Look at the parameter type.", "`&T` versus `&mut T` versus `T`."]),

        // ── L15 Returning References ───────────────────────────────────────
        SeedExercise::new(
            Some(14),
            ExerciseKind::PredictCompile,
            "A function returns a reference to part of a string it borrowed from the caller. Predict.",
            &[
                "It compiles — the returned reference borrows from the input, which outlives the call",
                "It does not compile — a function may never return a reference",
                "It does not compile — the slice outlives `s`",
            ],
            0,
            "The returned `&str` points into the `String` the caller still owns after the call, so it \
             stays valid. The rule of thumb: a returned reference must come from an input. Returning a \
             reference to a value created *inside* the function is the case that cannot work.",
        )
        .with_code(
            "fn first_word(s: &str) -> &str {\n    match s.find(' ') {\n        Some(i) => &s[..i],\n        None => s,\n    }\n}\n\nfn main() {\n    let text = String::from(\"hello world\");\n    println!(\"{}\", first_word(&text));\n}",
        )
        .with_hints(&["Where does the returned reference point?", "Which value outlives the call?"]),

        // ── L16 Dangling References ────────────────────────────────────────
        SeedExercise::new(
            Some(15),
            ExerciseKind::FixCode,
            "This function tries to return a reference to a value it creates locally. Which fix is \
             correct?",
            &[
                "Add a lifetime parameter to the return type",
                "Return the `String` itself instead of a reference to it",
                "Call `.clone()` on `&s` before returning",
                "Make `s` a `&'static String`",
            ],
            1,
            "The value `s` is dropped when the function ends, so no reference to it can be returned — a \
             lifetime annotation cannot change that, because there is no input for the reference to \
             borrow from. Returning the `String` moves ownership to the caller, which keeps the value \
             alive and removes the borrow entirely.",
        )
        .with_code(
            "fn dangling() -> &String {\n    let s = String::from(\"hello\");\n    &s\n}",
        )
        .with_hints(&["`s` is freed when the function returns.", "What could own the value instead?"]),
    ]
}
