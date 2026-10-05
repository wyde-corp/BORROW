# BORROW — Module 09: Option & Result

> **Status:** DRAFT SPEC — for review. No implementation starts until this is
> approved. This document is the source of truth the lessons, exercises, and
> interactive blocks are built from.

---

## 0. Decisions locked

| Decision | Choice |
|---|---|
| Position | Module 09, slug `option-and-result`, id 9, ID block 9000 |
| Depth | Deep dive (~30 lessons) |
| Continuous project | **Todo CLI** — a new codebase that grows across the module |
| Error depth | Practical: a custom error enum, `From` conversions, `?`. `Box<dyn Error>` named only |
| Interactive UI | Reuse `gap_fill`, `step_through`, `guess_run` — no new front-end code |
| Build order | Spec → content → verify |

---

## 1. Module Purpose

Every lookup in Module 08 returned an `Option`, and the learner handled it with
`match` and `if let` without ever being told the full story. This module tells
it: `Option` and `Result` are the two ways Rust says *this might not be here* and
*this might fail*, and they are the reason so little Rust code panics.

The beginner failure this module must prevent is the reflex to call `.unwrap()`
to make the compiler stop complaining, and the related belief that errors are
something you handle "later". By the end, the learner should reach for `?`,
`ok_or`, and a small error enum before reaching for `unwrap`, and should know the
narrow cases where `unwrap` is genuinely fine.

Central idea:

> `Option<T>` is **"a T, or nothing"**. `Result<T, E>` is **"a T, or a reason it
> failed"**. Both are enums, and both make the missing case impossible to
> ignore.

---

## 2. Audience & Prerequisites

Assume the learner can already:

- define and match enums with data-carrying variants, and use `if let` / `let
  else` (Module 07)
- use `Vec`, `HashMap`, and their `Option`-returning methods (Module 08)
- read and write methods, and choose `&self` / `&mut self` / `self`

Do **not** re-teach `Option` as if it were new — Module 07 already revealed it is
an enum. Build on that.

Do **not** teach, and explicitly defer:

- generics and trait bounds (Module 10) — use `Result<T, E>` and `impl
  From<...>`, and explain only as much as the task needs
- iterators as a topic (Module 11) — `map`, `and_then`, and `collect` on
  `Result` appear here, but as named methods, not a tour of the iterator trait
- closures as a topic (Module 11) — `unwrap_or_else(|| ...)` and
  `or_insert_with(...)` are shown by example without explaining closure syntax
  in depth

---

## 3. Beginner-First Principles

1. **Motivation before syntax.** Open with the crash ("this unwrap panicked on
   an id that was not in the list"), not "`Result` is…".
2. **One idea per lesson.**
3. **The Todo CLI never resets.** Every lesson continues one program.
4. **Predict before reveal.**
5. **Plain language; define before use.**
6. **Gentle failure.** Hints available, no timers, no locking.
7. **Show, then say.**
8. **No incorrect simplifications.** Never: "`unwrap` is always wrong", "`?`
   only works in `main`", "`Result` is for IO and `Option` is for everything
   else", "`panic!` is the same as returning an error".

---

## 4. The Continuous Project — Todo CLI

One program grows across the module. The learner always sees the current whole
file and the new lines.

### The domain (introduced piece by piece)

```rust
// Appears in this order over the module:
struct Task {
    id: u32,
    title: String,
    done: bool,
}

struct TodoList {
    tasks: Vec<Task>,
    next_id: u32,
}

// The custom error type, built across the "errors" lessons.
#[derive(Debug)]
enum TodoError {
    NotFound(u32),
    EmptyTitle,
    Parse(String),
    Io(std::io::Error),
}
```

### Lesson-by-lesson growth (high level)

| Stage | Feature added |
|---|---|
| Option | `find(&self, id) -> Option<&Task>`; `match`/`if let`/`unwrap_or` |
| Result | `mark_done(&mut self, id) -> Result<(), TodoError>`; `?` |
| Errors | the `TodoError` enum; `From<io::Error>`; storing a title that is missing |
| Patterns | flattening a chain of fallible steps with `?` |
| Real world | `parse_line` and file load/save; the assembled CLI |

### Why Todo CLI

- **Relatable.** Everyone has used a to-do list; no domain setup needed.
- **Naturally produces both cases.** Finding a task by id is *absence*
  (`Option`); marking an absent task done, rejecting an empty title, or failing
  to read the file are *errors* (`Result`).
- **Carries the previous modules forward.** A `Vec<Task>`, a custom enum, and
  `derive(Debug)` are all reused from Modules 07–08.
- **Ends in a real program.** Load a file, add and search tasks, mark done,
  save, and report what went wrong — the shape of countless command-line tools.

---

## 5. Interactive Components (reused)

No new front-end code; the three widgets carry the load where they fit.

### `gap_fill` — the method you actually want
- `unwrap_or`, `unwrap_or_else`, `unwrap_or_default`
- `ok_or` / `ok_or_else` (turning `None` into an error)
- `map` / `and_then` on `Option`
- the `?` in a function returning `Result`
- a custom error variant with data (`NotFound(id)`)

### `step_through` — what actually happens
- **`?` propagation**: an `Err` unwinds out of the function one step at a time
- **`From` conversion**: an `io::Error` becomes a `TodoError::Io` implicitly at
  the `?`
- **matching a `Result`**: `Ok` arm binds the value, `Err` arm binds the reason
- **`unwrap_or`**: the default is only built when the value is missing

### `guess_run` — predict, then compile
- `.unwrap()` on `None` panics; `unwrap_or` does not
- `?` in a function that does not return a compatible type fails to compile
- `find` returns `Option`, `mark_done` returns `Result` — the difference in
  handling

All three keep their no-JS fallbacks.

---

## 6. Module Structure — 30 Lessons

One lesson per part. "Hook" names the interactive block the lesson is built
around.

### Option (`option.rs`, 8 lessons)

1. **The Missing Task** — looking up id 7 that does not exist; the panic and the
   alternative. Hook: observe.
2. **`Option` Is a Value** — `Some`/`None` as an ordinary enum (recap of Module
   07). Hook: guess_run.
3. **Getting the Value Out** — `match`, `if let`, `let else`. Hook: gap_fill.
4. **`unwrap` and `expect`** — they panic; `expect` adds a message; when that is
   acceptable. Hook: guess_run.
5. **`unwrap_or` and `unwrap_or_else`** — real defaults, lazily built. Hook:
   gap_fill.
6. **`map` and `and_then`** — transforming inside an `Option` without unwrapping;
   the difference between them. Hook: step_through.
7. **`?` on `Option`** — early return from a function that yields `Option`.
   Hook: step_through.
8. **Turning `None` into an Error** — `ok_or` / `ok_or_else`. Hook: gap_fill.

### Result (`result.rs`, 8 lessons)

9. **The Failing Operation** — marking a task done that does not exist; why a
   panic is the wrong answer. Hook: observe.
10. **`Ok` and `Err`** — constructing and matching; the `Err` carries a reason.
    Hook: gap_fill.
11. **Reading the Error** — binding the error's data in a `match`. Hook:
    step_through.
12. **`?` on `Result`** — propagation, and what the error type must be. Hook:
    step_through.
13. **`map` and `map_err`** — transforming success or failure without matching.
    Hook: gap_fill.
14. **`and_then` and Chaining** — a sequence of fallible steps. Hook:
    guess_run.
15. **Defaults on `Result`** — `unwrap_or`, `unwrap_or_default`,
    `unwrap_or_else`. Hook: gap_fill.
16. **Combining `Option` and `Result`** — `ok_or` to cross from one to the other.
    Hook: step_through.

### Errors (`errors.rs`, 6 lessons)

17. **One Error Type for the Program** — the `TodoError` enum. Hook: observe.
18. **`From` and the `?` Conversion** — `impl From<io::Error> for TodoError`.
    Hook: step_through.
19. **Printing an Error** — `Debug` versus a human message; `Display` shown by
    example only. Hook: gap_fill.
20. **Errors That Carry Data** — why `NotFound(u32)` beats a bare
    `NotFound`. Hook: gap_fill.
21. **`Box<dyn Error>`** — named, for quick programs, not taught in depth.
    Hook: observe/table.
22. **When to Panic** — bugs versus expected errors; the narrow case for
    `panic!`/`expect`. Hook: table.

### Patterns (`patterns.rs`, 4 lessons)

23. **`match`, `if let`, `let else`** — choosing the shape for the job. Hook:
    gap_fill.
24. **Early Return versus Nesting** — flattening a fallible chain with `?`.
    Hook: step_through.
25. **Several Errors in One Function** — multiple `?`, one error type. Hook:
    step_through.
26. **Stopping at the First Failure** — `collect::<Result<Vec<_>, _>>()`. Hook:
    guess_run.

### Real World (`real_world.rs`, 3 lessons)

27. **Parsing a Todo Line** — `parse_line(&str) -> Result<Task, TodoError>`, and
    a `Parse` error carrying the bad input. Hook: step_through.
28. **Loading and Saving** — file IO with `From` and `?` all the way. Hook:
    gap_fill.
29. **The Todo CLI, Assembled** — the whole program, plus the ten common
    mistakes and a change-the-design set. Hook: all three.

### Model (`model.rs`, 1 lesson)

30. **Final Mental Model** — `Option` for absence, `Result` for failure, `?` to
    propagate, and the decision procedure. Points ahead to generics and traits
    (Module 10).

---

## 7. Exercise Plan

- **1 exercise per lesson** (~30 total), in lesson order.
- **Formats**: gap-fill (the method you want), predict-the-output,
  predict-the-compiler, fix-code, trace-the-error, choosing-the-shape.
- Each exercise is the *application* of the lesson; recognition practice lives
  in the interactive blocks.
- Every exercise has a one-sentence prompt, at least one hint, and an
  explanation that states the rule.
- No exercise requires writing a whole program; open-ended work lives in the
  challenges and projects.

---

## 8. Challenges (open-ended, `.code` + hints ≥ 2)

1. **RECOVER, DO NOT PANIC** — a working Todo program that calls `.unwrap()`
   everywhere. Refactor every unwrap into honest handling: `Option` where a
   thing may be missing, `Result` where an operation may fail, and a custom
   error enum. Justify each change.
2. **PARSE THE TODO FILE** — parse a small text format into `Vec<Task>`, where
   a malformed line is a `Result` error that names the offending input, not a
   panic. Decide whether the parser stops at the first bad line or collects all
   of them, and say why.
3. **DESIGN AN ERROR TYPE** — given a description of everything that can go
   wrong in a small program, design the error enum: which variants, which carry
   data, which convert from a library error. No code required; the design and
   its justification are the deliverable.

Each ships with progressive hints and a full walkthrough as the final hint.

---

## 9. Projects (6, escalating)

1. **Option Playground** — deliberately repetitive: `find`, `get`, and friends;
   match each against `unwrap`, `unwrap_or`, and `if let`.
2. **Safe Lookup** — a task list with `find` returning `Option`, and every
   caller handling the missing case.
3. **Fallible Operations** — `add` and `mark_done` returning `Result`, with a
   first custom error enum.
4. **Robust Parser** — `parse_line` returning `Result<Task, ParseError>`.
5. **Todo CLI** — load, add, search, mark done, save; errors reported, not
   panicked.
6. **Todo CLI (capstone)** — the finished program, plus the written task: *for
   every function that can fail, say whether it returns `Option` or `Result`,
   and why — and justify every remaining `.unwrap()`.*

Each includes name, difficulty, estimated time, goal, requirements, concepts,
constraints, example interaction, and stretch goals (matching `SeedProject`).

---

## 10. Diagram Requirements

Diagrams answer a question, never decorate.

- **`Option` as two boxes** — `Some(value)` and `None`, one at a time.
- **`Result` as two boxes** — `Ok(value)` and `Err(reason)`.
- **`?` as an early exit** — a success path continuing, a failure path leaving
  the function with the error.
- **`From` at the `?`** — an `io::Error` reshaped into a `TodoError` as it
  propagates.
- **`unwrap` as a trapdoor** — the value if present, a panic if not.

---

## 11. Content Style & Tone

- Short sentences, concrete nouns, the voice of an experienced Rust developer.
- Every lesson's first line names the problem, not the feature.
- Every lesson ends by advancing the Todo CLI.
- Prefer a 4-line example over a paragraph; a diagram when the idea is spatial;
  a table for comparisons.
- Avoid: fake enthusiasm, vague "this is important", jargon before definition,
  walls of prose.

---

## 12. Teaching Requirements

1. Motivation before syntax, always.
2. Make the learner predict before revealing.
3. Explain compiler errors in terms of the rule enforced and the design
   alternative.
4. Never teach a simplification you will have to un-teach (see §3.8).
5. Prefer the smallest realistic example that shows the idea.

---

## 13. Module Outcome

At the end, given a new function, the learner can:

- tell whether its "missing" case is absence (`Option`) or failure (`Result`)
- read a value out with `match`, `if let`, or `let else`, and choose the shape
- replace most `.unwrap()` calls with `?`, `unwrap_or`, `map`, or `ok_or`
- write a small error enum with data, and a `From` conversion so `?` works
- propagate errors up a call chain and report them once, near the top
- name why `.unwrap()` panics, and the narrow cases where it is fine
- recognise `Box<dyn Error>` as a quick option and know its trade-off

And they should hold the module's core idea without syntax:

> `Option` is for **absence**. `Result` is for **failure**. `?` is how failure
> travels.

---

## 14. Build Order & Verification

1. **Spec** — this document, reviewed and approved.
2. **Content** — `src/content/option_result/` (lessons, exercises, challenges,
   projects), registered in `content/mod.rs`; remove the placeholder.
3. **Verify** — `cargo test` (content integrity + widget rendering), the ignored
   `sweep_every_seeded_fragment`, a full-program compile check, and a fresh-DB /
   HTTP smoke test.

---

## 15. Resolved decisions

1. **`std::error::Error`** — implemented for `TodoError` (with `Display`), so it
   composes with `Box<dyn Error>`. The trait is shown by example, not taught as
   a topic.
2. **`?` in `main`** — taught. The assembled CLI has
   `fn main() -> Result<(), TodoError>`, and `Box<dyn Error>` is named as the
   quick alternative.
3. **`collect` on `Result`** — lesson 26 stays (stop at the first failure).
4. **`.unwrap` policy** — fine while prototyping, replace before shipping. The
   module teaches the replacement tools and the narrow cases where a panic is
   the honest answer.
