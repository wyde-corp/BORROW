//! Module 07 — Structs & Enums, exercises for lessons 14–20.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L14 Methods with impl
        SeedExercise::new(
            Some(13),
            ExerciseKind::Mcq,
            "What does the `&self` in `fn price(&self)` mean?",
            &[
                "The method owns the value and consumes it",
                "The method borrows the value read-only",
                "The method may change the value",
            ],
            1,
            "`&self` is a shared borrow: the method reads the fields and the caller keeps the value. \
             `&mut self` would allow changes; `self` would consume it.",
        )
        .with_code("impl Pizza {\n    fn price(&self) -> u32 {\n        self.base\n    }\n}"),

        // L15 The Three Receivers
        SeedExercise::new(
            Some(14),
            ExerciseKind::ApiDesign,
            "A method appends a topping to the pizza and the caller keeps using the pizza. Which \
             receiver should it take?",
            &[
                "`&self` — read-only",
                "`&mut self` — change in place, caller keeps it",
                "`self` — consume it",
            ],
            1,
            "The method changes the value, so a shared borrow is too little. The caller keeps the \
             pizza, so consuming `self` is too much. `&mut self` is exactly 'change it, and it is \
             still yours'.",
        )
        .with_code("impl Pizza {\n    fn add_topping(???, topping: Topping) {\n        self.toppings.push(topping);\n    }\n}"),

        // L16 Associated Functions
        SeedExercise::new(
            Some(15),
            ExerciseKind::Mcq,
            "How do you call `fn new(size: Size) -> Pizza` written with no `self`?",
            &[
                "`pizza.new(...)` on an existing pizza",
                "`Pizza::new(...)`",
                "`new(...)` as a free function",
            ],
            1,
            "With no `self` it is an associated function, called on the type with `::`. `Pizza::new` \
             is the constructor convention, not a keyword.",
        ),

        // L17 Multiple impl Blocks
        SeedExercise::new(
            Some(16),
            ExerciseKind::Mcq,
            "Does splitting a type's methods across two `impl Pizza` blocks change anything?",
            &[
                "Yes — the second block overrides the first",
                "No — all methods still belong to `Pizza`",
                "Yes — only the first block is used",
            ],
            1,
            "Multiple `impl` blocks are purely organisational; the compiler treats the methods the \
             same. They are useful later for grouping trait implementations, but do not mean anything \
             by themselves.",
        ),

        // L18 Deriving Debug
        SeedExercise::new(
            Some(17),
            ExerciseKind::PredictOutput,
            "A struct derives `Debug`. Predict what `println!(\"{t:?}\")` prints.",
            &[
                "Olives 150",
                "Topping { name: \"Olives\", extra_cost: 150 }",
                "It does not compile without a `Display` impl",
            ],
            1,
            "The derived `Debug` prints the type name and each field, using the debug formatter for \
             values — which is why the string has quotes. `{:#?}` prints it one field per line.",
        )
        .with_code(
            "#[derive(Debug)]\nstruct Topping { name: String, extra_cost: u32 }\n\nfn main() {\n    let t = Topping { name: String::from(\"Olives\"), extra_cost: 150 };\n    println!(\"{t:?}\");\n}",
        ),

        // L19 Deriving Clone and PartialEq
        SeedExercise::new(
            Some(18),
            ExerciseKind::PredictCompile,
            "The struct derives `Clone`. Predict the result of using the original after a clone.",
            &[
                "It compiles — `clone` makes a second owned value and the original is still usable",
                "It does not compile — cloning moves the original",
            ],
            0,
            "`clone` produces an independent copy and leaves the original intact — unlike a move. \
             It allocates, so reach for it deliberately, as Module 05 taught.",
        )
        .with_code(
            "#[derive(Clone)]\nstruct Topping { name: String, extra_cost: u32 }\n\nfn main() {\n    let a = Topping { name: String::from(\"Olives\"), extra_cost: 150 };\n    let b = a.clone();\n    println!(\"{} {}\", a.name, b.name);\n}",
        ),

        // L20 Methods That Consume self
        SeedExercise::new(
            Some(19),
            ExerciseKind::PredictCompile,
            "A method takes `self` and returns the field. Predict what happens if the caller uses the \
             value afterwards.",
            &[
                "It compiles — `self` is copied",
                "It does not compile — the value was consumed by the method",
            ],
            1,
            "A method with receiver `self` takes ownership. The caller's value is gone after the \
             call, so using it afterwards is a use-after-move. The `into_` naming convention signals \
             consumption by `self`.",
        )
        .with_code(
            "struct Pizza { size: String }\n\nimpl Pizza {\n    fn into_size(self) -> String {\n        self.size\n    }\n}\n\nfn main() {\n    let pizza = Pizza { size: String::from(\"large\") };\n    let size = pizza.into_size();\n    println!(\"{} {}\", size, pizza.size);\n}",
        ),
    ]
}
