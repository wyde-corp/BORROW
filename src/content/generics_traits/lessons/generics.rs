//! Module 10 — Generics & Traits, lessons 1–7.
//!
//! Generics. The duplicated function, a function parameterised over a type,
//! generic structs and enums (and the reveal that `Option` and `Result` were
//! always generic), monomorphisation, `where` clauses, and generics in methods.

use crate::content::types::{
    callout, code, diagram, dispatch, gap_fill, labelled, paragraphs, step_through, table,
    SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        the_duplicated_function(),
        generic_functions(),
        generic_types(),
        more_than_one(),
        monomorphisation(),
        where_clauses(),
        generics_in_methods(),
    ]
}

fn the_duplicated_function() -> SeedLesson {
    SeedLesson::new(
        "The Duplicated Function",
        vec![
            labelled(
                "THE IDEA",
                "The media library needs the longest item — the biggest page count, the most words, \
                 the longest video. You write a function that returns the largest value in a list. It \
                 works for `i32`. It does not work for `f64`, so you write it again.",
            ),
            code(
                "rust",
                r#"fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_f64(list: &[f64]) -> f64 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let pages = vec![120, 340, 87];
    let ratings = vec![4.5, 4.9, 3.8];

    println!("{} {}", largest_i32(&pages), largest_f64(&ratings));
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The two functions are identical except for the type. That is a smell: any bug fixed \
                 in one has to be fixed in the other, and the next type — a `String`, a `Book`'s page \
                 count, a `u64` — needs a third copy. You have been writing this duplication-removal \
                 by hand; Rust has a tool for it.",
            ),
            callout(
                "One function, any comparable type",
                "The body only ever does `>`, `=` and copy. It does not care *which* type it is \
                 sorting through — only that the type can be compared and copied. A **generic** \
                 function lets you say exactly that: 'any type `T` that can be compared'.",
            ),
            paragraphs(&[
                "You have already used generic types this whole course: `Vec<T>`, `Option<T>`, \
                 `Result<T, E>`, `HashMap<K, V>`. This module is where you learn to write your own.",
            ]),
        ],
    )
}

fn generic_functions() -> SeedLesson {
    SeedLesson::new(
        "Generic Functions",
        vec![
            labelled(
                "THE IDEA",
                "A generic function takes a type parameter in angle brackets, `fn largest<T>`, and \
                 uses `T` where a concrete type would go. A **bound** — `T: PartialOrd + Copy` — says \
                 what the function needs `T` to be able to do.",
            ),
            gap_fill(
                "Write the type parameter and the bound so the body compiles. Fill in the three \
                 blanks.",
                r#"fn largest<____>(list: &[T]) -> T
____ T: ____ + Copy,
{
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}"#,
                &[
                    ("t1", "T", "The type parameter's name."),
                    ("w1", "where", "Introduces the bound."),
                    ("b1", "PartialOrd", "The trait that gives `>`."),
                ],
            ),
            code(
                "rust",
                r#"fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let pages = vec![120, 340, 87];
    let ratings = vec![4.5, 4.9, 3.8];

    println!("{} {}", largest(&pages), largest(&ratings));
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`<T>` introduces a type parameter. `T: PartialOrd + Copy` is the bound: `PartialOrd` \
                 is the trait that provides `>`, and `Copy` is what lets `for &item` copy each element \
                 out. With that, one function replaces both copies. Call it with `&[i32]` and `T` is \
                 `i32`; call it with `&[f64]` and `T` is `f64`.",
            ),
            table(
                &["If the bound is", "`>` works", "`&item` copies", "Compiles"],
                &[
                    &["`T: PartialOrd + Copy`", "yes", "yes", "yes"],
                    &["`T: PartialOrd`", "yes", "no", "no — cannot copy the element"],
                    &["`T` (no bound)", "no", "yes", "no — `>` is not known"],
                ],
            ),
            callout(
                "The bound is a promise the caller keeps",
                "A bound is not something the function does; it is something the function *requires*. \
                 When the compiler says 'the trait bound `T: PartialOrd` is not satisfied', it means a \
                 caller passed a type that cannot do what the function needs.",
            ),
        ],
    )
}

fn generic_types() -> SeedLesson {
    SeedLesson::new(
        "Generic Structs and Enums",
        vec![
            labelled(
                "THE IDEA",
                "Types can be generic too. `struct Point<T> { x: T, y: T }` describes a pair of \
                 whatever type you choose. And the reveal: `Option` and `Result` are just generic \
                 enums — you have been writing generic code since Module 07.",
            ),
            step_through(
                "A Point of any one type",
                r#"struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let ints = Point { x: 1, y: 2 };
    let floats = Point { x: 1.0, y: 2.0 };
    println!("{} {}", ints.x, floats.y);
}"#,
                &[
                    (&[1][..], "`Point<T>` has one type parameter, `T`."),
                    (&[2][..], "Both fields are of type `T` — the *same* `T`."),
                    (&[7][..], "`Point { x: 1, y: 2 }` makes a `Point<i32>`."),
                    (&[8][..], "`Point { x: 1.0, y: 2.0 }` makes a `Point<f64>`. Same struct, two types."),
                ],
            ),
            code(
                "rust",
                r#"struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let ints = Point { x: 1, y: 2 };
    let floats = Point { x: 1.0, y: 2.0 };
    println!("{} {}", ints.x, floats.y);
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Notice `x` and `y` must be the *same* `T`. A `Point` cannot have an `i32` x and an \
                 `f64` y — that needs two type parameters, the next lesson. The compiler infers `T` \
                 from the values you put in, so you rarely write `Point::<i32>` by hand.",
            ),
            code(
                "rust",
                r#"enum Option<T> {
    None,
    Some(T),
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}"#,
            ),
            callout(
                "Nothing here is new to you",
                "These are the definitions from Modules 07 and 09, with the angle brackets made \
                 explicit. `Option<T>` holds a `T` or nothing; `Result<T, E>` holds a `T` or an `E`. \
                 When you matched on them, the compiler had already filled in `T` for you.",
            ),
        ],
    )
}

fn more_than_one() -> SeedLesson {
    SeedLesson::new(
        "More Than One Parameter",
        vec![
            labelled(
                "THE IDEA",
                "A type can take several type parameters. `struct Pair<T, U>` holds two values that may \
                 be different types, and `Result<T, E>` — which you know — is the two-parameter enum.",
            ),
            gap_fill(
                "Define a pair whose two halves may be different types. Fill in the type parameters.",
                r#"struct Pair<____> {
    first: T,
    second: ____,
}

fn main() {
    let mixed = Pair { first: 1, second: String::from("one") };
    println!("{} {}", mixed.first, mixed.second);
}"#,
                &[
                    ("p1", "T, U", "Two distinct type parameters."),
                    ("p2", "U", "The second type parameter."),
                ],
            ),
            code(
                "rust",
                r#"struct Pair<T, U> {
    first: T,
    second: U,
}

fn main() {
    let mixed = Pair { first: 1, second: String::from("one") };
    let same = Pair { first: 1.0, second: 2.0 };

    println!("{} {}", mixed.first, mixed.second);
    println!("{}", same.second);
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`Pair<T, U>` lets `first` and `second` differ. `Pair { first: 1, second: String::from(...) }` \
                 is a `Pair<i32, String>`. The names `T` and `U` are arbitrary; what matters is that \
                 they are distinct, so the compiler can track each position's type separately.",
            ),
            table(
                &["Type", "Parameters", "Means"],
                &[
                    &["`Point<T>`", "one", "both fields the same `T`"],
                    &["`Pair<T, U>`", "two", "fields may differ"],
                    &["`Result<T, E>`", "two", "a success `T` or an error `E`"],
                ],
            ),
            callout(
                "You have read two-parameter types for modules",
                "`Result<T, E>` and `HashMap<K, V>` are the same idea. Reading the signature as 'a map \
                 from `K` to `V`' or 'a result of `T` or `E`' is exactly reading the type parameters \
                 in order.",
            ),
        ],
    )
}

fn monomorphisation() -> SeedLesson {
    SeedLesson::new(
        "Monomorphisation",
        vec![
            labelled(
                "THE IDEA",
                "A generic function is not slower for being generic. When you compile, Rust fills in \
                 each concrete type and produces a separate ordinary function for each. This is called \
                 **monomorphisation**: 'one shape per type'.",
            ),
            dispatch(
                "One generic call, two dispatch worlds",
                r#"fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    list[0]
}

largest(&pages);
largest(&ratings);"#,
                &[
                    (
                        &[1][..],
                        "`largest<T>` is written once in your source.",
                        "`largest<T>` is written once in your source too — no code is duplicated by hand.",
                    ),
                    (
                        &[5][..],
                        "For `pages: &[i32]`, the compiler emits a concrete `largest_i32`.",
                        "With `dyn`, there would be one function taking a trait object, not one per type.",
                    ),
                    (
                        &[6][..],
                        "For `ratings: &[f64]`, it emits a second, concrete `largest_f64`.",
                        "The call would look up the method in a table at runtime instead.",
                    ),
                ],
            ),
            code(
                "rust",
                r#"fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let pages = vec![120, 340, 87];
    println!("{}", largest(&pages));
}"#,
            ),
            diagram(
                "One source function, many compiled functions",
                "your source:        fn largest<T>(...)      (one function)\n\n            ┌────────────┴────────────┐\n            ▼                         ▼\n   compiled: largest::<i32>   largest::<f64>   (one per concrete type)",
            ),
            callout(
                "Zero-cost abstraction",
                "Because each call site gets a concrete function, there is no lookup and no run-time \
                 type check — the compiled code is what you would have written by hand. That is why \
                 generics are called a *zero-cost* abstraction: you pay in compile time and binary \
                 size, not in speed. The `dyn Trait` lessons return to this.",
            ),
        ],
    )
}

fn where_clauses() -> SeedLesson {
    SeedLesson::new(
        "`where` Clauses",
        vec![
            labelled(
                "THE IDEA",
                "When the bounds get long, they crowd the signature. A `where` clause moves them below \
                 it, which is the same meaning written more readably.",
            ),
            gap_fill(
                "Move the bounds into a `where` clause. Fill in the keyword and the first bound.",
                r#"fn largest<T>(list: &[T]) -> T
____
    T: ____ + Copy,
{
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}"#,
                &[
                    ("w1", "where", "Introduces the bounds."),
                    ("b1", "PartialOrd", "The comparison bound on `T`."),
                ],
            ),
            code(
                "rust",
                r#"fn largest<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let pages = vec![120, 340, 87];
    println!("{}", largest(&pages));
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The `where` clause says the same thing as `<T: PartialOrd + Copy>`, but the bounds \
                 line up under each other. With one bound the inline form is fine; with four, the \
                 `where` form is far easier to read. Both are common in real code, so recognise both.",
            ),
            callout(
                "`where` is not a different feature",
                "It changes nothing about what the function does — it is purely a place to write the \
                 bounds. Use it when the inline angle brackets stop being readable.",
            ),
        ],
    )
}

fn generics_in_methods() -> SeedLesson {
    SeedLesson::new(
        "Generics in Methods",
        vec![
            labelled(
                "THE IDEA",
                "An `impl` block for a generic type must declare the type parameter — `impl<T> Point<T>` \
                 — and a method can introduce its own. You can also write a plain `impl Point<f64>` for \
                 one concrete type.",
            ),
            step_through(
                "Two impl blocks, two intentions",
                r#"impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}"#,
                &[
                    (&[1][..], "`impl<T> Point<T>` is generic: the method works for any `T`."),
                    (&[2][..], "`x` returns a reference to the field — the type is `&T`."),
                    (&[6][..], "`impl Point<f64>` is *not* generic."),
                    (&[7][..], "`distance_from_origin` exists only on `Point<f64>`."),
                ],
            ),
            code(
                "rust",
                r#"struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

fn main() {
    let ints = Point { x: 3, y: 4 };
    let floats = Point { x: 3.0, y: 4.0 };

    println!("{}", ints.x());
    println!("{}", floats.distance_from_origin());
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`impl<T> Point<T>` attaches `x` to every `Point`. `impl Point<f64>` attaches \
                 `distance_from_origin` only to `Point<f64>`, because that method needs floating-point \
                 maths. Calling it on `Point<i32>` would be an error — the method simply is not there.",
            ),
            callout(
                "Generics and concrete impls coexist",
                "A type can have generic methods and type-specific methods at once. The generic block \
                 serves everyone; the concrete block adds behaviour that only makes sense for one \
                 type. This is how collections provide both general methods and specialised ones.",
            ),
        ],
    )
}
