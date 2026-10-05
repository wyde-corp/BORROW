//! Module 09 — Option & Result, lessons 1–8.
//!
//! Option. The missing task, the two variants, three ways to read the value
//! out, the panic trap of `unwrap`, real defaults, transforming inside an
//! `Option`, the `?` operator, and turning absence into an error.

use crate::content::types::{
    callout, code, diagram, gap_fill, guess_run, labelled, paragraphs, step_through, table,
    SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        the_missing_task(),
        option_is_a_value(),
        reading_the_value(),
        unwrap_and_expect(),
        defaults(),
        map_and_then(),
        question_mark_on_option(),
        none_into_error(),
    ]
}

fn the_missing_task() -> SeedLesson {
    SeedLesson::new(
        "The Missing Task",
        vec![
            labelled(
                "CONCEPT",
                "The Todo CLI looks tasks up by id. Most of the time the id is there. Sometimes it is \
                 not, and the program has to do *something*. The interesting part is what.",
            ),
            code(
                "rust",
                r#"struct Task {
    id: u32,
    title: String,
}

fn main() {
    let tasks = vec![
        Task { id: 1, title: String::from("buy milk") },
        Task { id: 2, title: String::from("walk the dog") },
    ];

    let found = tasks.iter().find(|t| t.id == 2);
    println!("{:?}", found.map(|t| &t.title));
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`find` returns `Option<&Task>`: `Some(&task)` when the id is present, `None` when it \
                 is not. Here id 2 exists, so it prints `Some(\"walk the dog\")`. Ask for id 7 and it \
                 prints `None`. That `Option` is the whole point: the type says 'there might not be a \
                 task', and it says it before you write a single line of handling.",
            ),
            diagram(
                "Lookup may or may not succeed",
                "find(2) ──→ Some(Task { id: 2, ... })\nfind(7) ──→ None",
            ),
            labelled(
                "EXPLANATION",
                "The alternative you used to reach for is indexing or `unwrap`, which panics on a \
                 missing id. A to-do list is not a program where a missing id is a bug — it is an \
                 ordinary outcome. Panicking would take the whole program down over something the \
                 user can simply be told about.",
            ),
            callout(
                "Absence is a value, not a crash",
                "You have been using `Option` since Module 02 — every `get`, `pop`, and `find` returns \
                 one. This module is where you learn to handle it on purpose instead of reaching past \
                 it with `unwrap`.",
            ),
        ],
    )
}

fn option_is_a_value() -> SeedLesson {
    SeedLesson::new(
        "`Option` Is a Value",
        vec![
            labelled(
                "CONCEPT",
                "`Option<T>` is an ordinary enum with two variants. You met it in Module 07; here is \
                 the definition again, because everything in this module follows from it.",
            ),
            code(
                "rust",
                r#"enum Option<T> {
    None,
    Some(T),
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`Some` carries a value of type `T`; `None` carries nothing. A value of type \
                 `Option<Task>` is *either* one of those, never both and never something else. Because \
                 it is an enum, every tool from Module 07 works on it: `match`, `if let`, `let else`, \
                 and methods in an `impl` block.",
            ),
            guess_run(
                "Predict what this prints.",
                r#"fn main() {
    let id: Option<u32> = None;
    println!("{:?}", id);
}"#,
                &[
                    "None",
                    "0",
                    "It does not compile",
                ],
                0,
                "`None` is a variant, not an absence of a value. It prints as `None`. This matters: an \
                 `Option` always *exists* as a value, which is why the compiler can force you to think \
                 about the `None` case.",
            ),
            table(
                &["Type", "The value is", "You must handle"],
                &[
                    &["`Task`", "always a task", "nothing"],
                    &["`Option<Task>`", "a task, or nothing", "both cases"],
                ],
            ),
            callout(
                "The type is the documentation",
                "A function returning `Option<&Task>` tells you, in its signature, that it can miss. \
                 You cannot accidentally forget, because there is no task to use until you open the \
                 `Option`.",
            ),
        ],
    )
}

fn reading_the_value() -> SeedLesson {
    SeedLesson::new(
        "Getting the Value Out",
        vec![
            labelled(
                "CONCEPT",
                "An `Option` is not a `Task`; to use what is inside, you open it. There are three \
                 shapes, and which you pick depends on how much the missing case matters.",
            ),
            paragraphs(&["`match` handles both cases and gives you the value in the `Some` arm:"]),
            code(
                "rust",
                r#"fn main() {
    let found: Option<&str> = Some("buy milk");

    match found {
        Some(title) => println!("found: {title}"),
        None => println!("no such task"),
    }
}"#,
            ),
            paragraphs(&["`if let` is the compact form when only the present case does work:"]),
            code(
                "rust",
                r#"fn main() {
    let found: Option<&str> = Some("buy milk");

    if let Some(title) = found {
        println!("found: {title}");
    }
}"#,
            ),
            paragraphs(&["`let else` puts the missing case first and leaves, keeping the happy path flat:"]),
            code(
                "rust",
                r#"fn title_or_unknown(found: Option<&str>) -> String {
    let Some(title) = found else {
        return String::from("unknown");
    };

    format!("task: {title}")
}

fn main() {
    println!("{}", title_or_unknown(Some("walk the dog")));
    println!("{}", title_or_unknown(None));
}"#,
            ),
            gap_fill(
                "Open the `Option` with the compact form. Fill in the pattern.",
                r#"if ____(title) = found {
    println!("found: {title}");
}"#,
                &[("p1", "Some", "The present variant.")],
            ),
            callout(
                "Three shapes, one decision",
                "Both cases need work → `match`. One case matters → `if let`. The missing case is an \
                 early exit → `let else`. Choosing well is most of writing readable Rust around \
                 `Option`.",
            ),
        ],
    )
}

fn unwrap_and_expect() -> SeedLesson {
    SeedLesson::new(
        "`unwrap` and `expect`",
        vec![
            labelled(
                "CONCEPT",
                "`unwrap` pulls the value out of a `Some` — and panics if it is `None`. `expect` does \
                 the same but takes a message, so when it panics you at least know why.",
            ),
            guess_run(
                "Predict what this program does.",
                r#"fn main() {
    let found: Option<&str> = None;
    println!("{}", found.unwrap());
}"#,
                &[
                    "Prints an empty line",
                    "Panics at runtime with a message about `unwrap` on `None`",
                    "Does not compile",
                ],
                1,
                "`unwrap` compiles fine and panics when the value is missing. The panic ends the whole \
                 program. This is exactly what the type was warning you about — `unwrap` says 'I \
                 promise this is `Some`', and when it is not, the program stops.",
            ),
            code(
                "rust",
                r#"fn main() {
    let found: Option<&str> = None;

    // `expect` panics too, but the message is yours.
    let title = found.expect("a task should always exist here");
    println!("{title}");
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`expect` is `unwrap` with a better panic message. It is not safer — it still panics — \
                 but the message turns a mystery into a clue. Use it when you are certain the value is \
                 present and want the assumption written down.",
            ),
            table(
                &["Call", "If `Some`", "If `None`"],
                &[
                    &["`opt.unwrap()`", "the value", "panic with a generic message"],
                    &["`opt.expect(\"…\")`", "the value", "panic with your message"],
                ],
            ),
            callout(
                "Fine while prototyping, replace before shipping",
                "`unwrap` is a perfectly reasonable way to keep moving while you are sketching a \
                 program, because it makes the assumption visible and easy to find. It becomes a bug \
                 the moment the assumption is not guaranteed. This module teaches the tools that \
                 replace it; the compiled-in panic stays only where the value truly cannot be missing.",
            ),
        ],
    )
}

fn defaults() -> SeedLesson {
    SeedLesson::new(
        "`unwrap_or` and `unwrap_or_else`",
        vec![
            labelled(
                "CONCEPT",
                "When a missing value has a sensible substitute, `unwrap_or` gives the value if \
                 present and the substitute if not — no panic. `unwrap_or_else` is the same, but \
                 builds the substitute only when it is needed.",
            ),
            gap_fill(
                "Look a task up and fall back to a placeholder title. Fill in the two defaults \
                 methods.",
                r#"let found: Option<&str> = None;

let title = found.____("(no title)");
println!("{title}");

let title2 = found.____(|| "(built only if needed)");
println!("{title2}");"#,
                &[
                    ("u1", "unwrap_or", "A ready-made substitute."),
                    ("u2", "unwrap_or_else", "A substitute built by a function, only if missing."),
                ],
            ),
            code(
                "rust",
                r#"fn main() {
    let found: Option<&str> = None;

    let title = found.unwrap_or("(no title)");
    println!("{title}");

    let title2 = found.unwrap_or_else(|| "(built only if needed)");
    println!("{title2}");
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`unwrap_or(\"(no title)\")` always computes its argument, then uses it if needed. \
                 `unwrap_or_else(|| ...)` runs the function only when the `Option` is `None`. When the \
                 substitute is cheap — a literal — the difference does not matter. When it allocates \
                 or does real work, `unwrap_or_else` avoids paying for a value you may not use.",
            ),
            table(
                &["Method", "Argument", "When the substitute is built"],
                &[
                    &["`unwrap_or(x)`", "a value", "always (then maybe used)"],
                    &["`unwrap_or_else(f)`", "a function", "only if the option is `None`"],
                ],
            ),
            callout(
                "Defaults turn a maybe into a definitely",
                "Once you have a default, the `Option` is gone and you hold a plain value. That is \
                 often exactly right — 'the task's title, or a placeholder' is a real rule, and \
                 saying it in one method is clearer than a `match`.",
            ),
        ],
    )
}

fn map_and_then() -> SeedLesson {
    SeedLesson::new(
        "`map` and `and_then`",
        vec![
            labelled(
                "CONCEPT",
                "`map` transforms the value inside an `Option` without opening it: `Some(x)` becomes \
                 `Some(f(x))`, and `None` stays `None`. `and_then` chains an operation that itself \
                 returns an `Option`.",
            ),
            step_through(
                "Transforming without unwrapping",
                r#"let found: Option<&Task> = find(2);
let title = found.map(|t| t.title.as_str());
let len = title.map(|t| t.len());
println!("{:?}", len);"#,
                &[
                    (&[1][..], "`find` returns `Option<&Task>`."),
                    (&[2][..], "`map` reaches inside: if `Some`, it extracts the title; if `None`, it stays `None`."),
                    (&[3][..], "The second `map` transforms again, still without listing the `None` case."),
                    (&[4][..], "The result is `Option<usize>` — a length, or nothing."),
                ],
            ),
            code(
                "rust",
                r#"struct Task {
    title: String,
}

fn main() {
    let found: Option<Task> = Some(Task { title: String::from("buy milk") });

    let length: Option<usize> = found.map(|t| t.title.len());
    println!("{:?}", length);

    let missing: Option<Task> = None;
    println!("{:?}", missing.map(|t| t.title.len()));
}"#,
            ),
            paragraphs(&["`and_then` is for when the transformation *itself* can miss:"]),
            code(
                "rust",
                r#"fn parse_id(text: &str) -> Option<u32> {
    text.trim().parse().ok()
}

fn main() {
    let first = parse_id("42");
    let doubled = first.and_then(|n| n.checked_mul(2));

    println!("{doubled:?}");
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`map` wraps its result back in `Some`; if the function you pass returns an `Option`, \
                 you would get an `Option<Option<T>>`, which is a nuisance. `and_then` flattens it: \
                 the function returns an `Option`, and you get one level, not two. Use `map` for a \
                 plain transformation, `and_then` when the next step can also fail.",
            ),
            callout(
                "Stay inside the `Option`",
                "The point of `map` and `and_then` is to describe a pipeline — find it, take its \
                 title, measure it — without ever writing 'if present, do this; if not, do nothing'. \
                 The `None` case travels along for free.",
            ),
        ],
    )
}

fn question_mark_on_option() -> SeedLesson {
    SeedLesson::new(
        "`?` on `Option`",
        vec![
            labelled(
                "CONCEPT",
                "Inside a function that returns `Option`, the `?` operator opens an `Option` and, if \
                 it is `None`, returns `None` from the function immediately. It is `match` with the \
                 boring branch written for you.",
            ),
            step_through(
                "Early return on absence",
                r#"fn first_title(tasks: &[Task], id: u32) -> Option<&str> {
    let task = tasks.iter().find(|t| t.id == id)?;
    Some(task.title.as_str())
}"#,
                &[
                    (&[1][..], "The function returns `Option<&str>`."),
                    (&[2][..], "`find` returns `Option<&Task>`; the `?` opens it."),
                    (&[2][..], "If it is `None`, the whole function returns `None` right here."),
                    (&[2][..], "If it is `Some`, `task` is bound and execution continues."),
                ],
            ),
            code(
                "rust",
                r#"struct Task {
    id: u32,
    title: String,
}

fn first_title(tasks: &[Task], id: u32) -> Option<&str> {
    let task = tasks.iter().find(|t| t.id == id)?;
    Some(task.title.as_str())
}

fn main() {
    let tasks = vec![Task { id: 1, title: String::from("buy milk") }];
    println!("{:?}", first_title(&tasks, 1));
    println!("{:?}", first_title(&tasks, 7));
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`?` is only allowed when the function returns the same kind of thing. Here the \
                 function returns `Option`, so `?` on an `Option` is legal — it returns `None` on \
                 absence. It cannot be used in `main` unless `main` also returns an `Option` (or \
                 `Result`), which is the same rule you will meet with `Result`.",
            ),
            guess_run(
                "The function returns a plain `&str`, not an `Option`. Predict what the compiler does.",
                r#"struct Task { id: u32, title: String }

fn title(tasks: &[Task], id: u32) -> &str {
    let task = tasks.iter().find(|t| t.id == id)?;
    task.title.as_str()
}"#,
                &[
                    "It compiles — `?` returns the value",
                    "It does not compile — `?` needs the function to return `Option` (or `Result`)",
                ],
                1,
                "`?` has to return *something* when it bails out, and a plain `&str` has no 'nothing' \
                 to return. The function's return type must match: `Option` for `?` on an `Option`, \
                 `Result` for `?` on a `Result`. The error will say so.",
            ),
        ],
    )
}

fn none_into_error() -> SeedLesson {
    SeedLesson::new(
        "Turning `None` into an Error",
        vec![
            labelled(
                "CONCEPT",
                "`ok_or` converts an `Option` into a `Result`: present becomes `Ok`, missing becomes \
                 `Err` with the error you supply. `ok_or_else` is the lazy version. This is how you \
                 cross from 'might be missing' to 'this is a failure with a reason'.",
            ),
            gap_fill(
                "Return a `NotFound` error when the task is missing. Fill in the converting method.",
                r#"fn find_or_fail(tasks: &[Task], id: u32) -> Result<&Task, TodoError> {
    tasks
        .iter()
        .find(|t| t.id == id)
        .____(TodoError::NotFound(id))
}"#,
                &[("o1", "ok_or", "Turn the `Option` into a `Result` with this error.")],
            ),
            code(
                "rust",
                r#"#[derive(Debug)]
enum TodoError {
    NotFound(u32),
}

struct Task {
    id: u32,
    title: String,
}

fn find_or_fail(tasks: &[Task], id: u32) -> Result<&Task, TodoError> {
    tasks
        .iter()
        .find(|t| t.id == id)
        .ok_or(TodoError::NotFound(id))
}

fn main() {
    let tasks = vec![Task { id: 1, title: String::from("buy milk") }];

    match find_or_fail(&tasks, 7) {
        Ok(task) => println!("{}", task.title),
        Err(err) => println!("error: {err:?}"),
    }
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`find` gives `Option<&Task>`. `ok_or(TodoError::NotFound(id))` turns it into \
                 `Result<&Task, TodoError>`: `Some(task)` becomes `Ok(task)`, `None` becomes \
                 `Err(TodoError::NotFound(7))`. Now the missing case carries a reason, and the caller \
                 can tell the user exactly which id was not found.",
            ),
            table(
                &["Method", "Option becomes", "Error is built"],
                &[
                    &["`opt.ok_or(e)`", "`Result<T, E>`", "always (then maybe used)"],
                    &["`opt.ok_or_else(f)`", "`Result<T, E>`", "only if `None`"],
                ],
            ),
            callout(
                "This is the bridge",
                "`ok_or` is where the two halves of this module meet. Absence (`Option`) is usually the \
                 right answer inside a lookup, and failure (`Result`) is usually the right answer at \
                 the boundary where you report to the user. `ok_or` connects them.",
            ),
        ],
    )
}
