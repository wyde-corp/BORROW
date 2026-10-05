//! Module 09 — Option & Result, lessons 17–22.
//!
//! Errors. One error type for the program, the `From` conversion that powers
//! `?`, a human-readable message, errors that carry data, `Box<dyn Error>` as
//! the quick option, and the narrow case for panicking.

use crate::content::types::{
    broken, callout, code, diagram, gap_fill, labelled, step_through, table, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        one_error_type(),
        from_conversion(),
        printing_an_error(),
        errors_with_data(),
        boxed_errors(),
        when_to_panic(),
    ]
}

fn one_error_type() -> SeedLesson {
    SeedLesson::new(
        "One Error Type for the Program",
        vec![
            labelled(
                "CONCEPT",
                "A program has several ways to fail. Instead of a different error type for each \
                 function, define one enum that lists them all, and return it everywhere.",
            ),
            code(
                "rust",
                r#"#[derive(Debug)]
enum TodoError {
    NotFound(u32),
    EmptyTitle,
    Parse(String),
    Io(std::io::Error),
}"#,
            ),
            labelled(
                "EXPLANATION",
                "Each variant is a distinct failure: a task id that was not there, an empty title, a \
                 line that would not parse, and an underlying IO error. Because they live in one \
                 enum, every function in the program can return the same `Result<T, TodoError>`, and \
                 one `match` at the top can report any of them.",
            ),
            diagram(
                "Many failures, one type",
                "NotFound(u32) ┐\nEmptyTitle     ├──→ TodoError\nParse(String)  │\nIo(io::Error)  ┘",
            ),
            callout(
                "Why not a `String` for everything?",
                "A `String` can be printed but not matched. An enum lets a caller distinguish 'not \
                 found' from 'could not read the file' and respond differently — retry one, report \
                 the other. One type, but a precise one.",
            ),
        ],
    )
}

fn from_conversion() -> SeedLesson {
    SeedLesson::new(
        "`From` and the `?` Conversion",
        vec![
            labelled(
                "CONCEPT",
                "When `?` propagates an error whose type does not match the function's error type, it \
                 converts automatically — as long as a `From` conversion exists. Implementing \
                 `From<std::io::Error> for TodoError` is what lets file operations use `?` directly.",
            ),
            step_through(
                "An `io::Error` becomes a `TodoError` at the `?`",
                r#"impl From<std::io::Error> for TodoError {
    fn from(err: std::io::Error) -> TodoError {
        TodoError::Io(err)
    }
}

fn load(path: &str) -> Result<String, TodoError> {
    let text = std::fs::read_to_string(path)?;
    Ok(text)
}"#,
                &[
                    (&[1][..], "`From<io::Error>` teaches Rust how to turn an IO error into a `TodoError`."),
                    (&[2][..], "The conversion wraps the original error in the `Io` variant, keeping it."),
                    (&[7][..], "`read_to_string` returns `Result<_, io::Error>`."),
                    (&[7][..], "The `?` sees the mismatch and calls `TodoError::from(io_error)` for you."),
                ],
            ),
            code(
                "rust",
                r#"use std::io;

#[derive(Debug)]
enum TodoError {
    Io(io::Error),
}

impl From<io::Error> for TodoError {
    fn from(err: io::Error) -> TodoError {
        TodoError::Io(err)
    }
}

fn load(path: &str) -> Result<String, TodoError> {
    let text = std::fs::read_to_string(path)?;
    Ok(text)
}

fn main() {
    match load("does-not-exist.txt") {
        Ok(text) => println!("{text}"),
        Err(err) => println!("could not load: {err:?}"),
    }
}"#,
            ),
            labelled(
                "EXPLANATION",
                "Without the `From` implementation, `?` would complain that an `io::Error` cannot become \
                 a `TodoError`. With it, every `?` on an IO call converts silently. This is why the \
                 standard library's error types all ship `From` implementations: it is what makes `?` \
                 compose across libraries.",
            ),
            callout(
                "`From` is the bridge `?` crosses",
                "`?` does two things: it returns early on failure, and it converts the error via \
                 `From`. That second half is easy to forget, and it is the reason a custom error enum \
                 plus a handful of `From` impls makes an entire program's error handling click into \
                 place.",
            ),
        ],
    )
}

fn printing_an_error() -> SeedLesson {
    SeedLesson::new(
        "Printing an Error",
        vec![
            labelled(
                "CONCEPT",
                "`{:?}` (Debug) prints an error for a developer. `{}` needs the `Display` trait, which \
                 is how an error is shown to a user. Implementing `Display` — and the empty `Error` \
                 impl — makes your error behave like a standard one.",
            ),
            gap_fill(
                "Give the error a human-readable message. Fill in the trait and the macro.",
                r#"use std::fmt;

impl fmt::____ for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TodoError::NotFound(id) => ____!(f, "no task with id {id}"),
            TodoError::EmptyTitle => write!(f, "a task needs a title"),
        }
    }
}"#,
                &[
                    ("d1", "Display", "The trait for user-facing text."),
                    ("w1", "write", "Writes formatted text into `f`."),
                ],
            ),
            code(
                "rust",
                r#"use std::fmt;

#[derive(Debug)]
enum TodoError {
    NotFound(u32),
    EmptyTitle,
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TodoError::NotFound(id) => write!(f, "no task with id {id}"),
            TodoError::EmptyTitle => write!(f, "a task needs a title"),
        }
    }
}

impl std::error::Error for TodoError {}

fn main() {
    let err = TodoError::NotFound(7);
    println!("{err}");    // Display
    println!("{err:?}");  // Debug
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`Display` turns the error into the sentence a user should see. The `impl \
                 std::error::Error for TodoError {}` line looks empty because it is: once a type is \
                 `Debug + Display`, it already satisfies the trait's requirements, so the impl adds \
                 nothing but the promise that it *is* an error. That promise is what lets `?` box it.",
            ),
            table(
                &["Format", "Trait", "For"],
                &[
                    &["`{err:?}`", "`Debug`", "developers — full detail"],
                    &["`{err}`", "`Display`", "users — a readable sentence"],
                ],
            ),
            callout(
                "Traits by example",
                "This is the first trait you have implemented on purpose. You do not need the full \
                 story yet — Module 10 is where traits live. Here, `Display` is just 'how do I write \
                 myself as text', and the empty `Error` impl is 'yes, I am an error'.",
            ),
        ],
    )
}

fn errors_with_data() -> SeedLesson {
    SeedLesson::new(
        "Errors That Carry Data",
        vec![
            labelled(
                "CONCEPT",
                "A good error says what went wrong *and where*. `NotFound(u32)` carries the id; \
                 `Parse(String)` carries the offending line. The data is what turns a vague message \
                 into a specific one.",
            ),
            gap_fill(
                "Store the bad line inside the parse error. Fill in the payload type.",
                r#"enum TodoError {
    Parse(____),
}

fn parse_line(line: &str) -> Result<u32, TodoError> {
    line.trim()
        .parse()
        .map_err(|_| TodoError::Parse(line.to_string()))
}"#,
                &[("s1", "String", "The offending text, kept for the message.")],
            ),
            code(
                "rust",
                r#"#[derive(Debug)]
enum TodoError {
    Parse(String),
}

fn parse_line(line: &str) -> Result<u32, TodoError> {
    line.trim()
        .parse::<u32>()
        .map_err(|_| TodoError::Parse(line.to_string()))
}

fn main() {
    println!("{:?}", parse_line("42"));
    println!("{:?}", parse_line("forty-two"));
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`Parse(String)` keeps the input that failed, so the eventual message can quote it. A \
                 bare `Parse` variant with no data could only say 'a parse error happened'. The extra \
                 field is the difference between an error a user can act on and one they can only \
                 stare at.",
            ),
            callout(
                "Add data when it helps the message",
                "Not every variant needs data — `EmptyTitle` says everything. But when the message \
                 would otherwise be vague, carry the value that failed: the id, the line, the key. The \
                 `match` arm that reads it is one pattern away.",
            ),
        ],
    )
}

fn boxed_errors() -> SeedLesson {
    SeedLesson::new(
        "`Box<dyn Error>`",
        vec![
            labelled(
                "CONCEPT",
                "Sometimes you do not want to define an error enum — a small script, or a program that \
                 just reports whatever went wrong. `Box<dyn Error>` is a box that can hold *any* \
                 error, and `?` converts into it automatically.",
            ),
            code(
                "rust",
                r#"use std::error::Error;

fn load(path: &str) -> Result<String, Box<dyn Error>> {
    let text = std::fs::read_to_string(path)?;
    Ok(text)
}

fn run() -> Result<(), Box<dyn Error>> {
    let text = load("todo.txt")?;
    println!("{text}");
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
    }
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`Box<dyn Error>` means 'some error, we do not care which at compile time'. Any type \
                 that implements `Error` can be boxed into it, and `?` does the boxing for you. The \
                 trade is that you cannot `match` on the specific error later, because at compile time \
                 it is just 'an error'.",
            ),
            table(
                &["Approach", "You can", "You cannot", "Use when"],
                &[
                    &["a custom enum", "match each variant", "hold arbitrary errors easily", "the program cares about failure kinds"],
                    &["`Box<dyn Error>`", "hold any error", "match the kind", "a quick program that just reports"],
                ],
            ),
            callout(
                "Convenient, at the cost of detail",
                "`Box<dyn Error>` is perfect for a small tool and for `fn main() -> Result<(), Box<dyn \
                 Error>>`, where all you do is print the error and exit. As a program grows and \
                 callers need to react to failures, replace it with a custom enum — which is exactly \
                 what the Todo CLI does.",
            ),
        ],
    )
}

fn when_to_panic() -> SeedLesson {
    SeedLesson::new(
        "When to Panic",
        vec![
            labelled(
                "CONCEPT",
                "Not every problem is an error to return. A panic is for a bug: a state the program \
                 promised could never happen, or a broken assumption. Expected failures — a missing \
                 file, a bad input — are returned, not panicked.",
            ),
            table(
                &["Situation", "Return or panic?", "Why"],
                &[
                    &["the user typed an unknown id", "return `Result`", "expected, and the user can fix it"],
                    &["the file does not exist", "return `Result`", "expected, and the caller can retry or report"],
                    &["a line will not parse", "return `Result`", "expected input from outside"],
                    &["an `id` that the code just inserted is missing", "`expect(...)` is fair", "its absence means a logic bug"],
                    &["indexing past a fixed known length", "`expect(...)` is fair", "the invariant is broken"],
                ],
            ),
            labelled(
                "EXPLANATION",
                "The dividing line is *who can do something about it*. If the program's caller or user \
                 can act on it, it is an error and belongs in a `Result`. If it indicates the code \
                 itself is wrong, a panic is honest — it fails loudly at the exact spot the invariant \
                 broke, instead of limping on. `expect` is the panic that leaves a note.",
            ),
            broken(
                r#"fn mark_done(id: u32) -> Result<(), String> {
    // `id` might not exist — that is expected, so it must be returned, not panicked.
    let task = find_task(id).expect("task must exist");
    task.done = true;
    Ok(())
}"#,
            ),
            labelled(
                "EXPLANATION",
                "This is the mistake in the other direction: an expected failure turned into a panic. If \
                 the user passes an id that is not in the list, the program crashes instead of saying \
                 'no such task'. `expect` is reserved for the cases the *program* controls, not the \
                 cases the *user* does.",
            ),
            callout(
                "Fine while prototyping, replace before shipping",
                "`unwrap` and `expect` are legitimate scaffolding. They make assumptions visible, which \
                 is useful while you are still finding the shape of a program. The shipping question is \
                 only ever: can this value be missing or fail in a way someone else can trigger? If \
                 yes, it becomes a `Result`.",
            ),
        ],
    )
}
