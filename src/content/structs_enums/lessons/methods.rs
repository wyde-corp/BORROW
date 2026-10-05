//! Module 07 — Structs & Enums, lessons 14–20.
//!
//! Methods. Data becomes behaviour: `impl` blocks, the three receivers, the
//! constructor convention, and the derives that make structs printable,
//! comparable, and clonable — each introduced only when the Pizza Order needs
//! it.

use crate::content::types::{
    broken, callout, code, gap_fill, guess_run, labelled, step_through, table, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        methods_with_impl(),
        receivers(),
        associated_functions(),
        multiple_impls(),
        derive_debug(),
        derive_clone_eq(),
        consuming_methods(),
    ]
}

fn methods_with_impl() -> SeedLesson {
    SeedLesson::new(
        "Methods with `impl`",
        vec![
            labelled(
                "CONCEPT",
                "A method is a function that belongs to a type. You attach it in an `impl` block: \
                 `impl Pizza { fn price(&self) -> u32 { … } }`. Inside, `self` is the value the method \
                 was called on.",
            ),
            gap_fill(
                "Attach a method to `Pizza`. Fill in the keyword that introduces the block and the \
                 receiver type for a method that only reads.",
                r#"____ Pizza {
    fn topping_total(____) -> u32 {
        let mut sum = 0;
        for t in &self.toppings {
            sum += t.extra_cost;
        }
        sum
    }
}"#,
                &[
                    ("i1", "impl", "The block that holds methods."),
                    ("i2", "&self", "Read-only access to the pizza."),
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

impl Pizza {
    fn topping_total(&self) -> u32 {
        let mut sum = 0;
        for t in &self.toppings {
            sum += t.extra_cost;
        }
        sum
    }
}

fn main() {
    let pizza = Pizza {
        size: String::from("large"),
        toppings: vec![Topping { name: String::from("Olives"), extra_cost: 150 }],
    };

    println!("toppings: {} cents", pizza.topping_total());
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`pizza.topping_total()` is sugar for `Pizza::topping_total(&pizza)`. The first \
                 parameter `&self` is what makes it a method rather than a free function, and it is \
                 why you call it with a dot. The method borrows the pizza, sums the fields, and gives \
                 the total back without changing anything.",
            ),
            callout(
                "Methods keep behaviour next to data",
                "A `topping_total` free function would need the pizza passed in every time. As a \
                 method it reads as `pizza.topping_total()`, and it lives next to the fields it \
                 understands. That is the whole reason `impl` blocks exist.",
            ),
        ],
    )
}

fn receivers() -> SeedLesson {
    SeedLesson::new(
        "The Three Receivers",
        vec![
            labelled(
                "CONCEPT",
                "A method's first parameter is always some form of `self`, and there are three: \
                 `&self` to read, `&mut self` to change, and `self` to take ownership. Choosing the \
                 right one is the method-level version of the borrowing decision you already know.",
            ),
            table(
                &["Receiver", "What the method may do", "Caller keeps the value?"],
                &[
                    &["`&self`", "read the fields", "yes"],
                    &["`&mut self`", "change the fields", "yes"],
                    &["`self`", "consume or hand the value on", "no"],
                ],
            ),
            step_through(
                "Same method, three receivers",
                r#"let mut pizza = Pizza::new(String::from("large"));
pizza.add_topping(Topping { name: String::from("Olives"), extra_cost: 150 });
let total = pizza.topping_total();"#,
                &[
                    (&[1][..], "`Pizza::new` is an associated function; it builds a pizza and gives it back."),
                    (&[2][..], "`add_topping` takes `&mut self`, so it can change the pizza in place while `pizza` keeps owning it."),
                    (&[3][..], "`topping_total` takes `&self`; it only reads, so a shared borrow is enough."),
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

impl Pizza {
    fn new(size: String) -> Pizza {
        Pizza { size, toppings: Vec::new() }
    }

    fn add_topping(&mut self, topping: Topping) {
        self.toppings.push(topping);
    }

    fn topping_total(&self) -> u32 {
        self.toppings.iter().map(|t| t.extra_cost).sum()
    }
}

fn main() {
    let mut pizza = Pizza::new(String::from("large"));
    pizza.add_topping(Topping { name: String::from("Olives"), extra_cost: 150 });

    println!("{} cents", pizza.topping_total());
}"#,
            ),
            callout(
                "Read the receiver as a promise",
                "`&self` promises the caller their pizza is untouched. `&mut self` promises it may \
                 change. `self` promises it will be gone afterwards. A caller can tell what a method \
                 does to their value from the receiver alone.",
            ),
        ],
    )
}

fn associated_functions() -> SeedLesson {
    SeedLesson::new(
        "Associated Functions",
        vec![
            labelled(
                "CONCEPT",
                "A function in an `impl` block with no `self` is an *associated function*. It belongs \
                 to the type but is called as `Pizza::new(...)`, not `pizza.new(...)`. Constructors \
                 are the classic example.",
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

impl Pizza {
    fn new(size: String) -> Pizza {
        Pizza { size, toppings: Vec::new() }
    }

    fn margherita() -> Pizza {
        Pizza {
            size: String::from("medium"),
            toppings: vec![Topping { name: String::from("Cheese"), extra_cost: 0 }],
        }
    }
}

fn main() {
    let plain = Pizza::new(String::from("large"));
    let classic = Pizza::margherita();

    println!("{} and {}", plain.size, classic.size);
}"#,
            ),
            gap_fill(
                "Fill in the two calls. One builds an empty pizza; one builds the classic.",
                r#"let a = Pizza____(String::from("large"));
let b = Pizza____();"#,
                &[
                    ("a1", "::new", "The constructor-style associated function."),
                    ("a2", "::margherita", "The named preset."),
                ],
            ),
            labelled(
                "EXPLANATION",
                "`Pizza::new` is a convention many Rust types follow, not a keyword. It returns a \
                 `Pizza`. `Pizza::margherita` is a second constructor that names a common \
                 configuration. Both are ordinary associated functions — the only difference from a \
                 method is the missing `self`.",
            ),
            callout(
                "`::` for types, `.` for values",
                "`Pizza::new(...)` names a function that lives on the type. `pizza.price()` calls a \
                 method on a value. The separator tells you which kind of call you are reading.",
            ),
        ],
    )
}

fn multiple_impls() -> SeedLesson {
    SeedLesson::new(
        "Multiple `impl` Blocks",
        vec![
            labelled(
                "CONCEPT",
                "A type may have more than one `impl` block, and there is no behavioural difference. \
                 They are a way to organise methods by intention.",
            ),
            code(
                "rust",
                r#"struct Pizza {
    size: String,
    toppings: Vec<String>,
}

impl Pizza {
    fn new(size: String) -> Pizza {
        Pizza { size, toppings: Vec::new() }
    }
}

impl Pizza {
    fn describe(&self) -> String {
        format!("{} with {} toppings", self.size, self.toppings.len())
    }
}

fn main() {
    let pizza = Pizza::new(String::from("small"));
    println!("{}", pizza.describe());
}"#,
            ),
            labelled(
                "EXPLANATION",
                "Splitting `impl` blocks changes nothing the compiler sees; the methods all attach to \
                 `Pizza`. The usual reasons to split are to keep constructors together, or — in later \
                 modules — to separate methods provided by different traits. Do not split just to \
                 look organised.",
            ),
            callout(
                "One `impl` is the default",
                "Until you have a reason, one block is clearer. This lesson exists so that code you \
                 meet later, which does split them, does not look like it means something it does \
                 not.",
            ),
        ],
    )
}

fn derive_debug() -> SeedLesson {
    SeedLesson::new(
        "Deriving `Debug`",
        vec![
            labelled(
                "CONCEPT",
                "You cannot print a struct with `{}` — Rust does not know how you would like it \
                 formatted. `{}` is for types with a human display; `{:?}` is the developer view, and \
                 `#[derive(Debug)]` gives your struct one.",
            ),
            gap_fill(
                "Make the pizza printable with the developer formatter. Fill in the attribute and the \
                 format specifier.",
                r#"____(Debug)]
struct Pizza {
    size: String,
    toppings: u32,
}

fn main() {
    let p = Pizza { size: String::from("large"), toppings: 2 };
    println!("{:____}", p);
}"#,
                &[
                    ("d1", "derive", "Asks the compiler to write the impl."),
                    ("d2", "?", "The debug format specifier."),
                ],
            ),
            guess_run(
                "Predict what this prints.",
                r#"#[derive(Debug)]
struct Topping {
    name: String,
    extra_cost: u32,
}

fn main() {
    let t = Topping { name: String::from("Olives"), extra_cost: 150 };
    println!("{t:?}");
}"#,
                &[
                    "Topping { name: \"Olives\", extra_cost: 150 }",
                    "Olives 150",
                    "It does not compile",
                ],
                0,
                "The derived `Debug` prints the type name and each field with its name, using the \
                 debug formatter for the values — which is why the string has quotes. The `{:#?}` \
                 form prints the same information one field per line, which is easier to read on a \
                 wide struct.",
            ),
            callout(
                "Debug is for you, Display is for users",
                "`{:?}` is not meant to be pretty; it is meant to be unambiguous and complete. When \
                 you want a friendly sentence, you write a method — which is exactly what \
                 `describe()` is for.",
            ),
        ],
    )
}

fn derive_clone_eq() -> SeedLesson {
    SeedLesson::new(
        "Deriving `Clone` and `PartialEq`",
        vec![
            labelled(
                "CONCEPT",
                "Two more derives earn their place quickly: `Clone` lets you copy a struct on purpose, \
                 and `PartialEq` lets `==` compare two of them. Both are opt-in, and both only work \
                 when every field supports them.",
            ),
            code(
                "rust",
                r#"#[derive(Clone, PartialEq, Debug)]
struct Topping {
    name: String,
    extra_cost: u32,
}

fn main() {
    let a = Topping { name: String::from("Olives"), extra_cost: 150 };
    let b = a.clone();

    println!("{}", a == b);
    println!("{:?}", b);
}"#,
            ),
            labelled(
                "EXPLANATION",
                "Without `Clone`, `let b = a;` would move and `a` would be gone. With it, \
                 `a.clone()` produces a second, independent value, and `a` is still usable. Without \
                 `PartialEq`, `a == b` is a compile error even though the fields match; the derive \
                 makes `==` compare every field. `Clone` allocates, so reach for it deliberately, as \
                 Module 05 taught.",
            ),
            guess_run(
                "Two toppings with the same name but different costs. Predict the result.",
                r#"#[derive(PartialEq, Debug)]
struct Topping {
    name: String,
    extra_cost: u32,
}

fn main() {
    let a = Topping { name: String::from("Olives"), extra_cost: 150 };
    let b = Topping { name: String::from("Olives"), extra_cost: 200 };
    println!("{}", a == b);
}"#,
                &[
                    "true — the names match",
                    "false — every field must match, and the costs differ",
                    "It does not compile",
                ],
                1,
                "Derived `PartialEq` compares every field in order. `name` matches but `extra_cost` \
                 does not, so the values are not equal. Equality means all fields equal, which is \
                 what you almost always want from `==`.",
            ),
            callout(
                "Derives are opt-in for a reason",
                "A `Clone` is an allocation and can hide a design problem; an `==` can be expensive. \
                 Rust makes you ask. Add a derive when the Pizza Order needs it, not by reflex on \
                 every struct.",
            ),
        ],
    )
}

fn consuming_methods() -> SeedLesson {
    SeedLesson::new(
        "Methods That Consume `self`",
        vec![
            labelled(
                "CONCEPT",
                "A method whose first parameter is `self` — no ampersand — takes ownership of the \
                 value. The caller cannot use it afterwards. This is right when the method transforms \
                 the value into a new one and the old one has no further use.",
            ),
            code(
                "rust",
                r#"struct Pizza {
    size: String,
    toppings: Vec<String>,
}

impl Pizza {
    fn with_cheese(mut self) -> Pizza {
        self.toppings.push(String::from("Cheese"));
        self
    }

    fn describe(&self) -> String {
        format!("{} with {} toppings", self.size, self.toppings.len())
    }
}

fn main() {
    let pizza = Pizza { size: String::from("large"), toppings: Vec::new() }
        .with_cheese()
        .with_cheese();

    println!("{}", pizza.describe());
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`with_cheese(mut self)` takes the pizza, adds a topping, and returns the same pizza \
                 back so calls can be chained. Because each call consumes and returns, the original \
                 binding is replaced by the result. This builder style reads top to bottom and never \
                 clones.",
            ),
            broken(
                r#"struct Pizza {
    size: String,
}

impl Pizza {
    fn into_size(self) -> String {
        self.size
    }
}

fn main() {
    let pizza = Pizza { size: String::from("large") };
    let size = pizza.into_size();

    println!("{}", pizza.size);
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`into_size(self)` moved the whole pizza in, so `pizza` is consumed and `pizza.size` \
                 afterwards is a use-after-move. The name `into_` signals consumption by convention; \
                 the receiver `self` is what enforces it. If the size were not needed enough to \
                 justify consuming, the method would take `&self` and return `&str` instead.",
            ),
            callout(
                "Choosing a receiver is a design decision",
                "Ask two questions: does the method need to change the value? Does the caller still \
                 need it afterwards? Read-only → `&self`; change in place → `&mut self`; hand it on \
                 and forget it → `self`. The Pizza Order is now a small, honest API.",
            ),
        ],
    )
}
