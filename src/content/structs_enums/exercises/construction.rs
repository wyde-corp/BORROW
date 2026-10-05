//! Module 07 — Structs & Enums, exercises for lessons 9–13.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L9 Field Init Shorthand
        SeedExercise::new(
            Some(8),
            ExerciseKind::FillGap,
            "Fill the gap: write the shorthand form that moves the local `name` into the field.",
            &[
                "Topping { name, extra_cost: 150 }",
                "Topping { name: name, extra_cost: 150 }",
                "Topping { String::from(\"Olives\") }",
            ],
            0,
            "Shorthand writes the field once when a variable of the same name is in scope. \
             `name: name` is the long form; both move the variable into the field.",
        )
        .with_code(
            "struct Topping { name: String, extra_cost: u32 }\n\nfn main() {\n    let name = String::from(\"Olives\");\n    let t = Topping { ____, extra_cost: 150 };\n}",
        ),

        // L10 Struct Update Syntax
        SeedExercise::new(
            Some(9),
            ExerciseKind::PredictCompile,
            "A non-`Copy` field is taken from another value with `..old`, then `old` is used again. \
             Predict the result.",
            &[
                "It compiles — `..old` copies the fields",
                "It does not compile — a `String` field was moved out of `old`",
            ],
            1,
            "Struct update syntax is a move for non-`Copy` fields. Taking `name` from `old` disarms \
             `old`; using it afterwards is a use-after-move. Clone the field explicitly if the \
             original is still needed.",
        )
        .with_code(
            "struct Topping { name: String, extra_cost: u32 }\n\nfn main() {\n    let a = Topping { name: String::from(\"Olives\"), extra_cost: 150 };\n    let b = Topping { extra_cost: 200, ..a };\n    println!(\"{}\", a.name);\n}",
        ),

        // L11 Defaults
        SeedExercise::new(
            Some(10),
            ExerciseKind::Mcq,
            "What does `#[derive(Default)]` give a struct?",
            &[
                "A method that prints the struct",
                "An associated function that builds an all-default value of the type",
                "A way to make every field public",
            ],
            1,
            "It generates `Type::default()`, using each field's own default. Combined with `..` \
             update syntax, it builds a value while filling in everything you did not mention.",
        )
        .with_code("#[derive(Default)]\nstruct Settings { cheese: bool, size: u32 }"),

        // L12 Nested Structs
        SeedExercise::new(
            Some(11),
            ExerciseKind::PredictOutput,
            "A pizza owns a vector of toppings. Predict what this prints.",
            &[
                "large 1",
                "large 0",
                "It does not compile — a struct cannot contain a `Vec`",
            ],
            0,
            "`pizza.toppings.len()` counts the toppings moved into the vector. Composition is just \
             fields: `toppings: Vec<Topping>` is an ordinary field whose type holds more structs.",
        )
        .with_code(
            "struct Topping { name: String, extra_cost: u32 }\nstruct Pizza { size: String, toppings: Vec<Topping> }\n\nfn main() {\n    let pizza = Pizza {\n        size: String::from(\"large\"),\n        toppings: vec![Topping { name: String::from(\"Olives\"), extra_cost: 150 }],\n    };\n    println!(\"{} {}\", pizza.size, pizza.toppings.len());\n}",
        ),

        // L13 Collections of Structs
        SeedExercise::new(
            Some(12),
            ExerciseKind::PredictCompile,
            "A loop borrows the vector and tries to push to it. Predict the result.",
            &[
                "It compiles and prints both names",
                "It does not compile — the vector is borrowed while `push` needs a mutable borrow",
            ],
            1,
            "`&toppings` borrows for the whole loop and `push` needs a mutable borrow at the same \
             time — the collection conflict from Module 06. Build a separate vector, or push after \
             the loop.",
        )
        .with_code(
            "struct Topping { name: String, extra_cost: u32 }\n\nfn main() {\n    let mut toppings = vec![Topping { name: String::from(\"Olives\"), extra_cost: 150 }];\n    for t in &toppings {\n        println!(\"{}\", t.name);\n        toppings.push(Topping { name: String::from(\"Onion\"), extra_cost: 100 });\n    }\n}",
        ),
    ]
}
