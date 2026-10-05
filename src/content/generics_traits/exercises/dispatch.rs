//! Module 10 — Generics & Traits, exercises for lessons 19–23.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L19 impl Trait Parameters
        SeedExercise::new(
            Some(18),
            ExerciseKind::FillGap,
            "Fill the gap with the shorthand for 'any type that implements `Summary`'.",
            &["impl Summary", "dyn Summary", "&Summary"],
            0,
            "`&impl Summary` is shorthand for a generic parameter bounded by `Summary`. `dyn Summary` is \
             a trait object (dynamic dispatch); `&Summary` alone is not valid because a bare trait is \
             not a type.",
        )
        .with_code("fn notify(item: &____) {\n    println!(\"{}\", item.summarise());\n}"),

        // L20 The Mixed Shelf Problem
        SeedExercise::new(
            Some(19),
            ExerciseKind::PredictCompile,
            "A `Book` is pushed onto a `Vec<Book>`. Predict the result.",
            &[
                "It compiles — a `Book` is a valid element",
                "It does not compile — the vector holds `Book`s, and only a `Book` may be pushed",
            ],
            1,
            "A `Book` is the correct element type, so it compiles. Push a `Video` instead and it would \
             fail: `Vec<Book>` holds only `Book`s. Mixed types need `Vec<Box<dyn Trait>>`.",
        )
        .with_code(
            "struct Book { title: String }\n\nfn main() {\n    let mut shelf: Vec<Book> = Vec::new();\n    shelf.push(Book { title: String::from(\"Dune\") });\n}",
        ),

        // L21 dyn Trait
        SeedExercise::new(
            Some(20),
            ExerciseKind::PredictOutput,
            "A mixed shelf of boxed trait objects is iterated. Predict the output.",
            &[
                "book: Dune / video: Rust intro",
                "It does not compile — two different types in one `Vec`",
                "The first item twice",
            ],
            0,
            "`Vec<Box<dyn Summary>>` has a single element type — the boxed trait object — so both types \
             coexist. Each `summarise` call dispatches to the right implementation at run time, giving \
             both summaries.",
        )
        .with_code(
            "trait Summary { fn summarise(&self) -> String; }\nstruct Book { title: String }\nstruct Video { title: String }\nimpl Summary for Book { fn summarise(&self) -> String { format!(\"book: {}\", self.title) } }\nimpl Summary for Video { fn summarise(&self) -> String { format!(\"video: {}\", self.title) } }\n\nfn main() {\n    let shelf: Vec<Box<dyn Summary>> = vec![\n        Box::new(Book { title: String::from(\"Dune\") }),\n        Box::new(Video { title: String::from(\"Rust intro\") }),\n    ];\n    for item in &shelf {\n        println!(\"{}\", item.summarise());\n    }\n}",
        ),

        // L22 Static vs Dynamic Dispatch
        SeedExercise::new(
            Some(21),
            ExerciseKind::Mcq,
            "What is the difference between `fn f<T: Summary>(x: &T)` and `fn f(x: &dyn Summary)`?",
            &[
                "None — they are identical",
                "The first resolves the call at compile time; the second looks it up at run time",
                "The second is always faster",
            ],
            1,
            "The generic form is monomorphised: the type is known and the call is direct. The `dyn` \
             form holds a trait object and looks the method up in a table at run time. The second can \
             hold mixed types; the first cannot.",
        ),

        // L23 Choosing
        SeedExercise::new(
            Some(22),
            ExerciseKind::ApiDesign,
            "You need a `Vec` that holds both `Book`s and `Video`s and calls `summarise` on each. Which \
             form?",
            &[
                "`Vec<T>` with `T: Summary`",
                "`Vec<Box<dyn Summary>>`",
                "`Vec<Book>` and a separate `Vec<Video>`",
            ],
            1,
            "The elements are different concrete types, so a single `Vec<T>` cannot hold them. \
             `Vec<Box<dyn Summary>>` erases the concrete type and stores the shared behaviour, which is \
             exactly the mixed-shelf case.",
        ),
    ]
}
