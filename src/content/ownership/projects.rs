//! Module 05 — Ownership projects.
//!
//! Ten projects, escalating from a deliberate experiment to a small program the
//! learner designs themselves. Every one of them requires the learner to make
//! an ownership decision on paper before writing code, because a project that
//! merely compiles teaches nothing about ownership.

use crate::content::types::{Difficulty, SeedProject};

pub fn projects() -> Vec<SeedProject> {
    vec![
        SeedProject::new(
            "String Ownership Playground",
            "Deliberately small, deliberately repetitive. You are not building anything — you are building a laboratory and running experiments in it.",
            Difficulty::Beginner,
            "30 min",
            &[
                "Write at least 12 tiny programs, each six lines or fewer, isolated so each fails on its own",
                "Six of them must move: create a String, move it, then try to use the original",
                "Three must clone: create a String, clone it, and mutate one of the two to show they are independent",
                "Three must copy: the same experiment with an i32 instead of a String",
                "For every program, write a comment with your prediction BEFORE running it, and the actual result after",
                "Print `as_ptr()` before and after a move, and record whether the address changed",
                "Keep a summary at the top of the file: one sentence per outcome, in your own words",
            ],
            &[
                "move",
                "Copy types",
                "clone",
                "use-after-move",
                "stack versus heap",
                "prediction before execution",
            ],
            &[
                "Every experiment must be its own file or its own commented-out block — a single run-on program proves nothing",
                "You may not use references anywhere in this project. That is the point: use ownership until it hurts",
                "Your predictions must be written before you run. Fixing them afterwards is not the exercise",
                "No `unsafe`, no `Box::leak`, no `mem::forget` — you are here to see what normal ownership does",
            ],
            "── experiment 3: clone independence ──\n\nlet a = String::from(\"shared\");\nlet b = a.clone();\n\n// both print: shared\n// a and b are EQUAL but not IDENTICAL\n// mutating one cannot touch the other\n// two buffers, two owners, two cleanups",
            &[
                "Add an experiment where the move and the clone produce the same printed output — and explain why that is misleading",
                "Write a small `Drop` impl that announces itself, and use it to show which owner frees what",
                "Find one program from this module you predicted wrong and write the corrected mental model at the bottom of the file",
            ],
        ),

        SeedProject::new(
            "Personal Greeting Generator",
            "The smallest project that forces real ownership decisions. Every function must be justified, and the justifications are the deliverable.",
            Difficulty::Beginner,
            "40 min",
            &[
                "Write `fn make_greeting(name: String) -> String` that returns a greeting containing the name",
                "Write `fn make_greeting_cloned(name: String) -> String` that does the same but has the caller keep the name too",
                "Write `fn greeting_length(greeting: String) -> usize` that only measures the string and gives it back",
                "Call each function in a `main` that then prints the name afterwards where possible",
                "For each function, write a comment stating: does it take ownership or borrow, and why is that the right choice here?",
                "Write a short note naming which of your functions would be different in real code, and what you would change",
            ],
            &[
                "ownership transfer into a function",
                "return values moving ownership back out",
                "clone versus move",
                "function signatures as design decisions",
                "explaining trade-offs",
            ],
            &[
                "No function may take `&str` or `&String` — decide between taking ownership and cloning, deliberately, in every case",
                "No function may both mutate its argument and return it",
                "Every function must have a comment explaining its ownership choice before its body",
                "If you cannot justify a clone in one sentence, remove it and find another way",
            ],
            "fn make_greeting(name: String) -> String {\n    // takes ownership: the name is not needed\n    // by the caller afterwards, and copying the\n    // text into a new String is happening anyway\n    format!(\"Hello, {name}!\")\n}\n\nfn make_greeting_cloned(name: String) -> String {\n    // clone: the caller still wants the name, so we\n    // pay for a second buffer rather than a move\n    let owned = name.clone();\n    format!(\"Hello, {owned}!\")\n}",
            &[
                "Add a `fn shout(name: String) -> String` that returns an uppercased version, and trace the original buffer's fate",
                "Build a version that handles several names from a `Vec<String>` and work out what the loop does to ownership",
                "Rewrite one function to take a reference and write a comment on what that cost you in readability",
            ],
        ),

        SeedProject::new(
            "Shopping List",
            "A `Vec<String>` forces the ownership rules out into the open, because collections are where beginners first meet move, clone, and borrow by accident.",
            Difficulty::Beginner,
            "45 min",
            &[
                "Build a shopping list as a `Vec<String>` and a `main` that adds at least four items",
                "Write `fn add_item(list: &mut Vec<String>, item: String)` that takes a new item and pushes it",
                "Write `fn remove_item(list: &mut Vec<String>, name: &str) -> bool` that removes by name and reports success",
                "Write `fn contains(list: &[String], name: &str) -> bool` that checks without modifying anything",
                "Write a function that takes a `Vec<String>` by value and returns it sorted, and call it twice to see what happens the second time",
                "For every function, comment whether it takes ownership, borrows mutably, or borrows immutably, and say why that is the minimum it needs",
            ],
            &[
                "Vec as an owner of heap data",
                "&mut for functions that modify",
                "& for functions that only read",
                "moving a collection into a function",
                "choosing the weakest sufficient borrow",
            ],
            &[
                "The add and remove functions may NOT take the `Vec` by value — a function that modifies a collection should not consume it",
                "The contains function may not take `&mut` — read-only is a real requirement, not a default",
                "The sorting function takes ownership and returns the value, and you must call it exactly once",
                "No `clone()` in `add_item` or `remove_item` without a comment explaining why",
            ],
            "── add ──\n> add milk\n  added\n\n── list ──\n  milk\n  bread\n  eggs\n\n── remove ──\n> remove milk\n  removed\n> remove milk\n  not found\n\n── contains ──\n> contains bread\n  true",
            &[
                "Add a `find_all` that returns every matching item as a new `Vec<String>` — and work out whether it should borrow or own",
                "Add a `dedup` that removes duplicates, and say why it can be done without cloning",
                "Track a `Vec<String>` through a chain of four functions and write down who owns it after each one",
            ],
        ),

        SeedProject::new(
            "Username Manager",
            "Ownership shows up in a realistic shape here: values that must be validated, stored, and returned, where the wrong choice quietly costs a copy per user.",
            Difficulty::Intermediate,
            "55 min",
            &[
                "Store usernames in a `Vec<String>` and provide add, remove, and lookup",
                "Write `fn validate(name: &str) -> Result<String, String>` that rejects empty, too-long, and whitespace-only names, and returns a *new owned* String on success",
                "Write `fn normalise(name: String) -> String` that trims and lowercases, consuming the input",
                "Write a lookup that returns `Option<String>` — an owned copy — and comment on whether that copy is justified",
                "Print the total number of allocations your program performs, counting clones and `normalise` calls, and comment on the biggest one",
                "Write a paragraph explaining where your program copies data unnecessarily, if anywhere",
            ],
            &[
                "owned returns",
                "Result and Option as ownership-aware types",
                "clone as a deliberate cost",
                "counting allocations",
                "validating before storing",
            ],
            &[
                "`validate` must not take a `String` — it only inspects, so taking ownership would be a bug",
                "`normalise` must take a `String` and must not clone — it is replacing the value, not duplicating it",
                "Every allocation in your program must be either necessary or justified in a comment",
                "No `unwrap()` outside a `match` you have already handled",
            ],
            "── add ──\n> add   GUS_  \n  invalid: whitespace not allowed\n> add gus\n  added\n> add Gus\n  already taken\n\n── list ──\n  gus\n  borrowck\n\n── lookup ──\n> lookup gus\n  found: gus",
            &[
                "Make lookup return `Option<&str>` instead and compare the two versions' allocation counts",
                "Add a rename that replaces a username in place and account for what happens to the old String",
                "Add a case-insensitive duplicate check that does not allocate",
            ],
        ),

        SeedProject::new(
            "Message Processor",
            "A pipeline where each stage takes ownership and hands it on. The interesting work is deciding which stages genuinely need to consume their input.",
            Difficulty::Intermediate,
            "60 min",
            &[
                "Build a pipeline: `raw: String` → `clean` → `tag` → `shout` → final message",
                "Each stage is a function taking a `String` and returning a `String`",
                "At least two stages must *genuinely* need ownership, and you must write a comment saying why",
                "At least one stage must be rewritten to take `&str` and return a new `String` instead — comment on what that changed",
                "Print the message after every stage so the output shows the transformation at each step",
                "Add a `fn process_all(messages: Vec<String>) -> Vec<String>` that runs the whole pipeline over a batch",
            ],
            &[
                "ownership travelling through a chain",
                "choosing move versus borrow per stage",
                "Vec of owned Strings",
                "iterating without destroying the collection",
                "justifying each design choice",
            ],
            &[
                "No stage may clone unless a comment explains why that stage must keep the original",
                "`process_all` may not lose the input messages — the caller must still be able to use them",
                "Every function signature must be the weakest one that works: `String` only when ownership is genuinely needed",
                "The final stage's output must be usable after the pipeline has run",
            ],
            "── pipeline ──\n  raw:    \"  hello there  \"\n  clean:  \"hello there\"\n  tag:    \"[hello there]\"\n  shout:  \"[HELLO THERE]\"\n\n── batch ──\n  processed 3 of 3\n  first result: [HELLO THERE]",
            &[
                "Make the pipeline return a `Vec<String>` of intermediate stages so you can inspect each one",
                "Add a stage that can fail, and decide what happens to the input when it does",
                "Rewrite the whole pipeline to take and return `&str`, then write a comment on where the ownership went instead",
            ],
        ),

        SeedProject::new(
            "Inventory Manager",
            "Ownership at volume. The same operations as the shopping list, but across many items and many functions, where a careless clone becomes a real cost.",
            Difficulty::Intermediate,
            "65 min",
            &[
                "Model stock as a `Vec<(String, u32)>` — name and quantity",
                "Write `fn add_item(stock: &mut Vec<(String, u32)>, name: String, qty: u32)` that adds or increments",
                "Write `fn remove_expired(stock: &mut Vec<(String, u32)>) -> Vec<String>` that removes and returns the names removed",
                "Write `fn restock(stock: Vec<(String, u32)>, name: String) -> Vec<(String, u32)>` that takes ownership and returns it",
                "Write a function that produces a report of all item names as a `Vec<String>` and accounts for every allocation it makes",
                "Comment on which of your functions could take `&` instead, and what that would have saved",
            ],
            &[
                "Vec as owner of many owned values",
                "&mut for modification",
                "returning owned collections",
                "consuming a collection and giving it back",
                "allocation accounting",
            ],
            &[
                "`restock` takes the whole `Vec` by value — trace who owns it at every line of that function",
                "No function may `clone()` the entire `Vec` to modify one element",
                "`remove_expired` returns owned Strings, so it must produce them by taking them out, not by copying",
                "The report function must state its allocation count in a comment",
            ],
            "── stock ──\n> add bolts 40\n  bolts: 40\n> add nuts 12\n  nuts: 12\n> add rope 5\n  rope: 5\n\n── restock rope 20 ──\n  rope: 25\n\n── report ──\n  bolts  40\n  nuts   12\n  rope   25\n  3 items, 2 allocations to build the report",
            &[
                "Add a function that merges two stock vectors, and account for what it does to the inputs",
                "Add a `Vec<Vec<String>>` and explain why a clone of it copies far more than its length suggests",
                "Rewrite `restock` to take `&mut` and write a comment explaining what you gave up",
            ],
        ),

        SeedProject::new(
            "Simple Text Processor",
            "Read, transform, and return owned strings. Smaller than the others on purpose — the goal is depth of reasoning, not volume of code.",
            Difficulty::Intermediate,
            "60 min",
            &[
                "Read a line of text into an owned `String` and process it through at least five functions",
                "Write `fn word_count(text: &str) -> usize`, `fn longest_word(text: &str) -> Option<&str>`, and `fn title_case(text: &str) -> String`",
                "Write one more function that takes a `String` by value and returns a `String`, and comment on why it is better as `&str`",
                "Add `fn summarize(text: String) -> String` that consumes the text and returns a shortened version, and trace what happens to the original",
                "Every function that returns a String must have a comment saying whether it allocated, borrowed, or moved",
                "Write a closing comment counting every allocation your program makes and naming the largest",
            ],
            &[
                "&str for reading",
                "String for producing",
                "consuming a function as a deliberate choice",
                "Option for possibly-absent results",
                "tracing an owned value to its drop",
            ],
            &[
                "`word_count` and `longest_word` must not allocate — they take `&str` and return counts or slices",
                "No function may both read its input and take ownership of it",
                "`summarize` must genuinely consume its input — if you clone to keep it, comment why and ask whether it was necessary",
                "Your allocation count must be in a comment, and it must be right",
            ],
            "── input ──\n  \"the quick brown fox jumps over the lazy dog\"\n\n── results ──\n  words:  9\n  longest: quick (5)\n  title:  The Quick Brown Fox Jumps Over The Lazy Dog\n  summary: the quick brown fox jumps over ...\n\n  allocations: 3 — two Strings built, one summary String",
            &[
                "Add `fn dedup_words(text: &str) -> String` and account for its allocations",
                "Add a function that returns the longest word *owned* rather than borrowed, and compare the two",
                "Make the whole program run on several lines from stdin, and comment on what that does to your ownership accounting",
            ],
        ),

        SeedProject::new(
            "Mini Library Manager",
            "Items that are created, moved between collections, and returned. This is where a real program would use structs — build it with tuples and note what hurts.",
            Difficulty::Advanced,
            "75 min",
            &[
                "Model the catalogue as a `Vec<String>` of titles and a separate `Vec<u32>` of copy counts",
                "Write `fn add_book(titles: &mut Vec<String>, counts: &mut Vec<u32>, title: String, copies: u32)`",
                "Write `fn remove_book(titles: &mut Vec<String>, counts: &mut Vec<u32>, title: &str) -> Option<u32>` that returns the count it removed",
                "Write `fn transfer(titles: Vec<String>, counts: Vec<u32>) -> (Vec<String>, Vec<u32>)` that consumes both and returns both",
                "Write `fn checkout(titles: &mut Vec<String>, counts: &mut Vec<u32>, title: &str) -> bool` that decrements a count",
                "Write a comment block naming every place your two parallel vectors could get out of sync, and how ownership does or does not help",
            ],
            &[
                "parallel collections and the bugs they invite",
                "consuming and returning collections",
                "Option for a failed removal",
                "&mut vs by-value trade-offs",
                "design consequences, not just compilation",
            ],
            &[
                "No struct definitions — use tuples, so you can feel what a struct would have solved",
                "`transfer` must take both vectors by value and return both, with no clone anywhere in it",
                "The comment block about desynchronisation is a required deliverable, not an optional extra",
                "`checkout` must not allocate — it modifies in place or it does not belong here",
            ],
            "── catalogue ──\n  \"Dune\"          x3\n  \"Neuromancer\"  x1\n  \"Blindsight\"   x2\n\n── checkout ──\n> checkout Dune\n  ok, 2 remaining\n> checkout Dune\n  ok, 1 remaining\n\n── remove ──\n> remove Neuromancer\n  removed 1 copy\n> remove Missing\n  not found",
            &[
                "Now rewrite it with a struct and write a comment on which ownership problems the struct solved and which it did not",
                "Add a `transfer` that merges two catalogues and account for what it does to both inputs",
                "Add a checkout queue as a `Vec<String>` and trace ownership as books move into and out of it",
            ],
        ),

        SeedProject::new(
            "Command-Line Data Pipeline",
            "Owned data flowing through several stages, which is the closest thing in this module to how a real program is shaped.",
            Difficulty::Advanced,
            "85 min",
            &[
                "Read lines from stdin into a `Vec<String>`",
                "Write a pipeline of at least four stages, each a function, moving ownership down the chain",
                "At least one stage must be able to fail, and you must decide what happens to the input when it does",
                "Write `fn run_all(items: Vec<String>) -> (Vec<String>, Vec<String>)` returning successes and failures separately",
                "Print a final summary with counts, and comment on the total number of allocations your pipeline performs",
                "Add a `--verbose` style flag that makes each stage report the value it received, and comment on the cost of the reporting itself",
            ],
            &[
                "ownership travelling through a multi-stage pipeline",
                "partial failure and where ownership goes",
                "partitioning owned data",
                "Vec lifecycle from input to output",
                "measuring what your program actually costs",
            ],
            &[
                "No stage may clone the input unless it must keep it after producing output — and if it must, say why",
                "On failure, the failed value must be either returned or explicitly dropped with a comment saying which",
                "`run_all` may not clone the input vector",
                "The allocation summary must be a comment, and it must be accurate",
            ],
            "── BORROW_ pipeline ──\n> run\n\n  [1/4] read       3 items\n  [2/4] validate   2 ok, 1 rejected\n  [3/4] transform  2 transformed\n  [4/4] emit       2 written\n\n  summary: 2 succeeded, 1 failed, 4 allocations\n  > run --verbose\n  [2/4] validate  \"  \" rejected: empty after trim",
            &[
                "Add a stage that reorders the pipeline's output and comment on what that does to ownership",
                "Make the pipeline work on `Vec<Vec<String>>` and compare the allocation counts",
                "Add a stage that can fail after some values have already been emitted, and write a comment on why that is the hard case",
            ],
        ),

        SeedProject::new(
            "Ownership Lab",
            "The capstone. You choose the program, and you must be able to defend every ownership decision in it. Working code is the entry requirement, not the goal.",
            Difficulty::Advanced,
            "90 min",
            &[
                "Build a small program of your choosing that genuinely uses `String`, `Vec`, and at least one function of your own design",
                "Before writing code, sketch the data flow: what is created, who owns it, and where it goes",
                "Include at least three functions and at least one that returns an owned value",
                "For every function, write a comment answering: why does this take ownership rather than borrow?",
                "For every `.clone()` in your program, write a comment answering: what am I duplicating, and is the cost acceptable?",
                "For every owned value, note in a comment where it is dropped",
                "Write a closing section titled 'Ownership decisions' with at least five paragraphs, one per design choice",
                "State plainly which decisions you would change now that you understand borrowing — the next module — and what each change would cost",
            ],
            &[
                "deliberate ownership design",
                "justifying move versus clone versus borrow",
                "documenting drop points",
                "identifying your own unnecessary copies",
                "anticipating borrowing before it arrives",
            ],
            &[
                "No clone may go uncommented. If you cannot explain one, it is either unnecessary or you do not understand it yet — both are worth knowing",
                "No `unsafe`. No `mem::forget`. No `Box::leak` to dodge a lifetime problem",
                "At least one function must take ownership for a real reason, not by accident — and the comment must say what the reason is",
                "The program must work. A program that is philosophically pure and does not run has failed the brief",
                "Do not use references to dodge the work. This project is about ownership, and borrowing is the next module",
            ],
            "── my_program ──\n> add widget\n  added \"widget\" (allocation 2)\n> add gadget\n  added \"gadget\" (allocation 4)\n> list\n  widget\n  gadget\n> remove widget\n  removed — its String was dropped here\n\n  ownership decisions:\n    add_item takes String by value: the caller has no\n      further use for the name, and we push it directly\n      rather than copying it\n    list returns Vec<String> because the caller needs to\n      own the names if it wants to keep them past the list\n    remove_item returns the removed String by value so the\n      caller can decide what to do with it",
            &[
                "Rewrite one function to take `&str` and note what that cost in clarity",
                "Add a second representation of the same data and compare how ownership behaves in each",
                "Write the 'what I would change with borrowing' section for real, and keep it — it is your bridge into module 06",
            ],
        ),
    ]
}
