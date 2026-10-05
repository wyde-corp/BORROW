//! Module 07 — Structs & Enums projects.
//!
//! Six projects that escalate from deliberate repetition to a small designed
//! program. The last is the Pizza Order built across the module, finished and
//! justified.

use crate::content::types::{Difficulty, SeedProject};

pub fn projects() -> Vec<SeedProject> {
    vec![
        SeedProject::new(
            "Struct Playground",
            "Deliberately repetitive. Build and print the same data as loose variables, then as a tuple, then as a named struct, and record what each version makes easy and hard.",
            Difficulty::Beginner,
            "30 min",
            &[
                "Describe a topping five ways: loose variables, a tuple, a tuple struct, a named struct, and a struct with a method",
                "Print each version",
                "Write a comment under each saying one thing it does well and one thing it does badly",
                "Add a second topping and see which versions get confusing",
                "Finish with one sentence: when would you pick each shape?",
            ],
            &[
                "struct definition vs struct literal",
                "field access",
                "tuple structs",
                "choosing a shape",
            ],
            &[
                "No `#[derive(Debug)]` on the first named struct — print fields by hand",
                "Do not copy another version; rewrite each from scratch",
                "Every version must represent the same topping",
                "The final sentence is required",
            ],
            "loose:  let name = String::from(\"Olives\"); let cost = 150;\ntuple:  let t = (String::from(\"Olives\"), 150);\ntuple struct: let t = Topping(String::from(\"Olives\"), 150);\nnamed:  let t = Topping { name: String::from(\"Olives\"), extra_cost: 150 };",
            &[
                "Add a third representation using `#[derive(Debug)]` and compare the printed output",
                "Write a function that takes each representation and say which signature you preferred",
            ],
        ),

        SeedProject::new(
            "Topping Cost Calculator",
            "Structs and methods, no enums yet. A small program that adds toppings to a pizza and reports the total through a method.",
            Difficulty::Beginner,
            "40 min",
            &[
                "Define `Topping { name: String, extra_cost: u32 }`",
                "Define `Pizza { size: String, toppings: Vec<Topping> }`",
                "Write `fn add_topping(&mut self, topping: Topping)`",
                "Write `fn topping_total(&self) -> u32`",
                "Build a pizza in `main`, add three toppings, print the total",
                "Print the pizza with `#[derive(Debug)]`",
            ],
            &[
                "struct construction",
                "`&mut self` versus `&self`",
                "methods",
                "`Vec` of structs",
                "`derive(Debug)`",
            ],
            &[
                "No enums — that is the next project",
                "`topping_total` may not take ownership of the pizza",
                "`add_topping` may not clone the topping",
                "Each method must have a comment stating its receiver and why",
            ],
            "let mut pizza = Pizza { size: String::from(\"large\"), toppings: Vec::new() };\npizza.add_topping(Topping { name: String::from(\"Olives\"), extra_cost: 150 });\nprintln!(\"toppings: {} cents\", pizza.topping_total());",
            &[
                "Add `fn most_expensive(&self) -> Option<&Topping>` and print its name",
                "Add `fn free_toppings(&self) -> usize`",
                "Change `size` to a `u32` base price and add it to a `total` method",
            ],
        ),

        SeedProject::new(
            "Pizza Builder",
            "A chainable builder using consuming methods and `derive(Clone)`. The goal is an API that reads top to bottom and never clones without a reason.",
            Difficulty::Intermediate,
            "1 h",
            &[
                "Define `Pizza` with a size, a list of toppings, and a stuffed-crust flag",
                "Write `fn new(size: Size) -> Pizza`",
                "Write `fn with_topping(mut self, topping: Topping) -> Pizza`",
                "Write `fn stuffed(mut self) -> Pizza`",
                "Build a pizza with a chain and print it",
                "Add `#[derive(Clone, Debug)]` and clone one pizza to show two independent pizzas",
            ],
            &[
                "associated functions",
                "consuming `self` methods",
                "method chaining",
                "`Clone` as a deliberate decision",
            ],
            &[
                "The `with_` methods must take `mut self` and return `Pizza`, not `&mut self`",
                "No cloning except in the one demonstration you can justify",
                "Every method must read as a sentence when chained",
                "Add a comment explaining who owns the pizza at each step of the chain",
            ],
            "let pizza = Pizza::new(Size::Large)\n    .with_topping(Topping { name: String::from(\"Olives\"), extra_cost: 150 })\n    .stuffed();\n\nprintln!(\"{pizza:#?}\");",
            &[
                "Add `fn without_topping(mut self, name: &str) -> Pizza`",
                "Explain in a comment why chaining needs consuming methods rather than `&mut self`",
            ],
        ),

        SeedProject::new(
            "Order Status Tracker",
            "A small CLI state machine. The order's status is an enum, and the program only allows legal transitions.",
            Difficulty::Intermediate,
            "1 h",
            &[
                "Define `enum OrderStatus` with at least five variants, one carrying data",
                "Write `fn label(&self) -> &str` using a `match`",
                "Write `fn advance(self) -> Option<OrderStatus>` for legal transitions",
                "Drive the order forward in `main` until it is delivered",
                "Try an illegal transition and handle the `None` gracefully",
                "Use an exhaustive `match`, not a wildcard, for the transitions",
            ],
            &[
                "enums",
                "`match` and exhaustiveness",
                "variants that carry data",
                "`Option` as an internal enum",
            ],
            &[
                "No `String` status anywhere",
                "Every transition decision must be a `match` arm, not an `if` on a string",
                "An illegal transition returns `None`; it must not panic",
                "A future variant must force a compile error in `advance` — so no `_` arm",
            ],
            "let mut status = OrderStatus::Placed;\n\nwhile let Some(next) = status.advance() {\n    status = next;\n    println!(\"{}\", status.label());\n}",
            &[
                "Add `Preparing { since_minutes: u32 }` and have `label` include the time",
                "Add a `Cancelled` path and decide which states may cancel",
                "Explain why the compiler's exhaustiveness check protects the transition table",
            ],
        ),

        SeedProject::new(
            "Menu & Order Manager",
            "Structs containing enums, searched and summarised. A menu of pizzas, an order that references them, and functions that inspect without unnecessary cloning.",
            Difficulty::Advanced,
            "2 h",
            &[
                "Define `Size`, `Topping`, `Pizza`, and `Order` — structs and enums in the right places",
                "Give `Pizza` a `price(&self) -> u32` that matches on size",
                "Give `Order` an `add_pizza(&mut self, pizza: Pizza)` and a `total(&self) -> u32`",
                "Write `fn find<'a>(order: &'a Order, name: &str) -> Option<&'a Pizza>`",
                "Write a summary function that borrows the order and prints every pizza",
                "Add one enum whose variant carries data, and match it exhaustively",
            ],
            &[
                "composition",
                "borrowing through structs and vectors",
                "`match` on enum variants",
                "API design",
            ],
            &[
                "No cloning of the order or its pizzas",
                "Search functions return borrowed slices, not owned values",
                "No wildcard in a `match` over a set you control",
                "Every function's receiver and parameter types must be justifiable in one sentence",
            ],
            "let mut order = Order::new(\"Gus\");\norder.add_pizza(Pizza { size: Size::Large, toppings: vec![] });\n\nif let Some(p) = find(&order, \"Margherita\") {\n    println!(\"{} cents\", p.price());\n}",
            &[
                "Add `fn available<'a>(menu: &'a [Pizza]) -> Vec<&'a Pizza>`",
                "Change a field to an enum and note how many `match` sites the compiler forces you to visit",
            ],
        ),

        SeedProject::new(
            "Pizza Order",
            "The capstone: the program built across the module, finished and justified. Structs for the things made of parts, enums for the things that are one of a set, methods for behaviour, and `match` for the choices.",
            Difficulty::Advanced,
            "3 h",
            &[
                "Model `Size`, `Topping`, `Pizza`, `OrderStatus`, and `Order`",
                "Give `Pizza` and `Order` methods for pricing, adding, and describing",
                "Enforce the status lifecycle with a `match` that returns `Option`",
                "Drive one full order in `main`: build, add, advance, total, print",
                "Write a design note: for every type, say struct or enum, and why in one sentence",
                "State one change you would make if the menu could carry seasonal items",
            ],
            &[
                "structs and enums together",
                "methods and receivers",
                "exhaustive matching",
                "composition",
                "API design and justification",
            ],
            &[
                "No `String` where a fixed set belongs",
                "No wildcard in a `match` over a set you control",
                "No clone unless the design note justifies it",
                "The design note is required — working code alone is not the deliverable",
            ],
            "let mut order = Order::new(\"Gus\");\n\nlet mut pizza = Pizza::new(Size::Large);\npizza.add_topping(Topping { name: String::from(\"Olives\"), extra_cost: 150 });\n\norder.add_pizza(pizza);\norder.advance();\n\nprintln!(\"{} — {} cents\", order.status_label(), order.total());",
            &[
                "Add half-and-half pizzas and decide which type now needs a new variant",
                "Add a discount as either a percentage or a fixed amount — which shape fits?",
                "Write the design note a second time, after the code, and compare the two",
            ],
        ),
    ]
}
