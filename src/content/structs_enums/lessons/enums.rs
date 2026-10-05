//! Module 07 — Structs & Enums, lessons 21–26.
//!
//! Enums. Where a struct says "all of these", an enum says "exactly one of
//! these" — and that lets Rust rule out the misspelling, the missing case, and
//! the state that cannot happen. The lesson ends by revealing that `Option` is
//! just an enum.

use crate::content::types::{
    callout, code, diagram, gap_fill, guess_run, labelled, paragraphs, step_through, table,
    SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        magic_strings(),
        defining_an_enum(),
        variant_data(),
        enums_vs_structs(),
        option_is_an_enum(),
        methods_on_enums(),
    ]
}

fn magic_strings() -> SeedLesson {
    SeedLesson::new(
        "The Magic-String Problem",
        vec![
            labelled(
                "CONCEPT",
                "An order moves through stages: placed, preparing, out for delivery, delivered, \
                 cancelled. The tempting way to store that is a `String`. Let us do it and see what \
                 the compiler cannot protect us from.",
            ),
            code(
                "rust",
                r#"fn main() {
    let status = String::from("preparing");

    if status == "prepairing" {
        println!("on its way");
    } else {
        println!("still waiting");
    }
}"#,
            ),
            labelled(
                "EXPLANATION",
                "This compiles and runs, and it prints `still waiting` even though the order is being \
                 prepared. The string `\"prepairing\"` is a typo, and nothing complained: to the \
                 compiler it is a perfectly good `String`, just a different one. Typos in string \
                 comparisons are invisible until a user notices.",
            ),
            labelled(
                "EXPLANATION",
                "There is a second, larger problem. `String` can hold anything — `\"banana\"`, an \
                 empty string, a value from a form. The set of legal statuses is four or five \
                 strings, but the type allows infinitely many. Every function that accepts a status \
                 must defend against values that should never exist.",
            ),
            callout(
                "The difference in one sentence",
                "A struct says *all of these together*. An enum says *exactly one of these, from a \
                 fixed list*. Status is not a string; it is one of five things. The type should say \
                 so.",
            ),
            paragraphs(&[
                "This is the same instinct as Module 07's structs: move a fact out of a name and into \
                 the type system, so the compiler can help. An enum is that move for 'one of a set'.",
            ]),
        ],
    )
}

fn defining_an_enum() -> SeedLesson {
    SeedLesson::new(
        "Defining an Enum",
        vec![
            labelled(
                "CONCEPT",
                "An enum lists its possible values, called *variants*. Making a value picks exactly \
                 one variant, written `Size::Medium`. Only listed variants exist, so a misspelling is \
                 now a compile error rather than a silent bug.",
            ),
            gap_fill(
                "Define a size that can only be one of three values. Fill in the keyword and the three \
                 variants.",
                r#"____ Size {
    ____,
    ____,
    ____,
}

fn main() {
    let s = Size::Medium;
    println!("{s:?}");
}"#,
                &[
                    ("e1", "enum", "The keyword for 'one of these'."),
                    ("v1", "Small", "A type name, so capitalised."),
                    ("v2", "Medium", "The middle one."),
                    ("v3", "Large", "The big one."),
                ],
            ),
            code(
                "rust",
                r#"#[derive(Debug)]
enum Size {
    Small,
    Medium,
    Large,
}

fn main() {
    let size = Size::Large;
    println!("{size:?}");
}"#,
            ),
            labelled(
                "EXPLANATION",
                "The variants are written with a leading capital because each is a type in its own \
                 right. You create a value with `Size::Large` — `Size` is the enum, `Large` is the \
                 variant. `#[derive(Debug)]` lets us print it while we are still learning; the next \
                 lesson after this one writes a method for readable output.",
            ),
            guess_run(
                "One variant name is misspelled. Predict what happens.",
                r#"#[derive(Debug)]
enum Size {
    Small,
    Medium,
    Large,
}

fn main() {
    let size = Size::Meduim;
    println!("{size:?}");
}"#,
                &[
                    "It compiles and prints `Meduim`",
                    "It does not compile — no variant named `Meduim`",
                    "It prints `Medium` after autocorrecting",
                ],
                1,
                "There is no variant called `Meduim`, so the compiler rejects it and suggests the \
                 closest real variant. This is the entire point: the error moved from a runtime \
                 behaviour a user would find, to a compile error the developer sees immediately.",
            ),
        ],
    )
}

fn variant_data() -> SeedLesson {
    SeedLesson::new(
        "Variants Can Carry Data",
        vec![
            labelled(
                "CONCEPT",
                "Enum variants are not limited to names. A variant can hold data, and different \
                 variants can hold different shapes. `Preparing` needs to know how long the order has \
                 been underway; `Delivered` does not.",
            ),
            step_through(
                "A status whose variants differ",
                r#"enum OrderStatus {
    Placed,
    Preparing { since_minutes: u32 },
    Delivered,
}
let status = OrderStatus::Preparing { since_minutes: 10 };"#,
                &[
                    (&[1][..], "`Placed` is a unit variant — it carries nothing."),
                    (&[2][..], "`Preparing` is a struct-like variant: it has a named field, `since_minutes`."),
                    (&[3][..], "`Delivered` carries nothing again."),
                    (&[5][..], "Constructing it looks like a struct literal, but with the variant name: `OrderStatus::Preparing { since_minutes: 10 }`."),
                ],
            ),
            code(
                "rust",
                r#"#[derive(Debug)]
enum OrderStatus {
    Placed,
    Preparing { since_minutes: u32 },
    OutForDelivery,
    Delivered,
    Cancelled,
}

fn main() {
    let status = OrderStatus::Preparing { since_minutes: 10 };
    println!("{status:?}");
}"#,
            ),
            labelled(
                "EXPLANATION",
                "Read `Preparing { since_minutes: u32 }` as a tiny anonymous struct bundled into the \
                 variant. `Placed` and `Delivered` hold nothing, like unit structs. The value is still \
                 exactly one variant at a time — you can never have a status that is both `Preparing` \
                 and `Delivered`.",
            ),
            callout(
                "This is why enums are more than constants",
                "If variants could only be names, an enum would be a fancier number. Because a \
                 variant can carry exactly the data that variant needs, the illegal state — a \
                 `Delivered` order with a preparation time — cannot be constructed at all.",
            ),
        ],
    )
}

fn enums_vs_structs() -> SeedLesson {
    SeedLesson::new(
        "Enums vs Structs",
        vec![
            labelled(
                "CONCEPT",
                "Struct and enum answer two different questions. This lesson is the decision, stated \
                 once, because confusing them is the most common modelling mistake beginners make.",
            ),
            diagram(
                "And versus or",
                "STRUCT — \"all of these\"      ENUM — \"exactly one of these\"\n\n ┌── name ──┐                  ┌── Small ──┐\n │  cost    │                  │  Medium   │\n │  veg     │                  └── Large ──┘\n └──────────┘                   (choose ONE)",
            ),
            table(
                &["Question about the data", "Answer", "Type"],
                &[
                    &["Is every one of these facts true at once?", "yes", "struct"],
                    &["Is it one option from a fixed set?", "one only", "enum"],
                    &["Can two of these be true together?", "yes", "struct"],
                    &["Is it now A, later B, but never both?", "one only", "enum"],
                ],
            ),
            gap_fill(
                "The Pizza Order has a size (one of three) and a topping (a name and a cost). In the \
                 blanks, write `struct` or `enum`.",
                r#"____ Size { Small, Medium, Large }

____ Topping {
    name: String,
    extra_cost: u32,
}"#,
                &[
                    ("k1", "enum", "A size is one of a fixed set."),
                    ("k2", "struct", "A topping has several facts at once."),
                ],
            ),
            callout(
                "A useful test",
                "Say the model out loud. \"A pizza has a size **and** a name **and** a price\" is a \
                 struct. \"A status is **either** placed **or** preparing **or** delivered\" is an \
                 enum. The conjunction and the disjunction are the tell.",
            ),
        ],
    )
}

fn option_is_an_enum() -> SeedLesson {
    SeedLesson::new(
        "`Option` Is an Enum",
        vec![
            labelled(
                "CONCEPT",
                "You have used `Option` and `.unwrap()` since the early modules. Here is the reveal: \
                 `Option` is not a language keyword. It is an ordinary enum with two variants, and \
                 you could have written it yourself.",
            ),
            step_through(
                "Some and None are variants",
                r#"enum Option<T> {
    None,
    Some(T),
}
let found: Option<u32> = Some(3);
let missing: Option<u32> = None;"#,
                &[
                    (&[1][..], "`Option` has exactly two variants: `None` and `Some`."),
                    (&[2][..], "`Some(T)` carries data — whatever is inside the option. The `<T>` is a generic, which we will meet properly later."),
                    (&[4][..], "A present value: `Some(3)`."),
                    (&[5][..], "An absent value: `None`. No data at all."),
                ],
            ),
            code(
                "rust",
                r#"fn main() {
    let found: Option<u32> = Some(3);
    let missing: Option<u32> = None;

    println!("{found:?} {missing:?}");
}"#,
            ),
            labelled(
                "EXPLANATION",
                "Everything you know about enums now applies to `Option`. It is one of two variants. \
                 It can carry data (`Some`) or not (`None`). A function returning `Option<Topping>` is \
                 saying 'either I found a topping, or there is none' — the same 'exactly one of' idea \
                 at the heart of every enum.",
            ),
            callout(
                "Nothing special, everything consistent",
                "The reason `Option` feels built-in is that Rust uses it for absence everywhere, and \
                 because the syntax `Some(x)` is imported for you. But it is an enum you understand \
                 completely after this lesson. Module 09 returns to it and to `Result`.",
            ),
        ],
    )
}

fn methods_on_enums() -> SeedLesson {
    SeedLesson::new(
        "Methods on Enums",
        vec![
            labelled(
                "CONCEPT",
                "An enum can have an `impl` block exactly like a struct. Methods are a natural fit for \
                 a question like 'what is this status called in the interface?', because they can ask \
                 which variant the value is.",
            ),
            gap_fill(
                "Give `OrderStatus` a method that returns a human label. Fill in the `impl` keyword and \
                 the missing arms.",
                r#"____ OrderStatus {
    fn label(&self) -> &str {
        match self {
            OrderStatus::Placed => "placed",
            OrderStatus::Preparing { .. } => ____,
            OrderStatus::OutForDelivery => "out for delivery",
            OrderStatus::Delivered => ____,
            OrderStatus::Cancelled => "cancelled",
        }
    }
}"#,
                &[
                    ("i1", "impl", "The block that holds methods."),
                    ("i2", "\"preparing\"", "The label for the Preparing variant."),
                    ("i3", "\"delivered\"", "The label for the Delivered variant."),
                ],
            ),
            labelled(
                "EXPLANATION",
                "Notice `OrderStatus::Preparing { .. }` — the `..` ignores the data inside the variant \
                 because the label does not depend on it. This `match` is the tool the next lessons \
                 are about; here it is enough to see that a method can look at which variant it has.",
            ),
            code(
                "rust",
                r#"#[derive(Debug)]
enum OrderStatus {
    Placed,
    Preparing { since_minutes: u32 },
    OutForDelivery,
    Delivered,
    Cancelled,
}

impl OrderStatus {
    fn label(&self) -> &str {
        match self {
            OrderStatus::Placed => "placed",
            OrderStatus::Preparing { .. } => "preparing",
            OrderStatus::OutForDelivery => "out for delivery",
            OrderStatus::Delivered => "delivered",
            OrderStatus::Cancelled => "cancelled",
        }
    }
}

fn main() {
    let status = OrderStatus::Preparing { since_minutes: 10 };
    println!("{}", status.label());
}"#,
            ),
            callout(
                "The Pizza Order now has both kinds of type",
                "`Pizza` and `Topping` are structs — several facts together. `Size` and \
                 `OrderStatus` are enums — one of a fixed set. The next lessons make `match` a tool \
                 you reach for without thinking.",
            ),
        ],
    )
}
