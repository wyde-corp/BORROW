# BORROW_ desktop app — handoff

This covers what is left to build. Read **How to work here** first — it
contains traps that cost real time when this was written, and each of them
looks like a compiler bug rather than a mistake.

---

## Where things are

Everything is on branch **`app`**. `master` still holds the working Axum web
app, untouched and deployable.

```
834c40d  Add HANDOFF.md for the remaining phases
9d2708e  Correct a debt that did not exist, and a label that did
4450d02  Put hints behind disclosures (debt)
1ba2c08  Render the practice screens under test (debt)
35d56ce  Lock the test-support dependency graph
a5ecbab  Make cards keyboard-reachable, and prove it headlessly
ba060b0  Add the practice runner (phase 4, second half)
c9a75b5  Phase 4  exercise screen, XP recording
```

**Current state: build clean, clippy clean, 160 tests green** (82 in the
desktop target, 78 in the web target), CI green on Linux, macOS and Windows.

### What already works on desktop

| | |
|---|---|
| Home → module → lesson | all 14 block types render, swept over all 284 lessons |
| Exercise screen | grades, records XP, feedback with explanation |
| Practice runner | builder → run → results, resumable, never touches XP |
| **Dashboard** | continue card, module list with bars, overall XP (phase 3 complete) |
| **Progress** | totals, stats grid, curriculum (phase 3 complete) |
| **Playground** | Rust editor with line numbers and tree-sitter colouring, RUN in all eight states, console, diagnostics with jump-to-line, ⌘/CTRL+ENTER, RESET, cancel a run in flight |
| **Five widgets** | gap-fill, step-through, dispatch, pipeline, guess-then-run — all interactive, per-line highlighting included |
| Design system | `borrow.theme.json`, generated from `static/style.css` |
| Database | migrations, seeding, `block_on` for local queries |
| Keyboard | cards focusable, Tab + Enter proven by tests |

### Source layout

`src/bin/app/` is the desktop app; `src/` is shared with the web.

```
src/bin/app.rs        crate root, module declarations, main()
src/bin/app/screen.rs     ~2300 lines — every screen, navigation, loaders, ui_tests
src/bin/app/components.rs  ~410 — shared div() builders, progress bar, status glyphs
src/bin/app/dashboard.rs   ~430 — dashboard + progress bodies (phase 3)
src/bin/app/playground.rs  ~400 — editor screen, RUN states, console, diagnostics
src/bin/app/widgets.rs     ~1100 — the five interactive lesson widgets
src/bin/app/exercise.rs    309 — grading, prompts, option lists
src/bin/app/lesson.rs      132 — LessonSection → elements, delegates widgets
src/bin/app/theme.rs       240 — BORROW_ theme config + apply()
src/bin/app/db.rs           69 — tokio runtime + pool, as a gpui Global
src/bin/app/borrow.theme.json  135 colour tokens, 31 syntax scopes
```

---

## How to work here

### Commands

```bash
cargo build --bin borrow --bin borrow-app   # both targets
cargo test                           # 160 tests
cargo clippy --all-targets           # must be clean
cargo run --bin borrow-app            # launch the desktop app
```

CI is `.github/workflows/app.yml` — builds both binaries and runs tests on
ubuntu, macos and windows. Push to `app` to trigger it. All three must pass.

### The traps

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
    use super::matches;
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

**5. Never read the view while it is rendering.**

`cannot read app::screen::AppView while it is already being updated` — a
runtime panic, from `view.read(cx)` inside anything called by `render`. Pass
what a renderer needs through the `WidgetCtx` instead of reaching for the
entity. The same applies to any `Entity::read` inside a closure that runs
under `view.update`.

**6. `--accent` in `style.css` is `theme.primary`, not `theme.accent`.**

`gen_theme.py` maps `--accent` to `primary.background`; gpui's own
`accent.background` is the *elevated surface* colour. Using `theme.accent`
where the site means `--accent` gives you a dark grey bar on a dark grey
track — invisible, not broken. `theme::tests` asserts the mapping.

**7. Gap-fill verdicts are graded on CHECK, not per keystroke.**

gpui's single-line input exposes no change hook reachable from a render. The
site marks a wrong blank only on blur anyway, so the difference a learner
sees is that a correct blank turns green on CHECK rather than while typing.

**8a. A keystroke an inner binding has claimed never reaches a key listener.**

`secondary-enter` (⌘ on macOS, CTRL elsewhere) is gpui-base's own Input
binding. Where it matches, the key is dispatched as an *action* and stops
there — so `div().on_key_down` sees it on Linux (where ⌘ is unbound) and
misses it on macOS. The playground's shortcut therefore listens to
`InputEvent::PressEnter { secondary: true }`, the event the editor emits
after its own binding runs. Same combination, same behaviour everywhere.
The test presses `"secondary-enter"` for the same reason.

**8b. A subscription callback already holds the view open.**

`cx.subscribe(&editor, |view, ...| ...)` hands you `&mut AppView`; calling
`view.read(cx)` or `view.update(cx, ...)` inside it panics with
"cannot read AppView while it is already being updated". Use the `&mut
AppView` you were given — see `begin_run` in `screen.rs`.

**8. Input element state is keyed, and the key must carry the lesson.**

`window.use_keyed_state("gap-{lesson}-{block}-{blank}", ...)` — without the
lesson id, lesson B's blanks pick up lesson A's text. The lesson screen
creates these (it needs `&mut Window`); the renderer only uses them.

### Conventions

- `content::all()` is compiled in — reading lessons needs no DB and no async.
  The database is for **progress and XP only**.
- Database access goes through `crate::app_db::Db` and `with_db()`. Queries use
  `block_on`; it is deliberately synchronous and the constraint is documented
  on the method. Anything that could take a lock or touch a network goes to
  `cx.background_executor()` — `run_program` in `screen.rs` is the model:
  `sandbox::run` blocks for seconds and must never run on the UI thread.
- Cards and any clickable row are **`Button`**, never `div().on_click` — that
  is how keyboard access is kept. Proven by `ui_tests`.
- Lesson block types are matched **exhaustively** in `lesson.rs`. Adding a type
  to `LessonSection` without a renderer is a compile error. Same for `Screen`.
- Interactive widgets keep their state in `AppView::widgets`, indexed by the
  block's position in its lesson, cleared in `clear_answer()`.
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

### Phase 7 — packaging (the only phase left)

No release artifacts exist yet. CI builds binaries but uploads nothing.

- **Installers:** Linux AppImage + .deb, macOS universal .dmg, Windows .msi
  (`cargo-wix`). Needs `[[bin]]`/bundle config in `Cargo.toml`.
- **Fonts.** `assets/fonts/` is **empty**. IBM Plex Mono and VT323 are not
  bundled, so the app falls back to system fonts — the same gap the web has.
  `borrow.theme.json` deliberately omits `font.family`, because gpui uses an
  explicitly named family *as-is* and naming an absent one renders no glyphs.
  Bundle the fonts, register them through gpui-kit's asset source, then set
  `font.family` and `mono_font.family` in the theme — and verify in a headless
  draw that text still shapes (a missing family renders nothing).
- **Signing.** macOS needs an Apple Developer account ($99/yr) or Gatekeeper
  blocks installers. Windows Authenticode is optional but SmartScreen warns.
- **Release build:** `cargo build --release --bin borrow-app` (the debug binary is
  756 MB; tree-sitter adds to this, so check the release size too).
- **Desktop integration:** an icon (there is none yet), `.desktop` file on
  Linux, bundle id / Info.plist on macOS, ProductName on Windows.

Do **not** enable `cargo-wix`/`cargo-bundle` until the fonts land: an
installer that ships an app with no course text is worse than no installer.

---

## Known gaps

- **The new screens have never been seen.** Dashboard, Progress, Playground and
  the five widgets pass headless draws — that proves they render, not that they
  look right. The same is true of the card conversion from phase 4: cards
  became `Button`s, which changed which layer owns hover. Open the app and look
  at the module list first.
- **Gap verdicts do not update while typing** (trap 7) and the verdict rides
  next to the input rather than colouring it — gpui's `Input` owns its border.
- **Hints render but are not styled as disclosures in every state** — check the
  ▸/▾ affordance at 320px.
- **Guess-then-run has no cancellation**; the button is disabled while a
  compile is in flight. The playground's RUN does cancel.
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
| `static/editor.js` | RUN states, console, diagnostics — ported in phase 5 |
| `static/interactive.js` | the five widgets — ported in phase 6 |
| `src/views/*.rs` | web markup, the reference for anything still to port |
| `tools/gen_theme.py` | regenerates `borrow.theme.json` after CSS changes |
| `~/.cargo/registry/src/*/gpui-kit-0.7.1/` | gpui-component source; `TESTING.md` and `src/button/button.rs` answer most API questions |
