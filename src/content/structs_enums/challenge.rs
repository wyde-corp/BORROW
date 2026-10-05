//! Module 07 — the three module challenges.
//!
//! Open-ended by design: the learner writes the types and the transitions, and
//! the walkthrough is the last hint. The first models a lifecycle, the second
//! refactors a working tuple-heavy program, and the third asks for a design
//! from requirements.

use crate::content::types::{ExerciseKind, SeedExercise};

const TUPLE_PROGRAM: &str = r#"// A working program, written entirely with tuples.
// Refactor it into structs and an enum, then justify every type.

fn make_pizza(size: u32, topping: (&str, u32)) -> (u32, Vec<(&str, u32)>) {
    let current = (size, vec![topping]);
    current
}

fn total(pizza: &(u32, Vec<(&str, u32)>)) -> u32 {
    let mut sum = pizza.0;
    for t in &pizza.1 {
        sum += t.1;
    }
    sum
}

fn main() {
    let pizza = make_pizza(1300, ("Olives", 150));
    println!("{}", total(&pizza));
}"#;

const STATE_MACHINE_START: &str = r#"#[derive(Debug)]
enum OrderStatus {
    Placed,
    Preparing,
    OutForDelivery,
    Delivered,
    Cancelled,
}

// TODO: enforce the legal transitions with `match`.
// Which states may follow which? Which transitions are illegal?
fn advance(status: OrderStatus) -> Option<OrderStatus> {
    todo!()
}

fn main() {
    let mut status = OrderStatus::Placed;
    for _ in 0..3 {
        status = advance(status).expect("a legal step");
    }
    println!("{status:?}");
}"#;

const MEMU_REQUIREMENTS: &str = r#"// Requirements for DESIGN A MENU. No starter code on purpose.
//
// 1. A menu holds many pizzas; each pizza has a name, a base price, and a size.
// 2. A size is one of Small, Medium, or Large.
// 3. Each pizza is either available now, or unavailable with a reason.
// 4. A topping can add to a pizza; a topping is either free or costs cents.
// 5. A customer order holds a customer name, several pizzas, and a status.
//
// Decide for EACH of the above whether it is a struct or an enum, then write
// the definitions and a `fn total(order: &Order) -> u32`."#;

pub const STATE_WALKTHROUGH: &str = r#"A status is a lifecycle: it moves forward one legal step at a time.

  Placed         -> Preparing
  Preparing      -> OutForDelivery | Cancelled
  OutForDelivery -> Delivered | Cancelled
  Delivered      -> (terminal)
  Cancelled      -> (terminal)

The clean way to express it is a `match` that returns `Option<OrderStatus>`:
`Some(next)` for a legal step and `None` for an illegal one. Using `Option`
rather than panicking means the caller must handle refusal, which is what a
real order system does.

Why not push a `String` through and mutate it? Because then `Delivered ->
Preparing` is representable, and a bug can silently resurrect a completed
order. The enum plus the `match` makes illegal transitions impossible to
*return*, even though the caller can still attempt them."#;

pub const TUPLE_WALKTHROUGH: &str = r#"The tuple program has two smells: `(u32, Vec<(&str, u32)>)` and
`(&str, u32)` both mean something specific that the code never names.

A faithful refactor:

  struct Topping { name: String, extra_cost: u32 }
  struct Pizza   { base: u32, toppings: Vec<Topping> }

`make_pizza` becomes `Pizza { base, toppings: vec![topping] }` and `total`
becomes a method `pizza.total()`. Two things changed:

  - `u32` sizes are named `base`; the meaning is in the field, not a comment.
  - `&str` became `String`, because the pizza now owns its topping names
    rather than borrowing string literals. That is a real trade: the data is
    owned, so it can outlive the literals.

The justification part matters more than the code. For each type, say which of
'these values are one thing' or 'one of a fixed set' applied. If a tuple was
fine because it was genuinely throwaway, saying so is a valid answer too — the
exercise is the reasoning, not the rewrite."#;

pub fn challenges() -> Vec<SeedExercise> {
    vec![
        SeedExercise::new(
            None,
            ExerciseKind::Challenge,
            "PIZZA STATE MACHINE — An order moves through a lifecycle: placed, preparing, out for \
             delivery, delivered, cancelled. Model the status as an enum and enforce the legal \
             transitions. An illegal transition must be impossible to perform. For every pair of \
             states, say whether the move is allowed and why.",
            &[
                "List every state that can follow each state",
                "Write `fn advance(status: OrderStatus) -> Option<OrderStatus>`",
                "Return `None` for an illegal step rather than panicking",
                "Say which states are terminal, and why",
                "Explain why a `String` status would fail to enforce any of this",
                "Do not add data-carrying variants until the basic transitions work",
            ],
            0,
            STATE_WALKTHROUGH,
        )
        .code(STATE_MACHINE_START)
        .xp(150)
        .hints(&[
            "Start by drawing the graph of allowed moves before writing any code.",
            "Terminal states have no outgoing edges: `Delivered` and `Cancelled`.",
            "`Option` is the right return type because an illegal move has no next state.",
            "The walkthrough is the last hint. Compare your transition table to it.",
        ]),
        SeedExercise::new(
            None,
            ExerciseKind::Challenge,
            "TUPLE REFACTOR — The program below works, and it is written entirely with tuples. \
             Refactor it to use named structs, then justify every type you introduce. The \
             justifications are the real deliverable.",
            &[
                "Give every tuple that means something its own named type",
                "Decide which data should be owned and which borrowed, and say why",
                "Turn `total` into a method if that reads better",
                "Write one sentence per type: why a struct rather than a tuple or an enum",
                "Do not introduce an enum just to look thorough — only where there is a fixed set",
            ],
            0,
            TUPLE_WALKTHROUGH,
        )
        .code(TUPLE_PROGRAM)
        .xp(160)
        .hints(&[
            "There are two tuples here that each mean something specific.",
            "Named fields stop `(price, 1300)` and `(1300, price)` from being confusable.",
            "A type that is genuinely throwaway can stay a tuple — but say so.",
            "The walkthrough is the last hint; it shows one faithful refactor.",
        ]),
        SeedExercise::new(
            None,
            ExerciseKind::Challenge,
            "DESIGN A MENU — You are given requirements, not code. Decide which pieces are structs \
             and which are enums, define them, and write a function that totals an order. Every \
             choice must be defended in one sentence.",
            &[
                "For each requirement, ask: all of these, or one of these?",
                "A size is a fixed set — decide how to represent it",
                "Availability is either available or unavailable-with-a-reason — which shape?",
                "A topping's cost is either free or some cents — which shape?",
                "Write `fn total(order: &Order) -> u32` using `match` where a set is involved",
                "Be ready to say why each choice is not the other kind of type",
            ],
            0,
            "There is no single correct design, which is the point — but every choice should be \
             defensible. Sizes are an enum; availability is an enum whose unavailable variant \
             carries a reason; a topping's free-or-paid cost is an enum carrying the cents when \
             paid; an order and a pizza are structs. The test of each answer is the same: does the \
             type make an illegal state impossible, or does it merely document the rules in a \
             comment?",
        )
        .code(MEMU_REQUIREMENTS)
        .xp(200)
        .hints(&[
            "Start with the pieces that are clearly 'one of a fixed set' — sizes, availability.",
            "A variant can carry the data only that case needs, such as an unavailable reason.",
            "Structs compose; an order of pizzas of toppings is structs all the way down.",
            "If a rule lives only in a comment, the type has not been chosen yet.",
        ]),
    ]
}
