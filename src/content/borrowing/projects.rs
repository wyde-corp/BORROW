//! Module 06 — Borrowing projects.
//!
//! A curated subset of the spec's ten, chosen as a progression: read-only
//! borrowing, then mutable borrowing, then borrowed return values, then
//! borrowing through containers, and finally a program whose deliverable is the
//! justification of its own API.

use crate::content::types::{Difficulty, SeedProject};

pub fn projects() -> Vec<SeedProject> {
    vec![
        SeedProject::new(
            "Borrowing Playground",
            "Deliberately small. You are not building an application — you are building a laboratory in which every function borrows instead of owning, and recording what the compiler does.",
            Difficulty::Beginner,
            "30 min",
            &[
                "Write `fn inspect(text: &str)` that prints a string it does not own",
                "Write `fn shout(text: &mut String)` that appends `!` in place",
                "Call both functions from `main` on one owned `String`",
                "Print the string after each call to prove the caller kept ownership",
                "Add one deliberately broken call that passes `&s` where `&mut s` is needed, and write down the error code",
                "Write a one-line comment above each function stating what it borrows and why",
            ],
            &[
                "immutable borrowing",
                "mutable borrowing",
                "ownership remaining with the caller",
                "function parameters as design decisions",
            ],
            &[
                "No function may take a `String` by value",
                "No `.clone()` anywhere — the point is to avoid ownership transfer, not to copy around it",
                "Every call to a `&mut` function must be preceded by a comment saying what it will change",
                "The broken call must stay in the file, commented out, with the error code recorded",
            ],
            "fn inspect(text: &str) {\n    println!(\"inspecting: {text}\");\n}\n\nfn shout(text: &mut String) {\n    text.push('!');\n}\n\n// main keeps `message` the whole time:\n//   inspect(&message);\n//   shout(&mut message);\n//   println!(\"{message}\");",
            &[
                "Add `fn length(text: &str) -> usize` and confirm it works on both a `&String` and a literal",
                "Write a second version of `shout` that takes ownership, and record the cost at every call site",
                "Explain in a comment why `&str` is the better parameter for `inspect` than `&String`",
            ],
        ),

        SeedProject::new(
            "Mutable Text Editor",
            "A small text transformation tool. Every operation edits the caller's string in place through a mutable borrow, and the caller's ownership is never in question.",
            Difficulty::Intermediate,
            "45 min",
            &[
                "Write `fn append(text: &mut String, extra: &str)`",
                "Write `fn trim_trailing(text: &mut String)` that removes trailing whitespace",
                "Write `fn replace_char(text: &mut String, from: char, to: char)`",
                "Write `fn surround(text: &mut String, prefix: &str, suffix: &str)`",
                "Chain several operations on one owned `String` in `main`",
                "Print the string between operations to make each change visible",
            ],
            &[
                "`&mut String`",
                "sequential mutable borrows",
                "borrowing a value you also read",
                "ownership remaining with the caller",
            ],
            &[
                "No function may return a new `String` — the mutations happen through the borrow",
                "No function may take ownership of the string it edits",
                "Between two mutable operations you must not hold any other borrow of the string",
                "If a method needs both a read and a write, end the read first rather than cloning",
            ],
            "let mut text = String::from(\"  hello  \");\n\nappend(&mut text, \"world\");\ntrim_trailing(&mut text);\nsurround(&mut text, \"[\", \"]\");\n\nprintln!(\"{text}\"); // [hello world]",
            &[
                "Add `fn collapse_spaces(text: &mut String)` and handle the borrows without cloning",
                "Write a version of one function that takes `&str` and returns owned text, and compare the ergonomics",
                "Explain in a comment which of your functions could have taken `&str` and why it does not",
            ],
        ),

        SeedProject::new(
            "Word Finder",
            "A function that receives text and returns a borrowed slice representing a selected word. The return value is a view into the caller's data, and that is the whole exercise.",
            Difficulty::Intermediate,
            "45 min",
            &[
                "Write `fn nth_word(text: &str, n: usize) -> Option<&str>`",
                "Write `fn longest_word(text: &str) -> &str` returning a slice of the input",
                "Write `fn first_after(text: &str, marker: char) -> Option<&str>`",
                "Call each function on a `String` owned by `main` and print the slices",
                "Confirm the original string is still usable after every call",
                "Add one function that cannot return a slice, and explain in a comment why it returns `String` instead",
            ],
            &[
                "`&str`",
                "slices",
                "borrowed return values",
                "reference validity",
            ],
            &[
                "Returning functions must return a slice of the input, not a new `String`",
                "No lifetime annotations unless the compiler genuinely requires them",
                "No `.clone()` in the returning functions",
                "Slice only on boundaries produced by `find`, `split_whitespace`, or `char_indices`",
            ],
            "let text = String::from(\"Rust makes systems programming fun\");\n\nprintln!(\"{}\", nth_word(&text, 2).unwrap());   // systems\nprintln!(\"{}\", longest_word(&text));            // programming\nprintln!(\"{text}\");                            // still owned by main",
            &[
                "Make `longest_word` handle the empty string without panicking",
                "Add `fn words(text: &str) -> Vec<&str>` and explain why the vector owns its slices' lengths but not their bytes",
                "Write a paragraph explaining why the returned slice cannot outlive `text`",
            ],
        ),

        SeedProject::new(
            "Inventory Inspector",
            "An inventory held in a collection. Functions search, inspect, and modify items through borrows, and the design has to keep immutable and mutable access from colliding.",
            Difficulty::Advanced,
            "1.5 h",
            &[
                "Represent the inventory as a `Vec<(String, u32)>` of name and quantity",
                "Write `fn find<'a>(items: &'a [(String, u32)], name: &str) -> Option<&'a (String, u32)>`",
                "Write `fn restock(items: &mut [(String, u32)], name: &str, amount: u32) -> bool`",
                "Write `fn total_units(items: &[(String, u32)]) -> u32`",
                "Write a report function that borrows the inventory immutably and prints it",
                "Call a mutating function, then a reading one, without holding a borrow across them",
            ],
            &[
                "borrowing collection elements",
                "immutable versus mutable borrows",
                "avoiding conflicting borrows",
                "`&[T]` slices as borrowed views of a collection",
            ],
            &[
                "No function may take the `Vec` by value",
                "No `.clone()` of the inventory or its items",
                "Any line that holds an element reference may not also mutate the collection",
                "Return borrowed slices from search functions, owned values from measurements",
            ],
            "let mut stock = vec![\n    (String::from(\"bolt\"), 12),\n    (String::from(\"nut\"), 30),\n];\n\nrestock(&mut stock, \"bolt\", 8);\n\nprintln!(\"{}\", total_units(&stock)); // 50\n\nif let Some(item) = find(&stock, \"bolt\") {\n    println!(\"{}: {}\", item.0, item.1);\n}",
            &[
                "Change the inventory to a `HashMap<String, u32>` and re-solve the borrow conflicts",
                "Add `fn rename(items: &mut [...], from: &str, to: &str) -> bool` without cloning names",
                "Explain in a comment why `find` can return a reference while `total_units` returns a `u32`",
            ],
        ),

        SeedProject::new(
            "Borrowing Lab",
            "The capstone. One small program that uses owned text, borrowed views, mutable borrows, slices, and a collection — without cloning or moving anything it does not have to. The deliverable includes a written justification for every major function's signature.",
            Difficulty::Advanced,
            "2 h",
            &[
                "Build a program that reads a text document into a `String`",
                "Write functions that inspect it via `&str` and borrow slices of it",
                "Write functions that modify it via `&mut String`",
                "Store derived data (word counts) in a collection without cloning the text",
                "Include at least one function that returns a borrowed slice and one that returns an owned value, and justify each",
                "For every major function, write a comment: does it take ownership, `&T`, or `&mut T`, and why is that right here?",
            ],
            &[
                "owned `String` versus `&str`",
                "immutable and mutable borrowing",
                "slices",
                "collection borrowing",
                "functions returning references and owned values",
                "API design justification",
            ],
            &[
                "No `.clone()` unless the comment above it justifies a second owner in one sentence",
                "No `.to_string()` or `.to_owned()` to sidestep a borrow error",
                "At no point may two conflicting borrows be live at once",
                "Every function must have a comment above it stating its ownership choice and the reason",
            ],
            "// read\nfn word_count(text: &str) -> usize\n\n// borrow a slice\nfn longest_word(text: &str) -> &str\n\n// mutate in place\nfn normalise(text: &mut String)\n\n// own the result\nfn summary(text: &str) -> String\n\n// The comment above each answers: why this parameter type?",
            &[
                "Add a function that returns an iterator over borrowed words and explain what it borrows",
                "Refactor one owning function into a borrowing one and record what changed at the call sites",
                "Write a short design note: list every function, its signature, and the one question that decided it",
            ],
        ),
    ]
}
