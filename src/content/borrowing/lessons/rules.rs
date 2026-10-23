//! Module 06 — Borrowing, lessons 5–7.
//!
//! The two flavours of borrow and the single rule that governs them. By the end
//! of Part 07 the learner has the one mental model the rest of the module
//! applies: many readers, or one writer, never both.

use crate::content::types::{
    broken, callout, code, diagram, fixed, labelled, paragraphs, table, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![immutable_borrowing(), mutable_borrowing(), the_core_rule()]
}

fn immutable_borrowing() -> SeedLesson {
    SeedLesson::new(
        "Immutable Borrowing",
        vec![
            labelled(
                "THE IDEA",
                "An immutable borrow is a read-only view. `&s` gives you a reference you can look \
                 through but cannot write through. Because nothing can change, any number of readers \
                 can hold a view at the same time without getting in each other's way.",
            ),
            code(
                "rust",
                "fn main() {\n    let s = String::from(\"hello\");\n\n    let r1 = &s;\n    let r2 = &s;\n\n    println!(\"{r1}\");\n    println!(\"{r2}\");\n}",
            ),
            paragraphs(&[
                "`r1` and `r2` both refer to the same `String`, owned by `s`. Neither can modify it. \
                 The compiler accepts this without complaint — and more importantly, accepts it on \
                 principle rather than because two happens to be a magic number.",
            ]),
            diagram(
                "Many readers",
                "             ┌── r1\n             │\ns ───────────┼── r2\n             │\n             ↓\n          \"hello\"",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Why is this safe? A reader cannot observe a change, because there are no changes to \
                 observe. Two readers of the same bytes will always agree, and neither can invalidate \
                 the other's view. That is the whole argument. It is not that Rust permits two \
                 references — it is that simultaneous *read-only* access is free of the hazard the \
                 borrow rules exist to prevent.",
            ),
            callout(
                "Not a count, a property",
                "Do not memorise 'Rust allows two immutable references'. It allows *any number* of \
                 them, precisely because none of them can write. The thing being protected is the \
                 absence of mutation, not a number.",
            ),
            code(
                "rust",
                "fn main() {\n    let s = String::from(\"hello\");\n\n    let a = &s;\n    let b = &s;\n    let c = &s;\n\n    println!(\"{a} {b} {c}\");\n}",
            ),
        ],
    )
}

fn mutable_borrowing() -> SeedLesson {
    SeedLesson::new(
        "Mutable Borrowing",
        vec![
            labelled(
                "THE IDEA",
                "A mutable borrow is a read-write view. `&mut s` gives you a reference you may write \
                 through. Because a write can be observed by anyone else looking at the same value, a \
                 mutable borrow needs exclusive access while it is active.",
            ),
            code(
                "rust",
                "fn main() {\n    let mut s = String::from(\"hello\");\n\n    let r = &mut s;\n\n    r.push_str(\" world\");\n\n    println!(\"{r}\");\n}",
            ),
            paragraphs(&[
                "There are two `mut`s of different kinds in play, and both are required here. The \
                 owner must be declared `let mut s`, because the owner is the thing whose value can \
                 change. The borrow must be written `&mut s`, because this particular borrow asks for \
                 permission to write.",
            ]),
            diagram(
                "Both permissions are needed",
                "let mut s = ...   → the OWNER permits mutation\nlet r = &mut s    → the BORROW is a mutable one",
            ),
            paragraphs(&["Now watch what each permission buys you. First, the borrow that is readable but not writable:"]),
            broken(
                "fn main() {\n    let s = String::from(\"hello\");\n    let r = &s;\n\n    r.push_str(\" world\");\n}",
            ),
            paragraphs(&["The owner is not `mut`, so the value cannot be modified at all. Now the same intent with both permissions:"]),
            fixed(
                "fn main() {\n    let mut s = String::from(\"hello\");\n    let r = &mut s;\n\n    r.push_str(\" world\");\n}",
            ),
            table(
                &["Owner declaration", "Borrow", "Can write through it?"],
                &[
                    &["`let s`", "`&s`", "no — the value is not mutable"],
                    &["`let mut s`", "`&s`", "no — the borrow is read-only"],
                    &["`let mut s`", "`&mut s`", "yes"],
                    &["`let s`", "`&mut s`", "no — will not even compile"],
                ],
            ),
            paragraphs(&[
                "The first column is about the value. The second is about the access this reference \
                 grants. A mutable owner can be borrowed either way; a mutable reference to an \
                 immutable owner is rejected before it can exist.",
            ]),
            callout(
                "These are two different permissions",
                "`mut` on the owner says *this binding may change which value it is responsible for, \
                 and its value may be modified*. `&mut` says *this reference grants write access to the \
                 value it borrows*. They are related but not the same, and Part 13 is entirely about \
                 telling them apart.",
            ),
        ],
    )
}

fn the_core_rule() -> SeedLesson {
    SeedLesson::new(
        "The Core Borrowing Rule",
        vec![
            labelled(
                "THE IDEA",
                "Here is the rule the rest of the module is built on. At any point in time, a value \
                 may have either any number of immutable references, or exactly one mutable reference — \
                 but not both at once.",
            ),
            diagram(
                "Many readers OR one writer",
                "MANY READERS          OR          ONE WRITER\n\n&s  &s  &s                            &mut s",
            ),
            paragraphs(&["First, the shape the rule permits on the left-hand side — many readers:"]),
            code(
                "rust",
                "fn main() {\n    let s = String::from(\"hello\");\n\n    let r1 = &s;\n    let r2 = &s;\n\n    println!(\"{r1} {r2}\");\n}",
            ),
            paragraphs(&["Then the shape it permits on the right-hand side — a single writer:"]),
            code(
                "rust",
                "fn main() {\n    let mut s = String::from(\"hello\");\n\n    let r = &mut s;\n    r.push_str(\"!\");\n\n    println!(\"{r}\");\n}",
            ),
            paragraphs(&["And now the shape it forbids — a reader and a writer over the same value at once:"]),
            broken(
                "fn main() {\n    let mut s = String::from(\"hello\");\n\n    let r1 = &s;\n    let r2 = &mut s;\n\n    println!(\"{r1} {r2}\");\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The third program is rejected, and the reason is not bureaucracy. It has a reader \
                 (`r1`) and a writer (`r2`) alive over the same value at the same time. If `r2` is \
                 allowed to change the string, what does `r1` see? The reader can no longer make any \
                 assumption about what it is reading, because the bytes may change underneath it.",
            ),
            diagram(
                "The situation the rule forbids",
                "reader ──┐\n         ├── same value\nwriter ──┘",
            ),
            callout(
                "Many readers OR one writer",
                "This is the most important sentence in the module. Every borrow-checker error you \
                 meet from here on is this rule firing: some access pattern has put a reader and a \
                 writer over the same value at the same time, or asked for two writers.",
            ),
            paragraphs(&[
                "Hold onto the asymmetry. Readers are cheap and can be duplicated freely. Writers are \
                 exclusive because a write is the one thing another view can observe. Rust is not \
                 counting references; it is keeping the invariant 'no one observes a value changing'.",
            ]),
        ],
    )
}
