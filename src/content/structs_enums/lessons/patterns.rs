//! Module 07 — Structs & Enums, lessons 33–35.
//!
//! Patterns and the real world. Destructuring inside `match`, the ten mistakes
//! that most often mean the model has not fully landed, and the completed Pizza
//! Order program read end to end.

use crate::content::types::{
    callout, code, guess_run, labelled, paragraphs, step_through, table, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![destructuring(), common_mistakes(), putting_it_together()]
}

fn destructuring() -> SeedLesson {
    SeedLesson::new(
        "Destructuring in `match`",
        vec![
            labelled(
                "CONCEPT",
                "A pattern can take a struct apart field by field. You do not need to match a value \
                 and then read its fields with dots — the pattern binds them directly.",
            ),
            step_through(
                "Pulling a struct apart in a pattern",
                r#"struct Topping {
    name: String,
    extra_cost: u32,
    vegetarian: bool,
}

fn label(t: &Topping) -> String {
    match t {
        Topping { vegetarian: true, name, .. } => format!("{name} (veg)"),
        Topping { name, .. } => format!("{name}"),
    }
}"#,
                &[
                    (&[7][..], "The first arm matches a `Topping` whose `vegetarian` field is `true`."),
                    (&[7][..], "`name` binds that field, and `..` says the remaining fields are ignored."),
                    (&[8][..], "The second arm matches any other topping and binds only `name`."),
                    (&[9][..], "Every topping matches exactly one arm, so the `match` is exhaustive."),
                ],
            ),
            code(
                "rust",
                r#"struct Topping {
    name: String,
    extra_cost: u32,
    vegetarian: bool,
}

fn label(t: &Topping) -> String {
    match t {
        Topping { vegetarian: true, name, .. } => format!("{name} (veg)"),
        Topping { name, .. } => format!("{name}"),
    }
}

fn main() {
    let olives = Topping {
        name: String::from("Olives"),
        extra_cost: 150,
        vegetarian: true,
    };

    println!("{}", label(&olives));
}"#,
            ),
            labelled(
                "EXPLANATION",
                "Field order in a pattern is free and `..` stands in for the fields you do not mention. \
                 This works on enums too: an `OrderStatus::Preparing { since_minutes }` pattern is \
                 destructuring a variant. Patterns and construction are mirror images — you build \
                 with `Name { field: value }` and take apart with `Name { field }`.",
            ),
            callout(
                "`vegetarian: true` is a literal pattern",
                "The pattern is not just binding a name; it is requiring the field to equal `true`. \
                 A binding like `name` matches anything, but `true` matches only `true`. Mixing \
                 bindings and literals in one pattern is exactly what makes `match` expressive.",
            ),
        ],
    )
}

fn common_mistakes() -> SeedLesson {
    SeedLesson::new(
        "The Ten Common Mistakes",
        vec![
            labelled(
                "CONCEPT",
                "These are the mistakes that survive long after the syntax is learned. Read them as a \
                 checklist against your own understanding.",
            ),
            table(
                &["#", "Mistake", "The correction"],
                &[
                    &["1", "treating a struct definition as a value", "it is a shape; you must construct one"],
                    &["2", "forgetting a field in a literal", "every field is required; the error names the missing one"],
                    &["3", "thinking field order in a literal matters", "names connect values to fields, not order"],
                    &["4", "using `.` to move a `String` field", "reading a field may move it; borrow with `&` if you need it"],
                    &["5", "expecting `{}` to print a struct", "derive `Debug` and use `{:?}`"],
                    &["6", "thinking an enum can be two variants at once", "a value is exactly one variant"],
                    &["7", "using a `String` for a fixed set of states", "that is what an enum is for"],
                    &["8", "matching on an enum and missing a variant", "exhaustiveness makes this a compile error — add the arm"],
                    &["9", "reaching for `_` everywhere", "a wildcard silences future additions; list variants when a new one should force a decision"],
                    &["10", "confusing `Option` with a special language feature", "`Option` is an ordinary enum with `Some` and `None`"],
                ],
            ),
            guess_run(
                "A field is read with `.` and then used again, and the field is a `String`. Predict \
                 what happens.",
                r#"struct Topping {
    name: String,
    extra_cost: u32,
}

fn main() {
    let t = Topping { name: String::from("Olives"), extra_cost: 150 };

    let moved = t.name;
    println!("{}", t.extra_cost);
    println!("{}", t.name);
}"#,
                &[
                    "It compiles — reading a field copies it",
                    "It does not compile — `t.name` was moved out of `t`",
                    "It compiles and prints the name twice",
                ],
                1,
                "`let moved = t.name;` moves the `String` out of the struct, which partially moves \
                 `t`. Reading `t.extra_cost` is fine because `u32` is `Copy`, but the second \
                 `t.name` is a use-after-move. Use `t.name.clone()` or borrow with `&t.name` when \
                 the field must stay.",
            ),
            callout(
                "Use the list in reverse",
                "When something will not compile, find which of these ten it is. Nine times in ten \
                 the answer is here, and the correction is the sentence that follows it.",
            ),
        ],
    )
}

fn putting_it_together() -> SeedLesson {
    SeedLesson::new(
        "Putting It All Together",
        vec![
            labelled(
                "CONCEPT",
                "Here is the whole Pizza Order program as it stands: structs for the things made of \
                 parts, enums for the things that are one of a set, methods for behaviour, and a \
                 `match` to price a size and name a status.",
            ),
            code(
                "rust",
                r#"#[derive(Debug, Clone, PartialEq)]
enum Size {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, PartialEq)]
struct Topping {
    name: String,
    extra_cost: u32,
}

struct Pizza {
    size: Size,
    toppings: Vec<Topping>,
}

impl Pizza {
    fn new(size: Size) -> Pizza {
        Pizza { size, toppings: Vec::new() }
    }

    fn add_topping(&mut self, topping: Topping) {
        self.toppings.push(topping);
    }

    fn price(&self) -> u32 {
        let base = match self.size {
            Size::Small => 800,
            Size::Medium => 1000,
            Size::Large => 1300,
        };
        base + self.toppings.iter().map(|t| t.extra_cost).sum::<u32>()
    }
}

#[derive(Debug)]
enum OrderStatus {
    Placed,
    Preparing { since_minutes: u32 },
    OutForDelivery,
    Delivered,
    Cancelled,
}

struct Order {
    customer: String,
    pizzas: Vec<Pizza>,
    status: OrderStatus,
}

impl Order {
    fn new(customer: &str) -> Order {
        Order {
            customer: customer.to_string(),
            pizzas: Vec::new(),
            status: OrderStatus::Placed,
        }
    }

    fn add_pizza(&mut self, pizza: Pizza) {
        self.pizzas.push(pizza);
    }

    fn total(&self) -> u32 {
        self.pizzas.iter().map(|p| p.price()).sum()
    }

    fn status_label(&self) -> String {
        match &self.status {
            OrderStatus::Placed => String::from("placed"),
            OrderStatus::Preparing { since_minutes } => {
                format!("preparing ({since_minutes} min)")
            }
            OrderStatus::OutForDelivery => String::from("out for delivery"),
            OrderStatus::Delivered => String::from("delivered"),
            OrderStatus::Cancelled => String::from("cancelled"),
        }
    }
}

fn main() {
    let mut order = Order::new("Gus");

    let mut pizza = Pizza::new(Size::Large);
    pizza.add_topping(Topping { name: String::from("Olives"), extra_cost: 150 });

    order.add_pizza(pizza);
    order.status = OrderStatus::Preparing { since_minutes: 5 };

    println!("{} — {} — {} cents", order.customer, order.status_label(), order.total());
}"#,
            ),
            labelled(
                "EXPLANATION",
                "Trace the types, not the lines. `Size` and `OrderStatus` are enums — one of a set. \
                 `Topping`, `Pizza`, and `Order` are structs — several facts together. `Pizza` and \
                 `Order` are also composed: an order owns a list of pizzas, each owning its toppings. \
                 `price` and `status_label` use `match` to turn a variant into a number or a string.",
            ),
            paragraphs(&[
                "Try the design questions yourself before the challenges. What would change if a \
                 pizza could be half-and-half? Where would the two sizes live? Is a discount a field \
                 on the order or a variant of something? These are enum-versus-struct decisions, and \
                 having an opinion is the point of the module.",
            ]),
            callout(
                "Change the design, do not just read it",
                "Reading this program proves you can follow Rust. Changing it — adding a variant, \
                 moving a field, choosing a different receiver — is what proves you can write it.",
            ),
        ],
    )
}
