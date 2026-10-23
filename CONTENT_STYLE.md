# BORROW_ — Content Style & Checklist

Every module is written and reviewed against this checklist. It exists so the
writing is consistent across the whole course: motivation first, every term
defined before it is used, working code that compiles, and at least one
interactive beat per lesson.

---

## 1. The shape of a lesson

1. **Motivation** — open with the problem or the feeling, never the feature name.
2. **Define** — every new term, type, and syntax is defined before it is used.
3. **Example** — the smallest realistic, runnable snippet.
4. **Explain** — 2–4 real sentences that carry the concept. Not one line before
   an error dump.
5. **Interactive beat** — at least one of `gap_fill`, `step_through`,
   `guess_run`, `dispatch`, `pipeline`.
6. **Reinforce** — a `table` or `diagram` only when it adds; not by default.
7. **Advance the running project / link forward.**

A lesson may end with a short "try it" callout (1–2 sentences) only if it needs
no untaught concept. Long practice prose does not belong here — it becomes a
widget or an exercise.

---

## 1b. Paragraph labels

- Prose paragraphs introduced with `labelled()` use exactly two default
  labels, in uppercase with an em-dash: **`THE IDEA`** (introduces the idea)
  and **`LET'S BREAK IT DOWN`** (unpacks it).
- Never `CONCEPT` / `EXPLANATION` — they read like a textbook, not a person.
- Free-form labels are allowed when the paragraph is a specific experiment or
  case (`"Experiment 1 — …"`), but they keep the same warm register.

---

## 2. Defined-first rules

- The first use of any term, type, or syntax is defined in the same paragraph or
  an immediately following **bridge**.
- Forward references from later modules get an explicit bridge, e.g.
  *"`fn name(x: T) -> U` is a reusable block that takes a value and gives one
  back — functions are Module 04."*
- Each module keeps a one-line glossary; a term is defined once, consistently,
  and reused thereafter.
- Guard list to check on every lesson: `if`, `for`, `while`, `fn`, `Vec`,
  `String`, `&str`, `.parse()`, `.unwrap()`, `format!`, method calls (dot),
  ranges, `()`, format specifiers (`{:?}`, `{:.2}`).

---

## 3. Code-panel rules

| Kind | Builder | Renders |
|---|---|---|
| Valid, runnable | `code("rust", …)` | a normal panel with TRY THIS |
| Deliberately invalid | `broken(…)` | the ✗ “THIS DOES NOT COMPILE” banner |
| The corrected version | `fixed(…)` | the ✓ “CORRECTED” banner |
| Compiler output | `broken_text(…)` | a flagged text panel |
| Plain data / pseudo | `code("text", …)` | a non-runnable panel |

- **Never** write “this does not compile” as a comment on a normal `code` panel.
- Every `code` and `fixed` fragment must compile (the fragment sweep checks
  them). Widgets are exempt — their code may be a partial illustration.
- `diagram` titles are single-line.

---

## 4. Interactivity

- Every lesson has at least one interactive beat.
- **`guess_run`** — “does it compile / what prints?”, then the real compiler.
- **`gap_fill`** — the keyword or type the learner must supply.
- **`step_through`** — what happens, line by line.
- **`dispatch`** / **`pipeline`** — for the conceptual lessons that need them.
- Practice is a widget or an exercise, not unverifiable prose.

---

## 5. Voice (lighter, still precise)

- Warm, direct, a little wry — an experienced Rust developer explaining to a
  smart friend.
- Never at the reader's expense. No non-teaching jokes. No swipes at other
  languages.
- A joke must serve the teaching or be cut.
- A 4-line example beats a paragraph; a diagram beats an example when the idea
  is spatial.

---

## 6. Exercises

- The exercise is the *application* of the lesson, not pure recall.
- Prefer `predict_compile`, `fix_code`, `gap_fill`, `predict_output`, and
  `api_design` over bare `mcq` when the lesson is about behaviour.
- The prompt and its attached code must match.
- Every exercise has at least one hint and an explanation that states the rule.
- No exercise requires an untaught concept without a bridge or a hint.

---

## 7. Recap

- The final lesson of every module includes a `checklist` of “You can now do
  this”.
- Near the start, a one-paragraph map of what the module covers.

---

## 8. Machine checks before shipping a module

- No `does not compile` or `error[E` inside a `code("rust", …)` string.
- `broken(` / `fixed(` used wherever invalid code is taught.
- No lesson with zero interactive beats.
- Every lesson has a runnable example and a real exercise.
- `cargo test`, the ignored `sweep_every_seeded_fragment`, and a full-program
  compile check all pass.
- After editing content, reseed the module (delete its rows and rerun) so the
  database matches the source.
