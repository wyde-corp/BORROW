//! Module 06 — Borrowing, exercises for lessons 21–24.
//!
//! Reasoning. Two tracing exercises for the system and the checker, one error
//! to interpret, and one repair drawn from the common-mistakes list.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // ── L21 Borrowing Rules as a System ────────────────────────────────
        SeedExercise::new(
            Some(20),
            ExerciseKind::TraceBorrow,
            "Reconstruct the system for this snippet: owner, each borrower, borrow type, and whether \
             the accesses conflict.",
            &[
                "Owner `s`; `a` and `b` are immutable; the later `push` conflicts with them",
                "Owner `s`; `a` and `b` are mutable; no conflict",
                "No owner; `a` and `b` own the value; the push is fine",
            ],
            0,
            "`s` owns the `String`. `a` and `b` are immutable borrows. The `push` needs mutable access \
             to `s`, and both borrows are used after it, so their regions overlap the write — a reader \
             with a writer. Move the prints above the push and the overlap disappears.",
        )
        .with_code(
            "let mut s = String::from(\"hello\");\n\nlet a = &s;\nlet b = &s;\n\ns.push_str(\"!\");\n\nprintln!(\"{a} {b}\");",
        )
        .with_hints(&["Find the last use of `a` and `b`.", "Does the write happen before or after that?"]),

        // ── L22 The Borrow Checker ─────────────────────────────────────────
        SeedExercise::new(
            Some(21),
            ExerciseKind::TraceBorrow,
            "Run the borrow checker's procedure by hand. At which step does this program fail?",
            &[
                "Step 7 — a mutation happens while a reader is still used later",
                "Step 3 — a borrow is mutable when it should be immutable",
                "Step 8 — a reference outlives its value",
            ],
            0,
            "Following the procedure: `s` owns the value; `r` is created, immutable; its last use is \
             the final print, so it is live when `s.push_str` mutates the owner. That is a read \
             overlapping a write — step seven. The fix is the familiar one: end the reader first.",
        )
        .with_code(
            "let mut s = String::from(\"hello\");\nlet r = &s;\n\ns.push_str(\"!\");\n\nprintln!(\"{r}\");",
        )
        .with_hints(&["When is `r` last used?", "What does the `push_str` need?"]),

        // ── L23 Reading Borrow Checker Errors ──────────────────────────────
        SeedExercise::new(
            Some(22),
            ExerciseKind::CompilerChallenge,
            "Read the compiler output below. What rule did Rust enforce, and where is the conflict?",
            &[
                "A mutable borrow cannot exist while an immutable borrow that is used later is live",
                "The string must be declared `mut` before it can be printed",
                "The two borrows are fine; the error is about the missing return value",
            ],
            0,
            "The error names three things: the immutable borrow on line 4, the mutable borrow on line \
             5, and the later use on line 7 that keeps the immutable borrow alive. The enforced rule is \
             'many readers OR one writer' — here a reader's region spans a write. Fix the ordering, not \
             by cloning.",
        )
        .with_code(
            "error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable\n  --> main.rs:5:14\n   |\n 4 |     let r1 = &s;\n   |               - immutable borrow occurs here\n 5 |     let r2 = &mut s;\n   |              ^^^^^^^^ mutable borrow occurs here\n 6 |\n 7 |     println!(\"{r1}\");\n   |               ---- immutable borrow later used here",
        )
        .with_hints(&["Find the first borrow, the second access, and the later use.", "Which one keeps the other alive?"]),

        // ── L24 Common Borrowing Mistakes ──────────────────────────────────
        SeedExercise::new(
            Some(23),
            ExerciseKind::FixCode,
            "This compiles, but the clones are the mistake. What is the fix that states the real design \
             intent?",
            &[
                "Have `longer` borrow both strings and return a reference, so no clone is needed",
                "Clone only once instead of twice",
                "Make both parameters `&String` and return `&String` with no changes elsewhere",
                "Change the strings to `&str` but keep the clones",
            ],
            0,
            "`longer` never needs to own either string: it reads both and picks one. Borrowing both and \
             returning a reference expresses that, and clones nothing. Cloning to satisfy the compiler \
             changes the ownership design rather than deciding it. The fix is to say what the function \
             actually needs.",
        )
        .with_code(
            "fn longer(a: String, b: String) -> String {\n    if a.len() >= b.len() { a } else { b }\n}\n\nfn main() {\n    let x = String::from(\"short\");\n    let y = String::from(\"longer\");\n\n    let best = longer(x.clone(), y.clone());\n\n    println!(\"{best} {x} {y}\");\n}",
        )
        .with_hints(&["Does `longer` need to keep either string?", "Could a returned reference borrow from the inputs?"]),
    ]
}
