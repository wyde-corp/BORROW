//! Module 06 — Borrowing, exercises for lessons 25–30.
//!
//! Consolidation. Change-one-line prediction, a tracing case, two API-design
//! decisions, and the conceptual questions that catch a half-learned model.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // ── L25 Borrowing Experiments ──────────────────────────────────────
        SeedExercise::new(
            Some(24),
            ExerciseKind::PredictCompile,
            "Move the `println!(\"{r}\")` above the mutation. What happens?",
            &[
                "It compiles — the immutable borrow ends before the mutation",
                "It still does not compile — the borrow is immutable either way",
                "It compiles only because `s` is now cloned implicitly",
            ],
            0,
            "Ordering is the whole experiment. As written, `r`'s last use follows the write, so their \
             regions overlap. Moved above, the borrow ends before the write, exclusivity is restored, \
             and the program is accepted. Nothing about the value changed — only the schedule of its \
             uses.",
        )
        .with_code(
            "let mut value = String::from(\"hello\");\nlet r = &value;\n\nvalue.push('!');\n\nprintln!(\"{r}\");",
        )
        .with_hints(&["Where is `r` last used?", "Move that use above the write and re-check the regions."]),

        // ── L26 Borrowing Detective ────────────────────────────────────────
        SeedExercise::new(
            Some(25),
            ExerciseKind::TraceBorrow,
            "Detective case: two readers, then a writer, then a final read of the owner. Does it \
             compile, and why?",
            &[
                "Compiles — both immutable borrows finish before the mutable borrow; the owner is read last",
                "Does not compile — `edit` conflicts with `name` at the end",
                "Does not compile — three borrows is one too many",
            ],
            0,
            "`first` and `second` finish at their prints, before `edit` is created. `edit` then has \
             exclusive access and is done before the final print, which reads the owner directly with \
             no borrow outstanding. Three borrows, no overlap. The detective always resolves to the \
             same region question.",
        )
        .with_code(
            "let mut name = String::from(\"Gus\");\n\nlet first = &name;\nlet second = &name;\n\nprintln!(\"{first} {second}\");\n\nlet edit = &mut name;\nedit.push('!');\n\nprintln!(\"{name}\");",
        )
        .with_hints(&["Mark the last use of each borrow.", "Do any two regions overlap?"]),

        // ── L27 Borrowing Design Decisions ─────────────────────────────────
        SeedExercise::new(
            Some(26),
            ExerciseKind::ApiDesign,
            "A function appends a suffix to the caller's string, and the caller keeps using the string. \
             Which signature is right?",
            &[
                "`fn add_suffix(s: String)` — take ownership",
                "`fn add_suffix(s: &mut String)` — borrow mutably",
                "`fn add_suffix(s: &String)` — borrow immutably",
            ],
            1,
            "The function modifies the value, so it needs write access: `&mut`. It does not need \
             ownership, because the caller keeps the string, so `T` is too much. `&String` is too \
             little, because a shared borrow cannot modify. Follow the procedure: ownership needed? \
             no. must modify? yes → `&mut T`.",
        )
        .with_code(
            "fn add_suffix(s: ???) {\n    s.push_str(\"!\");\n}\n\nfn main() {\n    let mut msg = String::from(\"hi\");\n    add_suffix(???);\n    println!(\"{msg}\");\n}",
        )
        .with_hints(&["Does the caller keep the value?", "Does the function need to change it?"]),

        // ── L28 Borrowing and Ownership Together ───────────────────────────
        SeedExercise::new(
            Some(27),
            ExerciseKind::TraceBorrow,
            "Narrate this program in terms of ownership and borrowing. Who owns `message`, and when is \
             write access available?",
            &[
                "`main` owns `message` throughout; write access is available only during `add_exclamation`",
                "`add_exclamation` takes ownership, so `main` cannot print afterwards",
                "`print_text` takes ownership, so the second print would fail",
            ],
            0,
            "Ownership never leaves `main`. `print_text(&message)` borrows read-only for one call, then \
             releases it. `add_exclamation(&mut message)` holds exclusive write access for one call, \
             then releases it. The final read is safe because the write is over. Nothing moved, nothing \
             cloned.",
        )
        .with_code(
            "fn add_exclamation(text: &mut String) {\n    text.push('!');\n}\n\nfn print_text(text: &str) {\n    println!(\"{text}\");\n}\n\nfn main() {\n    let mut message = String::from(\"hello\");\n\n    print_text(&message);\n    add_exclamation(&mut message);\n    print_text(&message);\n}",
        )
        .with_hints(&["Which function takes ownership, if any?", "How long does each borrow last?"]),

        // ── L29 What Borrowing Does NOT Mean ───────────────────────────────
        SeedExercise::new(
            Some(28),
            ExerciseKind::Mcq,
            "Which of these statements about borrowing is correct?",
            &[
                "Borrowing copies the value so the original can be mutated",
                "A borrow provides access without transferring ownership, and ends at its last use",
                "A mutable reference may coexist with immutable references as long as you are careful",
            ],
            1,
            "Borrowing allocates nothing and changes no ownership; a borrow lasts until its last use, \
             not to the closing brace. And a mutable reference is exclusive while active — 'as long as \
             you are careful' is not a rule Rust offers. The other two options are two of the ten \
             common mistakes.",
        ),

        // ── L30 Final Borrowing Mental Model ───────────────────────────────
        SeedExercise::new(
            Some(29),
            ExerciseKind::Mcq,
            "You meet unfamiliar code. Put the final reasoning procedure in the right order.",
            &[
                "Who owns it → who borrows it → immutable or mutable → when is it used → do borrows conflict → does the value outlive the reference",
                "Does it compile → who owns it → how many clones → does it run",
                "How many references → what type are they → did anyone clone → is it fast",
            ],
            0,
            "The procedure traces the value first, then the accesses, then the regions, then validity: \
             owner, borrower, borrow kind, use, conflict, outlive, verdict. If you can run those six \
             questions over code you have never seen, you are reasoning about borrowing before \
             compiling rather than after — which is the outcome of this module.",
        ),
    ]
}
