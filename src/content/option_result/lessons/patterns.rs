//! Module 09 — Option & Result, lessons 23–26.
//!
//! Patterns. Choosing between `match`, `if let`, and `let else`; flattening a
//! fallible chain with `?`; combining several fallible steps; and collecting a
//! list of results.

use crate::content::types::{
    callout, code, gap_fill, guess_run, labelled, step_through, table, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        choosing_the_shape(),
        early_return(),
        several_errors(),
        first_failure(),
    ]
}

fn choosing_the_shape() -> SeedLesson {
    SeedLesson::new(
        "`match`, `if let`, `let else`",
        vec![
            labelled(
                "CONCEPT",
                "The three shapes for opening an `Option` or `Result` are the same three from Module \
                 07. Which you reach for is decided by how much the failing case matters.",
            ),
            table(
                &["Shape", "Use when", "Keeps the happy path"],
                &[
                    &["`match`", "both cases need real work", "one arm"],
                    &["`if let`", "only the success case matters here", "inside the block"],
                    &["`let else`", "the failure case is an early exit", "at the top level"],
                ],
            ),
            gap_fill(
                "The missing case should leave the function, so the success path stays flat. Fill in \
                 the form.",
                r#"fn title(tasks: &[Task], id: u32) -> String {
    let ____(task) = tasks.iter().find(|t| t.id == id) ____ {
        return String::from("(unknown)");
    };

    task.title.clone()
}"#,
                &[
                    ("p1", "Some", "The present variant pattern."),
                    ("e1", "else", "The branch when it does not match."),
                ],
            ),
            code(
                "rust",
                r#"struct Task {
    id: u32,
    title: String,
}

fn title(tasks: &[Task], id: u32) -> String {
    let Some(task) = tasks.iter().find(|t| t.id == id) else {
        return String::from("(unknown)");
    };

    task.title.clone()
}

fn main() {
    let tasks = vec![Task { id: 1, title: String::from("buy milk") }];
    println!("{}", title(&tasks, 1));
    println!("{}", title(&tasks, 7));
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`let Some(task) = ... else { return ... };` binds `task` on success and bails out on \
                 failure, so the rest of the function reads at one indent level. Compare it to the \
                 `match` version, where the real work sits inside an arm. Both are correct; `let else` \
                 is chosen when the missing case is the exception, not the point.",
            ),
            callout(
                "Do not cargo-cult one shape",
                "Beginners often learn `if let` and use it everywhere, including cases where the `None` \
                 branch needed real handling. Pick the shape that matches the decision, and the code \
                 reads the way it thinks.",
            ),
        ],
    )
}

fn early_return() -> SeedLesson {
    SeedLesson::new(
        "Early Return versus Nesting",
        vec![
            labelled(
                "CONCEPT",
                "A chain of fallible steps written with `match` nests one level per step. The same \
                 chain written with `?` stays flat. Flatter code is easier to read, and `?` is the tool \
                 that gets you there.",
            ),
            step_through(
                "Un-nesting with `?`",
                r#"fn add_and_done(tasks: &mut Vec<Task>, title: &str) -> Result<u32, TodoError> {
    let id = next_id(tasks);
    add(tasks, id, title)?;
    mark_done(tasks, id)?;
    Ok(id)
}"#,
                &[
                    (&[1][..], "The function is fallible, so it returns `Result`."),
                    (&[2][..], "An infallible step — no `?` needed."),
                    (&[3][..], "`add(...)?` returns early with the error if adding fails."),
                    (&[4][..], "`mark_done(...)?` does the same for the second fallible step."),
                    (&[5][..], "Only if both steps succeeded do we reach `Ok(id)`."),
                ],
            ),
            code(
                "rust",
                r#"#[derive(Debug)]
enum TodoError {
    EmptyTitle,
    NotFound(u32),
}

struct Task {
    id: u32,
    title: String,
    done: bool,
}

fn add(tasks: &mut Vec<Task>, id: u32, title: &str) -> Result<(), TodoError> {
    if title.is_empty() {
        return Err(TodoError::EmptyTitle);
    }
    tasks.push(Task { id, title: title.to_string(), done: false });
    Ok(())
}

fn mark_done(tasks: &mut Vec<Task>, id: u32) -> Result<(), TodoError> {
    let task = tasks.iter_mut().find(|t| t.id == id).ok_or(TodoError::NotFound(id))?;
    task.done = true;
    Ok(())
}

fn add_and_done(tasks: &mut Vec<Task>, id: u32, title: &str) -> Result<(), TodoError> {
    add(tasks, id, title)?;
    mark_done(tasks, id)?;
    Ok(())
}

fn main() {
    let mut tasks = Vec::new();
    println!("{:?}", add_and_done(&mut tasks, 1, "buy milk"));
    println!("{:?}", add_and_done(&mut tasks, 2, ""));
}"#,
            ),
            labelled(
                "EXPLANATION",
                "Without `?`, each call would be a `match` with an `Err` arm that returns, and the \
                 success path would sink one level deeper per step. With `?`, the fallible steps read \
                 as a list, and all the failure handling happens in one line each. This is why `?` is \
                 everywhere in real Rust.",
            ),
            callout(
                "Flat is not just prettier",
                "Deeply nested code hides the order of operations and makes the failure paths easy to \
                 get wrong. `?` makes the sequence explicit and the exits automatic. When a function \
                 has three fallible steps, the `?` version is almost always the one to read.",
            ),
        ],
    )
}

fn several_errors() -> SeedLesson {
    SeedLesson::new(
        "Several Errors in One Function",
        vec![
            labelled(
                "CONCEPT",
                "A function can fail in more than one way, and `?` handles each of them — as long as \
                 they all convert into the function's one error type. Mixing an IO error and a parse \
                 error in a single function is the normal case, and `From` is what makes it work.",
            ),
            code(
                "rust",
                r#"use std::io;

#[derive(Debug)]
enum TodoError {
    Io(io::Error),
    Parse(String),
}

impl From<io::Error> for TodoError {
    fn from(err: io::Error) -> TodoError {
        TodoError::Io(err)
    }
}

fn load_first_id(path: &str) -> Result<u32, TodoError> {
    let text = std::fs::read_to_string(path)?;          // io::Error -> TodoError
    let first = text.lines().next().unwrap_or("");       // Option -> &str

    first
        .trim()
        .parse::<u32>()
        .map_err(|_| TodoError::Parse(first.to_string())) // manual convert
}

fn main() {
    println!("{:?}", load_first_id("does-not-exist.txt"));
}"#,
            ),
            labelled(
                "EXPLANATION",
                "Two different failures travel out of one function. The IO error converts via `From` at \
                 the first `?`. The parse error does not come with a `From`, so it is converted by \
                 hand with `map_err`. Both come out as `TodoError`, which is what lets one `match` at \
                 the top handle everything.",
            ),
            callout(
                "One error type per function, usually",
                "When a function calls several fallible things, they should all funnel into the same \
                 error type. `From` covers the library errors you do not own; `map_err` covers the \
                 conversions you write yourself. The result is a single return type and a single place \
                 to report.",
            ),
        ],
    )
}

fn first_failure() -> SeedLesson {
    SeedLesson::new(
        "Stopping at the First Failure",
        vec![
            labelled(
                "CONCEPT",
                "When you map over a list of fallible steps, `collect` can gather them into a *single* \
                 `Result`: `Ok` with all the values, or the first `Err`. One line turns a pile of \
                 results into a result of a pile.",
            ),
            guess_run(
                "One line fails to parse. Predict what `parse_all` returns.",
                r#"fn parse_all(lines: &[&str]) -> Result<Vec<u32>, String> {
    lines
        .iter()
        .map(|l| l.parse::<u32>().map_err(|_| format!("bad: {l}")))
        .collect()
}

fn main() {
    println!("{:?}", parse_all(&["1", "2", "x", "4"]));
}"#,
                &[
                    "Ok([1, 2, 4]) — the bad line is skipped",
                    "Err(\"bad: x\") — it stops at the first failure",
                    "Ok([1, 2, 0, 4]) — the bad line becomes 0",
                ],
                1,
                "Collecting `Result`s stops at the first `Err`, so the whole call returns \
                 `Err(\"bad: x\")` and the `4` is never parsed. That short-circuit is usually what you \
                 want: a parse that quietly skips bad input hides problems. Collecting into \
                 `Result<Vec<_>, _>` is the one-liner that makes the whole list succeed or the whole \
                 list fail.",
            ),
            code(
                "rust",
                r#"fn parse_all(lines: &[&str]) -> Result<Vec<u32>, String> {
    lines
        .iter()
        .map(|l| l.parse::<u32>().map_err(|_| format!("bad: {l}")))
        .collect()
}

fn main() {
    println!("{:?}", parse_all(&["1", "2", "3"]));
    println!("{:?}", parse_all(&["1", "x", "3"]));
}"#,
            ),
            labelled(
                "EXPLANATION",
                "The `collect` target is `Result<Vec<u32>, String>`, not `Vec<Result<...>>`. Given that \
                 target, `collect` keeps collecting while the results are `Ok` and returns the first \
                 `Err` immediately. The type of the annotation is doing the work, exactly as it did \
                 when collecting into a `HashSet` in Module 08.",
            ),
            callout(
                "Whole-list success, or the first failure",
                "Collecting into a `Result` is all-or-nothing by design. If you instead want to keep \
                 the successes *and* the failures, collect into a pair of vectors or a `Vec<Result>` \
                 and sort them out afterwards. Both are valid; the exercise is noticing which one the \
                 program needs.",
            ),
        ],
    )
}
