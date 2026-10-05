//! Module 09 — the three module challenges.
//!
//! One refactor away from panics, one robust parser, and one error-type design.
//! The walkthrough is always the last hint.

use crate::content::types::{ExerciseKind, SeedExercise};

const PANICKY: &str = r#"use std::collections::HashMap;

// This program works on happy input and panics on everything else.
// Replace every `unwrap()` and `expect()` with honest handling:
//   - absence -> Option
//   - failure -> Result with a reason
//   - and a custom error enum so callers can match.

fn parse_amount(text: &str) -> u32 {
    text.trim().parse().unwrap()
}

fn find<'a>(items: &'a HashMap<String, u32>, name: &str) -> &'a u32 {
    items.get(name).expect("item must exist")
}

fn remove(items: &mut HashMap<String, u32>, name: &str) -> u32 {
    items.remove(name).unwrap()
}

fn main() {
    let mut items: HashMap<String, u32> = HashMap::new();
    items.insert(String::from("apples"), parse_amount("3"));
    println!("{}", find(&items, "apples"));
    println!("{}", remove(&mut items, "bananas"));
}"#;

const PARSE_FILE: &str = r#"// A todo file looks like this, one task per line:
//
//   1|buy milk|todo
//   2|walk the dog|done
//
// Write `fn parse_file(text: &str) -> Result<Vec<Task>, TodoError>`.
// A line with the wrong number of parts, or a bad id, is an error that
// names the offending line. Decide whether to stop at the first bad line
// or collect them all, and say why.

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

fn parse_file(text: &str) -> Result<Vec<Task>, String> {
    todo!()
}

fn main() {
    let text = "1|buy milk|todo\n2|walk the dog|done\n";
    println!("{:?}", parse_file(text));
}"#;

const DESIGN_BRIEF: &str = r#"// DESIGN AN ERROR TYPE. No code required — the design and its
// justification are the deliverable.
//
// A small command-line tool can fail in these ways:
//
//   1. the config file does not exist
//   2. the config file contains invalid syntax
//   3. a required key is missing
//   4. a value is out of the allowed range
//   5. the output file cannot be written
//
// Design an error enum: which variants, which carry data, and which should
// have a `From` conversion so `?` works. Then say which failure you would
// let the program panic on, if any, and why.
"#;

pub const PANICKY_WALKTHROUGH: &str = r#"Each panic is a promise the code could not keep.

  parse_amount  -> Result<u32, AppError>, error `BadAmount(String)` carrying the text
  find          -> Option<&u32>            (absence), or Result if callers need a reason
  remove        -> Option<u32>             (absence: the key may not be present)

  #[derive(Debug)]
  enum AppError { BadAmount(String), NotFound(String), Io(std::io::Error) }

Then `main` can use `?`:

  fn run() -> Result<(), AppError> {
      let mut items = HashMap::new();
      items.insert(String::from("apples"), parse_amount("3")?);
      if let Some(n) = find(&items, "apples") { println!("{n}"); }
      let _ = remove(&mut items, "bananas"); // returns Option
      Ok(())
  }

The judgement call is `find` and `remove`. Both are lookups, so `Option` is
the honest type — a missing key is not a failure. `parse_amount` is an
operation on external text, so `Result` is right. The exercise is placing
each panic on the correct side of that line."#;

pub const PARSE_WALKTHROUGH: &str = r#"The per-line parser is the one from the module:

  fn parse_line(line: &str) -> Result<Task, TodoError> {
      let parts: Vec<&str> = line.split('|').collect();
      if parts.len() != 3 {
          return Err(TodoError::Parse(line.to_string()));
      }
      let id = parts[0].trim().parse::<u32>()
          .map_err(|_| TodoError::Parse(line.to_string()))?;
      let done = parts[2].trim() == "done";
      Ok(Task { id, title: parts[1].trim().to_string(), done })
  }

For the file:

  fn parse_file(text: &str) -> Result<Vec<Task>, TodoError> {
      text.lines()
          .filter(|l| !l.trim().is_empty())
          .map(parse_line)
          .collect()
  }

That `collect` is the all-or-nothing version: it stops at the first bad line,
because the target is `Result<Vec<Task>, _>`. If you decided to collect *every*
error instead, you would map into `Vec<Result<Task, _>>` and separate the
successes from the failures afterwards. Both are defensible — the deliverable is
saying which you chose and why a config-style tool usually wants the first."#;

pub const DESIGN_WALKTHROUGH: &str = r#"A reasonable design:

  #[derive(Debug)]
  enum ConfigError {
      Missing(String),          // the path that could not be read
      Syntax { line: usize, text: String },
      MissingKey(String),       // the key name
      OutOfRange { key: String, value: String },
      Io(std::io::Error),
  }

  impl From<std::io::Error> for ConfigError { ... }  // so `?` works on IO

Each data-carrying variant exists so the message can be specific: which path,
which line, which key. Variants 1 and 5 both come from IO, so they could merge —
but naming them separately lets a caller distinguish 'I looked for a config and
it was not there' (often fine, use defaults) from 'I tried to write output and
could not' (a real failure). That split is a design decision worth defending.

Which to panic on: none, in a config tool. Every one of these is reachable from
outside the program. A panic is for a broken internal invariant, and none of
these is that. The one candidate would be a hard-coded default that is somehow
invalid, which is a bug, not an error.

If your design used one `String` variant for everything, compare: can a caller
tell 'the file was missing' from 'the key was missing'? If not, the type has not
captured the decision."#;

pub fn challenges() -> Vec<SeedExercise> {
    vec![
        SeedExercise::new(
            None,
            ExerciseKind::Challenge,
            "RECOVER, DO NOT PANIC — The program below works only on happy input and panics on \
             everything else. Replace every `unwrap` and `expect` with honest handling: `Option` for \
             absence, `Result` for failure, and a custom error enum. Justify each change.",
            &[
                "List every place the program can panic",
                "Decide for each whether it is absence or failure",
                "Introduce an error enum with data-carrying variants",
                "Add a `From` conversion where a library error is involved",
                "Rewrite the callers to use `?` and `match`",
                "Say in one sentence why each remaining unwrap, if any, is safe",
            ],
            0,
            PANICKY_WALKTHROUGH,
        )
        .code(PANICKY)
        .xp(170)
        .hints(&[
            "Lookups are absence; parsing and IO are failures.",
            "A `Result` error that carries the bad input makes a specific message possible.",
            "`?` needs the function's return type and a `From` if the error types differ.",
            "The walkthrough shows one faithful refactor.",
        ]),
        SeedExercise::new(
            None,
            ExerciseKind::Challenge,
            "PARSE THE TODO FILE — Parse a small text format into `Vec<Task>`. A malformed line is an \
             error that names the offending input, never a panic. Decide whether to stop at the first \
             bad line or collect them all, and say why.",
            &[
                "Write `parse_line` returning a `Result` with the bad line in the error",
                "Write `parse_file` over the lines",
                "Choose all-or-nothing or collect-every-error, and justify it",
                "Carry enough data in the error to quote the failure",
                "Do not panic anywhere in the parser",
            ],
            0,
            PARSE_WALKTHROUGH,
        )
        .code(PARSE_FILE)
        .xp(170)
        .hints(&[
            "`split('|')` and check the part count first.",
            "`map_err` converts a parse error into your own error type.",
            "`collect::<Result<Vec<_>, _>>()` is the all-or-nothing one-liner.",
            "The walkthrough shows the all-or-nothing version and when to prefer it.",
        ]),
        SeedExercise::new(
            None,
            ExerciseKind::Challenge,
            "DESIGN AN ERROR TYPE — Given five ways a small tool can fail, design the error enum: which \
             variants, which carry data, and which need `From`. Then say which failure, if any, you \
             would let the program panic on. No code required; the design is the deliverable.",
            &[
                "One variant per distinct failure, or merged where they mean the same thing",
                "Carry data wherever the message would otherwise be vague",
                "Add `From` for library errors so `?` works",
                "Name the one candidate for a panic, and why it is a bug rather than an error",
                "Be ready to defend each merge and each split",
            ],
            0,
            DESIGN_WALKTHROUGH,
        )
        .code(DESIGN_BRIEF)
        .xp(150)
        .hints(&[
            "Ask of each variant: does a caller need to tell this apart from the others?",
            "Variants that both come from IO may or may not deserve to stay separate.",
            "A panic is for a broken invariant the code controls, not input from outside.",
            "The walkthrough shows one defensible design and the trade-offs in it.",
        ]),
    ]
}
