# BORROW — Module 10: Generics & Traits

> **Status:** DRAFT SPEC — for review. No implementation starts until this is
> approved. This document is the source of truth the lessons, exercises, and
> the new widget are built from.

---

## 0. Decisions locked

| Decision | Choice |
|---|---|
| Position | Module 10, slug `generics-and-traits`, id 10, ID block 10000 |
| Depth | Deep dive (~32 lessons) |
| Continuous project | **Media Library** — a new codebase that grows across the module |
| Trait depth | Core: generics, defining traits, bounds, derive, `Display`/`From`, `impl Trait` vs `dyn Trait`. Associated types and operator traits named only |
| Interactive UI | Reuse the three widgets, plus **one new widget: a dispatch visualiser** |
| Build order | Spec → widget → content → verify |

---

## 1. Module Purpose

The learner has been *using* generics and traits for four modules without
naming them. `Option<T>`, `Result<T, E>`, `impl From<io::Error>`, `Display`,
`#[derive(Debug)]`, `HashSet<T>` — every one is a generic type or a trait
implementation. This module makes the machinery explicit so the learner can
write their own instead of only consuming the standard library's.

The beginner failure this module must prevent is treating generics and traits as
advanced trivia, reached for only in library code, when in fact they are the
tools that remove duplication and express "any type that can do X". The module
must connect every new idea back to something the learner has already done with
`Option`, `From`, or `derive`.

Central idea:

> A **generic** lets one piece of code work for many types. A **trait** says
> which types are allowed, by naming the behaviour they share.

---

## 2. Audience & Prerequisites

Assume the learner can already:

- define structs and enums, and write methods (Module 07)
- use `Vec`, `HashMap`, `HashSet` and iterate them (Module 08)
- use `Option`, `Result`, `?`, and implement `Display` and `Error` by hand
  (Module 09)

Do **not** re-teach `Option`/`Result` from scratch — use them as the first
examples of generics, and `From`/`Display` as the first traits.

Do **not** teach, and explicitly defer:

- iterators as a topic (Module 11) — `Iterator` appears in one preview lesson,
  named not taught
- lifetimes as a topic (later) — generic code here avoids needing explicit
  lifetime annotations
- associated types beyond naming (e.g. `Iterator::Item`) — mention only
- macros

---

## 3. Beginner-First Principles

1. **Motivation before syntax.** Open with the duplicated function, not with
   `<T>`.
2. **One idea per lesson.**
3. **The Media Library never resets.**
4. **Predict before reveal.**
5. **Plain language; define before use.**
6. **Gentle failure.**
7. **Show, then say.**
8. **No incorrect simplifications.** Never: "generics are templates like C++",
   "traits are interfaces and nothing more", "`dyn` is always slower", "generics
   have no runtime cost so you should always use them", "`#[derive]` copies code
   at runtime".

---

## 4. The Continuous Project — Media Library

One program grows across the module: a small catalogue of media items that can
all be summarised, stored together, and searched.

### The domain (introduced piece by piece)

```rust
// The trait the whole module is built on.
trait Summary {
    fn summarise(&self) -> String;
    fn author(&self) -> String;
}

struct Book    { title: String, author: String, pages: u32 }
struct Article { headline: String, author: String, words: u32 }
struct Video   { title: String, creator: String, minutes: u32 }

// Implemented for each type, by hand and then compared with derive.
impl Summary for Book { /* ... */ }
impl Summary for Article { /* ... */ }
impl Summary for Video { /* ... */ }

// A generic function bounded by the trait.
fn notify<T: Summary>(item: &T) { /* ... */ }

// A container parameterised over a type, held to the trait.
struct Library<T: Summary> { items: Vec<T> }

// And the dynamic version, for a mixed collection.
let everything: Vec<Box<dyn Summary>> = vec![/* ... */];
```

### Lesson-by-lesson growth (high level)

| Stage | Feature added |
|---|---|
| Generics | `fn largest<T>` handles `i32` and `f64`; `Library<T>` is parameterised |
| Traits | `trait Summary`; implemented for `Book`, `Article`, `Video` |
| Derives | `#[derive(Debug, Clone, PartialEq)]` on the media types |
| Dispatch | `fn notify<T: Summary>` vs `fn print_all(&[Box<dyn Summary>])` |
| Real world | the assembled library, search and summary, and a design change |

### Why Media Library

- **Three natural types sharing one behaviour.** A book, an article, and a video
  are different structs that can all be summarised — the textbook reason traits
  exist, without a contrived example.
- **Needs both dispatch styles.** A homogeneous `Library<Book>` is static; a
  mixed shelf of media is `Box<dyn Summary>`.
- **Carries earlier modules forward.** Structs (07), a `Vec` catalogue (08), and
  `Display`/`From` (09) all reappear as the traits under discussion.

---

## 5. New Interactive Widget — Dispatch Visualiser

The one new component. It makes the abstract "static vs dynamic dispatch"
difference concrete by showing both paths over the same call.

**What:** a code panel plus a **Static / Dynamic** toggle. A single Prev/Next
control advances both paths in step, narrating what happens at each stage.

- **Static column** (generics / `impl Trait`): the compiler produces one
  concrete copy of the function per type; the call is direct.
- **Dynamic column** (`dyn Trait`): one function, plus a pointer to a table of
  method pointers; the call goes through the table.

**Behaviour**
- A two-column layout on wide screens, stacked on narrow ones.
- The toggle switches which column is emphasised, but both stay visible.
- Prev/Next moves a highlight over the code and updates both notes for the step.
- Keyboard: ←/→ step, and the toggle is a real radio group for screen readers.

**No-JS fallback:** both columns render as labelled lists of steps; the code
panel is shown once, unhighlighted.

**Data shape (proposed `LessonSection` variant):**

```jsonc
{
  "type": "dispatch",
  "title": "Calling a bounded function vs a trait object",
  "code": "fn notify<T: Summary>(item: &T) { println!(\"{}\", item.summarise()); }\nnotify(&book);\nnotify(&video);",
  "steps": [
    {
      "lines": [1],
      "static_note": "The compiler writes one `notify` for `Book` and another for `Video`.",
      "dynamic_note": "With `dyn Summary`, there is a single `notify` taking a trait object."
    }
  ]
}
```

### Engineering notes

- New `LessonSection` variant `Dispatch` in `src/models/content.rs`; a renderer
  in `views/components.rs`; an initialiser in `static/interactive.js`; styles in
  `static/style.css`. It follows the same progressive-enhancement contract as
  the other three widgets.
- Pure-logic tests (step bounds, unique control groups) live in Rust; the
  no-JS fallback is asserted by a render test.

---

## 6. Module Structure — 32 Lessons

One lesson per part. "Hook" names the block the lesson is built around.

### Generics (`generics.rs`, 7 lessons)

1. **The Duplicated Function** — `largest` written once for `i32`, once for
   `f64`. Hook: observe.
2. **Generic Functions** — `fn largest<T: PartialOrd + Copy>`. Hook: gap_fill.
3. **Generic Structs and Enums** — `Point<T>`; and the reveal that `Option<T>`
   and `Result<T, E>` were always generic. Hook: step_through.
4. **More Than One Parameter** — `<T, U>`, and the `Result<T, E>` shape. Hook:
   gap_fill.
5. **Monomorphisation** — one concrete copy per type, at no runtime cost. Hook:
   dispatch widget.
6. **`where` Clauses** — bounds moved below the signature for readability.
   Hook: gap_fill.
7. **Generics in Methods** — `impl<T> Point<T>` and `impl Point<f64>`. Hook:
   step_through.

### Traits (`traits.rs`, 8 lessons)

8. **The Missing Shared Behaviour** — three types that can all be summarised,
   with no shared type today. Hook: observe.
9. **Defining a Trait** — `trait Summary`. Hook: gap_fill.
10. **Implementing a Trait** — `impl Summary for Book`. Hook: gap_fill.
11. **Calling Trait Methods** — why the trait must be in scope. Hook:
    guess_run.
12. **Default Methods** — a provided `author` built from `summarise`. Hook:
    step_through.
13. **Trait Bounds** — `fn notify<T: Summary>(item: &T)`. Hook: gap_fill.
14. **Bounds on `impl` Blocks** — `impl<T: Summary> Library<T>`. Hook:
    step_through.
15. **Multiple Bounds** — `T: Summary + Clone`, and `where`. Hook: gap_fill.

### Derives (`derives.rs`, 3 lessons)

16. **What `#[derive]` Does** — generates trait implementations. Hook: observe.
17. **The Common Derives** — `Debug`, `Clone`, `PartialEq`, `Default`, `Hash`,
    `PartialOrd`. Hook: table/gap_fill.
18. **When Derive Is Not Enough** — the manual `impl Summary for Book`. Hook:
    step_through.

### Dispatch (`dispatch.rs`, 5 lessons)

19. **`impl Trait` Parameters** — `fn notify(item: &impl Summary)`. Hook:
    gap_fill.
20. **The Mixed Shelf Problem** — a `Vec<Book>` and a `Vec<Video>` cannot share
    one list; the types differ even though the behaviour does not. Hook:
    observe.
21. **`dyn Trait`** — trait objects and `Box<dyn Summary>`. Hook: step_through.
22. **Static vs Dynamic Dispatch** — the trade-off. Hook: dispatch widget.
23. **Choosing** — when each is the right tool. Hook: table.

### Real World (`real_world.rs`, 8 lessons)

24. **`Display` and `ToString`** — implementing a std trait by hand. Hook:
    gap_fill.
25. **`From` and `Into`** — the conversion traits; `Into` comes for free. Hook:
    step_through.
26. **Implementing versus Deriving** — when the generated impl is not what you
    want. Hook: guess_run.
27. **`Iterator`** — a one-lesson preview: what the trait is, deferred to Module
    11. Hook: observe.
28. **Operator Traits** — `Add`, `PartialEq` named; one `impl Add` example.
    Hook: table.
29. **Common Mistakes** — ten, each with a broken example and a rule. Hook:
    guess_run.
30. **The Media Library, Assembled** — the whole program: a generic `Library<T>`,
    `Box<dyn Summary>` mixed shelves, `Display`, and search. Hook: all four.
31. **Change the Design** — extend: sort by a trait method, add a new media type,
    decide dispatch. Hook: all four.

### Model (`model.rs`, 1 lesson)

32. **Final Mental Model** — generics name the shape, traits name the behaviour,
    bounds connect them, dispatch picks the cost. Points ahead to iterators.

---

## 7. Exercise Plan

- **1 exercise per lesson** (~32 total), in lesson order.
- **Formats**: gap-fill (the bound, the `impl`, the type parameter),
  predict-the-output, predict-the-compiler, fix-code, trace-the-dispatch,
  choosing-the-form.
- Every exercise has a one-sentence prompt, at least one hint, and an
  explanation that states the rule.
- No exercise requires writing a whole program; open-ended work lives in the
  challenges and projects.

---

## 8. Challenges (open-ended, `.code` + hints ≥ 2)

1. **DERIVE OR IMPLEMENT** — given several types and required behaviours, decide
   for each whether `#[derive]` suffices or a manual impl is needed, and write
   the ones that need writing. The decisions are the deliverable.
2. **ONE FUNCTION, MANY TYPES** — replace a set of nearly-identical functions
   with a single generic function plus a trait, then use it on the media types.
   Justify the bound you chose.
3. **STATIC OR DYNAMIC?** — given five scenarios (a homogeneous list, a mixed
   list, a hot loop, a plugin-like registry, a function returning "some
   iterator"), choose generic/`impl Trait` or `dyn Trait` for each and defend it.

Each ships with progressive hints and a full walkthrough as the final hint.

---

## 9. Projects (6, escalating)

1. **Generic Playground** — `largest<T>`, `smallest<T>`, and a generic pair,
   run on several concrete types.
2. **Summarise Them All** — the `Summary` trait implemented for three media
   types, called through a generic function.
3. **Derive and Compare** — `#[derive]` on media types; equality, cloning, and
   ordering.
4. **A Generic Library** — `Library<T: Summary>` with `add`, `len`, and a
   summary method.
5. **Mixed Shelves** — `Box<dyn Summary>` for a heterogeneous collection, with
   `Display` and a report.
6. **Media Library (capstone)** — the finished program, plus the written task:
   *for every trait in your program, say whether it is derived or implemented,
   and for every generic function or boxed trait object, say why that dispatch
   form was chosen.*

Each includes name, difficulty, estimated time, goal, requirements, concepts,
constraints, example interaction, and stretch goals.

---

## 10. Diagram Requirements

Diagrams answer a question, never decorate.

- **Duplication vs one generic** — two copies of `largest` collapsing into one.
- **A trait as a contract** — types that promise to provide a method.
- **A generic function as an open slot** — `T` filled in by each caller's type.
- **Static dispatch** — one concrete function generated per type, called
  directly.
- **Dynamic dispatch** — a trait object holding a data pointer and a vtable
  pointer.
- **`Box<dyn Summary>` shelves** — a `Vec` whose elements differ in type.

---

## 11. Content Style & Tone

- Short sentences, concrete nouns, the voice of an experienced Rust developer.
- Every lesson's first line names the problem, not the feature.
- Every lesson ends by advancing the Media Library.
- Prefer a 4-line example over a paragraph; a diagram when the idea is spatial;
  a table for comparisons.
- Avoid: fake enthusiasm, vague "this is important", jargon before definition,
  walls of prose.

---

## 12. Teaching Requirements

1. Motivation before syntax, always.
2. Make the learner predict before revealing.
3. Explain compiler errors in terms of the rule enforced and the design
   alternative. (This module produces many "trait bound not satisfied" errors —
   teach reading them.)
4. Never teach a simplification you will have to un-teach (see §3.8).
5. Prefer the smallest realistic example that shows the idea.

---

## 13. Module Outcome

At the end, given a new problem, the learner can:

- recognise duplication that a generic function or type would remove
- write a generic function with a bound, and read a "trait bound not satisfied"
  error
- define a trait, implement it for a type, and provide a default method
- use `#[derive]` knowingly, and know when a manual impl is required
- choose a bound (`T: Summary + Clone`), and use a `where` clause when it helps
- tell `impl Trait` from `dyn Trait`, and describe the cost of each
- implement `Display` and `From` by hand, and get `ToString`/`Into` for free
- explain why `Option<T>`, `Result<T, E>`, and `Vec<T>` are generics

And they should hold the module's core idea without syntax:

> A **generic** names the shape; a **trait** names the behaviour; a **bound**
> connects them; **dispatch** chooses the cost.

---

## 14. Build Order & Verification

1. **Spec** — this document, reviewed and approved.
2. **Widget** — the `Dispatch` `LessonSection` variant, renderer, JS, CSS,
   no-JS fallback, render tests. Verify it alone first.
3. **Content** — `src/content/generics_traits/` (lessons, exercises,
   challenges, projects), registered in `content/mod.rs`; remove the placeholder.
4. **Verify** — `cargo test`, the ignored `sweep_every_seeded_fragment`, a
   full-program compile check, and a fresh-DB / HTTP smoke test.

---

## 15. Resolved decisions

1. **Monomorphisation** — conceptual "one concrete copy per type", plus a
   diagram. No generated-code sketches.
2. **`impl Trait` returns** — not taught. `impl Trait` is used in parameter
   position only; mixed collections use `Box<dyn Summary>`.
3. **Operator traits** — one worked `impl Add for Point` example.
4. **Dispatch widget** — two columns advanced in tandem.
