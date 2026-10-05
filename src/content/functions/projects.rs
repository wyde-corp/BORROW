//! Projects for Module 04 — Functions.
//!
//! Difficulty progression per spec §25:
//!   1 — defining, parameters, simple returns
//!   2 — parameters, returns, expressions
//!   3 — multiple functions, match, composition
//!   4 — functions plus control flow
//!   5 — boolean returns, early returns
//!   6 — loops, collections, borrowing
//!   7 — match, loops, state
//!   8 — decomposition, &mut, state changes
//!   9 — composition, input handling, score state
//!  10 — capstone, problem spec only

use crate::content::types::{Difficulty, SeedProject};

pub fn projects() -> Vec<SeedProject> {
    vec![
        SeedProject::new(
            "Greeting Generator",
            "A function that takes a name and produces a greeting. The smallest useful exercise there is.",
            Difficulty::Beginner,
            "15 min",
            &[
                "Write `greet(name: &str) -> String` that returns a greeting",
                "Write `greet_formal(name: &str) -> String` that is more formal",
                "Write `choose(name: &str, formal: bool) -> String` that picks between the two",
                "Call all three and print the results",
            ],
            &["defining functions", "parameters", "return values", "&str"],
            &[
                "No `println!` inside `greet` or `greet_formal` — they return, the caller prints",
                "No `mut` anywhere; nothing here changes",
            ],
            "Casual:  Hello there, Gus!\nFormal:  Good day, Gus.\nChosen:  Good day, Gus.",
            &["Add a `shout_greeting` that returns the greeting in capitals", "Add an `Option` return for an empty name"],
        ),
        SeedProject::new(
            "Temperature Converter",
            "Parameters in, converted values out. Build up from a single conversion to a menu.",
            Difficulty::Beginner,
            "25 min",
            &[
                "Write `to_celsius(f: f64) -> f64` and `to_fahrenheit(c: f64) -> f64`",
                "Write `is_freezing(c: f64) -> bool` that uses the result",
                "Write a `describe(c: f64) -> &'static str` using an `if` expression as the return",
                "Convert five hardcoded temperatures and print a table",
            ],
            &["parameters", "f64", "return values", "if as an expression"],
            &[
                "Every conversion is its own function — no arithmetic in main",
                "No `-> f64` function may contain a `println!`",
            ],
            "Celsius  Fahrenheit  Freezing?\n0.00      32.00       yes\n37.00     98.60       no\n-40.00    -40.00      yes",
            &["Add a `feels_like` function with a wind-chill adjustment", "Loop over a range of temperatures"],
        ),
        SeedProject::new(
            "Calculator Engine",
            "Several small functions composed into a working calculator, dispatched with `match`.",
            Difficulty::Beginner,
            "35 min",
            &[
                "Write `add`, `subtract`, `multiply`, and `divide`, each taking two f64 and returning f64",
                "`divide` returns `Option<f64>` so division by zero is handled by the caller",
                "Write `apply(op: char, a: f64, b: f64) -> Result<f64, String>` that dispatches with a `match`",
                "Write `format_result` and have the caller print",
            ],
            &["multiple functions", "match", "function composition", "Option", "Result"],
            &[
                "No arithmetic in the dispatch function — it only chooses which function to call",
                "`divide` must not panic on zero; it returns None",
                "Every public function gets a doc comment",
            ],
            "> 10 * 4\n10 * 4 = 40.00\n\n> 10 / 0\nCannot divide by zero",
            &["Add power and modulo", "Support a `Result` from a function that parses the operator"],
        ),
        SeedProject::new(
            "Grade Analyzer",
            "Functions plus the control flow from module 03, returning a full analysis.",
            Difficulty::Intermediate,
            "40 min",
            &[
                "Write `letter_grade(score: u32) -> Option<&'static str>` returning None for out-of-range",
                "Write `class_average(scores: &[u32]) -> Option<f64>` returning None for an empty slice",
                "Write `highest(scores: &[u32]) -> Option<u32>` using a loop and an early return",
                "Write `analyse(scores: &[u32]) -> String` that composes the three and formats a report",
            ],
            &["functions", "match", "Option", "loops", "early returns", "borrowing"],
            &[
                "The numeric helpers take &[u32] — they only read",
                "Only `analyse` prints, and only via the returned String",
                "No function returns () — every one produces something",
            ],
            "Scores: 88, 95, 72, 61\nAverage: 79.00 (C)\nHighest: 95 (A)\nLowest:  61 (F)",
            &["Add a `median` function that works on a sorted copy", "Report how many students failed"],
        ),
        SeedProject::new(
            "Password Validator",
            "Boolean returns and early returns, doing real work. Almost every function returns `bool`.",
            Difficulty::Intermediate,
            "40 min",
            &[
                "Write `has_min_length(s: &str) -> bool`, `has_digit(s: &str) -> bool`, `has_upper(s: &str) -> bool`",
                "Write `validate(password: &str) -> Result<&str, &'static str>` using early returns",
                "The error must say which rule failed, not just 'invalid'",
                "Write `strength(password: &str) -> u8` scoring 0-4, and print a bar",
            ],
            &["boolean returns", "early returns", "Result", "string methods", "composition"],
            &[
                "No `unwrap()` — validate returns Result because failures are expected, not exceptional",
                "Each rule is its own function so they can be tested independently",
                "The order of checks in validate is the order the user sees",
            ],
            "> hunter2\n  x  x  x  \nWeak — 3 of 4 rules passed\n  rule failed: needs an uppercase letter",
            &["Add a rule rejecting common passwords", "Return the failed rules as a Vec instead of the first one"],
        ),
        SeedProject::new(
            "Shopping Cart Calculator",
            "Loops, collections, and borrowing together — the first project where ownership bites.",
            Difficulty::Intermediate,
            "50 min",
            &[
                "Write `line_total(price: f64, quantity: u32) -> f64`",
                "Write `subtotal(items: &[(f64, u32)]) -> f64` that loops and borrows",
                "Write `apply_discount(subtotal: f64, percent: f64) -> f64`",
                "Write `with_tax(amount: f64) -> f64` and `receipt(items: &[(f64, u32)]) -> String`",
            ],
            &["loops", "collections", "borrowing", "tuples", "function composition"],
            &[
                "Every helper takes &[...] — no function takes ownership of the item list",
                "The tuple is (price, quantity); document which is which in a comment",
                "No arithmetic inside the printing function",
            ],
            "Coffee      2 x  9.00  18.00\nMug         1 x 12.50  12.50\nBeans       1 x 14.00  14.00\n\nSubtotal: 44.50\nDiscount:  4.45\nTax:       4.81\nTotal:     44.86",
            &["Add a `most_expensive` helper returning the item tuple", "Support a `&mut Vec` to add items as you go"],
        ),
        SeedProject::new(
            "Text Adventure Engine",
            "A small state machine. Every location and verb is a `match` arm.",
            Difficulty::Advanced,
            "60 min",
            &[
                "Define an enum for at least four locations and one for the player's action",
                "Write `describe(location: &Location) -> &'static str`",
                "Write `apply(action: Action, state: &mut State) -> String` dispatching on a tuple pattern",
                "Loop until the player quits or a win/lose condition fires",
            ],
            &["enums", "match", "loops", "&mut state", "tuples", "error messages for impossible actions"],
            &[
                "Every (location, action) pair that is impossible must produce a message, not silence",
                "The engine functions must not read from stdin — main does that and passes the action in",
                "No `unwrap()` in the parser",
            ],
            "You are in the ATTIC.\n> go north\nYou cannot go north from here.\n> search chest\nFound a rusty key.\n> go south\nYou are in the HALLWAY.\n> unlock door\nThe door opens. You win!",
            &["Add an inventory limit that fails a pick-up when full", "Add a second ending reached by running out of turns"],
        ),
        SeedProject::new(
            "Mini Banking System",
            "Decomposition and `&mut` together. Every operation changes shared state.",
            Difficulty::Advanced,
            "60 min",
            &[
                "Write `validate_amount(amount: f64) -> Result<f64, &'static str>` rejecting zero and negatives",
                "Write `deposit(balance: &mut f64, amount: f64) -> Result<f64, String>`",
                "Write `withdraw(balance: &mut f64, amount: f64) -> Result<f64, String>` rejecting overdrafts",
                "Write `statement(balance: f64, count: u32) -> String` and a menu loop that dispatches with `match`",
            ],
            &["&mut references", "decomposition", "Result", "match dispatch", "state changes"],
            &[
                "deposit and withdraw take &mut f64 — they change the caller's balance",
                "No function may mutate the balance and also print; split them",
                "All validation lives in validate_amount, not repeated in each operation",
            ],
            "1. Balance\n2. Deposit\n3. Withdraw\n4. Quit\n\n> 3\nAmount: 500.00\nInsufficient funds. Balance: 100.00\n\n> 4\nBalance: 100.00  Transactions: 1",
            &["Record transactions in a Vec and print a full statement", "Add a withdrawal limit and a daily cap"],
        ),
        SeedProject::new(
            "CLI Quiz Game",
            "Function composition end to end: input handling, scoring state, and a question bank.",
            Difficulty::Advanced,
            "75 min",
            &[
                "Define a `Question` type carrying the prompt, four options, and the correct index",
                "Write `read_answer() -> Option<usize>` that parses input and returns None on anything invalid",
                "Write `is_correct(q: &Question, answer: usize) -> bool`",
                "Write `ask(q: &Question) -> bool` that reads, checks, and reports",
                "Run questions in a loop, tracking score with a `&mut` parameter, and print a final summary",
            ],
            &["composition", "input handling", "control flow", "&mut score", "Option", "user-facing error messages"],
            &[
                "read_answer returns Option, never unwraps, and re-prompts on bad input",
                "Only ask() reads stdin. The quiz loop does not.",
                "Score is passed as &mut so ask can update it — or ask returns a bool and the loop counts",
            ],
            "Q1. Which keyword makes a variable mutable?\n  1. let\n  2. mut\n  3. var\n  4. change\n> 2\nCorrect!\n\nFinal: 2 of 3 correct",
            &["Shuffle the question order", "Add categories and a score per category at the end"],
        ),
        SeedProject::new(
            "Mini CLI Application",
            "The capstone. Here is a problem, not a recipe. You decide what the functions are.",
            Difficulty::Advanced,
            "90 min",
            &[
                "Build a task tracker that can add, complete, list, and remove items",
                "It must survive a restart — persist to a file and load on start",
                "Every action the user can take is dispatched from one place with a match",
                "Parsing of user input is isolated in its own function, separate from the logic",
                "The core data operations (add, complete, remove) take no input from stdin at all",
                "Errors are handled and reported; nothing panics on bad input",
                "Print a clear summary on exit, including how many tasks remain",
            ],
            &[
                "decomposition",
                "&mut parameters",
                "match dispatch",
                "Result and error propagation",
                "file I/O",
                "state changes",
            ],
            &[
                "No function longer than roughly 40 lines — if one is, it is doing two jobs",
                "No `unwrap()` in the parser, the file layer, or the command dispatcher",
                "The match on the user's command must be exhaustive over the command type",
                "Persistence functions return Result and the caller decides what to do about failures",
            ],
            "BORROW_ TASKS\n1. add <task>     add a task\n2. done <n>     mark task n complete\n3. list          show all tasks\n4. quit          exit\n\n> add write the module 4 content\n  Added.\n> list\n[ ] 1. write the module 4 content\n> done 1\n  Done.\n> quit\n1 task remaining. Goodbye.",
            &[
                "Add an edit command that changes a task's text",
                "Add categories and a `list <category>` filter",
                "Add an undo stack — this is where a real design decision appears",
            ],
        ),
    ]
}
