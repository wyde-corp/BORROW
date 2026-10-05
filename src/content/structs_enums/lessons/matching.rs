//! Module 07 — Structs & Enums, lessons 27–32.
//!
//! Matching. `match` is how an enum's "exactly one of" becomes useful: it asks
//! which variant a value is and runs the arm for it. Exhaustiveness is the
//! headline feature, and `if let` / `let else` are the focused forms for when
//! you only care about one variant.

use crate::content::types::{
    broken, callout, code, fixed, gap_fill, guess_run, labelled, step_through, table, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        match_basics(),
        matching_variant_data(),
        exhaustiveness(),
        wildcards(),
        if_let(),
        let_else(),
    ]
}

fn match_basics() -> SeedLesson {
    SeedLesson::new(
        "`match` Basics",
        vec![
            labelled(
                "CONCEPT",
                "A `match` takes a value and a list of arms. Each arm is a pattern, `=>`, and the \
                 code to run if the pattern fits. Exactly one arm runs, because the value is exactly \
                 one variant.",
            ),
            code(
                "rust",
                r#"enum Size {
    Small,
    Medium,
    Large,
}

fn price(size: Size) -> u32 {
    match size {
        Size::Small => 800,
        Size::Medium => 1000,
        Size::Large => 1300,
    }
}

fn main() {
    println!("{}", price(Size::Medium));
}"#,
            ),
            labelled(
                "EXPLANATION",
                "Read each arm as a question: 'is it `Size::Small`? then 800.' The patterns use the \
                 variant names with the enum prefix. Because `price` returns a `u32`, each arm must \
                 produce a `u32`, and all three do. The result of the chosen arm becomes the result \
                 of the `match`.",
            ),
            callout(
                "An arm is an expression, not a statement",
                "Most arms here are a single value with no braces. If an arm needs several lines, wrap \
                 them in braces and the last expression is the arm's value — the same rule as a \
                 function body.",
            ),
            table(
                &["Piece", "Meaning"],
                &[
                    &["`match size {`", "the value being examined"],
                    &["`Size::Small => 800,`", "a pattern, an arrow, a result"],
                    &["`,` between arms", "required, including after the last"],
                ],
            ),
        ],
    )
}

fn matching_variant_data() -> SeedLesson {
    SeedLesson::new(
        "Matching Variant Data",
        vec![
            labelled(
                "CONCEPT",
                "When a variant carries data, the pattern can pull that data out into a name. \
                 `OrderStatus::Preparing { since_minutes }` both checks the variant and binds the \
                 value inside it.",
            ),
            step_through(
                "Binding data out of a variant",
                r#"enum OrderStatus {
    Placed,
    Preparing { since_minutes: u32 },
    Delivered,
}

fn message(status: &OrderStatus) -> String {
    match status {
        OrderStatus::Placed => String::from("your order is in"),
        OrderStatus::Preparing { since_minutes } => format!("preparing for {since_minutes} min"),
        OrderStatus::Delivered => String::from("delivered"),
    }
}"#,
                &[
                    (&[7][..], "The arm only matches the `Preparing` variant."),
                    (&[8][..], "`{ since_minutes }` binds the field to a new local named `since_minutes`."),
                    (&[8][..], "That local is usable in the arm body, just like a function parameter."),
                    (&[9][..], "The other variants ignore data because they have none."),
                ],
            ),
            code(
                "rust",
                r#"enum OrderStatus {
    Placed,
    Preparing { since_minutes: u32 },
    Delivered,
}

fn message(status: &OrderStatus) -> String {
    match status {
        OrderStatus::Placed => String::from("your order is in"),
        OrderStatus::Preparing { since_minutes } => {
            format!("preparing, started {since_minutes} minutes ago")
        }
        OrderStatus::Delivered => String::from("delivered"),
    }
}

fn main() {
    let status = OrderStatus::Preparing { since_minutes: 7 };
    println!("{}", message(&status));
}"#,
            ),
            labelled(
                "EXPLANATION",
                "Matching by reference (`status: &OrderStatus`) means the bound `since_minutes` is a \
                 copy, because `u32` is `Copy`. If the variant held a `String`, the pattern would \
                 need `ref` or the match would move — a detail Module 06 has prepared you for.",
            ),
            callout(
                "The pattern does two jobs at once",
                "It decides *which* variant this is, and it hands you the data that variant was \
                 holding. That combination is why `match` is the natural partner to enums.",
            ),
        ],
    )
}

fn exhaustiveness() -> SeedLesson {
    SeedLesson::new(
        "Exhaustiveness",
        vec![
            labelled(
                "CONCEPT",
                "A `match` must cover every possible variant. Miss one and the program does not \
                 compile. This is the feature that makes enums safe: you cannot forget a case, and \
                 adding a variant later turns every incomplete `match` into an error that points at \
                 the work to do.",
            ),
            broken(
                r#"enum Size {
    Small,
    Medium,
    Large,
}

fn price(size: Size) -> u32 {
    match size {
        Size::Small => 800,
        Size::Medium => 1000,
    }
}

fn main() {
    println!("{}", price(Size::Large));
}"#,
            ),
            labelled(
                "EXPLANATION",
                "The compiler reports a non-exhaustive pattern and names the missing variant, \
                 `Size::Large`. It is not being pedantic: `price(Size::Large)` has no answer, and the \
                 compiler refuses to let that become a runtime surprise. Adding the arm fixes it.",
            ),
            fixed(
                r#"enum Size {
    Small,
    Medium,
    Large,
}

fn price(size: Size) -> u32 {
    match size {
        Size::Small => 800,
        Size::Medium => 1000,
        Size::Large => 1300,
    }
}

fn main() {
    println!("{}", price(Size::Large));
}"#,
            ),
            callout(
                "Adding a variant is a guided change",
                "When you add `ExtraLarge` to `Size`, every `match` that forgot it fails to compile \
                 with the exact variant named. The compiler becomes a checklist for the places your \
                 new option needs handling — this is the strongest argument for enums over strings.",
            ),
        ],
    )
}

fn wildcards() -> SeedLesson {
    SeedLesson::new(
        "Wildcards and `_`",
        vec![
            labelled(
                "CONCEPT",
                "Sometimes many variants share one outcome, and listing them separately is noise. A \
                 `_` arm matches anything not already handled. It satisfies exhaustiveness, but it \
                 also opts out of the compiler's future warnings — which is a real trade.",
            ),
            gap_fill(
                "Collapse the three non-cancelled outcomes into one arm. Fill in the wildcard pattern.",
                r#"fn is_active(status: &OrderStatus) -> bool {
    match status {
        OrderStatus::Cancelled => false,
        ____ => true,
    }
}"#,
                &[("w1", "_", "Matches every remaining variant.")],
            ),
            code(
                "rust",
                r#"enum OrderStatus {
    Placed,
    Preparing,
    Delivered,
    Cancelled,
}

fn is_active(status: &OrderStatus) -> bool {
    match status {
        OrderStatus::Cancelled => false,
        _ => true,
    }
}

fn main() {
    println!("{}", is_active(&OrderStatus::Preparing));
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`_ => true` covers `Placed`, `Preparing`, and `Delivered`. It reads well here because \
                 they genuinely share an outcome. But if you later add a `Refunded` variant, this \
                 `match` will *not* fail — `_` swallows it, and `is_active` silently answers `true` \
                 for a state you never considered.",
            ),
            guess_run(
                "A new variant is added, but the wildcard still compiles. Predict what the program \
                 prints.",
                r#"enum OrderStatus {
    Placed,
    Refunded,
}

fn is_active(status: &OrderStatus) -> bool {
    match status {
        OrderStatus::Placed => true,
        _ => true,
    }
}

fn main() {
    println!("{}", is_active(&OrderStatus::Refunded));
}"#,
                &[
                    "true — the wildcard catches Refunded",
                    "It does not compile, because Refunded is unhandled",
                    "false",
                ],
                0,
                "The `_` arm makes the compiler accepting even though `Refunded` was never \
                 considered, so a refunded order is reported as active. Use `_` only when the \
                 remaining variants really do share one behaviour, and prefer listing variants when \
                 a future addition should force a decision.",
            ),
            callout(
                "Wildcards trade safety for brevity",
                "When the set is small and closed, name every variant and let exhaustiveness protect \
                 you. When a wildcard is right, be sure the 'everything else' branch is genuinely \
                 everything-else, forever.",
            ),
        ],
    )
}

fn if_let() -> SeedLesson {
    SeedLesson::new(
        "`if let`",
        vec![
            labelled(
                "CONCEPT",
                "When you care about exactly one variant and want to ignore the rest, a full `match` \
                 is heavy. `if let Some(x) = …` runs its block only when the pattern matches, and \
                 says nothing about the other cases.",
            ),
            code(
                "rust",
                r#"struct Topping {
    name: String,
    extra_cost: u32,
}

fn find<'a>(toppings: &'a [Topping], want: &str) -> Option<&'a Topping> {
    toppings.iter().find(|t| t.name == want)
}

fn main() {
    let toppings = vec![
        Topping { name: String::from("Olives"), extra_cost: 150 },
        Topping { name: String::from("Onion"), extra_cost: 100 },
    ];

    if let Some(found) = find(&toppings, "Olives") {
        println!("found {} at +{}", found.name, found.extra_cost);
    } else {
        println!("not on the menu");
    }
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`if let Some(found) = find(...)` runs the block only when the result is `Some`, \
                 binding the topping to `found`. The `else` is optional. It is shorthand for a \
                 `match` with one real arm and a wildcard, and it is the idiomatic way to act on a \
                 found-or-absent value.",
            ),
            callout(
                "Use `match` when you need the other cases",
                "`if let` deliberately stops the compiler from making you think about everything else. \
                 That is fine when 'otherwise, do nothing' is truly the intent. If the absent case \
                 needs a real response, write a `match`.",
            ),
        ],
    )
}

fn let_else() -> SeedLesson {
    SeedLesson::new(
        "`let … else`",
        vec![
            labelled(
                "CONCEPT",
                "`if let` buries the main path inside an `if`. `let … else` puts the main path first \
                 and sends the unmatched case away in the `else` block, which must leave the \
                 surrounding function — `return`, `break`, `continue`, or panic.",
            ),
            gap_fill(
                "Fill in the keyword that starts the early-exit branch, and the keyword that leaves \
                 the function.",
                r#"fn describe(size: Option<&str>) -> String {
    let ____(name) = size ____ {
        return String::from("unknown size");
    };

    format!("size: {name}")
}"#,
                &[
                    ("l1", "Some", "The pattern that must match."),
                    ("l2", "else", "The branch taken when it does not."),
                ],
            ),
            code(
                "rust",
                r#"enum Size {
    Small,
    Medium,
    Large,
}

fn parse(text: &str) -> Option<Size> {
    match text {
        "small" => Some(Size::Small),
        "medium" => Some(Size::Medium),
        "large" => Some(Size::Large),
        _ => None,
    }
}

fn price_or_default(text: &str) -> u32 {
    let Some(size) = parse(text) else {
        return 1000;
    };

    match size {
        Size::Small => 800,
        Size::Medium => 1000,
        Size::Large => 1300,
    }
}

fn main() {
    println!("{}", price_or_default("large"));
    println!("{}", price_or_default("huge"));
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`let Some(size) = parse(text) else { return 1000; };` reads as: bind `size` if the \
                 parse succeeded, otherwise leave now. The happy path then runs at the top level of \
                 the function instead of being indented inside a branch. The `else` block must \
                 diverge — it cannot fall through with no value.",
            ),
            callout(
                "Three shapes, one decision",
                "`match` when every case matters. `if let` when one case matters and the rest do \
                 nothing. `let … else` when one case matters and the rest are errors you handle up \
                 front. Choosing the right shape keeps functions flat and readable.",
            ),
        ],
    )
}
