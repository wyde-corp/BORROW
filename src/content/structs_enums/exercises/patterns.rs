//! Module 07 — Structs & Enums, exercises for lessons 33–35.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // L33 Destructuring in match
        SeedExercise::new(
            Some(32),
            ExerciseKind::PredictOutput,
            "The pattern matches a topping whose `vegetarian` field is `true`. Predict what this \
             prints.",
            &["Olives (veg)", "Olives", "It does not compile"],
            0,
            "The first arm requires `vegetarian: true` and binds `name`, so a vegetarian topping \
             matches it. The second arm catches everything else. Patterns can mix literal matches \
             with bindings.",
        )
        .with_code(
            "struct Topping { name: String, extra_cost: u32, vegetarian: bool }\n\nfn label(t: &Topping) -> String {\n    match t {\n        Topping { vegetarian: true, name, .. } => format!(\"{name} (veg)\"),\n        Topping { name, .. } => format!(\"{name}\"),\n    }\n}\n\nfn main() {\n    let t = Topping { name: String::from(\"Olives\"), extra_cost: 150, vegetarian: true };\n    println!(\"{}\", label(&t));\n}",
        ),

        // L34 The Ten Common Mistakes
        SeedExercise::new(
            Some(33),
            ExerciseKind::PredictCompile,
            "A `String` field is read into a variable, then the field is read again. Predict the \
             result.",
            &[
                "It compiles — reading a field copies it",
                "It does not compile — the `String` was moved out of the struct",
            ],
            1,
            "Reading a `String` field into a new variable moves it, partially moving the struct. The \
             next read of that field is a use-after-move. Clone the field, or borrow it with `&` if \
             the field must stay.",
        )
        .with_code(
            "struct Topping { name: String, extra_cost: u32 }\n\nfn main() {\n    let t = Topping { name: String::from(\"Olives\"), extra_cost: 150 };\n    let moved = t.name;\n    println!(\"{}\", t.name);\n    println!(\"{}\", moved);\n}",
        ),

        // L35 Putting It All Together
        SeedExercise::new(
            Some(34),
            ExerciseKind::PredictOutput,
            "The order has one large pizza with a 150-cent topping and is preparing. Predict the \
             output.",
            &["Gus — preparing (5 min) — 1450 cents", "Gus — placed — 1300 cents", "It does not compile"],
            0,
            "Base price for `Large` is 1300, plus 150 for the topping, is 1450. The status match \
             binds `since_minutes` and formats `preparing (5 min)`. Structs for the pizza and order, \
             enums for the size and status, methods for the behaviour.",
        )
        .with_code(
            "#[derive(Debug, Clone, PartialEq)]\nenum Size { Small, Medium, Large }\n\nstruct Topping { name: String, extra_cost: u32 }\n\nstruct Pizza { size: Size, toppings: Vec<Topping> }\n\nimpl Pizza {\n    fn price(&self) -> u32 {\n        let base = match self.size {\n            Size::Small => 800,\n            Size::Medium => 1000,\n            Size::Large => 1300,\n        };\n        base + self.toppings.iter().map(|t| t.extra_cost).sum::<u32>()\n    }\n}\n\nfn main() {\n    let pizza = Pizza {\n        size: Size::Large,\n        toppings: vec![Topping { name: String::from(\"Olives\"), extra_cost: 150 }],\n    };\n\n    println!(\"{}\", pizza.price());\n}",
        ),
    ]
}
