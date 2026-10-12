# BORROW_ download widget — integration guide

Audience: whoever builds the static landing page. One file, zero dependencies, no build step.

## What this is

`tools/download-widget/download-widget.js` (~250 lines, vanilla JS, IIFE, no exports to manage) turns the latest GitHub release of `Gus-D-Goat/BORROW` into working download links on your page. It:

1. Fetches `https://api.github.com/repos/Gus-D-Goat/BORROW/releases/latest` from the visitor's browser (CORS is open on this endpoint; no auth, no server needed).
2. Maps release assets to platforms by file extension (see contract below).
3. Detects the visitor's OS and marks the matching link as recommended.
4. If the API fails (rate limit: 60 req/hr per IP unauthenticated; GitHub outage; no stable release yet), every link silently falls back to `https://github.com/Gus-D-Goat/BORROW/releases/latest` — the visitor always gets a working path, never a broken button.

It never shows prereleases: GitHub's `/releases/latest` endpoint excludes them by design (tags with a hyphen, e.g. `v0.1.0-rc.1`, are marked prerelease by the BORROW_ release workflow). No beta toggle, deliberately.

## Quick start (auto-render mode)

```html
<div id="borrow-download"></div>
<script src="download-widget.js"></script>
```

That's the full integration. The widget detects the OS, injects into the div:

- one primary button ("Download for macOS" / "for Windows" / "for Linux") pointing at the right installer, plus a small muted unblock note under macOS/Windows buttons,
- an "Other platforms:" row with the remaining installers,
- a muted "Verify download (SHA-256)" link to the release's `SHA256SUMS` file.

It injects a tiny stylesheet once (id `borrow-download-styles`). Every element carries a `bdl-*` class (listed below) so you can override the look from your own CSS — specificity wins if you target the classes directly.

## Fill-your-own-DOM mode (you control the markup)

If the page contains **any** elements with a `data-borrow-dl` attribute, the widget switches behavior: it renders nothing and only wires your elements (sets `href`, toggles `data-recommended`). Use this when the landing page has its own designed buttons.

```html
<a data-borrow-dl="macos" href="#">Download for macOS</a>
<a data-borrow-dl="windows" href="#">Download for Windows</a>
<a data-borrow-dl="linux-appimage" href="#">Download for Linux</a>
<a data-borrow-dl="linux-deb" href="#">Linux (.deb)</a>
<script src="download-widget.js"></script>
```

Valid `data-borrow-dl` values: `macos`, `windows`, `linux-appimage`, `linux-deb`. Nothing else is recognized.

Behavior details:

- Before the fetch resolves, give links a sensible placeholder `href` (e.g. `#` or the releases page) — the widget overwrites it. If the fetch fails, all hrefs become the releases page.
- The element matching the detected OS gets `data-recommended="true"` (Linux users get it on `linux-appimage`, not the deb). Style it however you like: `[data-recommended] { … }`.
- Trust notes and the SHA-256 line are **not** auto-injected in this mode — your markup, your copy. Steal the text from the auto-render section below if you want it.

## OS detection

`navigator.userAgentData.platform` when available, else `navigator.platform` + UA string. Mobile (Android/iOS) is detected and treated as "unknown": no element is marked recommended, all links are shown equally (the app is desktop-only). macOS is universal (Apple Silicon + Intel, one dmg), Windows is x64 (runs on Windows-on-ARM via emulation), Linux recommends the AppImage.

## Asset-name contract

The release workflow (`.github/workflows/release.yml`) attaches installers named by `cargo-packager`, and the widget classifies by extension only:

| Asset name pattern (current)                     | `data-borrow-dl` value |
| ------------------------------------------------ | ---------------------- |
| `borrow-app_<ver>_amd64.dmg` (universal binary)  | `macos`                |
| `borrow-app_<ver>_x64-setup.exe` (NSIS)          | `windows`              |
| `borrow-app_<ver>_x86_64.AppImage`               | `linux-appimage`       |
| `borrow-app_<ver>_amd64.deb`                     | `linux-deb`            |
| `SHA256SUMS`                                      | (verify link)          |

Anything ending `.dmg` → macOS, `.exe` → Windows, `.AppImage` → Linux primary, `.deb` → Linux secondary. Version numbers in filenames are irrelevant to the widget. **Do not rename these extensions in the release workflow** without updating `classify()` in the widget.

## Events and small API

```js
document.addEventListener("borrow:download:ready", (e) => {
  // e.detail.links   — { version, macos, windows, linux-appimage, linux-deb, sha256sums }
  //                    version is null when the fallback engaged
  // e.detail.links.fallback — true when GitHub was unreachable
  // e.detail.detectedOS     — "macos" | "windows" | "linux" | null
});
```

Use this to print the version somewhere else on the page ("BORROW_ v0.1.0 — free forever") or to hide a spinner. Fires once, after links are resolved (also on fallback).

`window.BorrowDownload` exposes `{ init, run, detectOS }`. The script auto-inits on `DOMContentLoaded`; call `BorrowDownload.run()` again only if you inject `data-borrow-dl` elements late (SPA-style — unlikely on a static page).

Config, set **before** the script tag:

```html
<script>
  window.BORROW_DOWNLOAD_CONFIG = {
    repo: "Gus-D-Goat/BORROW",           // default; override for forks/staging
    autoInit: false,                      // default true
  };
</script>
```

## CSS classes (auto-render mode)

| Class        | What it styles                                        |
| ------------ | ----------------------------------------------------- |
| `.bdl`       | root container (added to `#borrow-download`)          |
| `.bdl-btn`   | primary download button (dark pill, inline-block)     |
| `.bdl-note`  | muted small text: unblock notes / fallback message    |
| `.bdl-other` | "Other platforms:" row                                |
| `.bdl-verify`| muted SHA-256 line                                    |

Defaults are intentionally plain (system font inherit, near-black button, ~65% opacity notes) so they won't fight your design system. Dark-mode pages: the button is `#111` on `#fff` text — override `.bdl-btn` if your page inverts.

## Copy you can reuse (trust notes, per decisions in design review)

- macOS: "If macOS says the app is from an unidentified developer: right-click the app → Open."
- Windows: "If Windows SmartScreen appears: More info → Run anyway."

These render automatically under the primary button in auto-render mode. Builds are unsigned for v0.1 (signing deferred); the notes are the mitigation.

## Known caveats (read before launch)

1. **No stable release exists yet.** The only tag is `v0.1.0-rc.1`, which is a prerelease, so `/releases/latest` currently 404s and the widget shows the fallback (links to the releases page, which lists the rc). The moment a stable `v*` tag **without a hyphen** (e.g. `v0.1.0`) is pushed, the same workflow attaches installers and the API path lights up with no widget change.
2. **Repo visibility**: the API calls are unauthenticated. If the GitHub repo is private, every visitor gets the fallback. The repo must be public for direct-asset links.
3. **Rate limit**: 60 unauthenticated API requests/hour per visitor IP. One fetch per page load — fine for any realistic landing-page traffic. If you ever exceed it, the fallback still works; a CDN/edge proxy in front of the API call is the fix, not a widget change.
4. **Signed builds later**: when code signing lands, only the trust-note copy changes (delete or reword `.bdl-note` text). Asset names, API, and widget logic stay the same.
5. **Broken sign-in in shipped binaries** (out of scope here, tracked as follow-up): release CI bakes no Convex/OAuth env vars (`option_env!` at compile time), so current installers launch but can't sign in. Fix belongs in `.github/workflows/release.yml`, not in this widget.

## Files

- `tools/download-widget/download-widget.js` — the widget. Copy it next to your page (or self-host from any CDN as a static file; it has no deps to hoist).
- `tools/download-widget/INTEGRATION.md` — this document.

## Verification done

Tested in Node with a DOM mock: wiring mode (all three OSes: correct hrefs + `data-recommended`), API-404 fallback (all hrefs → releases page, `fallback: true` in the event), auto-render structure (macOS and Linux: primary button, note, other-platforms row, verify link). Not yet opened in a real browser — do that once against a live stable release.
