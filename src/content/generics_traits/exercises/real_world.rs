//! Module 10 — Generics & Traits, exercises for lessons 24–31.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L24 Display and ToString
        SeedExercise::new(
            Some(23),
            ExerciseKind::FillGap,
            "Fill the gap with the trait that makes `println!(\"{book}\")` work.",
            &["Display", "Debug", "ToString"],
            0,
            "`{}` uses `Display`. `Debug` is `{:?}`, and `ToString` is provided automatically once \
             `Display` exists — you implement `Display`, not `ToString`.",
        )
        .with_code("impl std::fmt::____ for Book {\n    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n        write!(f, \"{}\", self.title)\n    }\n}"),

        // L25 From and Into
        SeedExercise::new(
            Some(24),
            ExerciseKind::PredictOutput,
            "`From` is implemented; `.into()` is used. Predict the output.",
            &["Dune", "Title { text: \"Dune\" }", "It does not compile — `Into` was not implemented"],
            0,
            "Implementing `From<&str> for Title` gives `Into<Title> for &str` for free, so `.into()` \
             works. The value built is `Title(\"Dune\")`, and `.0` prints `Dune`.",
        )
        .with_code(
            "struct Title(String);\n\nimpl From<&str> for Title {\n    fn from(text: &str) -> Title {\n        Title(text.to_string())\n    }\n}\n\nfn main() {\n    let t: Title = \"Dune\".into();\n    println!(\"{}\", t.0);\n}",
        ),

        // L26 Implementing vs Deriving
        SeedExercise::new(
            Some(25),
            ExerciseKind::PredictOutput,
            "Equality is implemented by hand to compare titles only. Predict what `a == b` prints.",
            &["true", "false", "It does not compile"],
            0,
            "The manual `PartialEq` compares titles, and both are `Dune`, so `a == b` is `true` even \
             though the page counts differ. A derived version would compare every field and print \
             `false`.",
        )
        .with_code(
            "struct Book { title: String, pages: u32 }\n\nimpl PartialEq for Book {\n    fn eq(&self, other: &Book) -> bool {\n        self.title == other.title\n    }\n}\n\nfn main() {\n    let a = Book { title: String::from(\"Dune\"), pages: 412 };\n    let b = Book { title: String::from(\"Dune\"), pages: 500 };\n    println!(\"{}\", a == b);\n}",
        ),

        // L27 Iterator Preview
        SeedExercise::new(
            Some(26),
            ExerciseKind::PredictOutput,
            "A list of page counts is summed with iterator methods. Predict the output.",
            &["803", "3", "It does not compile"],
            0,
            "`.map(|b| b.pages)` produces each page count, and `.sum()` adds them: 412 + 120 + 271 = \
             803. Both `map` and `sum` are methods on the `Iterator` trait, previewed in this module.",
        )
        .with_code(
            "struct Book { pages: u32 }\n\nfn main() {\n    let shelf = vec![Book { pages: 412 }, Book { pages: 120 }, Book { pages: 271 }];\n    let total: u32 = shelf.iter().map(|b| b.pages).sum();\n    println!(\"{total}\");\n}",
        ),

        // L28 Operator Traits
        SeedExercise::new(
            Some(27),
            ExerciseKind::PredictOutput,
            "`Add` is implemented for a point type. Predict what prints.",
            &["Point { x: 4, y: 6 }", "4 6", "It does not compile"],
            0,
            "`impl Add for Point` makes `+` call `add`, which sums the fields: `(4, 6)`. The `{:?}` \
             prints the derived debug form. Operators are traits, so `+` works exactly when `Add` is \
             implemented.",
        )
        .with_code(
            "use std::ops::Add;\n\n#[derive(Debug)]\nstruct Point { x: i32, y: i32 }\n\nimpl Add for Point {\n    type Output = Point;\n    fn add(self, other: Point) -> Point {\n        Point { x: self.x + other.x, y: self.y + other.y }\n    }\n}\n\nfn main() {\n    let p = Point { x: 1, y: 2 } + Point { x: 3, y: 4 };\n    println!(\"{p:?}\");\n}",
        ),

        // L29 Common Mistakes
        SeedExercise::new(
            Some(28),
            ExerciseKind::PredictCompile,
            "A generic function calls a method that no bound promises. Predict the result.",
            &[
                "It compiles — the method exists on the type used",
                "It does not compile — `T` has no bound giving it that method",
            ],
            1,
            "Inside a generic function, `T` could be any type the bound allows. If no bound gives `T` a \
             `summarise` method, the call is rejected — even if every current caller's type happens to \
             have one. The bound is the contract, not the call site.",
        )
        .with_code(
            "fn describe<T>(item: &T) -> String {\n    item.summarise()\n}\n\nfn main() {\n    println!(\"{}\", describe(&String::from(\"x\")));\n}",
        ),

        // L30 The Media Library, Assembled
        SeedExercise::new(
            Some(29),
            ExerciseKind::Mcq,
            "In the assembled library, `Library<Book>` is generic and the mixed shelf is \
             `Vec<Box<dyn Summary>>`. Why are both present?",
            &[
                "One is a mistake",
                "`Library<Book>` is one type and can be monomorphised; the mixed shelf holds several \
                 types and needs a trait object",
                "`dyn` is faster than a generic type",
            ],
            1,
            "A library of books is a single concrete type, so a generic `Library<T>` gives direct calls. \
             The mixed shelf holds books and videos together, which no single concrete type can, so it \
             needs `Box<dyn Summary>`. The two coexist because they answer different questions.",
        ),

        // L31 Change the Design
        SeedExercise::new(
            Some(30),
            ExerciseKind::ApiDesign,
            "You add a method `fn longest_title<T: Summary>(items: &[T]) -> Option<&T>`. Which bound \
             does it actually need?",
            &[
                "`T: Summary` — it only calls `summarise` to compare titles",
                "`T: Summary + Clone` — it returns a `T`",
                "No bound — it only iterates",
            ],
            0,
            "It returns `Option<&T>`, a *reference*, so no `Clone` is needed. It compares summaries, so \
             `T: Summary` is exactly the ability used. Bounds should name the abilities the body \
             actually needs, and no more.",
        ),
    ]
}
