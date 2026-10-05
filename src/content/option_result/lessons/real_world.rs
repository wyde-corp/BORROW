//! Module 09 — Option & Result, lessons 27–29.
//!
//! The Todo CLI, assembled. Parsing a task line, loading and saving with `?`,
//! and the whole program with `main` returning a `Result`.

use crate::content::types::{
    callout, code, gap_fill, labelled, paragraphs, steps, table, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![parsing_a_line(), loading_and_saving(), assembled()]
}

fn parsing_a_line() -> SeedLesson {
    SeedLesson::new(
        "Parsing a Todo Line",
        vec![
            labelled(
                "CONCEPT",
                "A task line looks like `1|buy milk|todo`. Parsing it can fail in two ways — the wrong \
                 number of parts, or an id that is not a number — and both should return an error that \
                 carries the offending line, not panic.",
            ),
            code(
                "rust",
                r#"#[derive(Debug)]
enum TodoError {
    Parse(String),
}

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

fn parse_line(line: &str) -> Result<Task, TodoError> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() != 3 {
        return Err(TodoError::Parse(line.to_string()));
    }

    let id = parts[0]
        .trim()
        .parse::<u32>()
        .map_err(|_| TodoError::Parse(line.to_string()))?;

    let done = parts[2].trim() == "done";
    Ok(Task {
        id,
        title: parts[1].trim().to_string(),
        done,
    })
}

fn main() {
    println!("{:?}", parse_line("1|buy milk|todo"));
    println!("{:?}", parse_line("oops"));
}"#,
            ),
            labelled(
                "EXPLANATION",
                "The bad-parts case returns early with `Parse(line)`. The bad-id case converts the \
                 parse error with `map_err` and uses `?` to return it. Both errors carry the original \
                 line, so the eventual message can quote exactly what failed. A well-behaved parser \
                 never panics on input from outside the program.",
            ),
            gap_fill(
                "Convert the parse error into the program's own error, then propagate it. Fill in the \
                 method and the operator.",
                r#"let id = parts[0]
    .trim()
    .parse::<u32>()
    .____(|_| TodoError::Parse(line.to_string()))____;"#,
                &[
                    ("m1", "map_err", "Rewrite the error, keep the success."),
                    ("q1", "?", "Propagate the failure out of the function."),
                ],
            ),
            callout(
                "Parse errors should carry the input",
                "`Parse(String)` with the offending line is far more useful than a bare `Parse`. The \
                 user sees which line broke, and you can act on it. This is the data-carrying error \
                 from the errors lessons, doing its real job.",
            ),
        ],
    )
}

fn loading_and_saving() -> SeedLesson {
    SeedLesson::new(
        "Loading and Saving",
        vec![
            labelled(
                "CONCEPT",
                "Reading and writing files both return `Result`, and both can fail for a reason the \
                 user should hear. With `From<io::Error>` in place, `?` carries them into the \
                 program's own error type without a `match`.",
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

fn save(path: &str, text: &str) -> Result<(), TodoError> {
    std::fs::write(path, text)?;
    Ok(())
}

fn main() {
    let path = "/tmp/borrow_todo_demo.txt";

    println!("{:?}", save(path, "1|buy milk|todo\n"));
    println!("{:?}", load(path));
    println!("{:?}", load("/tmp/borrow_todo_does_not_exist.txt"));
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`read_to_string` and `write` return `Result<_, io::Error>`. The `?` converts that into \
                 `TodoError::Io` via the `From` impl, so both functions speak the program's error \
                 type. A missing file is an `Err`, printed as data — not a crash.",
            ),
            gap_fill(
                "Let `?` convert the IO error into the program's error. Fill in the trait and the \
                 method it provides.",
                r#"impl ____<io::Error> for TodoError {
    fn ____(err: io::Error) -> TodoError {
        TodoError::Io(err)
    }
}"#,
                &[
                    ("f1", "From", "The conversion trait `?` uses."),
                    ("f2", "from", "The method that performs the conversion."),
                ],
            ),
            callout(
                "The `?` does the conversion quietly",
                "When you write `read_to_string(path)?`, two things happen: the error is converted via \
                 `From`, and it is returned. Learners often see only the early return and miss the \
                 conversion — but the conversion is why the function's error type and the library's \
                 error type do not have to match.",
            ),
        ],
    )
}

fn assembled() -> SeedLesson {
    SeedLesson::new(
        "The Todo CLI, Assembled",
        vec![
            labelled(
                "CONCEPT",
                "Here is the whole program. A `Vec<Task>` owns the tasks, `find` returns `Option`, \
                 `add` and `mark_done` return `Result`, file IO uses `From` and `?`, the error type \
                 implements `Display` and `Error`, and `main` returns `Result<(), TodoError>` so any \
                 failure ends the program with a message.",
            ),
            code(
                "rust",
                r#"use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

#[derive(Debug)]
enum TodoError {
    NotFound(u32),
    EmptyTitle,
    Parse(String),
    Io(io::Error),
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TodoError::NotFound(id) => write!(f, "no task with id {id}"),
            TodoError::EmptyTitle => write!(f, "a task needs a title"),
            TodoError::Parse(line) => write!(f, "could not parse: {line}"),
            TodoError::Io(err) => write!(f, "io error: {err}"),
        }
    }
}

impl Error for TodoError {}

impl From<io::Error> for TodoError {
    fn from(err: io::Error) -> TodoError {
        TodoError::Io(err)
    }
}

struct TodoList {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { tasks: Vec::new(), next_id: 1 }
    }

    fn add(&mut self, title: &str) -> Result<u32, TodoError> {
        if title.trim().is_empty() {
            return Err(TodoError::EmptyTitle);
        }
        let id = self.next_id;
        self.next_id += 1;
        self.tasks.push(Task { id, title: title.trim().to_string(), done: false });
        Ok(id)
    }

    fn find(&self, id: u32) -> Option<&Task> {
        self.tasks.iter().find(|t| t.id == id)
    }

    fn mark_done(&mut self, id: u32) -> Result<(), TodoError> {
        let task = self
            .tasks
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or(TodoError::NotFound(id))?;
        task.done = true;
        Ok(())
    }

    fn parse_line(line: &str) -> Result<Task, TodoError> {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 3 {
            return Err(TodoError::Parse(line.to_string()));
        }
        let id = parts[0]
            .trim()
            .parse::<u32>()
            .map_err(|_| TodoError::Parse(line.to_string()))?;
        let done = parts[2].trim() == "done";
        Ok(Task { id, title: parts[1].trim().to_string(), done })
    }

    fn load(path: &str) -> Result<TodoList, TodoError> {
        let text = std::fs::read_to_string(path)?;
        let mut list = TodoList::new();
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let task = TodoList::parse_line(line)?;
            list.next_id = list.next_id.max(task.id + 1);
            list.tasks.push(task);
        }
        Ok(list)
    }
}

fn main() -> Result<(), TodoError> {
    let mut list = TodoList::new();

    let id = list.add("buy milk")?;
    list.add("walk the dog")?;
    list.mark_done(id)?;

    for task in &list.tasks {
        let mark = if task.done { "x" } else { " " };
        println!("[{mark}] {} {}", task.id, task.title);
    }

    match list.find(999) {
        Some(task) => println!("found {}", task.title),
        None => println!("no task 999"),
    }

    Ok(())
}"#,
            ),
            table(
                &["Function", "Returns", "Why"],
                &[
                    &["`find`", "`Option<&Task>`", "a lookup may simply miss"],
                    &["`add`", "`Result<u32, TodoError>`", "an empty title is a real failure"],
                    &["`mark_done`", "`Result<(), TodoError>`", "the id may not exist"],
                    &["`parse_line`", "`Result<Task, TodoError>`", "outside input may be malformed"],
                    &["`load`", "`Result<TodoList, TodoError>`", "the file may be unreadable or malformed"],
                    &["`main`", "`Result<(), TodoError>`", "any failure ends the program with a message"],
                ],
            ),
            labelled(
                "The ten common mistakes",
                "1. `.unwrap()` on a lookup that can miss. 2. `.unwrap()` on a fallible operation. 3. \
                 using `Option` where the missing case needs a reason. 4. using `Result` where absence \
                 is genuinely all that happened. 5. a `String` error where a caller needed to match. 6. \
                 an error variant with no data, so the message is vague. 7. forgetting `From`, then \
                 fighting `?`. 8. `?` in a function whose return type does not match. 9. panicking on \
                 input the user controls. 10. catching an error only to re-raise it without adding \
                 anything.",
            ),
            steps(&[
                "Add a `remove` method returning `Option<Task>` and call it from `main`.",
                "Add a `save` method that writes every task back to a file with `?`.",
                "Make `parse_line` report the line number as well as the text, and explain what the error type had to gain.",
                "Decide which of `find` and `mark_done` should share an error type, and why.",
            ]),
            callout(
                "Absence where it belongs, failure where it matters",
                "The design is not 'use `Result` everywhere'. `find` returns `Option` because a lookup \
                 missing is not a failure. `add`, `mark_done`, and `load` return `Result` because \
                 something actually went wrong and the user deserves to know what. Placing the two \
                 correctly is the skill this module teaches.",
            ),
            paragraphs(&[
                "Notice `main` returning `Result<(), TodoError>`. On success it returns `Ok(())` and the \
                 program exits quietly. On any `?` failure it returns `Err`, Rust prints the `Debug` \
                 form of the error, and the process exits with a non-zero status — the correct \
                 behaviour for a command-line tool, with no `match` at the top at all.",
            ]),
        ],
    )
}
