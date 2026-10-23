//! Module 09 — Option & Result, lessons 9–16.
//!
//! Result. Failing operations, `Ok`/`Err`, reading the error, `?`
//! propagation, transforming success or failure, chaining, defaults, and
//! crossing between `Option` and `Result`.

use crate::content::types::{
    callout, code, diagram, gap_fill, guess_run, labelled, paragraphs, step_through, table,
    SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        the_failing_operation(),
        ok_and_err(),
        reading_the_error(),
        question_mark_on_result(),
        map_and_map_err(),
        chaining(),
        defaults_on_result(),
        combining(),
    ]
}

fn the_failing_operation() -> SeedLesson {
    SeedLesson::new(
        "The Failing Operation",
        vec![
            labelled(
                "THE IDEA",
                "Finding a task can miss, which is absence. *Marking* a task done can fail, because it \
                 is an operation with consequences: either the task existed and is now done, or it did \
                 not and nothing happened. That is a `Result`.",
            ),
            code(
                "rust",
                r#"#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

fn mark_done(tasks: &mut [Task], id: u32) -> Result<(), String> {
    match tasks.iter_mut().find(|t| t.id == id) {
        Some(task) => {
            task.done = true;
            Ok(())
        }
        None => Err(format!("no task with id {id}")),
    }
}

fn main() {
    let mut tasks = vec![Task { id: 1, title: String::from("buy milk"), done: false }];

    println!("{:?}", mark_done(&mut tasks, 7));
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The function returns `Result<(), String>`: `Ok(())` when it worked, and `Err(...)` \
                 with a reason when it did not. The `()` in `Ok(())` is the unit type — it means \
                 'succeeded, and there is no value to hand back'. The caller now has a choice, \
                 instead of a program that has already crashed.",
            ),
            diagram(
                "An operation with two outcomes",
                "mark_done(1) ──→ Ok(())\nmark_done(7) ──→ Err(\"no task with id 7\")",
            ),
            callout(
                "`Option` vs `Result`",
                "`Option` says 'there may be nothing'. `Result` says 'this may fail, and here is why'. \
                 Finding is absence; marking done is failure. When the missing case needs a reason \
                 the user can read, reach for `Result`.",
            ),
        ],
    )
}

fn ok_and_err() -> SeedLesson {
    SeedLesson::new(
        "`Ok` and `Err`",
        vec![
            labelled(
                "THE IDEA",
                "`Result<T, E>` is an enum with two variants: `Ok(T)` for success and `Err(E)` for \
                 failure. You build one by wrapping a value in the right variant, and read it by \
                 matching.",
            ),
            gap_fill(
                "Return success with no value, and failure with a reason. Fill in the two variants.",
                r#"fn add(title: &str) -> Result<u32, String> {
    if title.is_empty() {
        return ____(String::from("title cannot be empty"));
    }
    ____(1)
}"#,
                &[
                    ("e1", "Err", "The failure variant."),
                    ("o1", "Ok", "The success variant."),
                ],
            ),
            code(
                "rust",
                r#"fn add(title: &str) -> Result<u32, String> {
    if title.is_empty() {
        return Err(String::from("title cannot be empty"));
    }

    Ok(title.len() as u32)
}

fn main() {
    match add("buy milk") {
        Ok(n) => println!("added, length {n}"),
        Err(reason) => println!("could not add: {reason}"),
    }

    match add("") {
        Ok(n) => println!("added, length {n}"),
        Err(reason) => println!("could not add: {reason}"),
    }
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`Result<u32, String>` is 'either a `u32` or a `String` explaining what went wrong'. \
                 `Ok(title.len() as u32)` is success carrying a value; `Err(String::from(...))` is \
                 failure carrying a reason. The `match` binds the value in the `Ok` arm and the reason \
                 in the `Err` arm.",
            ),
            table(
                &["Variant", "Carries", "Means"],
                &[
                    &["`Ok(value)`", "the success value", "it worked"],
                    &["`Err(reason)`", "why it failed", "it did not"],
                ],
            ),
            callout(
                "The error type is part of the signature",
                "`Result<u32, String>` tells the caller not just that it can fail, but that the reason \
                 is a `String`. Changing the error type — to a `TodoError` enum, say — is a visible \
                 change to every caller, which is a feature.",
            ),
        ],
    )
}

fn reading_the_error() -> SeedLesson {
    SeedLesson::new(
        "Reading the Error",
        vec![
            labelled(
                "THE IDEA",
                "The `Err` variant carries data. Matching it lets you read *which* failure happened and \
                 act on it — not just report that something went wrong.",
            ),
            step_through(
                "An error enum with meaningful variants",
                r#"#[derive(Debug)]
enum TodoError {
    NotFound(u32),
    EmptyTitle,
}

fn mark_done(id: u32) -> Result<(), TodoError> {
    if id == 0 {
        return Err(TodoError::EmptyTitle);
    }
    Err(TodoError::NotFound(id))
}

match mark_done(7) {
    Ok(()) => println!("done"),
    Err(TodoError::NotFound(id)) => println!("no task with id {id}"),
    Err(TodoError::EmptyTitle) => println!("a task needs a title"),
}"#,
                &[
                    (&[2][..], "`TodoError` has two variants, one of which carries the offending id."),
                    (&[12][..], "The `Ok` arm binds nothing, because success has no value here."),
                    (&[13][..], "The `NotFound(id)` pattern binds the id from the error."),
                    (&[14][..], "The `EmptyTitle` arm handles the other shape."),
                ],
            ),
            code(
                "rust",
                r#"#[derive(Debug)]
enum TodoError {
    NotFound(u32),
    EmptyTitle,
}

fn main() {
    let err = TodoError::NotFound(7);

    match err {
        TodoError::NotFound(id) => println!("no task with id {id}"),
        TodoError::EmptyTitle => println!("a task needs a title"),
    }
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Because the error is an enum, each failure shape gets its own arm, and the \
                 `NotFound(id)` pattern pulls the id out so the message can name it. This is the same \
                 enum-plus-match skill from Module 07, now doing its most common real job: explaining \
                 failure precisely.",
            ),
            callout(
                "Errors are data",
                "A `String` error can only be printed. An error *enum* can be matched, so callers can \
                 respond differently to different failures — retry one, report another, ignore a \
                 third. That is worth the extra type.",
            ),
        ],
    )
}

fn question_mark_on_result() -> SeedLesson {
    SeedLesson::new(
        "`?` on `Result`",
        vec![
            labelled(
                "THE IDEA",
                "Inside a function that returns `Result`, `?` opens a `Result`: on success it gives \
                 you the value, and on failure it returns the `Err` from the function immediately. \
                 No `match`, no nesting.",
            ),
            step_through(
                "Propagating failure with one character",
                r#"fn mark_done(tasks: &mut [Task], id: u32) -> Result<(), TodoError> {
    let task = tasks
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or(TodoError::NotFound(id))?;

    task.done = true;
    Ok(())
}"#,
                &[
                    (&[1][..], "The function returns `Result<(), TodoError>`."),
                    (&[2][..], "`find` gives `Option`."),
                    (&[4][..], "`.ok_or(...)` turns the `Option` into a `Result`."),
                    (&[4][..], "The `?` opens the `Result`: `Ok` continues, `Err` returns from the function with that error."),
                    (&[6][..], "Only reached on success, so `task.done = true` is safe."),
                ],
            ),
            code(
                "rust",
                r#"#[derive(Debug)]
enum TodoError {
    NotFound(u32),
}

struct Task {
    id: u32,
    done: bool,
}

fn mark_done(tasks: &mut [Task], id: u32) -> Result<(), TodoError> {
    let task = tasks
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or(TodoError::NotFound(id))?;

    task.done = true;
    Ok(())
}

fn main() {
    let mut tasks = vec![Task { id: 1, done: false }];
    println!("{:?}", mark_done(&mut tasks, 1));
    println!("{:?}", mark_done(&mut tasks, 7));
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The `?` is shorthand for 'if this is `Err`, return it now; otherwise continue with the \
                 value'. The function's return type must be a `Result` whose error type matches, or can \
                 be converted into it (next lessons). This is the single most common way errors travel \
                 in Rust.",
            ),
            diagram(
                "`?` is an early exit",
                "let task = ...?;\n            │\n   Ok(task) ┤→ continue\n   Err(e)  ┘→ return Err(e) from the function",
            ),
            callout(
                "`?` works on `Option` too",
                "The operator is generic over 'things that can carry a failure'. In a function \
                 returning `Option`, `?` on an `Option` returns `None`; in one returning `Result`, `?` \
                 on a `Result` returns the `Err`. The return type decides which.",
            ),
        ],
    )
}

fn map_and_map_err() -> SeedLesson {
    SeedLesson::new(
        "`map` and `map_err`",
        vec![
            labelled(
                "THE IDEA",
                "`map` transforms the success value, leaving any error alone. `map_err` transforms the \
                 error, leaving the success alone. Together they let you shape a `Result` without a \
                 `match`.",
            ),
            gap_fill(
                "Change the error into a friendlier one. Fill in the method.",
                r#"let result: Result<u32, String> = Err(String::from("bad"));

let friendlier = result.____(|e| format!("could not parse: {e}"));
println!("{:?}", friendlier);"#,
                &[("m1", "map_err", "Transform the error, keep the success.")],
            ),
            code(
                "rust",
                r#"fn main() {
    let ok: Result<u32, String> = Ok(3);

    let doubled = ok.map(|n| n * 2);
    println!("{doubled:?}");

    let err: Result<u32, String> = Err(String::from("bad input"));
    let labelled = err.map_err(|e| format!("parse error: {e}"));
    println!("{labelled:?}");
}"#,
            ),
            table(
                &["Method", "Touches", "Leaves alone"],
                &[
                    &["`map(f)`", "the `Ok` value", "the `Err`"],
                    &["`map_err(g)`", "the `Err` value", "the `Ok`"],
                ],
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`ok.map(|n| n * 2)` turns `Ok(3)` into `Ok(6)`; if it had been `Err`, the error would \
                 pass through untouched. `map_err` is the mirror: it rewrites the reason, useful when \
                 you are converting a low-level error into your program's own error type.",
            ),
            callout(
                "Shape the result, not the branches",
                "`map` and `map_err` express 'change this part if it is there' without unpacking. They \
                 are the `Result` cousins of the `Option` methods from the first half of the module.",
            ),
        ],
    )
}

fn chaining() -> SeedLesson {
    SeedLesson::new(
        "`and_then` and Chaining",
        vec![
            labelled(
                "THE IDEA",
                "`and_then` runs a second operation on the success value, when that operation can \
                 itself fail. It chains fallible steps into a pipeline, stopping at the first failure.",
            ),
            code(
                "rust",
                r#"fn parse_id(text: &str) -> Result<u32, String> {
    text.trim()
        .parse::<u32>()
        .map_err(|_| format!("not a number: {text}"))
}

fn double(n: u32) -> Result<u32, String> {
    n.checked_mul(2).ok_or_else(|| String::from("overflow"))
}

fn main() {
    let result = parse_id("21").and_then(double);
    println!("{result:?}");

    let bad = parse_id("nope").and_then(double);
    println!("{bad:?}");
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`parse_id(\"21\")` gives `Ok(21)`; `and_then(double)` then runs `double(21)` to get \
                 `Ok(42)`. With `\"nope\"`, the first step already failed, so `and_then` never calls \
                 `double` — the error passes straight through. A chain of `and_then` is a sequence of \
                 steps that stops at the first failure.",
            ),
            guess_run(
                "Predict what this prints.",
                r#"fn main() {
    let r: Result<u32, String> = Err(String::from("first failed"));

    let out = r.and_then(|n| Ok(n + 1));
    println!("{out:?}");
}"#,
                &[
                    "Ok(1)",
                    "Err(\"first failed\")",
                    "It does not compile",
                ],
                1,
                "`and_then` only runs the next step when the result is `Ok`. Here it is `Err`, so the \
                 closure is never called and the original error is returned unchanged. That short-\
                 circuit is what makes a chain of `and_then` safe.",
            ),
            callout(
                "`map` vs `and_then`",
                "Use `map` when the next step cannot fail. Use `and_then` when it can. Passing a \
                 fallible function to `map` produces a nested `Result`, which `and_then` exists to \
                 avoid.",
            ),
        ],
    )
}

fn defaults_on_result() -> SeedLesson {
    SeedLesson::new(
        "Defaults on `Result`",
        vec![
            labelled(
                "THE IDEA",
                "`Result` has the same default methods as `Option`. `unwrap_or` gives the success value \
                 or a substitute; `unwrap_or_default` uses the value type's default; \
                 `unwrap_or_else` builds the substitute lazily.",
            ),
            gap_fill(
                "Turn a `Result` into a plain number with a fallback. Fill in the method.",
                r#"let parsed: Result<u32, String> = "nope".parse().map_err(|_| String::from("bad"));

let n = parsed.____(0);
println!("{n}");"#,
                &[("u1", "unwrap_or", "The success value, or 0.")],
            ),
            code(
                "rust",
                r#"fn main() {
    let ok: Result<u32, String> = Ok(5);
    let err: Result<u32, String> = Err(String::from("bad"));

    println!("{}", ok.unwrap_or(0));
    println!("{}", err.clone().unwrap_or(0));
    println!("{}", err.clone().unwrap_or_default());
    println!("{}", err.unwrap_or_else(|_| 99));
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`unwrap_or(0)` gives the success value or `0`. `unwrap_or_default()` gives the value or \
                 `u32::default()` — which is `0`, but you did not have to say so. \
                 `unwrap_or_else(|_| 99)` runs the closure only on failure. None of these panic; each \
                 turns a `Result` into a definite value.",
            ),
            table(
                &["Method", "On `Ok`", "On `Err`"],
                &[
                    &["`unwrap_or(x)`", "the value", "`x`"],
                    &["`unwrap_or_default()`", "the value", "the type's default"],
                    &["`unwrap_or_else(f)`", "the value", "`f(error)`"],
                ],
            ),
            callout(
                "The panic-free versions",
                "There is an `unwrap` on `Result` too, and it panics on `Err` just as it does on \
                 `None`. These three methods are what you reach for when a failure has a reasonable \
                 fallback and you want a plain value back.",
            ),
        ],
    )
}

fn combining() -> SeedLesson {
    SeedLesson::new(
        "Combining `Option` and `Result`",
        vec![
            labelled(
                "THE IDEA",
                "Real programs mix the two. A lookup returns `Option`; the operation that uses it \
                 returns `Result`. `ok_or` and `ok()` are the crossings between them.",
            ),
            step_through(
                "Crossing from `Option` to `Result` inside a fallible function",
                r#"fn complete(tasks: &mut [Task], id: u32) -> Result<(), TodoError> {
    let task = tasks
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or(TodoError::NotFound(id))?;

    task.done = true;
    Ok(())
}"#,
                &[
                    (&[2][..], "`find` returns `Option<&mut Task>` — a lookup, which may miss."),
                    (&[5][..], "`ok_or(...)` turns 'missing' into a specific failure with a reason."),
                    (&[5][..], "The `?` then propagates that failure out of the function."),
                    (&[7][..], "Only on success do we mark the task done and return `Ok(())`."),
                ],
            ),
            code(
                "rust",
                r#"#[derive(Debug)]
enum TodoError {
    NotFound(u32),
}

struct Task {
    id: u32,
    done: bool,
}

fn complete(tasks: &mut [Task], id: u32) -> Result<(), TodoError> {
    let task = tasks
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or(TodoError::NotFound(id))?;

    task.done = true;
    Ok(())
}

fn main() {
    let mut tasks = vec![Task { id: 1, done: false }];
    println!("{:?} {:?}", complete(&mut tasks, 1), complete(&mut tasks, 9));
}"#,
            ),
            paragraphs(&["The other direction is just as useful. `.ok()` turns a `Result` into an `Option`, discarding the reason:"]),
            code(
                "rust",
                r#"fn main() {
    let parsed: Result<u32, String> = "42".parse().map_err(|_| String::from("bad"));

    let as_option: Option<u32> = parsed.ok();
    println!("{as_option:?}");
}"#,
            ),
            table(
                &["From", "To", "Method"],
                &[
                    &["`Option<T>`", "`Result<T, E>`", "`ok_or(e)` / `ok_or_else(f)`"],
                    &["`Result<T, E>`", "`Option<T>`", "`ok()`"],
                ],
            ),
            callout(
                "Choose the type at the boundary",
                "Inside a lookup, absence is natural, so return `Option`. At the edge of the program, \
                 the user needs a reason, so convert to `Result` with `ok_or`. The crossing is one \
                 method call, and it names the moment the program stops guessing and starts \
                 explaining.",
            ),
        ],
    )
}
