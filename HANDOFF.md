# BORROW_ desktop app — handoff

This covers what is left to build. Read **How to work here** first — it
contains four traps that cost real time when this was written, and each of
them looks like a compiler bug rather than a mistake.

---

## Where things are

Everything is on branch **`app`**. `master` still holds the working Axum web
app, untouched and deployable.

```
c660915  Phase 1  gpui-kit spike, CI matrix
62e358c  Phase 1  playground works on Windows, honest about macOS
ccbe466  Phase 2  BORROW_ design tokens applied to gpui-component
656efb0  Phase 3  lesson screens + all 14 block types        (partial)
c9a75b5  Phase 4  exercise screen, XP recording
ba060b0  Phase 4  practice runner (builder / run / results)
a5ecbab  ..       cards keyboard-reachable + headless tests
1ba2c08  ..       practice screens rendered under test
4450d02  ..       hints behind disclosures
9d2708e  ..       corrected a false debt, fixed submit labels
```

**Current state: build clean, clippy clean, 150 tests green** (72 in the
desktop target, 78 in the web target), CI green on Linux, macOS and Windows.

### What already works on desktop

| | |
|---|---|
| Home → module → lesson | all 14 block types render, swept over all 284 lessons |
| Exercise screen | grades, records XP, feedback with explanation |
| Practice runner | builder → run → results, resumable, never touches XP |
| Design system | `borrow.theme.json`, generated from `static/style.css` |
| Database | migrations, seeding, `block_on` for local queries |
| Keyboard | cards focusable, Tab + Enter proven by tests |

### Source layout

`src/bin/app/` is the desktop app; `src/` is shared with the web.

```
src/bin/app.rs        crate root, module declarations, main()
src/bin/app/screen.rs     1694 lines — every screen, navigation, loaders
src/bin/app/components.rs  373 — shared div() builders
src/bin/app/exercise.rs    309 — grading, prompts, option lists
src/bin/app/lesson.rs      238 — LessonSection → elements (exhaustive match)
src/bin/app/theme.rs       240 — BORROW_ theme config + apply()
src/bin/app/db.rs           69 — tokio runtime + pool, as a gpui Global
src/bin/app/borrow.theme.json  135 colour tokens, 31 syntax scopes
```

---

## How to work here

### Commands

```bash
cargo build --bin borrow --bin app   # both targets
cargo test                           # 150 tests
cargo clippy --all-targets           # must be clean
cargo run --bin app                  # launch the desktop app
```

CI is `.github/workflows/app.yml` — builds both binaries and runs tests on
ubuntu, macos and windows. Push to `app` to trigger it. All three must pass.

### The four traps

**1. `use gpui_kit::*` shadows Rust's `#[test]`.**

gpui exports its own `test` attribute macro. A test module that does
`use super::*` inherits it from the parent, so `#[test]` silently becomes
gpui's macro — and the compiler **segfaults inside gpui_macros** before any
test runs. It looks like a compiler bug, not a mistake.

Fix: never use `use super::*` or `use gpui_kit::*` inside a test module.
Import exactly what you need:

```rust
#[cfg(test)]
mod tests {
    use super::{AppView, Screen, /* whatever you need */};
    use gpui_kit::{TestAppContext, TestDispatcher, VisualTestContext};
```

**2. `#[gpui_kit::test]` segfaults against this version of gpui.**

Build the context by hand instead:

```rust
let mut cx = TestAppContext::build(TestDispatcher::new(0), Some("test_name"));
cx.update(|cx| { gpui_kit::init(cx); let _ = crate::theme::apply(cx); });
let (view, cx) = cx.add_window_view(|_, _| AppView::new(crate::content::all()));
cx.update(|window, cx| window.draw(cx).clear(cx));
```

`add_window_view` borrows `cx`, so a helper must take `&mut TestAppContext`
and return `&mut VisualTestContext` — returning a local built inside will not
compile. See `screen.rs::ui_tests::open`.

**3. `Button` owns hover, and setting it twice panics.**

`hover style already set`. Use Button's own hover; a card's background, border
and padding still apply because `render` refines with the instance style last.

**4. `src/bin/app.rs` is a crate root, so `mod X;` resolves against `src/bin/`.**

Shared modules are pulled in with explicit paths:

```rust
#[path = "../models/mod.rs"]   mod models;
#[path = "../content/mod.rs"]  mod content;
#[path = "../services/mod.rs"] mod services;
```

Two consequences: cargo does not autobin-discover them, and **rustfmt follows
`#[path]`**, so formatting an app file also reformats shared content. When
touching app files only:

```bash
rustfmt --edition 2021 --config skip_children=true src/bin/app.rs src/bin/app/*.rs
```

### Conventions

- `content::all()` is compiled in — reading lessons needs no DB and no async.
  The database is for **progress and XP only**.
- Database access goes through `crate::app_db::Db` and `with_db()`. Queries use
  `block_on`; it is deliberately synchronous and the constraint is documented
  on the method. If anything could ever take a lock or touch a network, move it
  to `cx.background_executor()`.
- Cards and any clickable row are **`Button`**, never `div().on_click` — that is
  how keyboard access is kept. Proven by `ui_tests`.
- Lesson block types are matched **exhaustively** in `lesson.rs`. Adding a type
  to `LessonSection` without a renderer is a compile error. Same for `Screen`.
- Exercise screens branch on `options.is_empty()`, **not** on exercise type.
  All 528 exercises carry options; branching on type once made 41 look
  unanswerable. Guarded by `every_seeded_exercise_carries_options`.

### Verifying anything visual

You generally **cannot screenshot the window**: it needs focus, and the machine
is in use. Don't steal focus.

Instead render headlessly. `draw` *is* the assertion — a panic in a screen fails
the test rather than reaching a learner. See `ui_tests`, which opens the app,
navigates, and draws each screen. This is why `gpui-kit/test-support` is a
dev-dependency.

---

## Remaining work

### A. Dashboard + progress screens (Phase 3 remainder)

Phase 3 built home, module and lesson. Two screens are missing.

| Need | Call | Reference |
|---|---|---|
| Dashboard data | `services::progress::dashboard_data(pool)` | `views/dashboard.rs` (166 lines) |
| Progress totals | `services::progress::progress_summary(pool)` | `views/progress.rs` (59 lines) |

What they show: module cards with per-module completion, overall XP, lessons
completed, segmented progress bars. Port the layout from the web views; reuse
`components.rs` for bars, labels and cards.

Add `Screen::Dashboard` and `Screen::Progress`, a nav entry for each, and load
with `with_db(cx, ...)` on click (the pattern already used by practice). Assert
both with a headless render test like `practice_builder_renders_with_seeded_data`.

### B. Phase 5 — editor and console

The compiler already works. `services::sandbox.rs` (1612 lines) is untouched and
verified: `run(code) -> RunResult` with `Status`, `stdout`, `stderr`,
`diagnostics` (code, level, spans, explanation), `line_offset`.

What is missing is everything around it.

- **Editor.** gpui-component's `InputState` in code-editor mode:
  `InputState::new(window, cx).code_editor("rust").line_number(true)`.
  It needs `&mut Window`, which `render` has and the screen methods do not —
  create it there with `window.use_keyed_state(id, cx, |window, cx| ...)`,
  the same pattern Button uses for its focus handle.
- **The RUN button's eight states** — default, hover, focus-visible, active,
  disabled, loading, error, success. CSS rules for all eight already exist in
  `static/style.css` under `.run-btn`; port them, don't reinvent.
- **Console**: stdout, stderr, timing, truncation notice.
- **Diagnostics**: the structured ones — error code, message, jump-to-line
  (use `line_offset`), `help`/`note` children, and the collapsible official
  explanation. The web's `static/editor.js` is the reference implementation.
- **Guard**: on desktop `sandbox::is_enabled()` is not used; execution is local.
  Keep the timeouts. Show a clear message if `rustc` is missing — Phase 4 chose
  "require rustup" rather than bundling a toolchain.

Test it the way phase 4 did: assert on `RunResult` directly (the sandbox already
has 30 tests, most of them against a real `rustc`), then a headless render test for the UI.

**Also blocked by this:** the 41 `fill_gap`/`refactor` exercises are currently
answered by picking an option. If you want typed answers instead, that needs
this input too — but read `every_seeded_exercise_carries_options` first, the
options path already works.

### C. Phase 6 — the five interactive widgets

`static/interactive.js` (458 lines) to port. Each is state plus a `render`
variant in `lesson.rs`, which currently renders their readable content only.

| Widget | Source function | What it needs |
|---|---|---|
| gap-fill | `initGapFill`, `markGap`, `checkAllGaps`, `matches` | per-blank inputs, per-blank feedback |
| step-through | `initStepThrough` | current step, highlight its `lines` |
| dispatch | `initDispatch` | step index, two notes per step |
| pipeline | `initPipeline` | step index, `lazy`/`eager` badge |
| guess-run | `initGuessRun` | chosen option, compare to `correct`, show `explanation` |

`models/content.rs` defines each variant's fields (`GapBlank`, `Step`,
`DispatchStep`, `PipelineStep`). Keep the per-line highlight behaviour — each
step highlights the `lines` it talks about; that is the point of the widget.

State belongs on `AppView`, cleared in `clear_answer()`. Test each by rendering
against seeded content (a sweep like `every_block_in_the_course_renders` will
catch a missing branch).

### D. Phase 7 — packaging

No release artifacts exist yet. CI builds binaries but uploads nothing.

- **Installers:** Linux AppImage + .deb, macOS universal .dmg, Windows .msi
  (`cargo-wix`). Needs a `[[bin]]`/bundle config in `Cargo.toml`.
- **Fonts.** `assets/fonts/` is **empty**. IBM Plex Mono and VT323 are not
  bundled, so the app falls back to system fonts — the same gap the web has.
  `borrow.theme.json` deliberately omits `font.family`, because gpui uses an
  explicitly named family *as-is* and naming an absent one renders no glyphs.
  Bundle the fonts, register them through the asset source, then set
  `font.family` and `mono_font.family` in the theme.
- **Signing.** macOS needs an Apple Developer account ($99/yr) or Gatekeeper
  blocks installers. Windows Authenticode is optional but SmartScreen warns.
- Release build: `cargo build --release --bin app` (the debug binary is 756 MB).

---

## Known gaps

- **Appearance of the card conversion is unverified.** Cards became `Button`s,
  which changed which layer owns hover. The tests cover behaviour, not paint —
  nobody has seen the module list rendered since. Check it first.
- **Hints render but are not styled as disclosures in every state** — check the
  ▸/▾ affordance at 320px.
- **The web app still exists on `master`** and is deployable. Nothing here
  deletes it.
- **Playground memory cap is Linux-only.** `ulimit -v` is not enforced on macOS
  and there is no POSIX shell on Windows. Run timeout works everywhere; the
  memory cap does not. Documented in `services::sandbox.rs` as a platform
  table. Closing it needs `setrlimit` on Unix and job objects on Windows.

---

## Reference

| File | Why |
|---|---|
| `BORROW_UI_DESIGN.md` | locked design spec — palette, type, components |
| `CONTENT_STYLE.md` | how lesson content is written |
| `static/style.css` | the CSS the desktop theme is generated from |
| `static/editor.js` | RUN states, console, diagnostics rendering |
| `static/interactive.js` | the five widgets, to port |
| `src/views/*.rs` | web markup to port as `div()` builders |
| `tools/gen_theme.py` | regenerates `borrow.theme.json` after CSS changes |
| `~/.cargo/registry/src/*/gpui-kit-0.7.1/` | gpui-component source; `TESTING.md` and `src/button/button.rs` answer most API questions |
