//! The Borrowing Detective challenge.
//!
//! One set of programs, each using one mechanism, culminating in cases that
//! combine several. The learner predicts and explains every case before reading
//! the walkthrough, which is the final hint. A second, open-ended challenge
//! turns the same cases around and asks for the design behind them.

use crate::content::types::{ExerciseKind, SeedExercise};

/// Case 6. A returned reference whose source is a local. Included here so the
/// walkthrough can contrast it with case 5, where the source is an input.
pub const CASE_6: &str = r#"fn dangling() -> &String {
    let s = String::from("hello");
    &s
}

fn main() {
    let r = dangling();
    println!("{r}");
}"#;

/// Case 9. Borrowing a collection element and then growing the collection.
pub const CASE_9: &str = r#"fn main() {
    let mut names = vec![String::from("Gus"), String::from("Alice")];

    let first = &names[0];

    names.push(String::from("Bob"));

    println!("{first}");
}"#;

/// All ten cases as one listing. The set is the exercise.
pub const ALL_CASES: &str = r#"// ── CASE 1 ──────────────────────────────────────────────
fn main() {
    let s = String::from("hello");
    let r = &s;

    println!("{r}");
    println!("{s}");
}

// ── CASE 2 ──────────────────────────────────────────────
fn main() {
    let s = String::from("hello");

    let a = &s;
    let b = &s;
    let c = &s;

    println!("{a} {b} {c}");
}

// ── CASE 3 ──────────────────────────────────────────────
fn main() {
    let mut s = String::from("hello");

    let r = &mut s;
    r.push_str(" world");

    println!("{s}");
}

// ── CASE 4 ──────────────────────────────────────────────
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &mut s;

    println!("{r1} {r2}");
}

// ── CASE 5 ──────────────────────────────────────────────
fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

fn main() {
    let text = String::from("hello world");
    let word = first_word(&text);

    println!("{word} {text}");
}

// ── CASE 6 ──────────────────────────────────────────────
fn dangling() -> &String {
    let s = String::from("hello");
    &s
}

fn main() {
    let r = dangling();
    println!("{r}");
}

// ── CASE 7 ──────────────────────────────────────────────
fn main() {
    let mut s = String::from("hello");

    {
        let r = &mut s;
        r.push('!');
    }

    let r2 = &mut s;
    r2.push('?');

    println!("{s}");
}

// ── CASE 8 ──────────────────────────────────────────────
fn main() {
    let mut s = String::from("hello");

    let r = &s;

    println!("{r}");

    s.push_str(" world");
    println!("{s}");
}

// ── CASE 9 ──────────────────────────────────────────────
fn main() {
    let mut names = vec![String::from("Gus"), String::from("Alice")];

    let first = &names[0];

    names.push(String::from("Bob"));

    println!("{first}");
}

// ── CASE 10 ─────────────────────────────────────────────
fn length(s: &String) -> usize {
    s.len()
}

fn main() {
    let owned = String::from("hello");
    let n = length(&owned);

    println!("{n} {owned}");
}"#;

pub const WALKTHROUGH: &str = r#"Answers, in order. Commit to a prediction for every case
before reading. The point is to find the case your model got wrong, not to
score well.

CASE 1 — Compiles. `r` borrows `s` immutably and finishes at its print. `s`
  is still the owner and is read afterwards; both are legal.

CASE 2 — Compiles. Three immutable borrows, all finishing at the print.
  Read-only views never conflict, so any number is allowed.

CASE 3 — Compiles. `s` is `mut`, the borrow is `&mut`, and `r` is used
  (implicitly, by the write) before `s` is read at the end. One writer, no
  readers.

CASE 4 — Does not compile, error[E0502]. `r1` is an immutable borrow whose
  last use is the print, so its region overlaps `r2 = &mut s`. A reader and
  a writer over the same value at once. Fix: print `r1` before creating `r2`.

CASE 5 — Compiles. `first_word` returns a slice of its input, and the input
  outlives the call because the caller still owns `text`. A returned
  reference is valid when it borrows from an input. Note `word` is used
  before `text`; if the order were reversed and `text` mutated in between,
  it would fail.

CASE 6 — Does not compile, error[E0106]. The value `s` is dropped when
  `dangling` returns, so no reference to it can leave the function. There is
  no input for the returned reference to borrow from. Fix: return the
  `String` itself.

CASE 7 — Compiles. Two mutable borrows, but not simultaneous: the inner
  borrow ends at the closing brace before `r2` exists. Exclusivity is about
  overlap, not count.

CASE 8 — Compiles. Here the correct ordering is used: `r` is last used by
  its print, then the owner is mutated directly, then read. The borrow is
  over before the write, so there is no overlap. Contrast case 4, where the
  same lines in a different order fail.

CASE 9 — Does not compile, error[E0502]. `first` borrows element 0, then
  `push` may reallocate the vector and move every element. `first` is used
  after the push, so it would point into the old buffer. Same rule as case
  6 — the value the reference depends on may not survive the reference.

CASE 10 — Compiles. `length` borrows the string read-only and returns a
  `usize`, which is `Copy`. The caller keeps `owned` and can print it. This
  is the idiomatic shape case 6 was missing: take a borrow, return an owned
  value that does not depend on the borrow.

If you mispredicted case 4 or case 9, that is the common result. Both are
the same rule — a live reader overlapping a write — reached through a
different construct. If you mispredicted case 6 or 8, revisit the idea that
a borrow ends at its last use rather than at the closing brace."#;

pub fn challenges() -> Vec<SeedExercise> {
    vec![
        SeedExercise::new(
            None,
            ExerciseKind::Challenge,
            "BORROWING DETECTIVE — Ten programs. For each one, determine: does it compile; who owns \
             the value; what references exist; which are immutable and which mutable; when each borrow \
             becomes active; when it ends; whether there is an aliasing conflict; whether any reference \
             outlives its value; and the minimal fix if it does not compile. Commit to all ten before \
             opening a single hint.",
            &[
                "For each case, name the owner before you name any reference",
                "Mark the last use of every borrow; that is where its region ends",
                "Ask at every write: is any reader still live at this point?",
                "Ask of every returned or held reference: does its value still exist?",
                "If it does not compile, name the rule that fired — do not stop at 'it does not compile'",
                "Do all ten before reading the walkthrough. The overlap only becomes obvious across the set.",
            ],
            0,
            WALKTHROUGH,
        )
        .code(ALL_CASES)
        .xp(160)
        .hints(&[
            "Start mechanically: does the type move, and which borrows are live at each write?",
            "A borrow ends at its last use, not at the closing brace. Mark those points first.",
            "'It does not compile' is not an answer. Say which two accesses overlap, and where.",
            "Two cases fail for the same underlying reason but look completely different. Find the pair.",
            "One case compiles only because of the order of two statements. Find it, and predict the failure when you swap them.",
            "The walkthrough is the last hint. It is cheaper to be wrong here than to be confidently wrong later.",
        ]),
        SeedExercise::new(
            None,
            ExerciseKind::Challenge,
            "BORROWING API DESIGN — Take the same ten cases and redesign them. For each, decide whether \
             the functions involved should take ownership, `&T`, or `&mut T`, and write the signature \
             you would choose. For cases that fail, the fix is a design decision, not a trick.",
            &[
                "For each function, answer: does it need ownership? does it need to modify?",
                "Write the signature as `T`, `&T`, or `&mut T` — and justify it in one sentence",
                "For case 5, explain why returning a reference is right there but wrong in case 6",
                "For case 10, compare `&String` with `&str` and pick one",
                "Where a clone feels tempting, decide whether a second owner is genuinely wanted",
                "Be explicit about where you would clone on purpose, and why",
            ],
            0,
            "A borrow-checker error is usually a design question in disguise. The signature says what a \
             function does with its argument; choosing it well prevents most of the conflicts in these \
             cases before they arise. Prefer the least powerful parameter that does the job: `&str` \
             over `&String`, `&T` over `&mut T` over `T`. Clone when you have decided you want a \
             second owned value — never when you are trying to silence an error you do not understand.",
        )
        .code(CASE_6)
        .with_code_b(CASE_9)
        .xp(180)
        .hints(&[
            "Start with case 6: the function needs to produce a value, not borrow one.",
            "Case 9 is about when a collection may grow while a view into it exists.",
            "For every parameter, ask the three design questions before choosing a type.",
            "A clone is correct when you can justify the second owner in one sentence.",
            "The goal is not zero clones. It is clones you can point at and explain.",
        ]),
    ]
}
