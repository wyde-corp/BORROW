//! The Ownership Detective challenge.
//!
//! Several small programs, each of which the learner must diagnose. The
//! programs are ordered so that each one uses exactly one mechanism, and the
//! last few combine several — which is the point. There are no solutions
//! inline; the walkthrough is the last hint.

use crate::content::types::{ExerciseKind, SeedExercise};

/// Case 5. A vector moved into a function that only reads it. Paired with
/// CASE 7 in the second challenge, where the argument exercise starts.
pub const CASE_5: &str = r#"fn longest(values: Vec<i32>) -> i32 {
    *values.iter().max().unwrap_or(&0)
}

fn main() {
    let data = vec![3, 9, 2];
    let best = longest(data);

    println!("{best} {data:?}");
}"#;

/// Case 7. Everything at once: copy, move, clone, scope, and a return.
pub const CASE_7: &str = r#"fn shout(s: String) -> String {
    s.to_uppercase()
}

fn main() {
    let a = String::from("one");
    let b = a.clone();
    let c = b;

    let n = 5;
    let m = n;

    let d = shout(c);

    {
        let temp = String::from("scratch");
        println!("{temp}");
    }

    println!("{a} {m} {d}");
}"#;

/// All eight cases as one listing, so the learner can actually see the set.
/// The set is the exercise — a case in isolation is just a puzzle.
pub const ALL_CASES: &str = r#"// ── CASE 1 ──────────────────────────────────────────────
fn main() {
    let name = String::from("Gus");
    let other = name;

    println!("{name}");
    println!("{other}");
}

// ── CASE 2 ──────────────────────────────────────────────
fn take(s: String) -> String {
    s
}

fn main() {
    let name = String::from("Gus");
    let name = take(name);

    println!("{name}");
}

// ── CASE 3 ──────────────────────────────────────────────
fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    let n = 21;
    let result = double(n);

    println!("{n} {result}");
}

// ── CASE 4 ──────────────────────────────────────────────
fn main() {
    let a = String::from("shared");
    let b = a.clone();

    println!("{a} {b}");
    println!("{}", a == b);
}

// ── CASE 5 ──────────────────────────────────────────────
fn longest(values: Vec<i32>) -> i32 {
    *values.iter().max().unwrap_or(&0)
}

fn main() {
    let data = vec![3, 9, 2];
    let best = longest(data);

    println!("{best} {data:?}");
}

// ── CASE 6 ──────────────────────────────────────────────
fn main() {
    let mut label = String::from("draft");
    label = String::from("final");

    println!("{label}");
}

// ── CASE 7 ──────────────────────────────────────────────
fn shout(s: String) -> String {
    s.to_uppercase()
}

fn main() {
    let a = String::from("one");
    let b = a.clone();
    let c = b;

    let n = 5;
    let m = n;

    let d = shout(c);

    {
        let temp = String::from("scratch");
        println!("{temp}");
    }

    println!("{a} {m} {d}");
}

// ── CASE 8 ──────────────────────────────────────────────
fn main() {
    let count = 3;
    count = 4;

    println!("{count}");
}"#;

pub const WALKTHROUGH: &str = r#"Answers, in order. Read them after you have committed to a
prediction for every case — the point is not to score well, it is to find
the case your model got wrong.

CASE 1 — Does not compile. error[E0382]: borrow of moved value: `name`.
  One value, created, moved into `other`. `name` is disarmed: still in
  scope, still a name, no longer allowed to be used. The heap buffer is
  untouched and `other` frees it at end of scope, exactly once.

CASE 2 — Compiles. The parameter took ownership, and the final expression
  `s` moved the same value back out into the return slot. `main` owns it
  again. Nothing was copied and no clone happened; the second `let name`
  shadows the first, which is why the compiler says nothing at all. Same
  value, new holder, zero cost.

CASE 3 — Compiles, and `n` is still usable. `i32` implements `Copy`, so
  `double(n)` receives a copy and leaves `n` whole. The function body is
  structurally identical to a String-taking function. The signature is
  what decided the outcome, not the body.

CASE 4 — Compiles. `clone()` duplicated the buffer, so there are two
  values, two owners, and two cleanups. Note that `a == b` is true:
  they are *equal*, not *identical*. Mutating one would not affect the
  other, because they are separate buffers.

CASE 5 — Does not compile, error[E0382]. `Vec<i32>` is not `Copy` even
  though `i32` is — the element type does not make the container
  copyable. `data` moved into `longest` and is gone. Note that the
  function only ever *reads* the vector, so it did not need ownership at
  all. This is the fourth time this module that exact friction has
  appeared. It is the problem borrowing solves.

CASE 6 — Compiles. `let mut` is required because reassigning changes
  which value the binding owns. The first buffer is released at the
  moment of reassignment, not at end of scope. Note that "dropped
  mid-function" is the interesting fact here: Rust does not wait for the
  end to tidy up a value the variable no longer owns.

CASE 7 — Compiles, and this one is worth tracing line by line.
    let a   → a owns a fresh String.
    let b   → clone: b owns a SECOND buffer. a and b are both live.
    let c   → move: b's value transfers to c. b is disarmed, b's buffer
              is now c's responsibility.
    let n   → 5, no heap anything.
    let m   → i32 is Copy, so m is a second independent 5. n still live.
    let d   → shout takes c's String by value and builds a NEW String
              from it, so c is consumed. d owns the new value; c's
              original buffer is freed inside shout.
    the block → temp is created and dropped at the closing brace.
    the last line → a, m, d are all still usable. b and c are not, and
              the compiler is happy because you never used them.
  At end of scope: a's buffer, m (nothing to free), d's buffer, and
  temp's buffer, which was already freed. Every allocation has exactly
  one matching free.

CASE 8 — Does not compile, and NOT because of ownership. This is a plain
  mutability error: `count` is not declared `mut`, so it cannot be
  reassigned. Nothing was moved, nothing was cloned, and the value is
  perfectly alive. This case is in the set on purpose. "It didn't compile"
  and "ownership is broken" are different claims, and learning to tell
  them apart is worth more than any additional case.

If you got CASE 5 wrong, you are in very good company — almost everyone
does, and the reason is that the element type `i32` is Copy, which feels
like it should make the Vec Copy. It does not. The rule is about the
container."#;

pub fn challenges() -> Vec<SeedExercise> {
    vec![
        SeedExercise::new(
            None,
            ExerciseKind::Challenge,
            "OWNERSHIP DETECTIVE — Eight programs. For each one, determine: which values are created, \
             which own heap data, who owns what, where ownership moves, which variables become invalid, \
             what is copied, what is cloned, when each value is dropped, and whether it compiles. \
             Commit to an answer for all eight before opening anything.",
            &[
                "For each program, list every value created and say which own heap data",
                "For each, name the current owner of every String and Vec as you go line by line",
                "Mark every variable that becomes unusable, and say at which line",
                "For each, state whether it compiles — and if not, which error code",
                "Count the allocations and the frees, and check they match",
                "Do all eight before reading a single hint. The pattern only appears in the set.",
            ],
            0,
            WALKTHROUGH,
        )
        .code(ALL_CASES)
        .xp(150)
        .hints(&[
            "Start with the mechanical question: does the type implement Copy? Everything else follows.",
            "For each String, find the line where it stops being usable. If you cannot find one, look for a clone or a round trip.",
            "Case 7 is the one that pays off. Trace it one line at a time and write down the owner after every single line.",
            "Count allocations and frees separately. If they do not match, you have miscounted an ownership event.",
            "One case is not an ownership problem at all. Read every error code before assuming a cause.",
            "Full walkthrough is the last hint. It is much cheaper to be wrong here than to be confidently wrong later.",
        ]),
        SeedExercise::new(
            None,
            ExerciseKind::Challenge,
            "OWNERSHIP DETECTIVE — The same eight cases, but now argue the other side. For each program, \
             say what it would take to make it *better* — and what the cost of that change is. This is \
             the exercise that turns ownership from something the compiler checks into something you \
             design.",
            &[
                "For each case, write the smallest change that would make the intent clearer",
                "For each change, name its cost: an allocation, a reorder, or a borrow",
                "For case 5 specifically, write the signature you would actually use in real code",
                "Then say which of your changes you would defend in code review, and which you would not",
                "Be honest about where you would clone. Cloning on purpose is a decision; cloning by reflex is a habit.",
            ],
            0,
            "The diagnostic exercise teaches you to read ownership. This one teaches you to design it, \
             which is the harder half. A program that compiles and is hard to reason about is worse \
             than one that refuses to compile, because the error is at least honest. Module 6 gives you \
             a third option — borrowing — that solves most of these cases for free, and the reason to \
             understand the costs now is so you can recognise when you have not used it.",
        )
        .code(CASE_5)
        .with_code_b(CASE_7)
        .xp(180)
        .hints(&[
            "Start with case 5. It is the clearest example of a function taking ownership it does not need.",
            "A borrow is a third answer, alongside cloning and reordering. It costs nothing.",
            "For each change, ask: am I making this clearer for a reader, or just making the compiler quiet?",
            "If you cannot justify a clone in one sentence, it is a habit rather than a decision.",
            "The goal is not zero clones. It is clones you can point at and explain.",
        ]),
    ]
}
