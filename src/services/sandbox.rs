//! Compiling and running learner code.
//!
//! This is the only module that executes anything the user typed, and it is
//! deliberately the only one — swapping the backend later (a WASI sandbox, a
//! container) should mean rewriting this file and nothing else.
//!
//! ## What this is not
//!
//! It is not a security boundary. A Rust program compiled and run here has the
//! same privileges as the BORROW_ process. The mitigations here are the ones
//! that are cheap and reliable — a wall clock, an address-space cap, a scratch
//! directory that is deleted afterwards, and no stdin — and they exist to stop
//! a runaway `loop {}`, not a determined attacker. Network isolation needs a
//! container or a namespace, which this does not attempt. `config::playground_allowed`
//! refuses to arm the feature at all if the server is bound to anything but
//! loopback, so the exposure has to be opted into explicitly.
//!
//! ## Why structured diagnostics
//!
//! `rustc --error-format=json` gives us the error code, the message, and the
//! exact line/column spans with their labels — and, in `code.explanation`, the
//! official documentation for that error. For a course on ownership that last
//! field is the whole point: pressing Run on a broken `String` move should
//! surface the same explanation the compiler would print, with a link out to
//! the error index, rather than a wall of text we have to re-teach ourselves.

use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

/// Wall clock for `rustc`. Generous, because a learner's first program that
/// touches collections compiles a lot of std.
pub const COMPILE_TIMEOUT: Duration = Duration::from_secs(20);

/// Wall clock for the compiled program. A tight loop of anything instructive
/// finishes well inside this.
pub const RUN_TIMEOUT: Duration = Duration::from_secs(5);

/// Address space cap for the child, in bytes. 512 MB is far more than any
/// fundamentals exercise needs and well short of taking the machine down.
pub const MEMORY_LIMIT_KB: u64 = 512 * 1024;

/// Peak RSS at which we stop caring what the wall clock says and report a
/// memory problem instead.
///
/// Below the address-space cap on purpose: `ulimit -v` limits reservations,
/// and a program that grows slowly spends a long time reserving before it
/// finally fails. Sampling peak usage catches it while the reason is still
/// visibly memory.
const RSS_MEMORY_CEILING_KB: u64 = MEMORY_LIMIT_KB * 9 / 10;

/// Per-stream output cap. A program printing in a tight loop can emit
/// gigabytes inside the run timeout; we keep the first slice and discard the
/// rest so the child never blocks on a full pipe.
pub const MAX_OUTPUT_BYTES: usize = 256 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    /// Compiled and ran to completion.
    Ok,
    /// `rustc` refused the code.
    CompileError,
    /// Compiled, then panicked or exited non-zero.
    RuntimeError,
    /// Compiled, then hit the run wall clock.
    Timeout,
    /// Compiled, then asked for more memory than the cap allows.
    MemoryLimit,
    /// We could not run it at all — no `rustc`, disk full, etc.
    Unavailable,
}

#[derive(Debug, Clone, Serialize)]
pub struct Span {
    pub line_start: usize,
    pub column_start: usize,
    pub column_end: usize,
    pub is_primary: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Child {
    /// `help`, `note`, …
    pub level: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Diagnostic {
    /// `E0382` for compiler errors; lint names like `unused_variables` for
    /// warnings. Absent for `failure-note` records.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// `error`, `warning`, `note`, `help`, `failure-note`.
    pub level: String,
    pub message: String,
    pub spans: Vec<Span>,
    pub children: Vec<Child>,
    /// The official write-up for this error code, if rustc ships one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}

impl Diagnostic {
    /// `failure-note` and `aborting due to N previous errors` are rustc
    /// bookkeeping, not something a learner needs to read.
    fn is_noise(level: &str, message: &str) -> bool {
        level == "failure-note"
            || message.starts_with("aborting due to")
            || message.starts_with("For more information about this error")
            || message.starts_with("Some errors have detailed explanations")
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RunResult {
    pub status: Status,
    pub stdout: String,
    pub stderr: String,
    pub diagnostics: Vec<Diagnostic>,
    pub exit_code: Option<i32>,
    pub compile_ms: u64,
    pub run_ms: u64,
    /// Set when we truncated a stream, so the console can say so rather than
    /// silently lying about the output.
    pub output_truncated: bool,
    /// Lines the wrapper added above the learner's first line, so the gutter can
    /// number its rows the way the compiler does.
    pub line_offset: usize,
    /// Human-readable reason, for the cases where a status code is not enough.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl RunResult {
    fn unavailable(message: &str) -> Self {
        Self {
            status: Status::Unavailable,
            stdout: String::new(),
            stderr: String::new(),
            diagnostics: Vec::new(),
            exit_code: None,
            compile_ms: 0,
            run_ms: 0,
            output_truncated: false,
            line_offset: 0,
            message: Some(message.to_string()),
        }
    }
}

// ── rustc's JSON diagnostic schema ──────────────────────────────────────
//
// Field names match rustc's output exactly. Everything we do not use is
// ignored rather than modelled, so a rustc upgrade that adds fields will not
// break deserialization.

#[derive(Debug, Deserialize)]
struct RawDiagnostic {
    #[serde(rename = "$message_type", default)]
    message_type: String,
    message: String,
    #[serde(default)]
    code: Option<RawCode>,
    level: String,
    #[serde(default)]
    spans: Vec<RawSpan>,
    #[serde(default)]
    children: Vec<RawChild>,
}

#[derive(Debug, Deserialize)]
struct RawCode {
    code: Option<String>,
    explanation: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawSpan {
    line_start: usize,
    column_start: usize,
    column_end: usize,
    #[serde(default)]
    is_primary: bool,
    #[serde(default)]
    label: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawChild {
    level: String,
    message: String,
}

fn parse_diagnostics(stderr: &str) -> Vec<Diagnostic> {
    stderr
        .lines()
        .filter_map(|line| serde_json::from_str::<RawDiagnostic>(line).ok())
        .filter(|d| d.message_type == "diagnostic")
        .filter(|d| !Diagnostic::is_noise(&d.level, &d.message))
        .map(|d| {
            let (code, explanation) = match d.code {
                Some(c) => (c.code, c.explanation),
                None => (None, None),
            };

            Diagnostic {
                code,
                level: d.level,
                message: d.message,
                spans: d
                    .spans
                    .into_iter()
                    .map(|s| Span {
                        line_start: s.line_start,
                        column_start: s.column_start,
                        column_end: s.column_end,
                        is_primary: s.is_primary,
                        label: s.label,
                    })
                    .collect(),
                children: d
                    .children
                    .into_iter()
                    .map(|c| Child {
                        level: c.level,
                        message: c.message,
                    })
                    .collect(),
                explanation,
            }
        })
        .collect()
}

// ── Process plumbing ────────────────────────────────────────────────────

/// A unique-enough scratch directory name without pulling in `uuid`.
fn scratch_name() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("borrow-pg-{}-{}", std::process::id(), nanos)
}

/// Reads a pipe on a thread, keeping the first `MAX_OUTPUT_BYTES` and
/// discarding the rest.
///
/// Discarding rather than stopping matters: if we stopped reading, a program
/// printing in a loop would block on a full pipe and never exit, turning a
/// flood into a confusing timeout instead of an honest "output truncated".
fn drain<R: Read + Send + 'static>(mut reader: R) -> mpsc::Receiver<(String, bool)> {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        let mut buf = [0u8; 8192];
        let mut kept: Vec<u8> = Vec::new();
        let mut truncated = false;

        loop {
            match reader.read(&mut buf) {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    if kept.len() < MAX_OUTPUT_BYTES {
                        let room = MAX_OUTPUT_BYTES - kept.len();
                        let take = room.min(n);
                        kept.extend_from_slice(&buf[..take]);
                        truncated |= take < n;
                    } else {
                        truncated = true;
                    }
                }
            }
        }

        let _ = tx.send((String::from_utf8_lossy(&kept).into_owned(), truncated));
    });

    rx
}

/// Polls `child` until it exits, the clock runs out, or it approaches the
/// memory cap.
///
/// The memory check matters for honesty. A program that grows its heap slowly
/// takes longer to reach the address-space cap than the run timeout allows, so
/// without this the learner is told "ran too long" when the real cause was
/// memory. Reading peak RSS closes that gap and reports the actual reason.
///
/// Returns the exit code, or `None` if we had to kill it. `killed_for_memory`
/// distinguishes the two kill reasons.
fn wait_with_timeout(
    child: &mut std::process::Child,
    timeout: Duration,
) -> (Option<i32>, bool) {
    let start = Instant::now();

    loop {
        match child.try_wait() {
            Ok(Some(status)) => return (status.code(), false),
            Ok(None) => {
                if peak_rss_kb(child.id()).is_some_and(|kb| kb >= RSS_MEMORY_CEILING_KB) {
                    let _ = child.kill();
                    let _ = child.wait();
                    return (None, true);
                }

                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    return (None, false);
                }

                std::thread::sleep(Duration::from_millis(8));
            }
            Err(_) => {
                let _ = child.kill();
                return (None, false);
            }
        }
    }
}

/// Peak resident set size of a running process, in KB, from procfs.
///
/// Returns `None` off Linux or if the process has already exited. `VmHWM` is
/// the high-water mark, which is what we want: a spike counts even if the
/// process later frees it.
fn peak_rss_kb(pid: u32) -> Option<u64> {
    let status = std::fs::read_to_string(format!("/proc/{pid}/status")).ok()?;

    status.lines().find_map(|line| {
        let rest = line.strip_prefix("VmHWM:")?;
        rest.split_whitespace().next()?.parse().ok()
    })
}

/// rustc swallows the memory cap for us if we set it in the environment of the
/// compile itself; for the *run* we go through `sh` so `ulimit -v` applies and
/// `exec` keeps the pid we can kill.
fn run_with_limits(bin: &Path, dir: &Path) -> std::io::Result<std::process::Child> {
    Command::new("sh")
        .arg("-c")
        .arg(format!(
            "ulimit -v {} 2>/dev/null; exec {}",
            MEMORY_LIMIT_KB,
            bin.display()
        ))
        .current_dir(dir)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
}

// ── Public entry point ──────────────────────────────────────────────────

/// Compiles and runs one program. Always cleans up its scratch directory.
pub fn run(code: &str) -> RunResult {
    let dir = std::env::temp_dir().join(scratch_name());
    let created = std::fs::create_dir_all(&dir).is_ok();

    if !created {
        return RunResult::unavailable("could not create a scratch directory to compile in");
    }

    let prepared = prepare_for_run(code);
    let mut result = run_in(&dir, &prepared.code);

    // Diagnostics point into the program we compiled, which for a fragment is
    // not quite what the learner is looking at. Shift them back so a jump-to-
    // line lands on the line they recognise.
    result.line_offset = prepared.line_offset;
    if prepared.line_offset > 0 {
        shift_diagnostics(&mut result, prepared.line_offset);
    }

    let _ = std::fs::remove_dir_all(&dir);
    result
}

/// Rewrites every span and every `line N` in the raw output to account for the
/// lines the wrapper inserted.
///
/// The structured spans and the human-readable text are adjusted separately
/// because the editor renders each differently, and a jump-to-line link that
/// points at the wrong line is worse than no link at all.
fn shift_diagnostics(result: &mut RunResult, offset: usize) {
    for diagnostic in &mut result.diagnostics {
        for span in &mut diagnostic.spans {
            span.line_start = span.line_start.saturating_sub(offset);
        }
    }

    result.stderr = shift_text_references(&result.stderr, offset);
}

/// Renumbers source references in human-readable compiler output.
///
/// rustc points at a source location two ways and both have to move together.
/// The pointer line carries `--> /path/main.rs:12:5`, and the quoted source
/// underneath carries a matching `12 | ...` gutter. Renumbering only the
/// pointer would leave the quote underneath contradicting it, which reads as a
/// rendering bug rather than a shift.
///
/// Only references past the wrapper are renumbered. Anything inside the wrapper
/// is code the learner did not write, so it is replaced with a marker rather
/// than a line number that would point at the wrong place in their editor.
fn shift_text_references(text: &str, offset: usize) -> String {
    text.lines()
        .map(|line| {
            if let Some((line_number, column)) = rustc_source_pointer(line) {
                return match line_number.checked_sub(offset) {
                    Some(shifted) if shifted > 0 => {
                        replace_pointer_line(line, shifted, column)
                    }
                    _ if line.contains("-->") => {
                        WRAPPER_MARKER.to_string()
                    }
                    _ => line.to_string(),
                };
            }

            if let Some(gutter) = rustc_quoted_gutter(line) {
                return match gutter.checked_sub(offset) {
                    Some(shifted) if shifted > 0 => {
                        format!("{shifted}{}", &line[gutter_text_len(line)..])
                    }
                    _ => line.to_string(),
                };
            }

            line.to_string()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

const WRAPPER_MARKER: &str = "   in the wrapper BORROW_ added to run this fragment";

/// Splits a `-->` pointer line into its line and column numbers.
///
/// `--> /tmp/borrow-pg-1/main.rs:12:5` and `--> main.rs:12:5` are both accepted;
/// the temporary directory name in the real path has no colons, so the last two
/// components are the numbers and everything before them is the path.
fn rustc_source_pointer(line: &str) -> Option<(usize, usize)> {
    let after_arrow = line.split("-->").nth(1)?;
    let rest = after_arrow.trim_start();

    // Take the last two colon-separated components, so an absolute path does
    // not throw off the count.
    let numbers: Vec<&str> = rest.split(':').collect();
    if numbers.len() < 3 {
        return None;
    }

    let line_number = numbers[numbers.len() - 2].trim().parse().ok()?;
    let column = numbers[numbers.len() - 1].trim().parse().ok()?;

    Some((line_number, column))
}

/// Rewrites a `-->` pointer line with a new line number, keeping the column and
/// the rest of the line intact.
fn replace_pointer_line(line: &str, line_number: usize, column: usize) -> String {
    let after_arrow = line.split("-->").nth(1).expect("checked by caller");
    let head = &line[..line.len() - after_arrow.len()];

    // The path is everything up to the last two colons; the numbers after it
    // are what gets replaced.
    let path = &after_arrow[..after_arrow.rfind(':').expect("checked by caller")];
    let path = &path[..path.rfind(':').expect("checked by caller")];

    format!("{head}-->{path}:{line_number}:{column}")
}

/// The line number in a quoted-source gutter, e.g. the `12` in `12 | let x = 1;`.
///
/// The `|` is what makes this unambiguous: a wrapped line like
/// `    let x = 1; |` inside a string literal also contains a colon and digits,
/// and guessing from a number alone would corrupt it.
fn rustc_quoted_gutter(line: &str) -> Option<usize> {
    let bar = line.find('|')?;
    let number = line[..bar].trim();
    if number.is_empty() || !number.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    number.parse().ok()
}

/// Byte length of the gutter's digits, so the rest of the line can be reattached
/// with the column padding intact.
fn gutter_text_len(line: &str) -> usize {
    let bar = line.find('|').expect("checked by caller");
    let digits = line[..bar].trim_end();
    digits.len()
}


fn run_in(dir: &Path, code: &str) -> RunResult {
    let src = dir.join("main.rs");
    let bin = dir.join("program");

    if let Err(e) = std::fs::write(&src, code) {
        return RunResult::unavailable(&format!("could not write the program: {e}"));
    }

    // ── compile ──────────────────────────────────────────────────────
    let compile_start = Instant::now();

    let compile = Command::new("rustc")
        .arg("--edition")
        .arg("2021")
        .arg("--error-format=json")
        .arg("--crate-name")
        .arg("program")
        .arg(&src)
        .arg("-o")
        .arg(&bin)
        .current_dir(dir)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn();

    let mut compile = match compile {
        Ok(c) => c,
        Err(e) => return RunResult::unavailable(&format!("could not start rustc: {e}")),
    };

    let compile_err = match drain(compile.stderr.take().expect("stderr piped")).recv() {
        Ok((text, _)) => text,
        Err(_) => String::new(),
    };

    let compile_ms = compile_start.elapsed().as_millis() as u64;

    match wait_with_timeout(&mut compile, COMPILE_TIMEOUT) {
        (Some(0), _) => {}
        (Some(_), _) => {
            // Non-zero exit. Warnings alone still exit 0, so anything here is
            // either an error or rustc itself failing.
            let diagnostics = parse_diagnostics(&compile_err);
            if diagnostics.iter().any(|d| d.level == "error") {
                return RunResult {
                    status: Status::CompileError,
                    stdout: String::new(),
                    stderr: String::new(),
                    diagnostics,
                    exit_code: None,
                    compile_ms,
                    run_ms: 0,
                    output_truncated: false,
                    line_offset: 0,
                    message: None,
                };
            }
        }
        (_, _) => {
            return RunResult {
                status: Status::Timeout,
                stdout: String::new(),
                stderr: String::new(),
                diagnostics: Vec::new(),
                exit_code: None,
                compile_ms,
                run_ms: 0,
                output_truncated: false,
                line_offset: 0,
                message: Some(format!(
                    "compilation ran past {} seconds and was stopped",
                    COMPILE_TIMEOUT.as_secs()
                )),
            }
        }
    }

    // Diagnostics survive a successful compile, and so do warnings — an unused
    // binding is a thing this course talks about. Filtered on the diagnostic
    // level rather than on the exit code, which is why a program that both
    // warns and runs cleanly still reaches the console.
    let compile_diagnostics: Vec<Diagnostic> = parse_diagnostics(&compile_err)
        .into_iter()
        .filter(|d| d.level == "warning" || d.level == "error")
        .collect();

    // ── run ──────────────────────────────────────────────────────────
    let run_start = Instant::now();

    let mut child = match run_with_limits(&bin, dir) {
        Ok(c) => c,
        Err(e) => return RunResult::unavailable(&format!("could not start the program: {e}")),
    };

    let out_rx = drain(child.stdout.take().expect("stdout piped"));
    let err_rx = drain(child.stderr.take().expect("stderr piped"));

    let (exit, killed_for_memory) = wait_with_timeout(&mut child, RUN_TIMEOUT);
    let run_ms = run_start.elapsed().as_millis() as u64;

    let (stdout, out_trunc) = out_rx.recv().unwrap_or_default();
    let (stderr, err_trunc) = err_rx.recv().unwrap_or_default();

    // An allocation failure under `ulimit -v` aborts the process, which means
    // `status.code()` is `None` — indistinguishable from a kill at the exit-code
    // level. Rust's own abort message is the only reliable tell, so it is
    // checked before the exit code rather than inside one arm of the match.
    let allocator_failed =
        stderr.contains("memory allocation of") && stderr.contains("failed");

    let (status, message) = if exit == Some(0) {
        (Status::Ok, None)
    } else if allocator_failed {
        (
            Status::MemoryLimit,
            Some(format!(
                "this program tried to use more than {} MB of memory, so it was stopped",
                MEMORY_LIMIT_KB / 1024
            )),
        )
    } else if killed_for_memory {
        // Killed by us because peak memory crossed the ceiling while it was
        // still within the wall clock.
        (
            Status::MemoryLimit,
            Some(format!(
                "this program was using more than {} MB of memory, so it was stopped",
                RSS_MEMORY_CEILING_KB / 1024
            )),
        )
    } else {
        match exit {
            Some(_) => (Status::RuntimeError, None),
            // A genuine timeout: it kept running, using modest memory, and ran
            // out of clock. Reporting this as anything else would send the
            // learner hunting in the wrong place.
            None => (
                Status::Timeout,
                Some(format!(
                    "this program ran for {} seconds without finishing, so it was stopped",
                    RUN_TIMEOUT.as_secs()
                )),
            ),
        }
    };

    RunResult {
        status,
        stdout,
        stderr,
        diagnostics: compile_diagnostics,
        exit_code: exit,
        compile_ms,
        run_ms,
        output_truncated: out_trunc || err_trunc,
        line_offset: 0,
        message,
    }
}

/// Whether the playground may be used at all, given where the server listens.
pub fn is_enabled() -> bool {
    crate::config::playground_allowed()
}

/// Makes a lesson's code panel into something `rustc` will accept.
///
/// Most panels are already whole programs, and those are passed through
/// untouched — rewriting them would only move the line numbers a learner's
/// diagnostics point at.
///
/// The exceptions are the 13 panels that show a fragment: a bare `let`, a single
/// `fn`, a lone `match`. Pressing RUN on those is exactly the moment a learner
/// should get to see the error for themselves, so they get a `fn main` rather
/// than a disabled button. Which wrapper applies depends on what the fragment
/// is — a `fn add` at the top level is already a valid item and must *not* be
/// nested inside `main`, or it will not compile for a reason that has nothing
/// to do with the lesson.
pub fn prepare_for_run(code: &str) -> Prepared {
    if code.contains("fn main") {
        return Prepared::unchanged(code.to_string());
    }

    // Split first, and let the *body* decide. Deciding from the opening line
    // alone is what broke lesson 310: it opens with `enum Action {`, which
    // looks like a set of top-level items, but a `match` follows it and needs a
    // `main` to live in. Only a snippet with no statements at all is already a
    // valid crate.
    let (declarations, body) = split_declarations(code);

    if body.trim().is_empty() || is_top_level_items(&body) {
        // `fn main() {}` goes at the end, so every line the learner can see
        // keeps the number it had.
        return Prepared::unchanged(format!("{code}\nfn main() {{}}\n"));
    }

    // The declarations and the `fn main() {` line are inserted above the body,
    // so the body's first line moves down by however many lines went in ahead of
    // it. Reporting that offset is what lets the editor's gutter keep agreeing
    // with the line numbers the compiler prints.
    let inserted = declarations.lines().count() + 1;

    let indented = indent(&body, "    ");

    Prepared {
        code: format!("{declarations}fn main() {{\n{indented}\n}}\n"),
        line_offset: inserted,
    }
}

/// A program ready to hand to `rustc`, plus where the learner's own first line
/// ended up inside it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prepared {
    pub code: String,
    /// Lines inserted before the learner's first line. Zero when the snippet was
    /// already a whole program.
    pub line_offset: usize,
}

impl Prepared {
    fn unchanged(code: String) -> Self {
        Self {
            code,
            line_offset: 0,
        }
    }
}

/// Whether every line sits at the top level, i.e. the snippet is a set of items
/// (`fn`, `struct`, `enum`, `impl`, `use`, attributes) rather than statements.
///
/// Uses indentation of the first non-empty, non-comment line as the signal,
/// because that is the one thing a lesson fragment reliably gets right.
fn is_top_level_items(code: &str) -> bool {
    let first = code
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty() && !l.starts_with("//"))
        .unwrap_or("");

    if first.is_empty() {
        return false;
    }

    // Indented first line means these are statements, not items.
    if code
        .lines()
        .map(str::trim_end)
        .find(|l| !l.trim().is_empty() && !l.trim_start().starts_with("//"))
        .is_some_and(|l| l.starts_with(' '))
    {
        return false;
    }

    const ITEM_STARTS: [&str; 9] = [
        "fn ", "pub fn ", "struct ", "enum ", "impl", "use ", "trait ", "const ", "static ",
    ];

    // An attribute belongs to the item that follows it, so a fragment opening
    // with `#[derive(Debug)]` is a set of items, not statements.
    first.starts_with("#[") || ITEM_STARTS.iter().any(|p| first.starts_with(p))
}

/// Pulls leading `struct`/`enum` declarations out of a statement fragment.
///
/// Lesson 310 shows a `match` over an `Action` enum that is declared in an
/// earlier panel. Wrapping the `match` without the enum produces a wall of
/// "cannot find value" errors that bury the one the lesson is about, so the
/// declarations travel with the body.
fn split_declarations(code: &str) -> (String, String) {
    let mut declarations = String::new();
    let mut body = String::new();

    // Depth of the declaration we are currently inside, if any. Every line at
    // depth > 0 belongs to that declaration, not to the statement body — which
    // is the part the naive version got wrong, sending `Move { from: u32 },`
    // from the enum's arms into `main` and producing a parse error that had
    // nothing to do with the `match` the lesson is about.
    let mut in_declaration = false;
    let mut depth = 0usize;

    for line in code.lines() {
        let trimmed = line.trim_start();

        let opens_declaration = !in_declaration && depth == 0 && is_declaration_start(trimmed);
        if opens_declaration {
            in_declaration = true;
        }

        if in_declaration {
            declarations.push_str(line);
            declarations.push('\n');
        } else {
            body.push_str(line);
            body.push('\n');
        }

        // Brace counting is a heuristic, but these are teaching snippets with
        // balanced braces.
        depth += line.matches('{').count();
        depth = depth.saturating_sub(line.matches('}').count());

        if in_declaration && depth == 0 && trimmed.ends_with('}') {
            in_declaration = false;
        }
    }

    (declarations, body)
}

fn is_declaration_start(trimmed: &str) -> bool {
    trimmed.starts_with("#[")
        || trimmed.starts_with("struct ")
        || trimmed.starts_with("enum ")
        || trimmed.starts_with("pub struct ")
        || trimmed.starts_with("pub enum ")
}

fn indent(text: &str, prefix: &str) -> String {
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| format!("{prefix}{l}\n"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // These run a real rustc against real programs. They are the reason the
    // diagnostic parsing is trustworthy: every assertion below is checked
    // against what the compiler actually emits, not against a fixture.

    #[test]
    fn compiles_and_runs_a_correct_program() {
        let r = run(
            r#"fn main() { println!("hello from the playground"); }"#,
        );

        assert_eq!(r.status, Status::Ok, "stderr was: {}", r.stderr);
        assert!(r.stdout.contains("hello from the playground"));
        assert!(r.run_ms < 3000, "run took {}ms", r.run_ms);
    }

    #[test]
    fn a_use_after_move_yields_a_structured_e0382() {
        let r = run(
            r#"fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s1}");
}"#,
        );

        assert_eq!(r.status, Status::CompileError);

        let err = r
            .diagnostics
            .iter()
            .find(|d| d.level == "error")
            .expect("an error diagnostic");

        assert_eq!(err.code.as_deref(), Some("E0382"));
        assert!(err.message.contains("borrow of moved value"));

        // Two spans, and the primary one is the *use*, not the move.
        assert!(err.spans.len() >= 2, "expected move site and use site");
        assert!(
            err.spans.iter().any(|s| s.is_primary),
            "the primary span carries the caret"
        );

        // The whole point: rustc ships the official write-up.
        let explanation = err
            .explanation
            .as_deref()
            .expect("E0382 must carry an explanation");
        assert!(
            explanation.contains("moved"),
            "explanation should discuss moves: {}",
            &explanation[..explanation.len().min(80)]
        );
    }

    #[test]
    fn rustc_bookkeeping_records_are_filtered_out() {
        let r = run(
            r#"fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s1}");
}"#,
        );

        assert!(
            !r.diagnostics
                .iter()
                .any(|d| d.message.starts_with("aborting due to")),
            "the abort summary is noise"
        );
        assert!(
            !r.diagnostics.iter().any(|d| d.level == "failure-note"),
            "failure-note is noise"
        );
    }

    #[test]
    fn a_type_mismatch_reports_its_code_and_annotations() {
        let r = run(
            r#"fn main() {
    let s: String = "hello";
    println!("{s}");
}"#,
        );

        assert_eq!(r.status, Status::CompileError);
        let err = r.diagnostics.iter().find(|d| d.level == "error").unwrap();
        assert_eq!(err.code.as_deref(), Some("E0308"));
        assert!(err.spans.iter().any(|s| s.is_primary));
    }

    #[test]
    fn warnings_survive_a_successful_compile() {
        let r = run(
            r#"fn main() { let unused_thing = 5; println!("ok"); }"#,
        );

        assert_eq!(r.status, Status::Ok);
        assert!(r.stdout.contains("ok"));
        assert!(
            r.diagnostics.iter().any(|d| d.level == "warning"),
            "an unused binding should still be reported"
        );
    }

    #[test]
    fn a_panic_is_a_runtime_error_not_a_crash() {
        let r = run(
            r#"fn main() { let v: Vec<i32> = vec![]; println!("{}", v[3]); }"#,
        );

        assert_eq!(r.status, Status::RuntimeError);
        assert!(r.stderr.contains("panicked"), "stderr: {}", r.stderr);
    }

    #[test]
    fn a_syntax_error_is_reported_with_a_location() {
        let r = run("fn main( { println!(\"x\") }");

        assert_eq!(r.status, Status::CompileError);
        assert!(!r.diagnostics.is_empty());
        assert!(
            r.diagnostics.iter().any(|d| d.spans.iter().any(|s| s.is_primary)),
            "a syntax error must point at a span"
        );
    }

    #[test]
    fn std_collections_compile() {
        // Guards against silently dropping to a std-less target: if someone
        // changes the rustc invocation, this is the canary.
        let r = run(
            r#"use std::collections::HashMap;
fn main() {
    let mut m: HashMap<&str, i32> = HashMap::new();
    m.insert("one", 1);
    for (k, v) in &m { println!("{k}={v}"); }
}"#,
        );

        assert_eq!(r.status, Status::Ok, "stderr: {}", r.stderr);
        assert!(r.stdout.contains("one=1"));
    }

    /// The wall clock is the mitigation for a runaway loop. This is slow by
    /// design — it has to actually wait for the timeout to fire.
    #[test]
    fn an_infinite_loop_is_stopped_by_the_run_timeout() {
        let started = Instant::now();
        let r = run("fn main() { let mut i = 0u64; loop { i = i.wrapping_add(1); } }");
        let elapsed = started.elapsed();

        assert_eq!(r.status, Status::Timeout);
        assert!(
            r.message.as_deref().is_some_and(|m| m.contains("without finishing")),
            "the learner needs to know it was stopped, not that it 'failed': {:?}",
            r.message
        );

        // It should fire near the configured limit, and definitely not hang.
        assert!(
            elapsed >= RUN_TIMEOUT,
            "returned after {elapsed:?}, before the {RUN_TIMEOUT:?} limit"
        );
        assert!(
            elapsed < RUN_TIMEOUT + Duration::from_secs(10),
            "took {elapsed:?} to notice the timeout"
        );
    }

    /// The address-space cap, hit by one oversized allocation. Deterministic:
    /// the allocator fails on the first try, so this does not race the clock.
    #[test]
    fn an_oversized_allocation_hits_the_memory_cap() {
        let r = run("fn main() { let v: Vec<u8> = vec![0u8; 600 * 1024 * 1024]; println!(\"{}\", v.len()); }");

        assert_eq!(
            r.status,
            Status::MemoryLimit,
            "stderr was: {}",
            &r.stderr[..r.stderr.len().min(200)]
        );
        assert!(r.message.as_deref().is_some_and(|m| m.contains("MB")));
    }

    /// A program that *grows* its heap slowly would otherwise be reported as a
    /// timeout, because reaching the allocator's own failure takes ~10s — well
    /// past the run clock. Asserting only the diagnosis here: which of the two
    /// signals got there first is genuinely a near-tie for this workload, and
    /// `peak_rss_kb` below covers that mechanism directly.
    #[test]
    fn a_slowly_growing_heap_is_reported_as_memory_not_as_a_timeout() {
        let r = run(
            "fn main() { let mut v: Vec<Vec<u8>> = Vec::new(); loop { v.push(vec![0u8; 64 * 1024 * 1024]); } }",
        );

        assert_eq!(
            r.status,
            Status::MemoryLimit,
            "should be diagnosed as memory, not as a slow loop — stderr: {}",
            &r.stderr[..r.stderr.len().min(120)]
        );
        assert!(
            r.message.as_deref().is_some_and(|m| m.contains("memory")),
            "message should name memory, not a timeout: {:?}",
            r.message
        );
    }

    /// If `peak_rss_kb` silently stopped working, memory failures would be
    /// misdiagnosed as timeouts and nothing else would catch it — the
    /// end-to-end tests above can't tell the two signals apart.
    #[test]
    fn peak_rss_is_readable() {
        let peak = peak_rss_kb(std::process::id())
            .expect("this process should have a readable peak RSS");

        // Any live process is using kilobytes, and the ceiling is in the
        // hundreds of megabytes, so this also confirms the units are right —
        // a bytes-vs-KB mixup would put this far above the ceiling.
        assert!(
            peak > 0 && peak < RSS_MEMORY_CEILING_KB,
            "peak RSS of {peak}KB is implausible for a test binary"
        );

        assert_eq!(
            peak_rss_kb(u32::MAX),
            None,
            "a pid that cannot exist has no RSS to report"
        );
    }

    #[test]
    fn output_is_captured_and_truncation_is_reported() {
        let r = run(
            r#"fn main() {
    for i in 0..50 { println!("line {i}"); }
}"#,
        );

        assert_eq!(r.status, Status::Ok);
        assert!(r.stdout.contains("line 0"));
        assert!(r.stdout.contains("line 49"));
        assert!(!r.output_truncated);
    }
}

#[cfg(test)]
mod prepare_tests {
    use super::*;

    /// Complete programs must survive byte-for-byte, or every diagnostic line
    /// number in the lesson shifts and points at the wrong line.
    #[test]
    fn complete_programs_are_untouched() {
        let code = "fn main() {\n    println!(\"hi\");\n}\n";
        let prepared = prepare_for_run(code);

        assert_eq!(prepared.code, code);
        assert_eq!(prepared.line_offset, 0);
    }

    /// A panel showing one function is a valid crate already. Nesting it inside
    /// `main` would fail to compile for reasons unrelated to the lesson.
    #[test]
    fn a_bare_function_gets_an_empty_main_not_a_nested_one() {
        let prepared = prepare_for_run("fn add(a: i32, b: i32) -> i32 {\n    a + b\n}");

        assert!(prepared.code.starts_with("fn add"), "the function must stay top level");
        assert!(
            prepared.code.contains("fn main() {}"),
            "a minimal main should be appended, got: {}",
            prepared.code
        );
        assert_eq!(prepared.line_offset, 0, "main goes at the end, so nothing shifts");
        assert!(run(&prepared.code).status == Status::Ok, "should compile");
    }

    /// A fragment that opens with an attribute is still a set of items. The
    /// wrapper used to treat `#[derive(Debug)]` as a statement, leaving it out
    /// of the declaration and producing a program that does not compile.
    #[test]
    fn a_derived_item_fragment_stays_at_the_top_level() {
        let prepared = prepare_for_run("#[derive(Debug)]\nenum TodoError {\n    NotFound(u32),\n}");

        assert_eq!(prepared.line_offset, 0, "nothing should move");
        assert!(
            run(&prepared.code).status == Status::Ok,
            "a derived enum fragment should compile, got: {}",
            prepared.code
        );
    }

    #[test]
    fn a_bare_statement_is_wrapped_in_main() {
        let prepared = prepare_for_run("let score = 100;\nprintln!(\"{score}\");");

        assert!(prepared.code.contains("fn main() {"), "got: {}", prepared.code);
        assert!(
            prepared.code.contains("    let score = 100;"),
            "body should be indented: {}",
            prepared.code
        );

        // One line of `fn main() {` was inserted above the learner's code.
        assert_eq!(prepared.line_offset, 1);

        let r = run(&prepared.code);
        assert_eq!(r.status, Status::Ok, "stderr: {}", r.stderr);
        assert!(r.stdout.contains("100"));
    }

    /// The borrow-checker panels are the reason fragments need wrapping: the
    /// lesson is a *deliberate* compile error, and it must still produce E0382
    /// after wrapping rather than a nest of unrelated syntax errors.
    #[test]
    fn a_wrapped_ownership_fragment_still_reports_e0382() {
        let prepared = prepare_for_run(
            "let s1 = String::from(\"hello\");\nlet s2 = s1;\nprintln!(\"{s1}\");",
        );

        // The reported line must be where the learner sees it, not where it sits
        // inside the wrapper.
        let r = run("let s1 = String::from(\"hello\");\nlet s2 = s1;\nprintln!(\"{s1}\");");

        assert_eq!(r.line_offset, prepared.line_offset);
        assert_eq!(r.status, Status::CompileError);

        let err = r
            .diagnostics
            .iter()
            .find(|d| d.level == "error")
            .expect("an error");

        assert_eq!(
            err.code.as_deref(),
            Some("E0382"),
            "the lesson's error must survive wrapping — got: {} ({:?})",
            err.message,
            err.code
        );
    }

    /// Lesson 310 matches over an `Action` enum declared in an earlier panel.
    /// Without carrying the declaration along, the `match` buries its own
    /// lesson under a wall of "cannot find value" errors.
    #[test]
    fn a_declaration_needed_by_the_body_travels_with_it() {
        let prepared = prepare_for_run(
            r#"enum Action {
    Move { from: u32 },
    Wait,
}

let action = Action::Move { from: 3 };
match action {
    Action::Move { from } => println!("moving from {from}"),
    _ => println!("not a move"),
}"#,
        );

        // The enum has to be outside main, or it is a local type and the match
        // on it still works but the lesson's shape is wrong.
        let enum_pos = prepared.code.find("enum Action").unwrap();
        let main_pos = prepared.code.find("fn main").unwrap();
        assert!(enum_pos < main_pos, "declaration must precede main: {}", prepared.code);

        let r = run(&prepared.code);
        assert_eq!(r.status, Status::Ok, "stderr: {}", r.stderr);
        assert!(r.stdout.contains("moving from 3"));
    }

    #[test]
    fn comments_do_not_count_as_the_first_line() {
        // Lesson 414's fragment opens with a comment above the `fn`.
        let prepared = prepare_for_run("// Naturally recursive\nfn count(node: &u32) -> u32 {\n    *node\n}");

        assert!(prepared.code.contains("fn main() {}"), "got: {}", prepared.code);
        assert_eq!(prepared.line_offset, 0);
    }

    #[test]
    fn a_use_statement_at_the_top_level_is_left_alone() {
        let prepared = prepare_for_run("use std::collections::HashMap;\nfn f() -> i32 { 1 }");
        assert!(prepared.code.contains("fn main() {}"), "got: {}", prepared.code);
    }
}

/// Sweeps every fragment currently seeded in the database through
/// `prepare_for_run` and compiles the result.
///
/// The unit tests above pin the wrapper's *shape*; this pins the claim that all
/// 13 real lesson fragments compile. A new lesson with a fragment shaped unlike
/// the ones in the tests would slip past unit tests and break RUN in
/// production, which is the failure this exists to prevent. Ignored by default
/// because it needs a populated `borrow.db`; run with
/// `cargo test -- --ignored sweep_every_seeded_fragment`.
#[tokio::test]
#[ignore]
async fn sweep_every_seeded_fragment() {
    use sqlx::Row;

    let Ok(pool) = sqlx::sqlite::SqlitePool::connect(&crate::config::database_url()).await
    else {
        panic!("sweep needs a seeded borrow.db at {}", crate::config::database_url());
    };

    let rows = sqlx::query("SELECT id, content FROM lessons")
        .fetch_all(&pool)
        .await
        .expect("lessons table");

    let mut checked = 0usize;
    let mut failures: Vec<String> = Vec::new();

    for row in rows {
        let lesson_id: i64 = row.get("id");
        let Ok(content) = serde_json::from_str::<serde_json::Value>(row.get::<&str, _>("content"))
        else {
            continue;
        };

        let Some(sections) = content.get("sections").and_then(|s| s.as_array()) else {
            continue;
        };

        for (i, section) in sections.iter().enumerate() {
            // The interactive widgets carry a `code` field too, but it is not a
            // plain panel: a `gap_fill` has `____` markers where the learner
            // types, and a `step_through` may show a fragment whose surrounding
            // declarations live in another panel. They are checked by their own
            // render tests, not by compiling every snippet as a crate.
            let kind = section.get("type").and_then(|t| t.as_str()).unwrap_or("");
            if matches!(kind, "gap_fill" | "step_through" | "guess_run") {
                continue;
            }

            let is_rust = section.get("language").and_then(|l| l.as_str()) == Some("rust");
            let Some(code) = section.get("code").and_then(|c| c.as_str()) else {
                continue;
            };

            if !is_rust || code.contains("fn main") {
                continue;
            }

            // A panel deliberately marked broken is *supposed* to fail to
            // compile — that is the lesson. Asserting the opposite would push
            // the fix in the wrong direction: making the compiler agree with
            // teaching material that exists precisely to disagree with it.
            if section.get("status").and_then(|s| s.as_str()) == Some("broken") {
                continue;
            }

            checked += 1;
            let prepared = prepare_for_run(code);

            let mut child = std::process::Command::new("rustc")
                .args(["--edition", "2021", "--crate-name", "fragment", "-"])
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .expect("rustc");

            {
                use std::io::Write;
                child
                    .stdin
                    .as_mut()
                    .expect("piped")
                    .write_all(prepared.code.as_bytes())
                    .expect("write source");
            }

            let out = child.wait_with_output().expect("rustc finishes");

            if !out.status.success() {
                failures.push(format!(
                    "lesson {lesson_id} section {i}:\n{}\n-- wrapped as --\n{}",
                    String::from_utf8_lossy(&out.stderr),
                    prepared.code
                ));
            }
        }
    }

    // Two of the original thirteen were marked broken, so the runnable set is
    // smaller. Pinned loosely because this is a floor against a lesson losing
    // its fragments entirely, not a count of how many lessons happen to have
    // them today.
    assert!(checked >= 8, "expected the known runnable fragments, found {checked}");
    assert!(
        failures.is_empty(),
        "{} of {checked} lesson fragments do not compile:\n\n{}",
        failures.len(),
        failures.join("\n\n")
    );
}

#[cfg(test)]
mod line_shift_tests {
    use super::*;

    /// The point of the whole shift: a diagnostic's line number is the only
    /// thing connecting a jump-to-line link to a line the learner recognises.
    #[test]
    fn a_span_inside_the_wrapper_is_pulled_back_to_the_learners_numbering() {
        let mut result = RunResult {
            status: Status::CompileError,
            stdout: String::new(),
            stderr: String::new(),
            diagnostics: vec![Diagnostic {
                code: Some("E0382".into()),
                level: "error".into(),
                message: "borrow of moved value".into(),
                spans: vec![Span {
                    line_start: 4,
                    column_start: 12,
                    column_end: 15,
                    is_primary: true,
                    label: Some("value borrowed here after move".into()),
                }],
                children: vec![],
                explanation: None,
            }],
            exit_code: None,
            compile_ms: 0,
            run_ms: 0,
            output_truncated: false,
            line_offset: 0,
            message: None,
        };

        // One line of `fn main() {` sat above the learner's code.
        shift_diagnostics(&mut result, 1);

        assert_eq!(result.diagnostics[0].spans[0].line_start, 3);
    }

    /// Columns are not shifted, and that is correct: the wrapper only inserted
    /// whole lines above, and the learner's own columns are unchanged. This
    /// test exists to stop someone "fixing" columns to match lines.
    #[test]
    fn columns_are_left_alone() {
        let mut result = RunResult {
            status: Status::CompileError,
            stdout: String::new(),
            stderr: String::new(),
            diagnostics: vec![Diagnostic {
                code: None,
                level: "error".into(),
                message: "m".into(),
                spans: vec![Span {
                    line_start: 4,
                    column_start: 12,
                    column_end: 15,
                    is_primary: true,
                    label: None,
                }],
                children: vec![],
                explanation: None,
            }],
            exit_code: None,
            compile_ms: 0,
            run_ms: 0,
            output_truncated: false,
            line_offset: 0,
            message: None,
        };

        shift_diagnostics(&mut result, 1);

        assert_eq!(result.diagnostics[0].spans[0].column_start, 12);
        assert_eq!(result.diagnostics[0].spans[0].column_end, 15);
    }

    #[test]
    fn the_raw_output_is_renumbered_too() {
        // The shape rustc actually emits, including the temporary path.
        let stderr = "error[E0308]: mismatched types\n --> /tmp/borrow-pg-1/main.rs:4:9\n  |\n4 |     let x: i32 = \"hi\";\n  |         ^ expected i32".to_string();

        let shifted = shift_text_references(&stderr, 1);

        assert!(
            shifted.contains("main.rs:3:9"),
            "the pointer should move to the learner's line 3: {shifted}"
        );
        assert!(
            !shifted.contains("main.rs:4:9"),
            "the old number should be gone: {shifted}"
        );
    }

    /// rustc quotes the offending source with the number in a gutter. If only
    /// the `-->` line is renumbered, the quote underneath contradicts it.
    #[test]
    fn quoted_source_lines_are_renumbered_with_their_reference() {
        let stderr = "error[E0308]: mismatched types\n --> /tmp/borrow-pg-1/main.rs:4:9\n  |\n4 |     let x: i32 = \"hi\";\n  |         ^ expected i32".to_string();

        let shifted = shift_text_references(&stderr, 1);

        assert!(
            shifted.contains("\n3 |     let x: i32"),
            "the quoted source gutter should agree with the pointer: {shifted}"
        );
    }

    /// A line that is entirely inside the wrapper has no learner-facing number
    /// at all, so pointing at one would be a lie.
    #[test]
    fn a_reference_inside_the_wrapper_becomes_a_marker() {
        let stderr = "error: something\n --> main.rs:1:1".to_string();

        let shifted = shift_text_references(&stderr, 3);

        assert!(!shifted.contains("line 1"), "got: {shifted}");
        assert!(shifted.contains("wrapper"), "got: {shifted}");
    }

    /// The pointer parser has to cope with the temporary path rustc is given,
    /// or every line number silently fails to parse and nothing gets shifted.
    #[test]
    fn a_pointer_is_read_with_and_without_a_full_path() {
        assert_eq!(rustc_source_pointer(" --> main.rs:12:5"), Some((12, 5)));
        assert_eq!(
            rustc_source_pointer("  --> /tmp/borrow-pg-42/main.rs:12:5"),
            Some((12, 5))
        );
        assert_eq!(rustc_source_pointer("no pointer here"), None);
        assert_eq!(rustc_source_pointer(" --> main.rs:oops:5"), None);
        assert_eq!(rustc_source_pointer(" --> main.rs:12"), None);
    }

    /// A `|` inside a string literal in the learner's own code must not be read
    /// as a source gutter and renumbered.
    #[test]
    fn a_pipe_inside_quoted_code_is_not_a_gutter() {
        assert_eq!(rustc_quoted_gutter("  |"), None);
        assert_eq!(rustc_quoted_gutter("no bar here"), None);
        assert_eq!(
            rustc_quoted_gutter(r#"4 |     println!("a | b");"#),
            Some(4)
        );
    }
}

#[cfg(test)]
mod diagnostic_passthrough_tests {
    use super::*;

    /// A panel marked broken is only useful if pressing RUN reproduces the
    /// error the lesson is about. The seeded broken panels each have a specific
    /// code, and losing any of them would leave a lesson that talks about an
    /// error it can no longer show.
    #[test]
    fn the_broken_lesson_codes_still_come_back() {
        // E0308: the `fn add` in lesson 405 returns `()` where `i32` is
        // declared, because the body ends in a semicolon.
        let r = run("fn add(a: i32, b: i32) -> i32 {\n    a + b;\n}\nfn main() {}");
        assert_eq!(r.status, Status::CompileError);
        assert!(
            r.diagnostics.iter().any(|d| d.code.as_deref() == Some("E0308")),
            "expected E0308, got {:?}",
            r.diagnostics.iter().filter_map(|d| d.code.clone()).collect::<Vec<_>>()
        );

        // E0061: lesson 415's `fn add() -> i32` returns a string literal.
        let r = run("fn add() -> i32 {\n    \"hello\"\n}\nfn main() {}");
        assert_eq!(r.status, Status::CompileError);
        assert!(
            r.diagnostics
                .iter()
                .any(|d| d.code.as_deref() == Some("E0308")),
            "expected E0308 for a string where i32 is declared"
        );

        // E0382 is the module's centrepiece.
        let r = run(
            "fn main() {\n    let s1 = String::from(\"a\");\n    let s2 = s1;\n    println!(\"{s1}{s2}\");\n}",
        );
        assert_eq!(r.status, Status::CompileError);
        assert!(r.diagnostics.iter().any(|d| d.code.as_deref() == Some("E0382")));
    }

    /// A panel marked broken compiles in the seed data as valid Rust once
    /// wrapped, which would silently turn a teaching moment into a green run.
    /// These two are the ones the course actually depends on failing.
    #[test]
    fn the_borrow_checker_fragments_still_fail_after_wrapping() {
        for (code, expected) in [
            (
                "let s1 = String::from(\"a\");\nlet s2 = s1;\nprintln!(\"{s1}\");",
                "E0382",
            ),
            (
                "let x = 5;\nx = 6;\nprintln!(\"{x}\");",
                "E0384",
            ),
        ] {
            let r = run(code);
            assert_eq!(
                r.status,
                Status::CompileError,
                "expected a compile error for: {code}"
            );
            assert!(
                r.diagnostics.iter().any(|d| d.code.as_deref() == Some(expected)),
                "expected {expected}, got {:?}",
                r.diagnostics.iter().filter_map(|d| d.code.clone()).collect::<Vec<_>>()
            );
        }
    }
}
