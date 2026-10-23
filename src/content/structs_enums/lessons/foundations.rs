//! Module 07 — Structs & Enums, lessons 1–8.
//!
//! Foundations. The learner starts with a pizza described by five loose
//! variables, feels why that falls apart, and is given the two named types that
//! fix it. From here the running Pizza Order program grows in every lesson.

use crate::content::types::{
    callout, code, diagram, gap_fill, guess_run, labelled, paragraphs, step_through, table,
    SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        five_variable_problem(),
        defining_a_struct(),
        creating_a_value(),
        reading_and_changing(),
        fields_are_ownership(),
        tuple_structs(),
        unit_structs(),
        choosing_a_shape(),
    ]
}

fn five_variable_problem() -> SeedLesson {
    SeedLesson::new(
        "The Five-Variable Problem",
        vec![
            labelled(
                "THE IDEA",
                "A pizza is one thing: a name, a size, a cost, whether it is vegetarian, whether it \
                 is available. The only way you know how to group data so far is to write each piece \
                 in its own variable. Let us do exactly that and watch the seams show.",
            ),
            code(
                "rust",
                r#"fn main() {
    let name = String::from("Margherita");
    let size = String::from("medium");
    let extra_cost = 250;
    let vegetarian = true;
    let available = true;

    println!("{name} ({size}) +{extra_cost} cents · veg {vegetarian} · in stock {available}");
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "This compiles and prints the right thing. Nothing is technically wrong. But these \
                 five variables only mean anything *together*: pass them to a function and the \
                 parameter list becomes a small crowd, and nothing stops one caller from handing \
                 over a pizza name where a size was expected.",
            ),
            diagram(
                "Five facts, no name for the group",
                "name       size      extra_cost   vegetarian   available\n  │          │            │           │            │\n  └──────────┴────────────┴───────────┴────────────┘\n                    five separate things",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "There is a second, quieter problem. A topping's cost is a number; a pizza's cost is \
                 also a number. Both are `u32`. If you accidentally use one where the other belongs, \
                 the compiler cannot tell — they are the same type. The only thing that ever knew \
                 which number was which was the variable name, and names are not checked.",
            ),
            callout(
                "The idea this module is built on",
                "A **struct** says *these values are one value*. It gives the group a name the \
                 compiler understands, so a pizza can no longer be confused with a topping. That \
                 single sentence is the reason structs exist.",
            ),
            paragraphs(&[
                "Over the next lessons we will rebuild this pizza as a struct, then add methods, then \
                 a size that can only be one of three values, then a status that can only be one of \
                 five. By the end, one realistic program will hold every idea in the module.",
            ]),
        ],
    )
}

fn defining_a_struct() -> SeedLesson {
    SeedLesson::new(
        "Defining a Struct",
        vec![
            labelled(
                "THE IDEA",
                "A struct definition names a group and lists the fields inside it. It is a plan, not \
                 a value: it describes what every topping looks like without creating a topping.",
            ),
            gap_fill(
                "Fill in the two field types. The names are already chosen; you are deciding what kind \
                 of data each one holds.",
                r#"struct Topping {
    name: ____,
    extra_cost: ____,
}"#,
                &[
                    ("f1", "String", "Owned text, as in the earlier modules."),
                    ("f2", "u32", "A whole number, never negative."),
                ],
            ),
            code(
                "rust",
                r#"struct Topping {
    name: String,
    extra_cost: u32,
}

fn main() {
    // The definition above describes a Topping.
    // No topping exists yet.
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Read the definition from the top. `struct Topping` introduces a new type called \
                 `Topping` — capitalised, because it is a type and `rustc` will warn if you are not. \
                 Inside the braces, each field has a name and a type, separated by a colon, and each \
                 line ends in a comma, including the last.",
            ),
            callout(
                "A struct definition creates no values",
                "This is the most common beginner mix-up. `struct Topping { … }` is a definition, \
                 like a function signature — it is the shape, not an instance. To get an actual \
                 topping you must construct one, which is the very next lesson.",
            ),
            paragraphs(&[
                "One more thing worth noticing now: the fields are private by default, and that is \
                 fine. Rust does not need them public for other code in the same file, and we will \
                 not reach for `pub` until a later module makes it necessary.",
            ]),
        ],
    )
}

fn creating_a_value() -> SeedLesson {
    SeedLesson::new(
        "Creating a Value",
        vec![
            labelled(
                "THE IDEA",
                "To make a topping you write its name, then a pair of braces with every field set to \
                 a value. This is a *struct literal*, and it is how every struct value begins life.",
            ),
            code(
                "rust",
                r#"struct Topping {
    name: String,
    extra_cost: u32,
}

fn main() {
    let pepperoni = Topping {
        name: String::from("Pepperoni"),
        extra_cost: 250,
    };

    println!("{} costs {} cents", pepperoni.name, pepperoni.extra_cost);
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Each line inside the braces names a field and gives it a value. The order does not \
                 have to match the definition — `extra_cost` before `name` works just as well — \
                 because the field name is what connects the value to its slot. But every field must \
                 be provided.",
            ),
            guess_run(
                "One field is left out. Predict what the compiler does.",
                r#"struct Topping {
    name: String,
    extra_cost: u32,
}

fn main() {
    let t = Topping {
        name: String::from("Olives"),
    };

    println!("{}", t.name);
}"#,
                &[
                    "It compiles and defaults `extra_cost` to 0",
                    "It does not compile — a field is missing",
                    "It compiles, but `t.extra_cost` panics when used",
                ],
                1,
                "Rust requires every field in a struct literal. The error names the missing field, \
                 `extra_cost`, and even offers to add it for you. This is one of the places the \
                 compiler does your bookkeeping: a half-built value is not representable.",
            ),
            callout(
                "Order is irrelevant, completeness is not",
                "Field order in the literal is free. Missing fields are not. Remember that pairing: \
                 the names matter, the arrangement does not.",
            ),
        ],
    )
}

fn reading_and_changing() -> SeedLesson {
    SeedLesson::new(
        "Reading and Changing Fields",
        vec![
            labelled(
                "THE IDEA",
                "A field is reached with a dot: `pizza.name`. Reading needs nothing special. Writing \
                 needs the value behind the binding to be declared `mut`, exactly as a plain variable \
                 did in Module 02.",
            ),
            code(
                "rust",
                r#"struct Topping {
    name: String,
    extra_cost: u32,
}

fn main() {
    let mut olives = Topping {
        name: String::from("Olives"),
        extra_cost: 150,
    };

    olives.extra_cost = 200;

    println!("{} now costs {}", olives.name, olives.extra_cost);
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Reading `olives.name` copies nothing and moves nothing — it gives you a reference to \
                 the field for the length of the expression. Assigning `olives.extra_cost = 200` \
                 needs `let mut olives`, because that is a mutation of the struct.",
            ),
            guess_run(
                "Here the binding is missing `mut`. Predict what happens.",
                r#"struct Topping {
    name: String,
    extra_cost: u32,
}

fn main() {
    let olives = Topping {
        name: String::from("Olives"),
        extra_cost: 150,
    };

    olives.extra_cost = 200;
}"#,
                &[
                    "It compiles — changing a field does not need `mut`",
                    "It does not compile — `olives` is not declared mutable",
                    "It compiles but the assignment is ignored",
                ],
                1,
                "Mutability is a property of the binding, not the struct definition. `let olives` \
                 makes the whole value — including its fields — read-only. Adding `mut` is the fix, \
                 and it is the same rule you already know from `let mut count = 0;`.",
            ),
            table(
                &["You write", "What it does"],
                &[
                    &["`olives.name`", "read a field"],
                    &["`olives.extra_cost = 200;`", "write a field — needs `let mut`"],
                    &["`let n = &olives.name;`", "borrow a field (Module 06 rules apply)"],
                ],
            ),
        ],
    )
}

fn fields_are_ownership() -> SeedLesson {
    SeedLesson::new(
        "Field Types Are Ownership",
        vec![
            labelled(
                "THE IDEA",
                "A struct does not get a special memory model. When a field's type is `String`, the \
                 struct *owns* that string — and moving a value into a struct field moves the value, \
                 under all the rules from Modules 05 and 06.",
            ),
            step_through(
                "Moving a String into a struct",
                r#"let name = String::from("Olives");
let topping = Topping { name, extra_cost: 150 };
println!("{}", name);"#,
                &[
                    (&[1][..], "`name` owns a fresh String on the heap."),
                    (&[2][..], "The shorthand `name` moves the String into the `name` field. `name` is now disarmed."),
                    (&[3][..], "Using `name` here is borrow-of-moved-value — a compile error. The struct owns it now."),
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

    let topping = Topping { name, extra_cost: 150 };

    // `name` is gone; use `topping.name` instead.
    println!("{}", topping.name);
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Nothing new happened — this is the move you learned in Module 05, just with a struct \
                 as the destination. The struct is now responsible for the string and will free it \
                 when the struct is dropped. If the caller still needed the name, they would pass a \
                 clone, or borrow instead of moving.",
            ),
            callout(
                "Fields are just values with a home",
                "Because a `String` field owns its bytes, the ownership rules you already know apply \
                 unchanged. That is the point: structs did not add a new memory model, they added a \
                 way to *name* a group of values that was already there.",
            ),
        ],
    )
}

fn tuple_structs() -> SeedLesson {
    SeedLesson::new(
        "Tuple Structs",
        vec![
            labelled(
                "THE IDEA",
                "Sometimes the fields do not need names — the *type* is the point. A tuple struct \
                 gives the whole value a name while leaving the fields numbered, which stops two \
                 different numbers from being confused for each other.",
            ),
            gap_fill(
                "This tuple struct wraps a single number of cents. Fill in the missing type.",
                r#"struct Cents(____);

fn main() {
    let price = Cents(150);
    let Cents(inner) = price;
    println!("{inner}");
}"#,
                &[("t1", "u32", "Cents are whole numbers.")],
            ),
            code(
                "rust",
                r#"struct Cents(u32);

fn total(a: Cents, b: Cents) -> Cents {
    Cents(a.0 + b.0)
}

fn main() {
    let base = Cents(900);
    let topping = Cents(250);

    let Cents(sum) = total(base, topping);
    println!("{sum} cents");
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`Cents(u32)` is a struct with one unnamed field, reached as `.0`. The reason to write \
                 it rather than `u32` is the function signature: `total(a: Cents, b: Cents)` cannot be \
                 called with a plain number, so a `u32` that meant something else can never be passed \
                 by mistake. You can destructure it too, as `let Cents(inner) = price;` does.",
            ),
            table(
                &["Shape", "Access", "Use it when"],
                &[
                    &["`struct Topping { name, cost }`", "`.name`, `.cost`", "fields mean different things"],
                    &["`struct Cents(u32)`", "`.0`", "the type is the meaning"],
                    &["`struct Marker;`", "—", "the type carries no data (next lesson)"],
                ],
            ),
            callout(
                "A named wrapper beats a bare number",
                "`total(900, 250)` tells a reader nothing. `total(base, topping)` with both typed as \
                 `Cents` tells them the units. The compiler now enforces the units too.",
            ),
        ],
    )
}

fn unit_structs() -> SeedLesson {
    SeedLesson::new(
        "Unit Structs",
        vec![
            labelled(
                "THE IDEA",
                "A struct with no fields at all is a *unit struct*. It carries no data, but it still \
                 has a name and a type, which is occasionally exactly what you need.",
            ),
            code(
                "rust",
                r#"struct Takeaway;
struct DineIn;

fn describe(service: Takeaway) {
    println!("on the way");
}

fn main() {
    describe(Takeaway);
    // describe(DineIn); // would not compile: wrong type
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`Takeaway` and `DineIn` hold nothing, yet the compiler can tell them apart. That is \
                 the whole trick: a value of type `Takeaway` can never be passed where `DineIn` is \
                 expected, even though both are empty. You will meet this again when an enum variant \
                 carries no data — a unit struct is the single-value version of that idea.",
            ),
            callout(
                "Do not reach for one without a reason",
                "A unit struct that nothing distinguishes is noise. Use one when the type itself is \
                 the information — a marker, a choice, a state — and the data is irrelevant.",
            ),
        ],
    )
}

fn choosing_a_shape() -> SeedLesson {
    SeedLesson::new(
        "Struct vs Tuple vs Loose Variables",
        vec![
            labelled(
                "THE IDEA",
                "You now have three ways to hold grouped data. This lesson is a decision table rather \
                 than new syntax, because choosing well is the skill the rest of the module depends \
                 on.",
            ),
            table(
                &["Shape", "Reach for it when", "Example"],
                &[
                    &["loose variables", "never — unless the values are genuinely unrelated", "`x`, `y` as screen coordinates you never pass together"],
                    &["tuple `(T, U)`", "the order is obvious and the pair is throwaway", "`(String, u32)` returned once"],
                    &["tuple struct", "you want a named type but not named fields", "`Cents(u32)`"],
                    &["named struct", "fields mean different things and can be confused", "`Topping { name, extra_cost }`"],
                ],
            ),
            gap_fill(
                "The Pizza Order program stores each topping as a name and a cost. Which declaration \
                 fixes the bare tuple? Fill in the struct keyword and the field names.",
                r#"____ Topping {
    ____: String,
    ____: u32,
}"#,
                &[
                    ("s1", "struct", "It introduces a new type."),
                    ("s2", "name", "The text of the topping."),
                    ("s3", "extra_cost", "How much it adds, in cents."),
                ],
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The tuple `(String, u32)` would work, but every reader has to remember which slot is \
                 which, and nothing stops a caller writing `(250, \"Olives\")`. Naming both fields makes \
                 the value self-describing and the mistake impossible. That is the trade: a few more \
                 keystrokes for a lot less ambiguity.",
            ),
            callout(
                "The running program so far",
                "The Pizza Order now has a real `Topping` type instead of loose variables. Next we \
                 give it behaviour with methods, so a topping can describe and price itself.",
            ),
        ],
    )
}
