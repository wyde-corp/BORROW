// Interactive lesson widgets: fill-the-gaps, step-through, and guess-then-run.
//
// Loaded on every page like app.js, but each initialiser returns immediately
// unless its root is present, so a lesson with no widgets pays nothing.
// Everything here degrades gracefully: without this script the gap answers sit
// in a <details>, the step notes are a plain list, and the guess explanation is
// a <details> too. JavaScript upgrades the page; it is never the only way to
// read it.

document.addEventListener("DOMContentLoaded", () => {
  document.querySelectorAll("[data-gap-fill]").forEach(initGapFill);
  document.querySelectorAll("[data-step-through]").forEach(initStepThrough);
  document.querySelectorAll("[data-guess-run]").forEach(initGuessRun);
  document.querySelectorAll("[data-dispatch]").forEach(initDispatch);
  document.querySelectorAll("[data-pipeline]").forEach(initPipeline);
});

// ── fill the gaps ───────────────────────────────────────────────────────
//
// Per-blank feedback: a correct blank turns green as soon as it matches, a
// wrong one is only flagged on blur or Enter. Marking "wrong" on every
// keystroke would punish the act of typing.

function initGapFill(root) {
  const inputs = Array.from(root.querySelectorAll("[data-gap]"));
  const status = root.querySelector("[data-gap-status]");
  if (!inputs.length) return;

  inputs.forEach((input) => {
    input.addEventListener("input", () => {
      if (matches(input.value, input.dataset.answer)) {
        markGap(input, "correct");
      } else {
        // Clear a stale verdict while the learner is still typing.
        markGap(input, "");
      }
    });

    input.addEventListener("blur", () => {
      if (input.value.trim() !== "" && !matches(input.value, input.dataset.answer)) {
        markGap(input, "wrong");
      }
    });

    input.addEventListener("keydown", (event) => {
      if (event.key === "Enter") {
        event.preventDefault();
        checkAllGaps(inputs, status, null);
      }
    });
  });

  root.querySelector("[data-gap-check]").addEventListener("click", () => {
    checkAllGaps(inputs, status, null);
  });

  root.querySelector("[data-gap-reset]").addEventListener("click", () => {
    inputs.forEach((input) => {
      input.value = "";
      markGap(input, "");
    });
    if (status) status.textContent = "";
    inputs[0].focus();
  });
}

function markGap(input, state) {
  input.classList.toggle("gap-correct", state === "correct");
  input.classList.toggle("gap-wrong", state === "wrong");
  input.setAttribute("aria-invalid", state === "wrong" ? "true" : "false");
  input.title = state === "wrong" && input.dataset.hint ? input.dataset.hint : "";
}

function checkAllGaps(inputs, status, fallbackFocus) {
  let right = 0;
  let firstWrong = null;

  inputs.forEach((input) => {
    const value = input.value.trim();

    if (value === "") {
      markGap(input, "");
      if (!firstWrong) firstWrong = input;
      return;
    }

    if (matches(value, input.dataset.answer)) {
      markGap(input, "correct");
      right += 1;
    } else {
      markGap(input, "wrong");
      if (!firstWrong) firstWrong = input;
    }
  });

  if (status) {
    status.textContent =
      right === inputs.length
        ? "All correct."
        : `${right} of ${inputs.length} correct.`;
  }

  const target = fallbackFocus || firstWrong;
  if (target) target.focus();
}

// Whitespace in an answer is never meaningful here, so it is normalised away.
// Case is only ignored when the expected answer is entirely lowercase, so `mut`
// accepts `MUT` but `String` does not accept `string` — precisely the
// distinction a beginner needs to keep.
function matches(given, answer) {
  const normalise = (s) => s.trim().replace(/\s+/g, " ");
  const g = normalise(given);
  const a = normalise(answer);

  if (a === a.toLowerCase()) return g.toLowerCase() === a;
  return g === a;
}

// ── step through ────────────────────────────────────────────────────────

function initStepThrough(root) {
  const steps = JSON.parse(root.dataset.steps || "[]");
  if (!steps.length) return;

  const lines = Array.from(root.querySelectorAll(".st-line"));
  const controls = root.querySelector(".st-controls");
  const counter = root.querySelector("[data-st-counter]");
  const note = root.querySelector("[data-st-note]");
  const fallback = root.querySelector(".st-fallback");
  const prev = root.querySelector("[data-st-prev]");
  const next = root.querySelector("[data-st-next]");
  if (!controls || !counter || !note || !prev || !next) return;

  let index = 0;

  function render() {
    const step = steps[index];
    const active = new Set(step.lines);

    lines.forEach((line) => {
      line.classList.toggle("active", active.has(Number(line.dataset.line)));
    });

    counter.textContent = `Step ${index + 1} of ${steps.length}`;
    note.textContent = step.note;
    prev.disabled = index === 0;
    next.disabled = index === steps.length - 1;
  }

  prev.addEventListener("click", () => {
    if (index > 0) {
      index -= 1;
      render();
    }
  });

  next.addEventListener("click", () => {
    if (index < steps.length - 1) {
      index += 1;
      render();
    }
  });

  // Arrow keys work while focus is anywhere in the widget, so a keyboard user
  // does not have to tab back to the buttons between steps.
  root.addEventListener("keydown", (event) => {
    if (event.key === "ArrowLeft" && index > 0) {
      index -= 1;
      render();
    }
    if (event.key === "ArrowRight" && index < steps.length - 1) {
      index += 1;
      render();
    }
  });

  if (fallback) fallback.hidden = true;
  controls.hidden = false;
  note.hidden = false;
  render();
}

// ── dispatch visualiser ─────────────────────────────────────────────────
//
// One code panel, two columns, one step control. Each step advances both the
// static and the dynamic narration together, so the learner sees the two
// dispatch forms diverge at the same points.

function initDispatch(root) {
  const steps = JSON.parse(root.dataset.steps || "[]");
  if (!steps.length) return;

  const lines = Array.from(root.querySelectorAll(".dsp-line"));
  const controls = root.querySelector(".dsp-controls");
  const counter = root.querySelector("[data-dsp-counter]");
  const staticNote = root.querySelector("[data-dsp-static]");
  const dynamicNote = root.querySelector("[data-dsp-dynamic]");
  const fallback = root.querySelector(".dsp-fallback");
  const prev = root.querySelector("[data-dsp-prev]");
  const next = root.querySelector("[data-dsp-next]");
  if (!controls || !counter || !staticNote || !dynamicNote || !prev || !next) return;

  let index = 0;

  function render() {
    const step = steps[index];
    const active = new Set(step.lines);

    lines.forEach((line) => {
      line.classList.toggle("active", active.has(Number(line.dataset.line)));
    });

    counter.textContent = `Step ${index + 1} of ${steps.length}`;
    staticNote.textContent = step.static_note;
    dynamicNote.textContent = step.dynamic_note;
    prev.disabled = index === 0;
    next.disabled = index === steps.length - 1;
  }

  prev.addEventListener("click", () => {
    if (index > 0) {
      index -= 1;
      render();
    }
  });

  next.addEventListener("click", () => {
    if (index < steps.length - 1) {
      index += 1;
      render();
    }
  });

  root.addEventListener("keydown", (event) => {
    if (event.key === "ArrowLeft" && index > 0) {
      index -= 1;
      render();
    }
    if (event.key === "ArrowRight" && index < steps.length - 1) {
      index += 1;
      render();
    }
  });

  if (fallback) fallback.hidden = true;
  controls.hidden = false;
  render();
}

// ── pipeline (lazy/eager) ───────────────────────────────────────────────
//
// One code panel, a phase badge, and a step control. The first steps are the
// lazy phase — the adapters described but idle — and the later steps are the
// eager phase, when a consumer pulls elements through. The badge changes with
// the step, which is the whole point of the widget.

function initPipeline(root) {
  const steps = JSON.parse(root.dataset.steps || "[]");
  if (!steps.length) return;

  const lines = Array.from(root.querySelectorAll(".pipe-line"));
  const controls = root.querySelector(".pipe-controls");
  const phase = root.querySelector("[data-pipe-phase]");
  const counter = root.querySelector("[data-pipe-counter]");
  const note = root.querySelector("[data-pipe-note]");
  const fallback = root.querySelector(".pipe-fallback");
  const prev = root.querySelector("[data-pipe-prev]");
  const next = root.querySelector("[data-pipe-next]");
  if (!controls || !phase || !counter || !note || !prev || !next) return;

  let index = 0;

  function render() {
    const step = steps[index];
    const active = new Set(step.lines);
    const state = String(step.phase || "lazy").toLowerCase();

    lines.forEach((line) => {
      line.classList.toggle("active", active.has(Number(line.dataset.line)));
    });

    phase.textContent = state.toUpperCase();
    phase.classList.toggle("eager", state === "eager");
    phase.classList.toggle("lazy", state !== "eager");

    counter.textContent = `Step ${index + 1} of ${steps.length}`;
    note.textContent = step.note;
    prev.disabled = index === 0;
    next.disabled = index === steps.length - 1;
  }

  prev.addEventListener("click", () => {
    if (index > 0) {
      index -= 1;
      render();
    }
  });

  next.addEventListener("click", () => {
    if (index < steps.length - 1) {
      index += 1;
      render();
    }
  });

  root.addEventListener("keydown", (event) => {
    if (event.key === "ArrowLeft" && index > 0) {
      index -= 1;
      render();
    }
    if (event.key === "ArrowRight" && index < steps.length - 1) {
      index += 1;
      render();
    }
  });

  if (fallback) fallback.hidden = true;
  controls.hidden = false;
  note.hidden = false;
  render();
}

// ── guess, then run ─────────────────────────────────────────────────────
//
// The only widget that talks to the server, and it reuses /playground/run
// exactly as the editor does. The result renderer below is deliberately a
// smaller copy of the editor's: it targets an inline box rather than the
// shared dialog, and it does not need the gutter, jump links, or storage.

function initGuessRun(root) {
  const config = JSON.parse(root.dataset.config || "{}");
  const submit = root.querySelector("[data-guess-submit]");
  const status = root.querySelector("[data-guess-status]");
  const result = root.querySelector("[data-guess-result]");
  const options = Array.from(root.querySelectorAll('input[type="radio"]'));
  if (!submit || !options.length) return;

  options.forEach((option) => {
    option.addEventListener("change", () => {
      submit.disabled = false;
      if (status) status.textContent = "";
    });
  });

  submit.addEventListener("click", async () => {
    const chosen = options.findIndex((option) => option.checked);

    if (chosen === -1) {
      if (status) status.textContent = "Pick a prediction first.";
      return;
    }

    submit.disabled = true;
    submit.textContent = "RUNNING…";
    if (status) status.textContent = "compiling";
    if (result) {
      result.hidden = false;
      result.innerHTML = '<p class="console-notice">Compiling…</p>';
    }

    try {
      const response = await fetch("/playground/run", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ code: config.code }),
      });

      const payload = await response.json();
      const verdict =
        chosen === config.correct
          ? '<p class="guess-verdict correct"><span aria-hidden="true">✓</span> Your prediction was right.</p>'
          : '<p class="guess-verdict wrong"><span aria-hidden="true">✗</span> Not quite — here is what actually happened.</p>';

      if (result) {
        result.innerHTML =
          verdict +
          `<p class="guess-explanation">${escapeHtml(config.explanation)}</p>` +
          renderConsole(payload);
      }
    } catch (err) {
      if (result) {
        result.hidden = false;
        result.innerHTML = `<pre class="console-error">Could not reach the playground.

${escapeHtml(String(err))}</pre>`;
      }
    } finally {
      submit.disabled = false;
      submit.textContent = "GUESS, THEN RUN";
      if (status) status.textContent = "";
    }
  });
}

function renderConsole(result) {
  const parts = [];

  if (result.message) {
    parts.push(`<p class="console-notice">${escapeHtml(result.message)}</p>`);
  }

  if (result.diagnostics && result.diagnostics.length) {
    parts.push(result.diagnostics.map(renderDiagnostic).join(""));
  }

  if (result.stderr && result.stderr.trim()) {
    parts.push(`<pre class="console-stderr">${escapeHtml(result.stderr)}</pre>`);
  }

  if (result.stdout && result.stdout.trim()) {
    parts.push(`<pre class="console-stdout">${escapeHtml(result.stdout)}</pre>`);
  }

  if (!parts.length) {
    parts.push('<p class="console-notice">This program ran without printing anything.</p>');
  }

  return parts.join("");
}

function renderDiagnostic(diagnostic) {
  const code = diagnostic.code
    ? `<span class="diag-code">${escapeHtml(diagnostic.code)}</span>`
    : "";
  const level = `<span class="diag-level diag-${escapeHtml(diagnostic.level)}">${escapeHtml(
    diagnostic.level.toUpperCase(),
  )}</span>`;

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
