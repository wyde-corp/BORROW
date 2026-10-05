//! Module 07 — Structs & Enums, exercises for lessons 1–8.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L1 The Five-Variable Problem
        SeedExercise::new(
            Some(0),
            ExerciseKind::Mcq,
            "Why is describing a pizza with five separate variables a problem, if it compiles?",
            &[
                "It uses too much memory",
                "Nothing ties the five values together, so they can drift apart or be passed out of order",
                "Rust cannot print more than four variables",
            ],
            1,
            "The five values are only meaningful together, but the type system has no idea they \
             belong together. A caller can pass the name where the size is expected. The fix is to \
             give the group a name the compiler understands: a struct.",
        ),

        // L2 Defining a Struct
        SeedExercise::new(
            Some(1),
            ExerciseKind::Mcq,
            "What does `struct Topping { name: String, extra_cost: u32 }` create?",
            &[
                "A `Topping` value with a name and cost",
                "A new type named `Topping`, and no value at all",
                "A variable called `Topping`",
            ],
            1,
            "A definition is a shape, not an instance — like a function signature. No `Topping` \
             exists until you write a struct literal. This is the most common first mix-up.",
        )
        .with_code("struct Topping {\n    name: String,\n    extra_cost: u32,\n}"),

        // L3 Creating a Value
        SeedExercise::new(
            Some(2),
            ExerciseKind::PredictCompile,
            "One field is left out of the literal. Predict what the compiler does.",
            &[
                "It compiles and sets the missing field to a default",
                "It does not compile — the missing field is named in the error",
                "It compiles but panics when the field is read",
            ],
            1,
            "Every field is required in a struct literal. The compiler names `extra_cost` and offers \
             to add it. Field order is free, but completeness is not.",
        )
        .with_code(
            "struct Topping {\n    name: String,\n    extra_cost: u32,\n}\n\nfn main() {\n    let t = Topping { name: String::from(\"Olives\") };\n    println!(\"{}\", t.name);\n}",
        ),

        // L4 Reading and Changing Fields
        SeedExercise::new(
            Some(3),
            ExerciseKind::PredictCompile,
            "The binding is not `mut`. Predict the result of assigning to a field.",
            &[
                "It compiles — field assignment does not need `mut`",
                "It does not compile — `t` is not declared mutable",
            ],
            1,
            "Mutability lives on the binding. `let t` makes the whole value read-only, fields \
             included. `let mut t` is the fix — the same rule as `let mut count = 0;`.",
        )
        .with_code(
            "struct Topping {\n    name: String,\n    extra_cost: u32,\n}\n\nfn main() {\n    let t = Topping { name: String::from(\"Olives\"), extra_cost: 150 };\n    t.extra_cost = 200;\n}",
        ),

        // L5 Field Types Are Ownership
        SeedExercise::new(
            Some(4),
            ExerciseKind::PredictOwnership,
            "A `String` is moved into a struct field. Who owns it afterwards?",
            &[
                "The original variable still owns it",
                "The struct owns it, and the original variable is disarmed",
                "They share ownership",
            ],
            1,
            "A `String` field owns its bytes, so moving a value into the field moves it. The struct \
             is now responsible for freeing it. This is Module 05's move, with a struct as the \
             destination.",
        )
        .with_code(
            "struct Topping {\n    name: String,\n    extra_cost: u32,\n}\n\nfn main() {\n    let name = String::from(\"Olives\");\n    let t = Topping { name, extra_cost: 150 };\n    println!(\"{}\", name);\n}",
        ),

        // L6 Tuple Structs
        SeedExercise::new(
            Some(5),
            ExerciseKind::Mcq,
            "Why write `struct Cents(u32)` instead of using a plain `u32`?",
            &[
                "It stores the number more efficiently",
                "It gives the number a distinct type, so a different `u32` cannot be passed by mistake",
                "It allows the number to be negative",
            ],
            1,
            "The wrapper makes the type itself the meaning. `total(a: Cents, b: Cents)` cannot be \
             called with a plain number, so units can never be mixed up. It costs nothing at runtime.",
        )
        .with_code("struct Cents(u32);\n\nfn total(a: Cents, b: Cents) -> Cents {\n    Cents(a.0 + b.0)\n}"),

        // L7 Unit Structs
        SeedExercise::new(
            Some(6),
            ExerciseKind::Mcq,
            "What is a unit struct such as `struct Takeaway;` useful for?",
            &[
                "Storing a single `bool`",
                "Marking a type so the compiler can tell two no-data choices apart",
                "Nothing — it is always a mistake",
            ],
            1,
            "It carries no data, but it still has a name and a type. `Takeaway` can never be passed \
             where `DineIn` is expected. It is the single-value version of an enum variant that holds \
             no data.",
        ),

        // L8 Struct vs Tuple vs Loose Variables
        SeedExercise::new(
            Some(7),
            ExerciseKind::Mcq,
            "Which shape fits 'each topping has a name and a cost, and the two must not be confused'?",
            &[
                "A tuple, `(String, u32)`",
                "A named struct, `Topping { name, extra_cost }`",
                "Two loose variables",
            ],
            1,
            "The fields mean different things, so they should be named. A tuple forces readers to \
             remember slot order and lets a caller swap the two. A struct makes the swap impossible.",
        ),
    ]
}
