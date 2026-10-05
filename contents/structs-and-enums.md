# BORROW — Module 07: Structs & Enums

> **Status:** DRAFT SPEC — for review. No implementation starts until this is
> approved. This document is the source of truth the lesson content, exercises,
> and interactive components are built from.

---

## 0. Decisions locked

| Decision | Choice |
|---|---|
| Position | Module 07, slug `structs-and-enums`, id 7, ID block 7000 |
| Depth | Deep dive (~36 lessons, 2 exercises each, 3 challenges, 6 projects) |
| Continuous project | **Pizza Order** — one codebase that grows across every lesson |
| New interactive UI | Three components: gap-fill inputs, step-through visualiser, predict-then-run |
| Audience | Absolute beginners; assumes only Modules 01–06 |
| Build order | Spec → components → content |

---

## 1. Module Purpose

Modules 01–06 taught how a single value behaves: what a variable is, how it
moves, how to borrow it. Every value so far has been a *primitive* (`i32`,
`bool`, `String`) or a container (`Vec`). This module teaches the two ways to
define **your own kinds of data**:

- **Structs** — several values that belong together and travel as one thing.
- **Enums** — one value that can be exactly one of several possibilities.

The beginner failure this module must prevent is the one that happens silently:
learners memorise `struct` and `enum` syntax but never feel *why* they exist, so
they keep passing five loose variables between functions and never model state
that can be "one of these things". The module must make the motivation arrive
before the syntax, every single time.

Central idea:

> A `struct` says **"these values are one value."**
> An `enum` says **"this value is exactly one of these."**

---

## 2. Audience & Prerequisites

Assume the learner can already, without help:

- declare variables, including `mut`
- write and call functions, including parameters and return values
- understand scope and moves
- borrow with `&T` and `&mut T`, and read a basic borrow error
- use `String` and `Vec<T>`
- recognise `Option` only as "the thing `.unwrap()` is called on" — not yet as
  an enum (that realisation is a payoff *inside* this module)

Do **not** re-teach any of the above. Use it as the floor.

Do **not** teach, and explicitly defer:

- generics and trait definitions (Module 10) — mention `derive` works but do not
  explain `impl Trait` machinery
- lifetimes (after this module)
- `HashMap` (Module 08) — use `Vec` for collections of data

---

## 3. Beginner-First Principles (non-negotiable)

These apply to every lesson, exercise, and widget.

1. **Motivation before syntax.** Never open with "a struct is…". Open with a
   problem the learner can feel: five separate variables that must be passed
   together, or a `status` string that can be silently misspelled.
2. **One idea per lesson.** A lesson that teaches defining *and* methods *and*
   deriving is three lessons wearing a trenchcoat.
3. **The running example never resets.** Every lesson continues the Pizza Order
   program. By the end the learner has watched one realistic program grow.
4. **Predict before reveal.** Every lesson asks a question before showing the
   answer. This is the difference between reading and learning.
5. **Plain language, define before use.** First use of `impl`, `derive`,
   `variant`, `associated function` is defined in the same breath.
6. **Gentle failure.** Wrong answers get a "why", not a score. Hints are always
   one click away. Nothing is timed. Nothing is locked behind passing.
7. **Show, then say.** A 4-line example beats a paragraph. A diagram beats an
   example when the idea is spatial (struct layout, enum variants).
8. **No incorrect simplifications.** Never: "enums are just constants",
   "structs are classes", "derive magically implements anything", "`Option` is
   built in and not an enum".

---

## 4. The Continuous Project — Pizza Order

One program, `pizza_order`, grows across the module. Each lesson adds a real
feature, so the learner always sees the *current* whole file and the *new*
lines. The final lesson shows the completed program.

### The domain (introduced piece by piece, never all at once)

```rust
// Appears in this order over the module:
struct Topping { name: String, extra_cost: u32 }
struct Pizza   { size: Size, toppings: Vec<Topping> }
struct Order   { customer: String, pizzas: Vec<Pizza>, status: OrderStatus }

enum Size        { Small, Medium, Large }
enum OrderStatus { Placed, Preparing, OutForDelivery, Delivered, Cancelled }
```

### Lesson-by-lesson growth (high level)

| Stage | Feature added |
|---|---|
| Foundations | Replace a `(String, u32)` tuple with `Topping` |
| Construction | Build a `Pizza` from a struct literal, then shorthand, then `..` |
| Methods | `Pizza::new`, `pizza.add_topping`, `pizza.price()`, `pizza.describe()` |
| Deriving | `#[derive(Debug, Clone, PartialEq, Default)]` as each is needed |
| Enums | `Size` and `OrderStatus` replace magic strings |
| Matching | `match` to price a size, narrate a status, and validate transitions |
| Patterns | destructure `Pizza`/`Order` in `match`, `if let`, `let else` |
| Capstone | the completed program, plus a written design note |

### Why Pizza Order

- **Relatable.** Everyone has ordered a pizza; no domain knowledge needed.
- **Naturally needs all four ideas.** Sizes are an enum, toppings are structs,
  an order is a struct *containing* enums, and status is an enum with behaviour.
- **Small enough to hold in your head**, large enough to show composition.
- **Ready for the next module**: the same shape reappears with `HashMap` (08)
  and `Option`/`Result` (09).

---

## 5. New Interactive Components

Three components, all beginner-facing. Each has a no-JS fallback that still
teaches (progressive enhancement, matching the existing `app.js` philosophy).

### Component A — Gap-fill inputs

**What:** a code panel with real inline `<input>` fields where the learner types
the missing piece. Pressing CHECK validates each blank and explains it.

**Used for:** field names, field types, struct-literal shorthand, variant names,
`match` arm keywords, `self` forms.

**Behaviour**
- Each blank is a normal text input, sized to its expected answer.
- **CHECK** marks each input with a word, not a colour alone: `✓ correct`,
  `✗ try again`; a wrong blank shows its hint, never the answer outright.
- **REVEAL** fills the current blank's answer and explains it.
- **RESET** clears. Re-checking is always allowed.
- Normalisation: trim whitespace; compare case-insensitively for keywords and
  identifiers; accept `pub ` prefix only where the answer expects it.

**No-JS fallback:** render each blank as an underlined underline-style gap and
put the answers in a `<details>` "Show answers" block.

**Data shape (proposed `LessonSection` variant):**

```jsonc
{
  "type": "gap_fill",
  "intro": "Fill in the three missing field types.",
  "language": "rust",
  "code": "struct Topping {\n    name: ____,\n    extra_cost: ____,\n}",
  "blanks": [
    { "id": "b1", "answer": "String", "hint": "It is owned text." },
    { "id": "b2", "answer": "u32", "hint": "A whole number of cents." }
  ],
  "after": "Optional prose shown after a correct check."
}
```

### Component B — Step-through visualiser

**What:** a Prev / Next panel that walks through one process, one step at a
time, highlighting the relevant code and narrating what just happened. It turns
a static snippet into a guided animation the learner drives.

**Used for:** building a struct field by field; `&self` vs `&mut self` vs `self`
effects; constructing an enum variant; choosing which `match` arm fires.

**Behaviour**
- The snippet is shown with line highlighting that moves with the step.
- A text line under it narrates the current step in beginner language, e.g.
  *"Step 2 of 5 — `size: Size::Large` is stored in the new `Pizza`."*
- Keyboard: ←/→ arrows step; the control is focusable and named for screen
  readers ("Previous step", "Next step", "Step 3 of 5").
- A small memory/ownership annotation can appear beside the code when relevant
  (which value is owned by whom), reusing the diagram style from Module 05.

**No-JS fallback:** render every step stacked as a labelled list; each step's
note sits under its code line.

**Data shape (proposed):**

```jsonc
{
  "type": "step_through",
  "title": "Building a pepperoni pizza",
  "language": "rust",
  "code": "let mut p = Pizza::new(Size::Medium);\np.add_topping(pepperoni);",
  "steps": [
    { "lines": [1], "note": "`new` runs and hands back a Pizza." },
    { "lines": [1], "note": "`Size::Medium` is stored in the `size` field." },
    { "lines": [2], "note": "`&mut self` lets `add_topping` change the pizza in place." }
  ]
}
```

### Component C — Predict-then-run

**What:** the learner locks in a prediction ("prints 8", "won't compile", …),
then presses **RUN**. The real `rustc` output appears beside their guess, and a
short explanation reconciles the two.

**Used for:** "does this compile?" moments, `match` exhaustiveness, what a
derived `Debug` prints, and the ownership interactions that structs introduce.

**Behaviour**
- Prediction options as large radio-style cards (touch-friendly, ≥44px).
- **RUN** posts the snippet to the existing `POST /playground/run` and renders
  stdout / compiler diagnostics exactly like the playground does.
- Feedback language is non-punitive: **"You predicted: won't compile. It
  compiled. Here's why…"** — teaching, not grading.
- If the playground is unavailable (server not on loopback), the RUN button is
  disabled with the existing explanatory message; the prediction + explanation
  still work.

**No-JS fallback:** render the options and a "Show what happens" `<details>`
containing the pre-computed output and the explanation.

**Data shape (proposed):** reuse the `ExerciseView` option machinery; add the
snippet and explanation. This may live as a lesson block *and* as an exercise
kind, sharing one renderer.

### Engineering notes (component implementation)

- **One new JS module**, `static/interactive.js`, loaded on lesson pages only.
  It follows `app.js`'s pattern: find `[data-widget]` roots, wire listeners,
  never assume it is the only thing on the page.
- **Server renders the data.** The blocks are new `LessonSection` variants in
  `src/models/content.rs`; `views/exercise.rs::render_section` gains arms for
  them; page loads include the new script via `views/layout.rs`.
- **Endpoints:** only predict-then-run needs the server, and it **reuses
  `/playground/run` unchanged** — no new compile path, no new sandbox surface.
- **Accessibility:** every widget is keyboard-operable, announces its state,
  and never encodes meaning in colour alone. Reduced-motion is respected.
- **Testing:** pure logic (answer normalisation, step boundaries) goes in Rust
  unit tests; the widgets get no-JS server-rendered tests asserting the fallback
  markup is present, matching the existing "works without JS" bar.

---

## 6. Module Structure — 36 Lessons

One lesson per part. "Hook" names the interactive component the lesson is
designed to use.

### Foundations (`foundations.rs`, 8 lessons)

1. **The Five-Variable Problem** — a pizza described by five loose variables
   that must stay in sync. Hook: observe.
2. **Defining a Struct** — `struct Topping { name: String, extra_cost: u32 }`.
   Hook: gaps (field types).
3. **Creating a Value** — the struct literal. Hook: predict-then-run.
4. **Reading and Changing Fields** — `t.name`, `t.extra_cost += 50`, requires
   `mut`. Hook: gaps (the `let mut`).
5. **Field Types Are Ownership** — a struct containing a `String` owns it; a
   moved-in field moves. Tie to Module 05. Hook: step-through (owner moves in).
6. **Tuple Structs** — `struct Cents(u32)` for when names are noise. Hook:
   gaps.
7. **Unit Structs** — `struct Marker;` and when it is honest. Hook: observe.
8. **Struct vs Tuple vs Loose Variables** — a decision table the learner
   completes. Hook: gaps / classification.

### Construction (`construction.rs`, 5 lessons)

9. **Field Init Shorthand** — `Topping { name, extra_cost }`. Hook: gaps.
10. **Struct Update Syntax** — `..other` to copy the rest. Hook: predict-then-run.
11. **`Default`** — `#[derive(Default)]` and `..Default::default()`. Hook: gaps.
12. **Nested Structs** — a `Pizza` that contains `Vec<Topping>`. Hook:
    step-through.
13. **Collections of Structs** — `Vec<Topping>` and iterating it. Hook:
    predict-then-run.

### Methods (`methods.rs`, 7 lessons)

14. **Methods with `impl`** — `impl Pizza { fn price(&self) -> u32 { … } }`.
    Hook: step-through.
15. **`&self`, `&mut self`, `self`** — the three receivers and what each can do.
    Hook: step-through (three parallel runs).
16. **Associated Functions** — `Pizza::new(...)`, the constructor convention.
    Hook: gaps.
17. **Multiple `impl` Blocks** — legal, and why you would. Hook: observe.
18. **Deriving `Debug`** — printing a struct; `{:?}` and `{:#?}`. Hook:
    predict-then-run.
19. **Deriving `Clone` and `PartialEq`** — copying and comparing structs. Hook:
    predict-then-run.
20. **Methods That Consume `self`** — taking ownership, and when it is right.
    Hook: step-through + borrow error.

### Enums (`enums.rs`, 6 lessons)

21. **The Magic-String Problem** — `status: String` can be `"prepairing"`.
    Hook: observe.
22. **Defining an Enum** — `enum Size { Small, Medium, Large }`. Hook: gaps.
23. **Variants Can Carry Data** — `OrderStatus::Preparing { since: u32 }`.
    Hook: step-through.
24. **Enums vs Structs** — "all of these" vs "one of these". Hook: classification.
25. **`Option` Is an Enum** — the reveal: `Some`/`None` are variants. Hook:
    step-through.
26. **Methods on Enums** — `impl OrderStatus { fn label(&self) -> &str }`.
    Hook: gaps.

### Matching (`matching.rs`, 6 lessons)

27. **`match` Basics** — choosing one arm. Hook: step-through.
28. **Matching Variant Data** — `Size::Large => 1600`. Hook: gaps.
29. **Exhaustiveness** — the compiler forces every case. Hook: predict-then-run.
30. **Wildcards and `_`** — the default arm. Hook: predict-then-run.
31. **`if let`** — when you only care about one variant. Hook: gaps.
32. **`let … else`** — bailing out early on the wrong variant. Hook:
    predict-then-run.

### Patterns & Real World (`patterns.rs`, 3 lessons)

33. **Destructuring in `match`** — pulling a struct apart by field. Hook:
    step-through.
34. **The Ten Common Mistakes** — each with a broken example, prediction,
    compiler behaviour, fix, and rule. Hook: predict-then-run / gaps.
35. **Putting It All Together** — the complete Pizza Order program read end to
    end, then a "change the design" set. Hook: all three.

### Capstone (`model.rs`, 1 lesson)

36. **Final Mental Model** — the two questions ("is this one thing?" / "is this
    one of these?"), the decision procedure, and a checklist of what the learner
    can now do. Points ahead to `Option`/`Result` and generics.

---

## 7. Exercise Plan

- **1 exercise per lesson**, in lesson order (~36 total).
- **Formats**, weighted for beginners: gap-fill, predict-the-output,
  predict-the-compiler, fix-code, trace-the-data, classification, API-design.
- The single exercise is the **application** of the lesson; recognition practice
  lives inside the lesson's interactive blocks.
- Every exercise has: a one-sentence prompt, at least one hint, and an
  explanation that states the rule, not just the answer.
- No exercise requires writing a whole program from a blank editor; open-ended
  work lives in the challenges and projects.
- The upgraded **gap-fill** component is a first-class exercise kind, not just a
  lesson decoration.

---

## 8. Challenges (open-ended, `.code` + hints ≥ 2)

1. **PIZZA STATE MACHINE** — model an order's lifecycle with an enum and enforce
   legal transitions with `match`. The learner writes the transitions and
   explains which states may follow which.
2. **TUPLE REFACTOR** — a working program written entirely with tuples; refactor
   it to structs and an enum, and justify every new type in one sentence.
3. **DESIGN A MENU** — given requirements, the learner defines the structs and
   enums themselves (a menu of pizzas, sizes, and availability), then argues why
   each thing is a struct rather than an enum and vice versa.

Each ships with progressive hints (≥2) and a full walkthrough as the final hint.

---

## 9. Projects (6, escalating)

1. **Struct Playground** — tiny, repetitive: build and print several structs;
   compare tuple vs struct. Beginner.
2. **Topping Cost Calculator** — structs + methods + `Vec`; no enums yet.
3. **Pizza Builder** — `impl`, `&mut self`, `derive(Debug, Clone)`.
4. **Order Status Tracker** — enums + `match`; a small CLI state machine.
5. **Menu & Order Manager** — structs containing enums; search and summarise.
6. **Pizza Order (capstone)** — the full running project, plus the written task:
   *for every type, say whether it is a struct or an enum and why.*

Each project includes name, difficulty, estimated time, goal, requirements,
concepts, constraints, example interaction, and stretch goals (matching the
existing `SeedProject` shape).

---

## 10. Diagram Requirements

Diagrams answer a question, never decorate.

- **Struct layout** — fields as labelled boxes, showing a struct is one value
  containing several.
- **Ownership in a struct** — which fields own heap data, echoing Module 05.
- **Enum variants** — one value, a set of mutually exclusive shapes; the
  "exactly one of" picture.
- **`Option` reveal** — `None` and `Some(T)` drawn as two variants.
- **`match` as a funnel** — a value flowing to exactly one arm.
- **Pizza Order composition** — `Order` containing `Pizza` containing `Topping`,
  with `Size`/`OrderStatus` hanging off the right places.

---

## 11. Content Style & Tone

- Short sentences. Concrete nouns. The voice of an experienced Rust developer
  explaining to a smart friend who is new to systems programming.
- Every lesson's first line names the problem, not the feature.
- Every lesson ends by advancing the running Pizza Order program.
- Prefer a 4-line example over a paragraph; a diagram over an example when the
  idea is spatial; a table for comparisons.
- Avoid: fake enthusiasm, "Rust is amazing", vague "this is important", jargon
  before definition, walls of prose, repeating one point five ways.

---

## 12. Teaching Requirements (rules)

1. Motivation before syntax, always.
2. Make the learner predict before revealing.
3. Explain compiler errors in terms of the rule enforced and the design
   alternative — never "Rust doesn't allow this".
4. Never teach a simplification you will have to un-teach (see §2 and §3.8).
5. Prefer the smallest realistic example that shows the idea.

---

## 13. Module Outcome

At the end, given unfamiliar code, the learner can:

- tell at a glance whether a type should be a struct or an enum
- define and construct structs, and explain why a struct is one value
- write methods and associated functions, and choose between `&self`, `&mut self`,
  and `self`
- derive `Debug`/`Clone`/`PartialEq`/`Default` and say what each gives them
- define enums whose variants carry data
- use `match` exhaustively, with patterns, guards, `if let`, and `let else`
- recognise `Option` as an ordinary enum, not a language special case
- model a small real domain (the pizza order) with named, honest types

And they should feel the module's core idea without having to recall the
syntax:

> A struct is **one thing made of several**.
> An enum is **one thing that is one of several**.

---

## 14. Build Order & Verification

1. **Spec** — this document, reviewed and approved.
2. **Components** — `LessonSection` variants, `views/` rendering arms,
   `static/interactive.js`, no-JS fallbacks, unit tests. Verify with a scratch
   lesson page before any content depends on them.
3. **Content** — `src/content/structs_enums/` (lessons, exercises, challenges,
   projects), registered in `content/mod.rs`; remove the placeholder.
4. **Verify** — `cargo test` (content integrity + widget logic), the ignored
   `sweep_every_seeded_fragment` (every valid Rust panel compiles), a fresh-DB
   smoke test, and a manual pass over each interactive component with and
   without JavaScript.

---

## 15. Resolved decisions

1. **Gap-fill grading** — per-blank instant feedback. Checking a blank marks it
   immediately; REVEAL explains it.
2. **Component C copy** — the button says **GUESS, THEN RUN**.
3. **Exercise count** — one per lesson (~36); extra practice lives in the
   projects.
4. **Challenges** — three open-ended challenges.
5. **`let … else`** — included in Module 07.
