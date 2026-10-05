//! Module 09 — Option & Result projects.
//!
//! Six projects escalating from deliberate repetition to the robust Todo CLI
//! capstone. Every one asks the learner to place each failure on the
//! absence-versus-failure line.

use crate::content::types::{Difficulty, SeedProject};

pub fn projects() -> Vec<SeedProject> {
    vec![
        SeedProject::new(
            "Option Playground",
            "Deliberately repetitive. Run the same lookup four ways — `unwrap`, `match`, `if let`, `unwrap_or` — and record what each does when the value is missing.",
            Difficulty::Beginner,
            "30 min",
            &[
                "Build a small `Vec<String>` and look something up by index or search",
                "Handle the missing case with `match`, `if let`, `unwrap_or`, and `unwrap`",
                "Comment out the `unwrap` case after seeing it panic",
                "Write one sentence per method on when you would use it",
                "Finish with a sentence on what the `Option` in the signature was warning you about",
            ],
            &[
                "`Option<&T>` from lookups",
                "`match` / `if let` / `let else`",
                "`unwrap` and its panic",
                "`unwrap_or` defaults",
            ],
            &[
                "No `Result` in this project — that is the next one",
                "The `unwrap` panic must be commented out before it stops you running the rest",
                "The final two sentences are required",
                "Do not reach for `.ok_or()` yet",
            ],
            "let names = vec![String::from(\"apples\"), String::from(\"pears\")];\nprintln!(\"{:?}\", names.get(0));\nprintln!(\"{:?}\", names.get(9));\nprintln!(\"{}\", names.get(9).unwrap_or(&String::from(\"none\")));",
            &[
                "Add a `find` by value and compare its `Option` with `get`",
                "Explain in a comment why `get` returns a reference",
            ],
        ),

        SeedProject::new(
            "Safe Lookup",
            "A task list where every lookup returns `Option` and every caller handles the missing case. The lookup side of the module, on its own.",
            Difficulty::Beginner,
            "45 min",
            &[
                "Define `Task { id, title, done }` and a `Vec<Task>` list",
                "Write `fn find(&self, id: u32) -> Option<&Task>`",
                "Write `fn find_mut(&mut self, id: u32) -> Option<&mut Task>`",
                "Call both from `main` and handle `Some` and `None`",
                "Print a task's title, or a clear message when it is missing",
                "Use `let else` in one function and `if let` in another, and say why",
            ],
            &[
                "`Option` from a lookup",
                "`find` and `find_mut`",
                "`let else` and `if let`",
                "borrowing through an `Option`",
            ],
            &[
                "No `.unwrap()` anywhere",
                "Every lookup result must be handled",
                "No panics on a missing id",
                "Comment each function with why it returns `Option` rather than `Result`",
            ],
            "fn find(&self, id: u32) -> Option<&Task> {\n    self.tasks.iter().find(|t| t.id == id)\n}\n\nif let Some(task) = list.find(2) {\n    println!(\"{}\", task.title);\n} else {\n    println!(\"no task 2\");\n}",
            &[
                "Add `fn find_by_title(&self, title: &str) -> Option<&Task>`",
                "Add `fn complete(&mut self, id: u32) -> bool` that returns whether a task was found",
            ],
        ),

        SeedProject::new(
            "Fallible Operations",
            "The first `Result` project. `add` and `mark_done` return `Result`, with a small custom error enum and a `?`-friendly signature.",
            Difficulty::Intermediate,
            "1 h",
            &[
                "Define `Task` and a `TodoError` enum with `NotFound(u32)` and `EmptyTitle`",
                "Write `fn add(&mut self, title: &str) -> Result<u32, TodoError>`",
                "Write `fn mark_done(&mut self, id: u32) -> Result<(), TodoError>`",
                "Write `fn run(&mut self) -> Result<(), TodoError>` that uses `?` on both",
                "Have `main` print the `Display` form of any error",
                "Implement `Display` for `TodoError`",
            ],
            &[
                "`Result<T, E>` construction",
                "`?` propagation",
                "a custom error enum",
                "`Display` for a readable message",
            ],
            &[
                "No `.unwrap()` in `run`",
                "The error enum must carry the offending id",
                "`add` must reject an empty title",
                "Each method gets a comment on why it returns `Result` rather than `Option`",
            ],
            "fn run(&mut self) -> Result<(), TodoError> {\n    let id = self.add(\"buy milk\")?;\n    self.mark_done(id)?;\n    Ok(())\n}\n\nif let Err(err) = list.run() {\n    eprintln!(\"error: {err}\");\n}",
            &[
                "Add a `remove(&mut self, id) -> Option<Task>` and explain why it is `Option`",
                "Return `Result` from `main` instead of matching inside it",
            ],
        ),

        SeedProject::new(
            "Robust Parser",
            "Parse a task file line by line, returning errors that name the bad input. The parse side of the module.",
            Difficulty::Intermediate,
            "1 h",
            &[
                "Define a line format such as `id|title|done`",
                "Write `fn parse_line(line: &str) -> Result<Task, TodoError>`",
                "Write `fn parse_file(text: &str) -> Result<Vec<Task>, TodoError>`",
                "Carry the offending line in the parse error",
                "Decide all-or-nothing or collect-every-error, and comment why",
                "Test with a good file and with a file containing a bad line",
            ],
            &[
                "`Result` from parsing",
                "`map_err` to convert errors",
                "data-carrying error variants",
                "`collect` into a `Result`",
            ],
            &[
                "No panics on bad input",
                "The parse error must include the line that failed",
                "The all-or-nothing or collect-all decision must be justified",
                "No `unwrap` after the first successful parse",
            ],
            "fn parse_line(line: &str) -> Result<Task, TodoError> {\n    let parts: Vec<&str> = line.split('|').collect();\n    if parts.len() != 3 {\n        return Err(TodoError::Parse(line.to_string()));\n    }\n    // ...\n}\n\nlet text = \"1|buy milk|todo\\n2|walk the dog|done\\n\";\nprintln!(\"{:?}\", parse_file(text));",
            &[
                "Add line numbers to the parse error so it names the row",
                "Add a companion that collects every error rather than stopping at the first",
            ],
        ),

        SeedProject::new(
            "Todo CLI",
            "A command-line to-do list that loads a file, adds and completes tasks, and saves — reporting errors instead of panicking.",
            Difficulty::Advanced,
            "2 h",
            &[
                "Model `Task` and a `TodoList` with `Vec<Task>` and a `next_id`",
                "Implement `add`, `find`, `mark_done`, `load`, and `save`",
                "Give the program one `TodoError` enum, with `From<io::Error>`",
                "Have `main` return `Result<(), TodoError>`",
                "From `main`, add two tasks, complete one, and save",
                "Print the list with a `[x]`/`[ ]` marker per task",
            ],
            &[
                "`Option` and `Result` together",
                "custom error enum with `From`",
                "`?` through several layers",
                "`main` returning `Result`",
            ],
            &[
                "No `.unwrap()` in the happy path",
                "Lookups return `Option`; operations return `Result`",
                "The error enum implements `Display` and `Error`",
                "Every failure that a user can cause is returned, not panicked",
            ],
            "fn main() -> Result<(), TodoError> {\n    let mut list = TodoList::new();\n    let id = list.add(\"buy milk\")?;\n    list.add(\"walk the dog\")?;\n    list.mark_done(id)?;\n    list.save(\"todo.txt\")?;\n    Ok(())\n}",
            &[
                "Add a `load` path and merge a saved file with the in-memory list",
                "Add a `remove` returning `Option<Task>` and report whether anything was removed",
            ],
        ),

        SeedProject::new(
            "Todo CLI",
            "The capstone: the robust to-do list finished and justified. `Option` where absence is the truth, `Result` where failure is, and a written defense of every remaining unwrap.",
            Difficulty::Advanced,
            "3 h",
            &[
                "Build the full `TodoList` with add, find, find_mut, mark_done, remove, load, and save",
                "One `TodoError` enum, with data-carrying variants and a `From` for IO",
                "`Display` and `Error` for the enum; `main` returns `Result<(), TodoError>`",
                "Parse and serialise the task file",
                "Handle a missing file gracefully (start empty) and a malformed file with an error",
                "Write a note: for every fallible function, say `Option` or `Result` and why, and justify every remaining `.unwrap()`",
            ],
            &[
                "absence versus failure",
                "custom errors and `From`",
                "`?` across layers",
                "parsing and IO",
                "API design and justification",
            ],
            &[
                "No `.unwrap()` in any code a user can reach",
                "Lookups return `Option`; operations return `Result`",
                "A malformed file must name the offending line",
                "The written note is required — working code alone is not the deliverable",
            ],
            "let mut list = match TodoList::load(\"todo.txt\") {\n    Ok(list) => list,\n    Err(TodoError::Io(e)) if e.kind() == std::io::ErrorKind::NotFound => TodoList::new(),\n    Err(err) => return Err(err),\n};\n\nlist.add(\"buy milk\")?;\nlist.save(\"todo.txt\")?;",
            &[
                "Add a subcommand that removes a task by id and reports whether it existed",
                "Replace the custom enum with `Box<dyn Error>` and write down what you lost",
                "Write the justification note a second time, after the code, and compare",
            ],
        ),
    ]
}
