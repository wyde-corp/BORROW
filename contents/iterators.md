# BORROW — Module 11: Iterators

> **Status:** DRAFT SPEC — for review. No implementation starts until this is
> approved. This document is the source of truth the lessons, exercises, and
> the new widget are built from.

---

## 0. Decisions locked

| Decision | Choice |
|---|---|
| Position | Module 11, slug `iterators`, id 11, ID block 11000 — **the final module** |
| Depth | Deep dive (~28 lessons) |
| Continuous project | **Log Analyzer** — a new codebase that grows across the module |
| Iterator depth | Using iterators in depth; writing `impl Iterator` by hand shown briefly |
| Interactive UI | Reuse the four widgets, plus **one new widget: a pipeline visualiser with a lazy/eager reveal** |
| Build order | Spec → widget → content → verify |

---

## 1. Module Purpose

One abstraction has been running under every module since the start: `Iterator`.
It is behind `for` loops, `.map`, `.filter`, `.sum`, and every `.collect()`. This
module names it, teaches the two halves — **adapters** that describe work and
**consumers** that drive it — and makes **laziness** concrete: nothing happens
until a consumer asks for it.

The beginner failure this module must prevent is writing out a manual index
loop, pushing into a `Vec`, and filtering with `if` when the same three lines
read as a pipeline — and, just as often, chaining six adapters where a `for`
loop was clearer. The skill is using the right tool, not always reaching for
chaining.

Central idea:

> An **adapter** (`map`, `filter`, `take`) describes work and does nothing yet.
> A **consumer** (`collect`, `sum`, `fold`) runs the whole pipeline, pulling
> elements one at a time.

---

## 2. Audience & Prerequisites

Assume the learner can already:

- use `Vec`, `HashMap`, `HashSet`, and iterate them with `for` (Module 08)
- use `Option` and `Result` with `?` (Module 09)
- read and write traits, bounds, and `impl Trait` parameters (Module 10)
- write closures as arguments, as seen in `or_insert_with` and `map` (Modules
  07–10)

Do **not** re-teach `Option`/`Result` or traits — use them as the values
flowing through pipelines.

Do **not** teach, and explicitly defer / omit:

- async iterators and `Stream`
- `size_hint` and iterator performance internals
- rayon / parallel iterators
- the full `Iterator` method list — teach a representative core

---

## 3. Beginner-First Principles

1. **Motivation before syntax.** Open with the manual loop, then show the
   pipeline that replaces it.
2. **One idea per lesson.**
3. **The Log Analyzer never resets.**
4. **Predict before reveal.**
5. **Plain language; define before use** — *adapter*, *consumer*, *lazy*,
   *exhausted* are each defined on first use.
6. **Gentle failure.**
7. **Show, then say.**
8. **No incorrect simplifications.** Never: "iterators are faster than loops",
   "`.iter()` and `.into_iter()` are the same", "an iterator always runs as soon
   as you call `.map`", "chains are always more readable than loops", "`collect`
   just makes a `Vec`".

---

## 4. The Continuous Project — Log Analyzer

One program grows across the module: a small tool that parses log lines and
answers real questions about them with iterators.

### The domain (introduced piece by piece)

```rust
#[derive(Debug, Clone, PartialEq)]
enum Level {
    Error,
    Warn,
    Info,
}

#[derive(Debug, Clone, PartialEq)]
struct LogEntry {
    level: Level,
    source: String,
    message: String,
}

// Parsed from lines like "ERROR db connection refused".
fn parse_line(line: &str) -> Result<LogEntry, ParseError>;

// The questions the module answers with iterators:
//   which entries are errors?
//   what messages did they carry?
//   how many of each level?
//   which sources log the most?
//   what are the top three messages?
```

### Lesson-by-lesson growth (high level)

| Stage | Feature added |
|---|---|
| Basics | iterate entries with `for`, then `.iter()` |
| Adapters | `.filter(|e| e.level == Level::Error)`, `.map(|e| e.message.as_str())` |
| Consumers | `.count()`, `.collect()`, `.fold()` for totals |
| Laziness | see why the pipeline does nothing until the consumer runs |
| Real world | the assembled analyzer: filter, count, top N |

### Why Log Analyzer

- **A list with a filter, a map, and a count** — the three iterator operations
  a beginner meets first, in a realistic context.
- **Carries Modules 07–10 forward.** `Level` is an enum, `LogEntry` is a struct,
  entries live in a `Vec`, `parse_line` returns a `Result`, and a `LogError`
  implements `Display` — all older material, now driven by iterators.
- **A believable tool.** Filtering errors and finding the noisiest source is
  what a real log analyzer does, on a scale where a pipeline reads better than a
  loop.

---

## 5. New Interactive Widget — Pipeline Visualiser (lazy/eager)

The one new component. It makes laziness concrete by showing the same pipeline
in two phases.

**What:** a code panel of a chain — `entries.iter().filter(...).map(...).collect()`
— plus a step control and a phase badge that reads **LAZY** or **EAGER**.

- **Lazy phase:** the adapters are described; no element has moved. The badge
  reads LAZY and the note explains that nothing runs yet.
- **Eager phase:** a consumer has driven the pipeline; each step shows an
  element flowing through filter → map → collect.

**Behaviour**
- A Prev/Next control steps through the authored steps; the first steps are
  labelled lazy, the rest eager.
- The phase badge switches automatically with the step, styled differently for
  each phase.
- The relevant line of the chain is highlighted at each step.
- Keyboard: ←/→ step; the control is a labelled, focusable element.

**No-JS fallback:** every step renders as a list item with its phase label and
note, and the code panel is shown unhighlighted.

**Data shape (proposed `LessonSection` variant):**

```jsonc
{
  "type": "pipeline",
  "title": "Nothing runs until collect",
  "code": "let errors: Vec<&str> = entries\n    .iter()\n    .filter(|e| e.is_error())\n    .map(|e| e.message.as_str())\n    .collect();",
  "steps": [
    { "lines": [3], "phase": "lazy", "note": "filter is set up. No element has moved." },
    { "lines": [4], "phase": "lazy", "note": "map is set up. Still nothing." },
    { "lines": [5], "phase": "eager", "note": "collect asks for the first item; the pipeline starts." },
    { "lines": [5], "phase": "eager", "note": "collect pulls until it has all the mapped errors." }
  ]
}
```

### Engineering notes

- New `LessonSection` variant `Pipeline` in `src/models/content.rs`; a builder
  in `content/types.rs`; a renderer in `views/components.rs`; an initialiser in
  `static/interactive.js`; styles in `static/style.css`.
- The fragment sweep must skip this widget type too, as it does the other four.
- Pure-logic tests (step bounds, phase switching) in Rust; the no-JS fallback
  asserted by a render test.

---

## 6. Module Structure — 28 Lessons

One lesson per part. "Hook" names the block the lesson is built around.

### Basics (`basics.rs`, 6 lessons)

1. **The Loop You Keep Writing** — an index loop that filters and collects by
   hand. Hook: observe.
2. **What an Iterator Is** — a thing you can call `.next()` on; the trait. Hook:
   step_through.
3. **Three Ways to Iterate** — `iter()`, `iter_mut()`, `into_iter()` and what
   each yields. Hook: gap_fill.
4. **The `for` Loop Desugared** — `for x in y` uses `IntoIterator` and
   `Iterator`. Hook: observe.
5. **Adapters versus Consumers** — the two halves of the ecosystem. Hook: table.
6. **Ranges and Sources** — `0..n`, `.chars()`, `.lines()`, `.split()`. Hook:
   gap_fill.

### Adapters (`adapters.rs`, 6 lessons)

7. **`map`** — transform each element. Hook: gap_fill.
8. **`filter`** — keep elements that pass. Hook: gap_fill.
9. **`take` and `skip`** — bound a stream. Hook: predict_output.
10. **`enumerate`** — pair each element with its index. Hook: predict_output.
11. **`zip` and `chain`** — combine two streams. Hook: step_through.
12. **`filter_map` and `flat_map`** — filter-and-transform, and flatten a
    stream of streams. Hook: gap_fill.

### Consumers (`consumers.rs`, 6 lessons)

13. **`collect`** — gather into a collection; the type annotation does the work.
    Hook: gap_fill.
14. **`sum` and `product`** — reduce to one number. Hook: predict_output.
15. **`count`, `any`, `all`** — ask about the whole stream. Hook: guess_run.
16. **`find` and `position`** — stop at the first match. Hook: predict_output.
17. **`fold`** — the general reduce. Hook: step_through.
18. **`max_by`, `min_by`, `last`** — pick one out. Hook: predict_output.

### Laziness (`laziness.rs`, 4 lessons)

19. **Nothing Runs Until a Consumer** — the lazy/eager split. Hook: **pipeline
    widget**.
20. **Infinite Iterators** — `0..`, `repeat`, `cycle`, bounded by `take`. Hook:
    pipeline widget / observe.
21. **Driving with `next`** — manual iteration and the `while let` loop. Hook:
    step_through.
22. **An Iterator Is Exhausted Once** — `by_ref`, and why a second `for` finds
    nothing. Hook: guess_run.

### Custom (`custom.rs`, 3 lessons)

23. **Returning `impl Iterator`** — a function that yields a pipeline without
    collecting it. Hook: gap_fill.
24. **Writing an Iterator by Hand** — `impl Iterator for ...` and `next`, shown
    briefly. Hook: step_through.
25. **Common Mistakes** — ten, each with a broken example and a rule. Hook:
    guess_run.

### Real World (`real_world.rs`, 2 lessons)

26. **The Log Analyzer, Assembled** — the whole program: parse, filter, count,
    group, top N. Hook: all five.
27. **Filtering, Counting, Top N** — the three questions the module exists to
    answer, each as a pipeline. Hook: pipeline widget.

### Model (`model.rs`, 1 lesson)

28. **Final Mental Model** — adapters describe, consumers drive, laziness means
    nothing runs early. Plus a short wrap of the whole course.

---

## 7. Exercise Plan

- **1 exercise per lesson** (~28 total), in lesson order.
- **Formats**: gap-fill (the adapter or consumer), predict-the-output,
  predict-the-compiler, fix-code, trace-the-pipeline, choosing-the-form.
- Every exercise has a one-sentence prompt, at least one hint, and an
  explanation that states the rule.
- Several exercises contrast the iterator form with the equivalent `for` loop,
  so the learner sees both and judges which reads better.

---

## 8. Challenges (open-ended, `.code` + hints ≥ 2)

1. **PIPELINE OR LOOP?** — given several data-processing tasks, write each both
   as an iterator chain and as a `for` loop, then say which you would ship and
   why. The judgment is the deliverable.
2. **TOP N BY SOURCE** — given log entries, find the sources that log the most,
   including one that requires a `HashMap` intermediate. Decide where the
   pipeline ends and an explicit loop begins.
3. **BUILD A LAZY PIPELINE** — write a function returning `impl Iterator` that
   filters and maps a slice, then drive it with different consumers, proving it
   is lazy by adding a `println!` inside a closure and observing when it runs.

Each ships with progressive hints and a full walkthrough as the final hint.

---

## 9. Projects (6, escalating)

1. **Iterator Playground** — deliberate repetition: the same list driven by
   `for`, by `.map`, and by `.filter()`, recording what each produces.
2. **Filter and Map** — pull the errors out of log entries and map them to
   messages.
3. **Count and Summarise** — count by level, sum a value, find the maximum.
4. **Top N** — a `HashMap` count fed by an iterator, then sorted and limited.
5. **A Lazy Report** — a function returning `impl Iterator` that the caller
   drives with whichever consumer fits.
6. **Log Analyzer (capstone)** — the finished tool, plus the written task: *for
   each pipeline, say what the adapters describe and which consumer drives it,
   and name one place a `for` loop would have read better.*

Each includes name, difficulty, estimated time, goal, requirements, concepts,
constraints, example interaction, and stretch goals.

---

## 10. Diagram Requirements

Diagrams answer a question, never decorate.

- **A pipeline** — source → adapter → adapter → consumer, left to right.
- **Lazy before, eager after** — the adapters built but idle until the consumer
  pulls.
- **One element's journey** — a single item passing filter, then map, then into
  the result.
- **Exhaustion** — an iterator consumed once, leaving nothing for a second pass.
- **The `for` loop desugared** — `for` as `into_iter` plus `next` in a loop.

---

## 11. Content Style & Tone

- Short sentences, concrete nouns, the voice of an experienced Rust developer.
- Every lesson's first line names the problem, not the feature.
- Every lesson ends by advancing the Log Analyzer.
- Prefer a 4-line example over a paragraph; a diagram when the idea is spatial;
  a table for comparisons.
- Avoid: fake enthusiasm, vague "this is important", jargon before definition,
  walls of prose.

---

## 12. Teaching Requirements

1. Motivation before syntax, always.
2. Make the learner predict before revealing.
3. Explain compiler errors in terms of the rule enforced and the design
   alternative. (This module produces many closure and borrow errors — teach
   reading them.)
4. Never teach a simplification you will have to un-teach (see §3.8).
5. Prefer the smallest realistic example that shows the idea.

---

## 13. Module Outcome

At the end, the learner can:

- read `entries.iter().filter(...).map(...).collect()` and say what each part
  does
- choose `iter`, `iter_mut`, or `into_iter` from what the body needs
- name the common adapters and consumers and reach for the right one
- explain laziness and why nothing runs until a consumer
- drive an iterator manually with `next`, and explain exhaustion
- return `impl Iterator` from a function instead of an intermediate `Vec`
- recognise when a chain is less readable than a `for` loop, and choose
- write `impl Iterator` by hand for a simple type

And they should hold the module's core idea without syntax:

> An **adapter** describes work; a **consumer** runs it; nothing happens until
> the consumer asks.

---

## 14. Build Order & Verification

1. **Spec** — this document, reviewed and approved.
2. **Widget** — the `Pipeline` `LessonSection` variant, builder, renderer, JS,
   CSS, no-JS fallback, and a step in the fragment sweep to skip it. Verify it
   alone first.
3. **Content** — `src/content/iterators/` (lessons, exercises, challenges,
   projects), registered in `content/mod.rs`; remove the last placeholder.
4. **Verify** — `cargo test`, the ignored `sweep_every_seeded_fragment`, a
   full-program compile check, and a fresh-DB / HTTP smoke test.

---

## 15. Resolved decisions

1. **`flat_map`** — included alongside `filter_map` in lesson 12.
2. **`fold`** — taught fully (lesson 17), as the general reduce.
3. **Writing `impl Iterator` by hand** — one small example (lesson 24).
4. **Final lesson** — the mental model only; no course recap.
