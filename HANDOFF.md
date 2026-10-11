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

**Current state: build clean, clippy clean, 170 tests green** (88 in the
desktop target, 82 in the web target), CI green on Linux, macOS and Windows.

### What already works on desktop

| | |
|---|---|
| Home → module → lesson | all 14 block types render, swept over all 284 lessons |
| Exercise screen | grades, records XP, feedback with explanation |
| Practice runner | builder → run → results, resumable, never touches XP |
| **Dashboard** | continue card, module list with bars, overall XP (phase 3 complete) |
| **Progress** | totals, stats grid, curriculum (phase 3 complete) |
| **Playground** | Rust editor with line numbers and tree-sitter colouring, RUN in all eight states, console, diagnostics with jump-to-line, ⌘/CTRL+ENTER, RESET, cancel a run in flight |
| **Auth & payments** | login gate (Google/GitHub via loopback PKCE), opaque sessions, $5/month Bachs paywall on modules 4–11 with a 7-day grace, progress sync — see SETUP.md and its section below |
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
cargo test                           # 170 tests
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

## Auth, payments & sync (desktop)

Sign-in and the $5/month subscription. The backend is a Convex deployment —
HTTP actions only, every secret in its env vars — and the desktop is a thin
client. **SETUP.md is the source of truth for configuration and the e2e
checklist**; this section is only the shape and the traps.

| | |
|---|---|
| Login | hard gate at launch; Google + GitHub via loopback (`127.0.0.1`, ephemeral port, PKCE S256); the code + verifier go to Convex, which holds the client secrets |
| Sessions | opaque token; the server stores only its HMAC (`SESSION_HASH_KEY`); sliding 30-day expiry; the desktop keeps the token in `session.json` (0600) beside the database |
| Paywall | modules 4–11 need premium **and** the sequential gate; a premium-locked dashboard row clicks through to the upgrade screen |
| Entitlement | computed from the webhooked subscription row: `trialing\|active\|past_due`, then a 7-day grace from the end anchor; unknown status grants nothing; no offline lockout (cached state, refreshed at launch and daily) |
| Checkout | hosted Bachs checkout in the browser; the app polls `/entitlement` every 2s for 15 minutes; the webhook is the source of truth |
| Sync | completions + XP only; full-set push (union merge) after each completion and at launch; pull merges the account's rows into SQLite |
| Tests | vitest + convex-test in `convex/` (entitlement/grace, sessions, webhook signature, idempotency, email linking); Rust unit tests for PKCE, the loopback listener, the session file, and the sync merge |

Traps:

- **Never touch the entity inside its own update.** Arming the refresh,
  daily-refresh or initial-sync tasks used to re-read the view from inside
  `view.update` — a lease violation that panicked the process the moment a
  login completed. Task builders take the *token* and a plain handle; only
  the running task ever leases the view (`screen::refresh_task`).
- **reqwest and sqlx need a tokio reactor; gpui's executors are not one.**
  Every network call runs on `Db::runtime()` and is awaited as a `JoinHandle`.
- **`cargo fmt` is not a repo-wide gate** — much of the web code predates it.
  Format only the files you touched (`rustfmt --edition 2021 <files>`).
- Webhooks post to the same `.convex.site` origin; a stale secret is a
  `400 invalid signature`, and Bachs disables endpoints after ~72h of
  continuous failure.

---

## Remaining work

### Phase 7 — packaging (Linux done, macOS/Windows untried)

| | |
|---|---|
| Binary | `borrow-app`, declared in `Cargo.toml` rather than left to inference |
| Database | per-user data directory (`config::app_database_path`), opened by **path**, not URL; `BORROW_DB_URL` still wins |
| Fonts | IBM Plex Mono embedded from `assets/fonts/` (SIL OFL), registered before the theme applies; `font.family` / `mono_font.family` name it |
| Icon | `assets/icon/borrow-512.png`, generated by `tools/gen_icon.py` |
| Release profile | `strip` + thin LTO + one codegen unit — **42 MB** stripped (the debug binary is 756 MB) |
| Installers | `cargo-packager`, configured in `Cargo.toml` under `[package.metadata.packager]`. **deb and AppImage built and inspected on Linux.** |
| Release CI | `.github/workflows/release.yml` — push a `v*` tag, get installers + SHA256SUMS on the release; `workflow_dispatch` for a dry run |
| Smoke test | the Linux CI job launches the app under Xvfb, asserts it survives 8s and wrote `~/.local/share/borrow/borrow.db` |

```bash
cargo packager --release --formats deb,appimage   # Linux
cargo packager --release --formats dmg            # macOS (universal — see tools/before-package.sh)
cargo packager --release --formats nsis           # Windows
```

Notes earned while building it:

- `excluded-libraries` in the appimage config is not optional: linuxdeploy
  strips what it deploys, and its bundled `strip` predates the RELR sections
  modern distros ship (`unknown type [0x13] section '.relr.dyn'`). The six
  X11/xkb libs are copied in and excluded from its processing.
- Deb `depends` is the `ldd` output of the release binary, mapped to Debian
  package names — ten lines, no guesswork. Fonts and SQLite are inside the
  binary.

Verified by the first release run (`v0.1.0-rc.1`, all three package jobs
green, artifacts attached with SHA256SUMS):

- **deb and AppImage** — built and unpacked here: control file, Depends,
  `.desktop` entry, six bundled xkb libs.
- **macOS dmg** — `BORROW_.app/Contents/MacOS/borrow-app` is a *universal*
  binary (`lipo -info`: x86_64 arm64); `Info.plist` carries
  `com.gusdgoat.borrow`, `BORROW_`, `LSMinimumSystemVersion 11.0`, and the
  `.icns` is inside.
- **Windows NSIS** — `borrow-app.exe` (PE32+ x86-64) plus `uninstall.exe`
  in the installer, built through `bash tools/before-package.sh` (packager
  shells the hook with `cmd /C` on Windows, where `./…` is not a command).
- A release build on Windows was a *console* binary — `file` said so — so
  `src/bin/app.rs` now sets `windows_subsystem = "windows"` for release
  builds; the debug build keeps its terminal.

What has never run yet:

- **The installers on a real machine.** Everything above was unpacked and
  inspected, not launched: nobody has installed the dmg or the .exe, and
  the AppImage has never been run where the xkb stack is absent.
- **Signing.** v0.1 ships unsigned: macOS Gatekeeper blocks it
  (`xattr -d com.apple.quarantine`, or right-click → Open) and Windows
  SmartScreen warns. Apple signing + notarization needs a $99/yr Developer
  account; Authenticode is optional. Both hooks exist in the config
  (`macos.signing-identity`, `windows.certificate-thumbprint`).
- **A project LICENSE.** The fonts carry their own (`assets/fonts/OFL.txt`);
  the code has none, so `license-file` is unset and the deb's control says
  `(none)`. Choosing one is a decision, not a chore to skip silently.
- **Running the AppImage on a clean machine.** It extracts and carries its
  six libs; nobody has launched it where the xkb stack is absent.

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
- **Release CI does not bake the Convex/OAuth env vars.** They are read with
  `option_env!` at compile time and `release.yml` passes none, so a tagged
  build ships with sign-in unconfigured. Bake them before the first public
  release.
- **A pending cancellation is invisible in the app.** Cancel-at-period-end
  keeps the status `active` until the period actually ends — the 7-day
  countdown only appears once the subscription has ended, and nothing shows
  "cancels on <date>" before that.
- **The live grace window has only been proven by tests.** The subscription
  ends Nov 10 in the sandbox; until a real (sandbox) end fires, the grace
  state rests on the unit suite and synthetic webhooks.

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
| `tools/gen_icon.py` | regenerates `assets/icon/borrow-512.png` |
| `tools/before-package.sh` | what `cargo packager` builds first — release binary, or a universal macOS pair |
| `SETUP.md` | one-time auth/payments setup, and the manual e2e checklist |
| `convex/` | the backend: auth exchange, sessions, billing, webhooks, progress sync — vitest suite beside it |
| `~/.cargo/registry/src/*/gpui-kit-0.7.1/` | gpui-component source; `TESTING.md` and `src/button/button.rs` answer most API questions |
