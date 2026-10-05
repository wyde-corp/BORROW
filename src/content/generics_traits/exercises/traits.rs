//! Module 10 — Generics & Traits, exercises for lessons 8–15.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L8 The Missing Shared Behaviour
        SeedExercise::new(
            Some(7),
            ExerciseKind::Mcq,
            "`Book` and `Video` share no fields but can both be summarised. What names that shared \
             thing?",
            &[
                "A generic type",
                "A trait",
                "An enum",
            ],
            1,
            "Generics need a shared shape or the same type parameter; an enum needs one type that is \
             'one of' the others. What `Book` and `Video` share is a behaviour, so the right tool is a \
             trait.",
        ),

        // L9 Defining a Trait
        SeedExercise::new(
            Some(8),
            ExerciseKind::FillGap,
            "Fill in the keyword that defines a set of required methods.",
            &["trait", "impl", "struct"],
            0,
            "`trait` introduces a contract of methods. `impl` provides those methods for a type. \
             `struct` defines data.",
        )
        .with_code("____ Summary {\n    fn summarise(&self) -> String;\n}"),

        // L10 Implementing a Trait
        SeedExercise::new(
            Some(9),
            ExerciseKind::FillGap,
            "Fill in the two keywords that connect the trait to the type.",
            &["impl … for", "fn … where", "trait … impl"],
            0,
            "`impl Summary for Book { ... }` — `impl` introduces the implementation and `for` names the \
             type it is for.",
        )
        .with_code("____ Summary ____ Book {\n    fn summarise(&self) -> String {\n        self.title.clone()\n    }\n}"),

        // L11 Calling Trait Methods
        SeedExercise::new(
            Some(10),
            ExerciseKind::PredictCompile,
            "The trait is defined in a module that is not imported at the call site. Predict the result.",
            &[
                "It compiles — the impl makes the method available",
                "It does not compile — the trait is not in scope to call the method",
            ],
            1,
            "Dot-call of a trait method requires the trait itself to be in scope. The impl exists on the \
             type, but the call site cannot see `Summary`, so the method is 'not found'. A `use` of the \
             trait fixes it.",
        ),

        // L12 Default Methods
        SeedExercise::new(
            Some(11),
            ExerciseKind::PredictOutput,
            "The type implements only `summarise`; `author` has a default. Predict what prints.",
            &["unknown", "It does not compile — `author` is missing"],
            0,
            "The default body of `author` is used because the type did not override it. The type is \
             complete: a default method is provided, not required.",
        )
        .with_code(
            "trait Summary {\n    fn summarise(&self) -> String;\n    fn author(&self) -> String { String::from(\"unknown\") }\n}\n\nstruct Book { title: String }\nimpl Summary for Book {\n    fn summarise(&self) -> String { self.title.clone() }\n}\n\nfn main() {\n    let b = Book { title: String::from(\"Dune\") };\n    println!(\"{}\", b.author());\n}",
        ),

        // L13 Trait Bounds
        SeedExercise::new(
            Some(12),
            ExerciseKind::FillGap,
            "Fill the gap with the bound the body needs in order to call `summarise`.",
            &["Summary", "Clone", "Display"],
            0,
            "The body calls `item.summarise()`, so `T` must implement `Summary`. Without the bound the \
             compiler cannot know `T` has that method.",
        )
        .with_code("fn notify<T: ____>(item: &T) {\n    println!(\"{}\", item.summarise());\n}"),

        // L14 Bounds on impl Blocks
        SeedExercise::new(
            Some(13),
            ExerciseKind::PredictCompile,
            "A `Library<i32>` is built and a method is called. Predict the result.",
            &[
                "It compiles — `i32` is a type",
                "It does not compile — the `impl` is bounded to `T: Summary`, and `i32` is not",
            ],
            1,
            "`Library<i32>` can exist as a type, but the methods live in `impl<T: Summary> Library<T>`, \
             and `i32` does not implement `Summary`. The call fails with a trait-bound error.",
        )
        .with_code(
            "trait Summary { fn summarise(&self) -> String; }\n\nstruct Library<T> { items: Vec<T> }\n\nimpl<T: Summary> Library<T> {\n    fn count(&self) -> usize { self.items.len() }\n}\n\nfn main() {\n    let lib = Library { items: vec![1, 2, 3] };\n    println!(\"{}\", lib.count());\n}",
        ),

        // L15 Multiple Bounds
        SeedExercise::new(
            Some(14),
            ExerciseKind::FillGap,
            "Fill the gap with the operator that joins two bounds.",
            &["+", "&", "|"],
            0,
            "`T: Summary + Clone` requires both abilities; `+` joins them. `&` and `|` are bitwise \
             operators, not bound combinators.",
        )
        .with_code("fn duplicate<T: Summary ____ Clone>(item: &T) -> (String, T) {\n    (item.summarise(), item.clone())\n}"),
    ]
}
