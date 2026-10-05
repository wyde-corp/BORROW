//! Projects for Module 03 — Control Flow.
//!
//! Difficulty curve, per spec:
//!   1 — mostly `if`
//!   2 — `if` + expressions
//!   3 — `loop` + `break`
//!   4 — `match`
//!   5 — `for` + ranges
//!   6 — `Option` + `match`
//!   7 — combining concepts
//!   8 — nested loops
//!   9 — a small state-driven application
//!   10 — a mini command-line game combining almost everything

use crate::content::types::{Difficulty, SeedProject};

pub fn projects() -> Vec<SeedProject> {
    vec![
        SeedProject::new(
            "Number Guessing Game",
            "The classic. A hidden number, guesses until it is right, and useful feedback on each attempt.",
            Difficulty::Beginner,
            "20 min",
            &[
                "Store a secret number between 1 and 100",
                "Use a loop to keep taking guesses until the player is correct",
                "Count the attempts and report them at the end",
                "Handle guesses outside 1-100 with a different message and let them retry",
            ],
            &["if", "loop", "break", "mutability", "comparison"],
            &[
                "The only loop construct allowed is `loop` with `break` — no `while`, no `for`",
                "Every branch must be an `if`, `if let` is not permitted",
                "The secret number must not change once chosen",
            ],
            "Guess a number between 1 and 100.\n> 50\nToo high.\n> 25\nToo low.\n> 37\nCorrect! You took 4 guesses.",
            &["Add a limit of 7 guesses and end the game if it is exceeded", "Add a high/low score table across multiple games"],
        ),
        SeedProject::new(
            "Simple Calculator",
            "Two numbers in, a result out — built with `if` expressions rather than statements.",
            Difficulty::Beginner,
            "25 min",
            &[
                "Store two operands and an operation as an enum or a char",
                "Compute the result using an `if` expression that produces a value",
                "Handle division by zero without panicking",
                "Print the full expression, like `12 + 4 = 16`",
            ],
            &["if as an expression", "enums", "match", "Option"],
            &[
                "The calculation must be a single `if` expression assigned to a variable, not a chain of \
                 statements that prints directly",
                "Division by zero must produce a message, not a panic",
            ],
            "Enter first number: 12\nEnter operation: +\nEnter second number: 4\n\n12 + 4 = 16",
            &["Support exponentiation", "Chain several operations in one session using a loop"],
        ),
        SeedProject::new(
            "CLI Menu System",
            "A menu that loops until the player quits — the shape almost every interactive program has.",
            Difficulty::Beginner,
            "30 min",
            &[
                "Present a numbered menu of at least four options",
                "Loop until the user chooses to quit",
                "Handle each option in its own `if` branch",
                "Handle invalid input by re-prompting without crashing",
                "Use `break` with a value so the loop can report how many actions were taken",
            ],
            &["loop", "break with value", "if/else if", "input validation"],
            &[
                "The menu must be a `loop`, not a `for` and not a `while`",
                "The loop's value must be the number of actions performed",
                "A non-numeric choice must not end the program",
            ],
            "=== MAIN MENU ===\n1. View status\n2. Add item\n3. Remove item\n4. Quit\n\n> 1\nHealth: 100/100\nItems: 0\n\n> 4\nYou performed 2 actions. Goodbye.",
            &["Add sub-menus so each action opens its own loop", "Add a confirmation prompt before destructive actions"],
        ),
        SeedProject::new(
            "Grade Evaluator",
            "Turn a score into a grade, then a summary — the natural home for `match`.",
            Difficulty::Beginner,
            "30 min",
            &[
                "Define an enum for grades: `Grade::A` through `Grade::F`",
                "Convert a score to a grade with a `match` using ranges",
                "Reject scores outside 0-100 with a separate `Option` return",
                "Print a summary: count of each grade, highest, lowest",
            ],
            &["match", "ranges in patterns", "enums", "Option", "if for the summary"],
            &[
                "The score-to-grade conversion must use a `match` with ranges, not if-else if",
                "Invalid scores must return `None` rather than a default grade",
                "You must call `.unwrap_or(...)` or match on the result — no panicking",
            ],
            "Enter a score: 94\nGrade: A\n\nEnter a score: 150\nInvalid score: 150",
            &["Add a + or - modifier (A-, B+) as extra enum variants", "Accept a list of scores and print a histogram"],
        ),
        SeedProject::new(
            "FizzBuzz with a Twist",
            "The familiar warm-up, extended to cover ranges, filtering, and loop control.",
            Difficulty::Beginner,
            "30 min",
            &[
                "Loop from 1 to 100 inclusive using a range",
                "Print 'Fizz' for multiples of 3, 'Buzz' for 5, 'FizzBuzz' for both",
                "Use `continue` to skip numbers that need no special handling",
                "After the main loop, count how many of each category appeared",
            ],
            &["for", "ranges", "continue", "modulo", "match on the category"],
            &[
                "The outer loop must be a `for` over a range, not a `while`",
                "Use `continue` at least once — do not wrap everything in if-else",
                "The counting must happen in the same loop, not a second pass",
            ],
            "1\n2\nFizz\n4\nBuzz\n...\nFizzBuzz\n...\n100\n\nSummary: 33 Fizz, 20 Buzz, 14 FizzBuzz, 33 plain",
            &["Extend to a 7s and 11s variant", "Print a summary for a user-supplied maximum"],
        ),
        SeedProject::new(
            "Password Retry System",
            "A lockout system built on `Option`, `match`, and a retry loop.",
            Difficulty::Intermediate,
            "35 min",
            &[
                "Store the correct password as a `&str`",
                "Allow exactly 3 attempts, using a `while` or `loop` with a counter",
                "Use `match` on an `Option` to represent a failed lookup when checking a lockout list",
                "Lock the account after the attempts run out and say so clearly",
            ],
            &["Option", "match", "loop with a counter", "if", "String comparison"],
            &[
                "No `unwrap()` — the failed-attempt check must be an Option handled with `match` or `if let`",
                "The attempt counter must be immutable after the loop ends; you may need shadowing",
                "The success path must exit the loop immediately rather than checking a flag at the end",
            ],
            "Password: \n> wrong\nIncorrect. 2 attempts remaining.\nPassword: \n> secret\nAccess granted.",
            &["Add exponential backoff with a sleep between attempts", "Keep a lockout list in a Vec and check it with while let"],
        ),
        SeedProject::new(
            "ATM Simulator",
            "A small state machine: balance, deposits, withdrawals, and a menu that does not fall over.",
            Difficulty::Intermediate,
            "45 min",
            &[
                "Store a balance as `f64` and a transaction count as `u32`",
                "Implement deposit and withdrawal, with a withdrawal rejected if it exceeds the balance",
                "Use a `match` to dispatch menu choices to handlers",
                "Loop until the user quits, and print a final statement",
                "Keep the menu immutable — it should never change during the session",
            ],
            &["match", "loop", "Option for the rejected withdrawal", "f64", "state management"],
            &[
                "A rejected withdrawal must be handled, not guarded against silently",
                "The balance must be read with `&` when passed to a printing function, not moved",
                "Menu dispatch must be a `match` on the choice, not a chain of if-else",
            ],
            "1. Balance\n2. Deposit\n3. Withdraw\n4. Quit\n\n> 2\nAmount: 100\nDeposited. New balance: 100.00\n\n> 3\nAmount: 500\nInsufficient funds. Balance: 100.00",
            &["Add a 4-digit PIN check before any operation", "Record every transaction in a Vec and print a statement at the end"],
        ),
        SeedProject::new(
            "Rock Paper Scissors",
            "Random choice against a random player, decided with a `match` over three cases each.",
            Difficulty::Intermediate,
            "45 min",
            &[
                "Define a `Move` enum with Rock, Paper, and Scissors",
                "Generate a random choice for the computer",
                "Resolve the round with a `match` on the pair of moves",
                "Track wins, losses, and ties across a series of rounds",
                "Loop until the player quits, then print the final tally",
            ],
            &["enums", "match", "loop", "if for scoring", "Option for the result of a round"],
            &[
                "The outcome of a round must be computed by a `match`, not by chained if-else comparisons",
                "You may not use a random number generator from a crate — derive a pseudo-random choice from \
                 something simple, or accept a fixed sequence of computer moves",
                "The loop must end on a clear condition, not an arbitrary count",
            ],
            "Round 1 — you: Rock, computer: Scissors\nYou win!\n\nRound 3 — you: Paper, computer: Paper\nTie.\n\nFinal: 2 wins, 1 loss, 1 tie",
            &["Add a best-of-five match with a clear winner announcement", "Validate the player's input and re-prompt on invalid moves"],
        ),
        SeedProject::new(
            "Inventory Manager",
            "A small state-driven application: stock levels, a search menu, and restocking.",
            Difficulty::Intermediate,
            "50 min",
            &[
                "Define an `Item` struct or enum carrying a name and a quantity",
                "Store inventory in a `Vec`",
                "Find an item by name with a `loop` that `break`s with an `Option<usize>`",
                "Add and remove stock, refusing to remove more than is present",
                "Print a sorted report using a `for` over the collection",
            ],
            &["Vec", "loop with break value", "Option for not-found", "for", "match for the report rows"],
            &[
                "The search must return `Option<usize>`, not a default index — a missing item is a normal \
                 outcome, not an error",
                "Iterate with `&` when reading so the inventory survives the loop",
                "Restocking must use `&mut` and modify elements in place",
            ],
            "1. List\n2. Search\n3. Restock\n4. Quit\n\n> 2\nItem: elixir\nNot found.\n\n> 1\nPotion   x 12\nElixir   x 3",
            &["Add a low-stock warning using a filter and collect", "Support removing an item entirely from the Vec"],
        ),
        SeedProject::new(
            "Number Detective",
            "A text adventure that leans on enums, matching, and multi-stage state.",
            Difficulty::Advanced,
            "1 hour",
            &[
                "Model the world as an enum with at least four locations",
                "Model the player's action as an enum with at least six actions",
                "Resolve an action with a `match` on the (location, action) pair",
                "Track inventory, health, and a win condition in mutable state",
                "Reject impossible actions with a message rather than silently doing nothing",
                "End the game on a win or a health-zero condition, and report which",
            ],
            &["enums", "nested match", "Option", "loop", "if expressions", "state machines"],
            &[
                "The (location, action) dispatch must be a single match on a tuple, not nested if-else",
                "Every unreachable action in a location must produce an explicit response",
                "The game loop must be able to end two different ways, and the ending must be reported",
            ],
            "You are in the ATTIC.\n> go north\nYou cannot go north from the ATTIC.\n> search chest\nFound a rusty key!\n> go south\nYou are in the HALLWAY.\n> unlock door\nThe door opens. You win!",
            &["Add an inventory limit so carrying too much fails", "Add a second win condition and a way to lose deliberately"],
        ),
        SeedProject::new(
            "The Gauntlet",
            "Everything at once. A turn-based dungeon crawl with combat, loot, and a real state machine.",
            Difficulty::Advanced,
            "90 min",
            &[
                "At least four distinct locations connected by directional movement",
                "A combat system using a match on an enemy type, each with its own attack logic",
                "An inventory using a Vec, with add and use operations",
                "A parser that reads a line, splits it, and dispatches on the verb with a match",
                "Loot drops returned as an `Option` that you handle explicitly",
                "Random events per turn, rolled from a simple deterministic sequence",
                "Clear win and lose conditions, both reported with a reason",
                "A turn log that records what happened each turn",
            ],
            &[
                "if",
                "match",
                "loop",
                "while",
                "for",
                "ranges",
                "break",
                "continue",
                "Option",
                "enums",
                "expressions",
            ],
            &[
                "No `unwrap()` anywhere in the parser or the combat system",
                "Every `match` on an enum must be exhaustive with no catch-all where the variants are closed",
                "Functions must stay under roughly 40 lines each — if one is longer, it is doing two jobs",
            ],
            "TURN 4 — You are in the FORGE.\n> attack golem\nYou hit the golem for 8.\nThe golem hits you for 12.\nHealth: 63/100\n> search rubble\nYou find a shard of iron.\n> go north\nYou are in the ARMORY.\nInventory: iron shard, healing draught x2",
            &[
                "Add a save and load system that serialises state to a file",
                "Add a difficulty setting that scales enemy behaviour through a match",
                "Add a limited number of actions so the game must be completed efficiently",
            ],
        ),
    ]
}
