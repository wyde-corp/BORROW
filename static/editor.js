// The playground editor: gutter, run, and error rendering.
//
// Kept apart from app.js because the two have different lifetimes. app.js wires
// up page furniture that exists on every page; this owns one dialog, and there
// is exactly one of them (rendered by views::layout), so it can hold module-level
// state without worrying about two editors fighting over a snippet.

const STORAGE_KEY = "borrow.playground.snippet";
const MAX_LINES = 2000;

// ── the eight states of the RUN button ────────────────────────���──────────
//
// Kept as a single function so the whole state machine is readable at once.
// Every state is reachable and the CSS in style.css carries the matching rule;
// a state added here without one there is the thing to check when a button
// looks inert in some situation.

const STATE = {
  IDLE: "idle",
  LOADING: "loading",
  SUCCESS: "success",
  ERROR: "error",
  DISABLED: "disabled",
};

let runButton;
let runLabel;
let statusEl;
let consoleEl;
let resultsEl;
let emptyEl;
let inputEl;
let gutterEl;
let dialog;
let currentRequest = 0;
let activeController = null;

const STARTER_CODE = `fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s2}");
}`;

document.addEventListener("DOMContentLoaded", () => {
  dialog = document.getElementById("playground");
  if (!dialog) return;

  runButton = dialog.querySelector("[data-pg-run]");
  runLabel = dialog.querySelector("[data-pg-run-label]");
  statusEl = dialog.querySelector("[data-pg-status]");
  consoleEl = dialog.querySelector("[data-pg-console]");
  resultsEl = dialog.querySelector("[data-pg-results]");
  emptyEl = dialog.querySelector("[data-pg-empty]");
  inputEl = dialog.querySelector("[data-pg-input]");
  gutterEl = dialog.querySelector("[data-pg-gutter]");

  inputEl.value = loadSnippet();
  syncGutter();
  refreshDisabled();

  runButton.addEventListener("click", run);
  inputEl.addEventListener("input", () => {
    syncGutter();
    saveSnippet(inputEl.value);
    // Editing after a result invalidates that result: leaving stale compiler
    // output on screen next to changed code is how beginners convince
    // themselves the fix worked when it did not.
    if (runButton.dataset.state !== STATE.IDLE) setState(STATE.IDLE);
    refreshDisabled();
  });
  inputEl.addEventListener("keydown", onKeydown);
  inputEl.addEventListener("scroll", () => {
    gutterEl.scrollTop = inputEl.scrollTop;
  });

  dialog.querySelector("[data-pg-close]").addEventListener("click", close);
  dialog.querySelector("[data-pg-reset]").addEventListener("click", () => {
    inputEl.value = STARTER_CODE;
    lineOffset = 0;
    syncGutter();
    saveSnippet(inputEl.value);
    clearResults();
    refreshDisabled();
    inputEl.focus();
  });

  // Clicking the backdrop closes, but only the backdrop — a click that started
  // inside the panel and drifted onto the scrim should not dismiss the editor
  // and lose the learner's place.
  dialog.addEventListener("click", (event) => {
    if (event.target === dialog) close();
  });

  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape" && dialog.open) close();
  });

  document.querySelectorAll("[data-try-this]").forEach((button) => {
    button.addEventListener("click", () => open(button));
  });
});

// ── opening ─────────────────────────────────────────────────────────────

function open(button) {
  const snippet = JSON.parse(button.dataset.tryThis);

  // Reopening resets to the panel that was clicked, so a learner who fixed the
  // last example and moved on is not silently looking at the old code. Their
  // own work is kept in storage under a separate key and is one RESET away.
  inputEl.value = snippet;
  lineOffset = 0;
  syncGutter();
  saveSnippet(snippet);
  clearResults();
  refreshDisabled();
  inputEl.setSelectionRange(0, 0);
  dialog.showModal();
  inputEl.focus();
}

function close() {
  dialog.close();
}

// ── the gutter ──────────────────────────────────────────────────────────
//
// A real line-number gutter next to a real textarea, rather than a contenteditable
// or a highlighter overlaid on one. The textarea keeps native editing,
// selection, IME, undo, and screen-reader behaviour; the gutter is decoration
// that we redraw. Highlighting inside a textarea is not possible without
// breaking one of those, which is the usual reason people reach for a
// contenteditable and lose undo.

function syncGutter() {
  const lineCount = inputEl.value.split("\n").length;
  const numbers = [];
  for (let i = 1; i <= lineCount; i += 1) numbers.push(i);

  gutterEl.textContent = numbers.join("\n");
  gutterEl.scrollTop = inputEl.scrollTop;
}

function onKeydown(event) {
  // Tab indents, Shift+Tab outdents, and never leaves the field. Losing focus
  // mid-edit is the single most annoying thing a code editor can do.
  if (event.key === "Tab") {
    event.preventDefault();
    indentSelection(event.shiftKey);
    return;
  }

  if (event.key === "Enter" && (event.metaKey || event.ctrlKey)) {
    event.preventDefault();
    run();
  }
}

function indentSelection(outdent) {
  const start = inputEl.selectionStart;
  const end = inputEl.selectionEnd;
  const value = inputEl.value;

  const lineStart = value.lastIndexOf("\n", start - 1) + 1;
  const lineEnd = value.indexOf("\n", end) === -1 ? value.length : value.indexOf("\n", end);
  const block = value.slice(lineStart, lineEnd);
  const lines = block.split("\n");

  let firstDelta = 0;
  let totalDelta = 0;

  const updated = lines.map((line, index) => {
    if (outdent) {
      const removed = line.match(/^ {1,4}/);
      if (!removed) return line;
      if (index === 0) firstDelta = -removed[0].length;
      totalDelta -= removed[0].length;
      return line.slice(removed[0].length);
    }
    if (index === 0) firstDelta = 4;
    totalDelta += 4;
    return `    ${line}`;
  });

  inputEl.value = value.slice(0, lineStart) + updated.join("\n") + value.slice(lineEnd);
  inputEl.selectionStart = Math.max(lineStart, start + firstDelta);
  inputEl.selectionEnd = Math.max(lineStart, end + totalDelta);
  syncGutter();
  saveSnippet(inputEl.value);
}

// ── running ─────────────────────────────────────────────────────────────

async function run() {
  if (runButton.disabled) return;

  // Pressing RUN while a compile is in flight means "I changed my mind", not
  // "do nothing": the previous request is abandoned rather than left to land on
  // top of this one.
  if (activeController) activeController.abort();

  const code = inputEl.value;
  const token = ++currentRequest;
  const started = performance.now();

  // A wait that can be cancelled by the learner's next run. rustc is allowed 20
  // seconds, which is long enough to be worth abandoning.
  const controller = new AbortController();
  activeController = controller;

  setState(STATE.LOADING);
  resultsEl.hidden = true;
  emptyEl.hidden = true;

  try {
    const response = await fetch("/playground/run", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ code }),
      signal: controller.signal,
    });

    const payload = await response.json();

    // A newer run started while this one was in flight. Its result is the one
    // the learner is waiting for; dropping this one stops a slow compile from
    // overwriting a newer answer.
    if (token !== currentRequest) return;

    render(payload, performance.now() - started);
    setState(payload.status === "ok" ? STATE.SUCCESS : STATE.ERROR);
  } catch (err) {
    if (token !== currentRequest) return;
    resultsEl.hidden = false;
    resultsEl.innerHTML = `<pre class="console-error">Could not reach the playground.

${escapeHtml(String(err))}</pre>`;
    setState(STATE.ERROR);
  }
}

/// Keeps the disabled state honest without letting it fight the result states:
/// an empty editor cannot be run, and a non-empty one always can.
function refreshDisabled() {
  if (inputEl.value.trim() === "") {
    setState(STATE.DISABLED);
  } else if (runButton.dataset.state === STATE.DISABLED) {
    setState(STATE.IDLE);
  }
}

function setState(state) {
  runButton.dataset.state = state;
  runButton.setAttribute("aria-busy", state === STATE.LOADING ? "true" : "false");

  runLabel.textContent = {
    [STATE.IDLE]: "RUN",
    [STATE.LOADING]: "RUNNING…",
    [STATE.SUCCESS]: "RUN AGAIN",
    [STATE.ERROR]: "RUN AGAIN",
    [STATE.DISABLED]: "RUN",
  }[state];

  // Disabled means there is nothing to run — an emptied editor. It is
  // deliberately not the loading state: a compile in flight is something the
  // learner can cancel by pressing RUN again, so greying the button there would
  // take away the only control they have.
  runButton.disabled = state === STATE.DISABLED;


  statusEl.textContent = {
    [STATE.IDLE]: "",
    [STATE.LOADING]: "compiling",
    [STATE.SUCCESS]: "",
    [STATE.ERROR]: "",
    [STATE.DISABLED]: "",
  }[state];
}

function clearResults() {
  resultsEl.hidden = true;
  resultsEl.innerHTML = "";
  emptyEl.hidden = false;
}

// ── rendering results ───────────────────────────────────────────────────

function render(result, elapsedMs) {
  const parts = [];

  if (result.message) {
    parts.push(`<p class="console-notice">${escapeHtml(result.message)}</p>`);
  }

  if (result.diagnostics && result.diagnostics.length) {
    parts.push(result.diagnostics.map(renderDiagnostic).join(""));
  }

  if (result.stderr && result.stderr.trim()) {
    parts.push(
      `<pre class="console-stderr">${escapeHtml(result.stderr)}</pre>`,
    );
  }

  if (result.stdout && result.stdout.trim()) {
    parts.push(`<pre class="console-stdout">${escapeHtml(result.stdout)}</pre>`);
  }

  if (result.output_truncated) {
    parts.push(
      `<p class="console-notice">Output was too long to show in full — the beginning is above.</p>`,
    );
  }

  if (!parts.length) {
    parts.push(
      `<p class="console-notice">This program ran without printing anything.</p>`,
    );
  }

  const timing =
    result.status === "ok"
      ? `compiled in ${result.compile_ms}ms · ran in ${result.run_ms}ms`
      : `compiled in ${result.compile_ms}ms`;

  resultsEl.innerHTML =
    `<p class="console-timing" data-pg-timing>${escapeHtml(timing)} · ${Math.round(elapsedMs)}ms round trip</p>` +
    parts.join("");

  resultsEl.hidden = false;
  emptyEl.hidden = true;

  adoptLineOffset(result);
  wireJumpLinks();
}

/// One listener per jump link, attached after each render.
///
/// The links are rebuilt from HTML on every result, so listeners attached at
/// render time would be lost — and were, once, when this lived in a function
/// that got cleaned up as dead code.
function wireJumpLinks() {
  resultsEl.querySelectorAll("[data-pg-jump]").forEach((link) => {
    link.addEventListener("click", (event) => {
      event.preventDefault();
      jumpTo(Number(link.dataset.pgJump));
    });
  });
}

function renderDiagnostic(diagnostic) {
  const code = diagnostic.code
    ? `<span class="diag-code">${escapeHtml(diagnostic.code)}</span>`
    : "";
  const level = `<span class="diag-level diag-${escapeHtml(diagnostic.level)}">${escapeHtml(
    diagnostic.level.toUpperCase(),
  )}</span>`;

  const snippets = (diagnostic.spans || [])
    .filter((span) => span.is_primary && span.line_start)
    .map(
      (span) =>
        `<a href="#" class="diag-jump" data-pg-jump="${span.line_start}">jump to line ${span.line_start}</a>`,
    )
    .join(" ");

  const children = (diagnostic.children || [])
    .map(
      (child) =>
        `<p class="diag-child diag-child-${escapeHtml(child.level)}">${escapeHtml(
          child.message,
        )}</p>`,
    )
    .join("");

  const explanation = diagnostic.explanation
    ? `<details class="diag-explanation"><summary>WHY THIS HAPPENS</summary><p>${escapeHtml(
        diagnostic.explanation,
      )}</p></details>`
    : "";

  return `<article class="diagnostic">
      <header class="diag-header">${code}${level}</header>
      <p class="diag-message">${escapeHtml(diagnostic.message)}</p>
      ${snippets ? `<p class="diag-snippets">${snippets}</p>` : ""}
      ${children}
      ${explanation}
    </article>`;
}

function escapeHtml(text) {
  return String(text)
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}

// The compiler may have wrapped a lesson fragment in a `fn main` before
// compiling it, which pushes every line the learner typed further down. The
// server already shifted the diagnostics back to the learner's numbering, so
// all that is left here is to stop the gutter from disagreeing with them.
let lineOffset = 0;

function adoptLineOffset(result) {
  lineOffset = result.line_offset || 0;
  syncGutter();
}

// ── storage ─────────────────────────────────────────────────────────────
//
// localStorage rather than a table: a saved draft is a personal convenience,
// not coursework, so there is nothing to migrate, no row to orphan, and nothing
// to reconcile if the database is reseeded.

function loadSnippet() {
  try {
    return window.localStorage.getItem(STORAGE_KEY) || STARTER_CODE;
  } catch (err) {
    // Private browsing, disabled storage, or a full quota. A starter program is
    // always better than an error dialog.
    return STARTER_CODE;
  }
}

function saveSnippet(code) {
  if (code.split("\n").length > MAX_LINES) return;
  try {
    window.localStorage.setItem(STORAGE_KEY, code);
  } catch (err) {
    // Losing a draft is not worth interrupting anyone over.
  }
}

function jumpTo(line) {
  const lines = inputEl.value.split("\n");
  const index = Math.max(0, Math.min(lines.length - 1, line - 1));
  const offset = lines.slice(0, index).reduce((sum, l) => sum + l.length + 1, 0);
  inputEl.focus();
  inputEl.setSelectionRange(offset, offset + (lines[index] || "").length);

  // Bring the line into view, then highlight it briefly. Without the flash the
  // caret jump is invisible in a long program.
  inputEl.setAttribute("data-pg-flash", "true");
  setTimeout(() => inputEl.removeAttribute("data-pg-flash"), 600);
}
