//! Module 07 — Structs & Enums, lessons 9–13.
//!
//! Construction. Building values gets shorter and more readable: shorthand,
//! update syntax, defaults, then structs that contain other structs and
//! collections of structs — which is where the Pizza Order starts to look like
//! real code.

use crate::content::types::{
    broken, callout, code, diagram, gap_fill, guess_run, labelled, step_through, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        field_init_shorthand(),
        struct_update(),
        defaults(),
        nested_structs(),
        collections_of_structs(),
    ]
}

fn field_init_shorthand() -> SeedLesson {
    SeedLesson::new(
        "Field Init Shorthand",
        vec![
            labelled(
                "THE IDEA",
                "When a local variable has exactly the same name as a field, you can write the field \
                 once instead of twice. `Topping { name: name }` becomes `Topping { name }`.",
            ),
            gap_fill(
                "Rewrite the literal using shorthand. Type the three short field entries.",
                r#"let name = String::from("Olives");
let extra_cost = 150;

let topping = Topping {
    ____,
    ____,
};"#,
                &[
                    ("f1", "name", "Same name as the variable."),
                    ("f2", "extra_cost", "Same name as the variable."),
                ],
            ),
            code(
                "rust",
                r#"struct Topping {
    name: String,
    extra_cost: u32,
}

fn main() {
    let name = String::from("Olives");
    let extra_cost = 150;

    let topping = Topping { name, extra_cost };

    println!("{} +{}", topping.name, topping.extra_cost);
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Shorthand does not change what the value is; the variable is still moved into the \
                 field. It removes the repetition of `name: name`, which is where typos breed. When \
                 the variable name and the field name differ, you write the long form, as before.",
            ),
            callout(
                "A small readability win that adds up",
                "Real Rust code uses shorthand constantly. Recognising it is not optional — a reader \
                 who does not know it will think a field is missing.",
            ),
        ],
    )
}

fn struct_update() -> SeedLesson {
    SeedLesson::new(
        "Struct Update Syntax",
        vec![
            labelled(
                "THE IDEA",
                "To build a new value that is mostly a copy of an existing one, name only the fields \
                 that change and finish with `..old`. Everything else is taken from `old`.",
            ),
            code(
                "rust",
                r#"struct Topping {
    name: String,
    extra_cost: u32,
    vegetarian: bool,
}

fn main() {
    let olives = Topping {
        name: String::from("Olives"),
        extra_cost: 150,
        vegetarian: true,
    };

    let fancy_olives = Topping {
        extra_cost: 200,
        ..olives
    };

    println!("{} +{}", fancy_olives.name, fancy_olives.extra_cost);
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`fancy_olives` takes `name` and `vegetarian` from `olives`, and uses the new \
                 `extra_cost`. The `..olives` must come last, and it behaves like a move: any field \
                 that is not `Copy` moves out of `olives`, so `olives` may not be fully usable \
                 afterwards. Fields that are `Copy`, like `bool` and `u32`, are copied.",
            ),
            broken(
                r#"struct Topping {
    name: String,
    extra_cost: u32,
}

fn main() {
    let a = Topping {
        name: String::from("Olives"),
        extra_cost: 150,
    };

    let b = Topping {
        extra_cost: 200,
        ..a
    };

    // `a.name` moved into `b`, so `a` cannot be used as a whole.
    println!("{}", a.name);
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The update moved the `String` out of `a` into `b`. Using `a.name` afterwards is a \
                 use-after-move — the same error as Module 05. If `a` is still needed, clone the field \
                 explicitly with `name: a.name.clone()` instead of relying on the update.",
            ),
            callout(
                "Update syntax is a move in disguise",
                "It reads like copying, but any non-`Copy` field is moved. When the original is still \
                 needed, clone exactly the fields you keep and be explicit about it.",
            ),
        ],
    )
}

fn defaults() -> SeedLesson {
    SeedLesson::new(
        "Defaults",
        vec![
            labelled(
                "THE IDEA",
                "`#[derive(Default)]` lets a struct build an all-zero version of itself, and update \
                 syntax then fills in only the fields you care about. It is the cleanest way to build \
                 a value with many optional fields.",
            ),
            gap_fill(
                "Complete the value so every unspecified field comes from the default. Fill in the \
                 attribute and the update marker.",
                r#"____(Default)]
struct Settings {
    cheese: bool,
    size: u32,
}

fn main() {
    let s = Settings { cheese: true, ____Default::default() };
    println!("{} {}", s.cheese, s.size);
}"#,
                &[
                    ("d1", "derive", "The attribute that generates an impl for you."),
                    ("d2", "..", "The update marker, before Default::default()."),
                ],
            ),
            code(
                "rust",
                r#"#[derive(Default)]
struct Settings {
    cheese: bool,
    size: u32,
}

fn main() {
    let plain = Settings::default();
    let cheesy = Settings { cheese: true, ..Default::default() };

    println!("plain={} cheesy={}", plain.cheese, cheesy.cheese);
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`Settings::default()` gives `cheese: false, size: 0` — the default for each field's \
                 type. The update form builds the same thing and then overrides `cheese`. This is \
                 shorter and, more importantly, will not break if a new field is added later: the \
                 default covers it.",
            ),
            callout(
                "Derive means generated, not magic",
                "`#[derive(Default)]` asks the compiler to write the `impl` for you based on the field \
                 types. It only works when every field is itself `Default`. The next lessons derive \
                 more, and the reason is always the same.",
            ),
        ],
    )
}

fn nested_structs() -> SeedLesson {
    SeedLesson::new(
        "Nested Structs",
        vec![
            labelled(
                "THE IDEA",
                "A field can be any type, including another struct. A `Pizza` has a size and a list of \
                 toppings; the toppings are their own values, each with a name and a cost.",
            ),
            step_through(
                "A Pizza that contains Toppings",
                r#"let topping = Topping { name: String::from("Olives"), extra_cost: 150 };
let pizza = Pizza { size: String::from("large"), toppings: vec![topping] };
println!("{}", pizza.toppings[0].name);"#,
                &[
                    (&[1][..], "A standalone `Topping` is built first."),
                    (&[2][..], "The `Pizza` owns a `Vec`. The topping is moved into that vector."),
                    (&[2][..], "After this line the pizza owns the topping; the local `topping` is gone."),
                    (&[3][..], "Reading a nested field chains the dots: pizza → toppings → first → name."),
                ],
            ),
            code(
                "rust",
                r#"struct Topping {
    name: String,
    extra_cost: u32,
}

struct Pizza {
    size: String,
    toppings: Vec<Topping>,
}

fn main() {
    let pizza = Pizza {
        size: String::from("large"),
        toppings: vec![
            Topping { name: String::from("Olives"), extra_cost: 150 },
            Topping { name: String::from("Onion"), extra_cost: 100 },
        ],
    };

    println!("{} {} topping(s)", pizza.size, pizza.toppings.len());
}"#,
            ),
            diagram(
                "Pizza owns its toppings",
                "Pizza\n ┌───────────────────────────┐\n │ size:   \"large\"            │\n │ toppings: Vec<Topping> ───┼──→ [ Topping, Topping ]\n └───────────────────────────┘         │         │\n                                    name/cost  name/cost",
            ),
            callout(
                "Composition is just fields",
                "There is no special nesting syntax. `toppings: Vec<Topping>` is an ordinary field \
                 whose type happens to hold more structs. This is how real programs are built: small \
                 types composed into bigger ones.",
            ),
        ],
    )
}

fn collections_of_structs() -> SeedLesson {
    SeedLesson::new(
        "Collections of Structs",
        vec![
            labelled(
                "THE IDEA",
                "A `Vec<Topping>` is one of the most common shapes in Rust. Iterating it gives you a \
                 reference to each struct, and you reach the fields through the reference.",
            ),
            code(
                "rust",
                r#"struct Topping {
    name: String,
    extra_cost: u32,
}

fn main() {
    let toppings = vec![
        Topping { name: String::from("Olives"), extra_cost: 150 },
        Topping { name: String::from("Onion"), extra_cost: 100 },
    ];

    let mut total = 0;
    for topping in &toppings {
        println!("{} +{}", topping.name, topping.extra_cost);
        total += topping.extra_cost;
    }

    println!("{total} cents of toppings");
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`for topping in &toppings` borrows each element, so the vector keeps its contents. \
                 `topping` is `&Topping`, and `topping.name` reaches a field through the reference. \
                 If you iterated by value (`for topping in toppings`), each topping would be moved out \
                 and the vector would be consumed — sometimes what you want, usually not.",
            ),
            guess_run(
                "This loop tries to add a topping while iterating. Predict what happens.",
                r#"struct Topping { name: String, extra_cost: u32 }

fn main() {
    let mut toppings = vec![
        Topping { name: String::from("Olives"), extra_cost: 150 },
    ];

    for t in &toppings {
        println!("{}", t.name);
        toppings.push(Topping { name: String::from("Onion"), extra_cost: 100 });
    }
}"#,
                &[
                    "It compiles and prints two names",
                    "It does not compile — pushing while borrowing the vector is rejected",
                    "It compiles but panics at runtime",
                ],
                1,
                "`&toppings` borrows the vector for the whole loop, and `push` needs a mutable borrow \
                 at the same time. This is the collection borrow conflict from Module 06. Building a \
                 new vector, or collecting the additions first and pushing after the loop, are the \
                 usual fixes.",
            ),
            callout(
                "The Pizza Order is taking shape",
                "One struct (`Topping`), one containing struct (`Pizza`), one collection. Next we give \
                 the pizza behaviour: methods for pricing and describing itself.",
            ),
        ],
    )
}
