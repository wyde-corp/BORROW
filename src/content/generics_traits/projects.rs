//! Module 10 — Generics & Traits projects.
//!
//! Six projects escalating from deliberate repetition to the Media Library
//! capstone. Each asks for a bound or a dispatch decision the learner can
//! justify.

use crate::content::types::{Difficulty, SeedProject};

pub fn projects() -> Vec<SeedProject> {
    vec![
        SeedProject::new(
            "Generic Playground",
            "Deliberately repetitive. Write one generic function and run it on several concrete types, and record what the compiler generated.",
            Difficulty::Beginner,
            "30 min",
            &[
                "Write `fn largest<T: PartialOrd + Copy>(list: &[T]) -> T`",
                "Write a matching `smallest`",
                "Run both on `i32`, `f64`, and `char` slices",
                "Remove a bound and record the error the compiler gives",
                "Add the bound back and confirm it compiles",
                "Write one sentence on what each bound was for",
            ],
            &[
                "generic functions",
                "trait bounds",
                "reading 'trait bound not satisfied'",
                "monomorphisation",
            ],
            &[
                "One generic function, not one per type",
                "Both bounds (`PartialOrd` and `Copy`) are required — explain each",
                "Do not use `dyn` here",
                "The sentence on each bound is required",
            ],
            "fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {\n    let mut largest = list[0];\n    for &item in list {\n        if item > largest { largest = item; }\n    }\n    largest\n}\n\nprintln!(\"{}\", largest(&[3, 9, 2]));",
            &[
                "Write `fn largest_by<T, F: Fn(&T, &T) -> bool>(list: &[T], cmp: F) -> &T`",
                "Explain in a comment why `char` works but `String` would not",
            ],
        ),

        SeedProject::new(
            "Summarise Them All",
            "The `Summary` trait implemented for three media types, and called through one generic function. The trait side of the module.",
            Difficulty::Beginner,
            "45 min",
            &[
                "Define `trait Summary { fn summarise(&self) -> String; }`",
                "Define `Book`, `Article`, and `Video`",
                "Implement `Summary` for each",
                "Write `fn notify<T: Summary>(item: &T)` that prints the summary",
                "Call it on one of each type",
                "Add a default method `short(&self)` that returns a trimmed summary",
            ],
            &[
                "defining and implementing traits",
                "trait bounds",
                "default methods",
                "static dispatch",
            ],
            &[
                "One `notify`, not three",
                "Each type gets its own `impl Summary`",
                "`short` has a default body and is not re-implemented per type",
                "Comment each type's summary format",
            ],
            "fn notify<T: Summary>(item: &T) {\n    println!(\"{}\", item.summarise());\n}\n\nnotify(&book);\nnotify(&video);",
            &[
                "Override `short` for one type and explain why",
                "Add a fourth type and confirm `notify` needs no change",
            ],
        ),

        SeedProject::new(
            "Derive and Compare",
            "`#[derive]` on the media types, plus one manual impl where the default is wrong. Equality, cloning, ordering, and printing.",
            Difficulty::Intermediate,
            "1 h",
            &[
                "Derive `Debug`, `Clone`, and `PartialEq` on `Book`",
                "Clone a book and compare it to the original",
                "Implement `Display` by hand with a chosen format",
                "Write a manual `PartialEq` for a second type that compares only one field",
                "Show the difference between the derived and manual equality",
                "Say which derives are harmless and which are a decision",
            ],
            &[
                "common derives",
                "implementing versus deriving",
                "`Display` versus `Debug`",
                "when the default is wrong",
            ],
            &[
                "`Display` must be a manual impl",
                "The manual `PartialEq` must differ from the derived one, visibly",
                "No `Copy` on a type with a `String`",
                "A comment per derive saying what it gives",
            ],
            "#[derive(Debug, Clone, PartialEq)]\nstruct Book { title: String, pages: u32 }\n\nimpl std::fmt::Display for Book {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        write!(f, \"{} ({} pages)\", self.title, self.pages)\n    }\n}",
            &[
                "Derive `Hash` on a type and use it as a `HashMap` key",
                "Write a manual `PartialOrd` that sorts by title length and explain the choice",
            ],
        ),

        SeedProject::new(
            "A Generic Library",
            "A `Library<T: Summary>` with a whole set of methods, and a bound that is neither too tight nor too loose.",
            Difficulty::Intermediate,
            "1 h",
            &[
                "Define `struct Library<T> { items: Vec<T> }`",
                "Bound the `impl` block so every method may call `summarise`",
                "Write `new`, `add`, `len`, and `summaries`",
                "Write `longest<T: Summary>(items: &[T]) -> Option<&T>`",
                "Use the library with `Book` and with `Video`",
                "Justify the bound on each `impl` block and each function",
            ],
            &[
                "bounds on `impl` blocks",
                "generic functions",
                "returning references",
                "choosing the smallest bound",
            ],
            &[
                "Do not clone items to return them — borrow",
                "No method may require an ability it does not use",
                "The library must work for at least two media types",
                "Each bound gets a one-sentence justification",
            ],
            "struct Library<T: Summary> { items: Vec<T> }\n\nimpl<T: Summary> Library<T> {\n    fn summaries(&self) -> Vec<String> {\n        self.items.iter().map(|i| i.summarise()).collect()\n    }\n}",
            &[
                "Add a method that transforms items with a caller-supplied closure",
                "Add a method returning an iterator over borrowed items (Module 11 preview)",
            ],
        ),

        SeedProject::new(
            "Mixed Shelves",
            "A heterogeneous collection with `Box<dyn Summary>`, plus `Display` and a report. The dynamic-dispatch side of the module.",
            Difficulty::Advanced,
            "1.5 h",
            &[
                "Build a `Vec<Box<dyn Summary>>` with at least three media types",
                "Iterate and print each summary",
                "Add a method that counts items and one that finds by a trait method",
                "Implement `Display` for one media type and use it in the report",
                "Add a fourth type and confirm the iteration code does not change",
                "Say in a comment why the collection must be dynamic",
            ],
            &[
                "trait objects",
                "`Box<dyn Trait>`",
                "dynamic dispatch",
                "heterogeneous collections",
            ],
            &[
                "The collection must hold different concrete types",
                "No `Vec<T>` may replace it without losing a type",
                "Adding a type must not change the loop",
                "The comment on why it is dynamic is required",
            ],
            "let shelf: Vec<Box<dyn Summary>> = vec![\n    Box::new(Book { title: String::from(\"Dune\") }),\n    Box::new(Video { title: String::from(\"Rust intro\") }),\n];\n\nfor item in &shelf {\n    println!(\"{}\", item.summarise());\n}",
            &[
                "Sort the shelf by a trait method and explain what you needed to add",
                "Compare the binary's behaviour when the same items are stored as generics",
            ],
        ),

        SeedProject::new(
            "Media Library",
            "The capstone: the library built across the module, finished and justified. Generics, traits, bounds, derives, and both dispatch forms, with a written defence of every choice.",
            Difficulty::Advanced,
            "3 h",
            &[
                "A `Summary` trait with a default method, implemented by at least three media types",
                "A generic `Library<T: Summary>` with add, search, and a summary report",
                "A mixed `Vec<Box<dyn Summary>>` shelf",
                "One hand-written `Display` and one hand-written `From`",
                "A bounded function that demonstrates static dispatch",
                "A note: for every trait, say derive or implement; for every generic function and every boxed trait object, say why that dispatch form",
            ],
            &[
                "traits and bounds",
                "derives versus manual impls",
                "static versus dynamic dispatch",
                "`Display` and `From`",
                "API design and justification",
            ],
            &[
                "No bound that the body does not use",
                "No `Display` via derive",
                "The mixed shelf and the generic library must both appear",
                "The written note is required — working code alone is not the deliverable",
            ],
            "trait Summary { fn summarise(&self) -> String; }\n\nstruct Library<T: Summary> { items: Vec<T> }\n\nfn notify<T: Summary>(item: &T) { println!(\"{}\", item.summarise()); }\n\nlet mixed: Vec<Box<dyn Summary>> = vec![Box::new(book), Box::new(video)];",
            &[
                "Add an `Article` type and route it through every path",
                "Replace one `Box<dyn Summary>` with a generic and record what you lost",
                "Write the justification note a second time, after the code, and compare",
            ],
        ),
    ]
}
