//! Module 07 — Structs & Enums, exercises for lessons 27–32.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L27 match Basics
        SeedExercise::new(
            Some(26),
            ExerciseKind::PredictOutput,
            "`match` picks the arm for the value. Predict what this prints.",
            &["800", "1000", "1300"],
            1,
            "`Size::Medium` matches the `Size::Medium => 1000` arm, so the `match` evaluates to \
             `1000`. Exactly one arm runs, because the value is exactly one variant.",
        )
        .with_code(
            "enum Size { Small, Medium, Large }\n\nfn price(s: Size) -> u32 {\n    match s {\n        Size::Small => 800,\n        Size::Medium => 1000,\n        Size::Large => 1300,\n    }\n}\n\nfn main() { println!(\"{}\", price(Size::Medium)); }",
        ),

        // L28 Matching Variant Data
        SeedExercise::new(
            Some(27),
            ExerciseKind::PredictOutput,
            "The pattern binds the data inside the variant. Predict what this prints.",
            &["preparing for 7 min", "preparing for 0 min", "It does not compile"],
            0,
            "`OrderStatus::Preparing { since_minutes }` binds the value `7` to a local named \
             `since_minutes`, which the arm body then formats. The pattern both selects the variant \
             and extracts its data.",
        )
        .with_code(
            "enum OrderStatus { Placed, Preparing { since_minutes: u32 }, Delivered }\n\nfn message(s: &OrderStatus) -> String {\n    match s {\n        OrderStatus::Placed => String::from(\"placed\"),\n        OrderStatus::Preparing { since_minutes } => format!(\"preparing for {since_minutes} min\"),\n        OrderStatus::Delivered => String::from(\"delivered\"),\n    }\n}\n\nfn main() {\n    println!(\"{}\", message(&OrderStatus::Preparing { since_minutes: 7 }));\n}",
        ),

        // L29 Exhaustiveness
        SeedExercise::new(
            Some(28),
            ExerciseKind::FixCode,
            "This `match` does not compile. What is the fix?",
            &[
                "Add the missing `Size::Large` arm",
                "Add `#[allow(non_exhaustive)]`",
                "Change the return type to `Option<u32>`",
            ],
            0,
            "The `match` omits `Size::Large`, and the compiler names it. Exhaustiveness is not \
             pedantry: `price(Size::Large)` must have an answer. Adding the arm is the fix.",
        )
        .with_code(
            "enum Size { Small, Medium, Large }\n\nfn price(s: Size) -> u32 {\n    match s {\n        Size::Small => 800,\n        Size::Medium => 1000,\n    }\n}",
        ),

        // L30 Wildcards
        SeedExercise::new(
            Some(29),
            ExerciseKind::PredictCompile,
            "A new variant is added, but the wildcard arm absorbs it. Predict what happens.",
            &[
                "It does not compile",
                "It compiles, and the new variant silently takes the wildcard's behaviour",
            ],
            1,
            "`_` matches anything not already handled, so adding `Refunded` does not break this \
             `match` — which is the danger. A wildcard is right only when 'everything else' really \
             shares one behaviour forever.",
        )
        .with_code(
            "enum OrderStatus { Placed, Preparing, Refunded }\n\nfn is_active(s: &OrderStatus) -> bool {\n    match s {\n        OrderStatus::Placed => true,\n        _ => true,\n    }\n}\n\nfn main() { println!(\"{}\", is_active(&OrderStatus::Refunded)); }",
        ),

        // L31 if let
        SeedExercise::new(
            Some(30),
            ExerciseKind::PredictOutput,
            "`if let` runs its block only when the pattern matches. Predict what this prints.",
            &["found Olives", "not on the menu", "It does not compile"],
            0,
            "The search returns `Some`, so `if let Some(found)` matches and binds the topping. `if \
             let` is the compact form for acting on exactly one variant while ignoring the rest.",
        )
        .with_code(
            "struct Topping { name: String, extra_cost: u32 }\n\nfn main() {\n    let found = Some(Topping { name: String::from(\"Olives\"), extra_cost: 150 });\n\n    if let Some(t) = found {\n        println!(\"found {}\", t.name);\n    } else {\n        println!(\"not on the menu\");\n    }\n}",
        ),

        // L32 let else
        SeedExercise::new(
            Some(31),
            ExerciseKind::Mcq,
            "What must the `else` block of a `let … else` do?",
            &[
                "Nothing — it is optional",
                "Diverge: `return`, `break`, `continue`, or panic",
                "Produce a default value of the same type",
            ],
            1,
            "The `else` block must not fall through, because the `let` only continues when the \
             pattern matched. It leaves the function or loop, keeping the happy path unindented. A \
             bare value would leave nothing to bind.",
        )
        .with_code(
            "fn price_or_default(text: &str) -> u32 {\n    let Some(size) = parse(text) else {\n        return 1000;\n    };\n\n    match size {\n        Size::Small => 800,\n        Size::Medium => 1000,\n        Size::Large => 1300,\n    }\n}",
        ),
    ]
}
