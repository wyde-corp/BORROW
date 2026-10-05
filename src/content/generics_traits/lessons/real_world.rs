//! Module 10 — Generics & Traits, lessons 24–31.
//!
//! Real world. Standard traits, `Display`/`From`, implementing versus deriving,
//! an iterator preview, operator traits, common mistakes, and the assembled
//! Media Library.

use crate::content::types::{
    callout, code, gap_fill, guess_run, labelled, paragraphs, step_through, steps, table,
    SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        display_and_to_string(),
        from_and_into(),
        implement_vs_derive(),
        iterator_preview(),
        operator_traits(),
        common_mistakes(),
        assembled(),
        change_the_design(),
    ]
}

fn display_and_to_string() -> SeedLesson {
    SeedLesson::new(
        "`Display` and `ToString`",
        vec![
            labelled(
                "CONCEPT",
                "`Display` is how a type shows itself to a user: `println!(\"{item}\")`. Implement it \
                 by hand — there is no derive — and `ToString` comes along for free.",
            ),
            gap_fill(
                "Implement the user-facing formatter. Fill in the trait and the macro.",
                r#"use std::fmt;

impl fmt::____ for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        ____!(f, "{} ({} pages)", self.title, self.pages)
    }
}"#,
                &[
                    ("d1", "Display", "The user-facing formatting trait."),
                    ("w1", "write", "Writes the formatted text into `f`."),
                ],
            ),
            code(
                "rust",
                r#"use std::fmt;

struct Book {
    title: String,
    pages: u32,
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({} pages)", self.title, self.pages)
    }
}

fn main() {
    let book = Book { title: String::from("Dune"), pages: 412 };

    println!("{book}");          // uses Display
    let text = book.to_string(); // ToString, for free
    println!("{text}");
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`Display` is the trait behind `{}`. Because implementing `Display` is enough to define \
                 a string form, the standard library provides `ToString` automatically for any type \
                 that has one — so `.to_string()` works without you writing another impl.",
            ),
            table(
                &["You implement", "You get", "Format"],
                &[
                    &["`Display`", "`{}` and `.to_string()`", "user-facing text"],
                    &["`Debug` (or derive)", "`{:?}`", "developer detail"],
                    &["neither", "nothing printable", "a compile error"],
                ],
            ),
            callout(
                "The first trait you implement on purpose",
                "In Module 09 you implemented `Display` for an error so it could be printed. That was \
                 this trait. Now it is named and understood: it is how a type becomes text for a \
                 human.",
            ),
        ],
    )
}

fn from_and_into() -> SeedLesson {
    SeedLesson::new(
        "`From` and `Into`",
        vec![
            labelled(
                "CONCEPT",
                "`From` converts one type into another: `Title::from(\"Dune\")`. Implement `From` once \
                 and `Into` is provided automatically, so both `Title::from(x)` and `x.into()` work.",
            ),
            step_through(
                "Implementing From gives Into",
                r#"struct Title(String);

impl From<&str> for Title {
    fn from(text: &str) -> Title {
        Title(text.to_string())
    }
}

fn main() {
    let a = Title::from("Dune");
    let b: Title = "Neuromancer".into();
    println!("{} {}", a.0, b.0);
}"#,
                &[
                    (&[3][..], "`From<&str>` teaches how to build a `Title` from a string slice."),
                    (&[4][..], "The conversion owns the text by copying it in."),
                    (&[9][..], "`Title::from(...)` calls the impl directly."),
                    (&[10][..], "`.into()` uses the same impl; `Into` is provided automatically."),
                ],
            ),
            code(
                "rust",
                r#"struct Title(String);

impl From<&str> for Title {
    fn from(text: &str) -> Title {
        Title(text.to_string())
    }
}

fn main() {
    let a = Title::from("Dune");
    let b: Title = "Neuromancer".into();

    println!("{} {}", a.0, b.0);
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`From` is the trait `?` uses when it converts errors — the `impl From<io::Error> for \
                 TodoError` from Module 09 was exactly this. `.into()` is the mirrored call: for every \
                 `impl From<A> for B`, `Into<B>` is implemented for `A`, so you never write `Into` by \
                 hand.",
            ),
            callout(
                "One direction to write, two to use",
                "Implement `From`, not `Into`. You get `Into` for free, and `From` is the form that \
                 reads clearly in a signature. This is why the standard library's conversion traits \
                 are almost always implemented as `From`.",
            ),
        ],
    )
}

fn implement_vs_derive() -> SeedLesson {
    SeedLesson::new(
        "Implementing versus Deriving",
        vec![
            labelled(
                "CONCEPT",
                "The derived `PartialEq` compares every field. Sometimes that is not what two equal \
                 items means — two editions of a book can share a title and still be 'the same book' \
                 for your purposes.",
            ),
            code(
                "rust",
                r#"struct Book {
    title: String,
    pages: u32,
}

// Equal by title, even if the page count differs.
impl PartialEq for Book {
    fn eq(&self, other: &Book) -> bool {
        self.title == other.title
    }
}

fn main() {
    let a = Book { title: String::from("Dune"), pages: 412 };
    let b = Book { title: String::from("Dune"), pages: 500 };

    println!("{}", a == b);
}"#,
            ),
            guess_run(
                "The same struct derives `PartialEq` instead. Predict what `a == b` prints.",
                r#"#[derive(PartialEq)]
struct Book {
    title: String,
    pages: u32,
}

fn main() {
    let a = Book { title: String::from("Dune"), pages: 412 };
    let b = Book { title: String::from("Dune"), pages: 500 };

    println!("{}", a == b);
}"#,
                &[
                    "true — the titles match",
                    "false — the derived version compares every field, and the page counts differ",
                ],
                1,
                "The derived `PartialEq` checks all fields, so differing page counts make the books \
                 unequal. The manual impl compares only the title. Neither is 'correct' in the \
                 abstract — the question is what equality should mean for your program, and that \
                 decision is the reason manual impls exist.",
            ),
            callout(
                "Derive for the obvious meaning, implement for the specific one",
                "If 'all fields equal' is right, derive. If equality means something narrower, write \
                 the impl and say so. The compiler does not care which; your readers do.",
            ),
        ],
    )
}

fn iterator_preview() -> SeedLesson {
    SeedLesson::new(
        "`Iterator` — a Preview",
        vec![
            labelled(
                "CONCEPT",
                "`Iterator` is an ordinary trait, and the methods you have been calling — `.map`, \
                 `.filter`, `.sum`, `.collect` — are its provided methods. This lesson names it; \
                 Module 11 teaches it.",
            ),
            code(
                "rust",
                r#"#[derive(Debug)]
struct Book {
    pages: u32,
}

fn main() {
    let shelf = vec![
        Book { pages: 412 },
        Book { pages: 120 },
        Book { pages: 271 },
    ];

    // `iter` gives an iterator; `map` and `sum` are Iterator methods.
    let total: u32 = shelf.iter().map(|b| b.pages).sum();
    println!("{total} pages");
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`shelf.iter()` produces something that implements `Iterator`. `.map(...)` transforms \
                 each item and returns another iterator; `.sum()` consumes it and adds them up. Every \
                 one of those is a method on the `Iterator` trait — a trait you have been using \
                 without realising, exactly as you were using `Option` before Module 09.",
            ),
            callout(
                "The last unnamed trait from earlier modules",
                "By the end of this module you have named every abstraction the course has leaned on: \
                 structs, enums, `Option`, `Result`, `From`, `Display`, and now `Iterator`. Module 11 \
                 is where `Iterator` stops being a preview and becomes the subject.",
            ),
        ],
    )
}

fn operator_traits() -> SeedLesson {
    SeedLesson::new(
        "Operator Traits",
        vec![
            labelled(
                "CONCEPT",
                "Operators like `+` are sugar for trait methods. `a + b` calls `Add::add`. Implementing \
                 the right trait makes `+` work on your own type.",
            ),
            code(
                "rust",
                r#"use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let sum = Point { x: 1, y: 2 } + Point { x: 3, y: 4 };
    println!("{sum:?}");
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`impl Add for Point` makes `+` work on points. The `type Output = Point` line is an \
                 *associated type* — it names what `add` produces. You do not need the full story yet; \
                 for now, note that operators are traits, which is why you could write `+` on a `String` \
                 (the standard library implemented `Add` for it) when you could not on your own type \
                 until you did the same.",
            ),
            table(
                &["Operator", "Trait"],
                &[
                    &["`+`", "`std::ops::Add`"],
                    &["`-`", "`std::ops::Sub`"],
                    &["`==`", "`PartialEq`"],
                    &["`[]`", "`std::ops::Index`"],
                ],
            ),
            callout(
                "Sugar over methods",
                "There is no operator magic: `a + b` is `a.add(b)`. Knowing that explains why operators \
                 only work when the trait is implemented, and why the standard library's many \
                 implementations make `+` work on numbers, strings, and durations alike.",
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
                "These are the mistakes that survive after the syntax is learned. Read them as a \
                 checklist against your own understanding.",
            ),
            table(
                &["#", "Mistake", "The correction"],
                &[
                    &["1", "thinking a bound is something the function does", "it is something the function requires of `T`"],
                    &["2", "forgetting `use` of a trait before calling its method", "the trait must be in scope at the call site"],
                    &["3", "expecting `derive(Display)` to exist", "`Display` is always written by hand"],
                    &["4", "deriving `Copy` on a type with a `String`", "`Copy` requires every field to be `Copy`"],
                    &["5", "trying to put two types in one `Vec<T>`", "use `Vec<Box<dyn Trait>>` for mixed types"],
                    &["6", "adding a bound the body does not need", "bounds should name exactly the abilities used"],
                    &["7", "confusing `impl Trait` (static) with `dyn Trait` (dynamic)", "`impl` erases at compile time, `dyn` at run time"],
                    &["8", "implementing `Into` by hand", "implement `From`; `Into` is provided for free"],
                    &["9", "reading 'trait bound not satisfied' as a bug", "it names a type that lacks a required ability"],
                    &["10", "using `dyn` to avoid thinking about performance", "use it when you need mixed types, not by default"],
                ],
            ),
            guess_run(
                "A generic function calls a method the bound does not promise. Predict the result.",
                r#"trait Summary { fn summarise(&self) -> String; }
struct Book { title: String }
impl Summary for Book {
    fn summarise(&self) -> String { self.title.clone() }
}

fn describe<T: Summary>(item: &T) -> String {
    item.summarise().to_uppercase()
}

fn main() {
    let b = Book { title: String::from("Dune") };
    println!("{}", describe(&b));
}"#,
                &[
                    "It compiles — `String` has `to_uppercase`",
                    "It compiles — the bound is not needed for `to_uppercase`",
                    "It compiles and prints DUNE",
                ],
                2,
                "`summarise` returns a `String`, and `String` has `to_uppercase`, so this compiles and \
                 prints `DUNE`. The bound `T: Summary` is exactly what the call to `item.summarise()` \
                 needs; `to_uppercase` is on the returned `String`, not on `T`, so no extra bound is \
                 required.",
            ),
            callout(
                "Read the bound errors as instructions",
                "A 'trait bound not satisfied' error names the type and the trait. Nine times in ten it \
                 means either the type needs an `impl`, or the call site needs a `use` of the trait. \
                 Both are mechanical once you know the two shapes.",
            ),
        ],
    )
}

fn assembled() -> SeedLesson {
    SeedLesson::new(
        "The Media Library, Assembled",
        vec![
            labelled(
                "CONCEPT",
                "Here is the whole library. A `Summary` trait with a default method, two media types \
                 implementing it, a `Display` impl by hand, a generic `Library<T: Summary>`, a bounded \
                 function, and a mixed shelf of `Box<dyn Summary>` — every idea in the module in one \
                 program.",
            ),
            code(
                "rust",
                r#"use std::fmt;

trait Summary {
    fn summarise(&self) -> String;

    fn author(&self) -> String {
        String::from("unknown")
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Book {
    title: String,
    author: String,
    pages: u32,
}

#[derive(Debug, Clone, PartialEq)]
struct Video {
    title: String,
    creator: String,
    minutes: u32,
}

impl Summary for Book {
    fn summarise(&self) -> String {
        format!("{} by {} ({} pages)", self.title, self.author, self.pages)
    }

    fn author(&self) -> String {
        self.author.clone()
    }
}

impl Summary for Video {
    fn summarise(&self) -> String {
        format!("{} by {} ({} min)", self.title, self.creator, self.minutes)
    }

    fn author(&self) -> String {
        self.creator.clone()
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} by {}", self.title, self.author)
    }
}

struct Library<T: Summary> {
    items: Vec<T>,
}

impl<T: Summary> Library<T> {
    fn new() -> Library<T> {
        Library { items: Vec::new() }
    }

    fn add(&mut self, item: T) {
        self.items.push(item);
    }

    fn summaries(&self) -> Vec<String> {
        self.items.iter().map(|item| item.summarise()).collect()
    }
}

fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarise());
}

fn main() {
    let mut books: Library<Book> = Library::new();
    books.add(Book { title: String::from("Dune"), author: String::from("Herbert"), pages: 412 });
    books.add(Book { title: String::from("Neuromancer"), author: String::from("Gibson"), pages: 271 });

    for line in books.summaries() {
        println!("{line}");
    }

    notify(&books.items[0]);
    println!("{}", books.items[0]);

    let mixed: Vec<Box<dyn Summary>> = vec![
        Box::new(Book { title: String::from("Dune"), author: String::from("Herbert"), pages: 412 }),
        Box::new(Video { title: String::from("Rust intro"), creator: String::from("Gus"), minutes: 18 }),
    ];

    for item in &mixed {
        println!("{} — {}", item.summarise(), item.author());
    }
}"#,
            ),
            table(
                &["Piece", "What it is", "Why"],
                &[
                    &["`Summary`", "a trait", "the shared behaviour with no shared type"],
                    &["`impl Summary for Book`", "a manual impl", "no derive exists for your own trait"],
                    &["`impl Display for Book`", "a manual impl", "the user-facing text is a choice"],
                    &["`Library<T: Summary>`", "a generic type with a bound", "one library works for any summarisable item"],
                    &["`notify<T: Summary>`", "a bounded function", "static dispatch, direct call"],
                    &["`Vec<Box<dyn Summary>>`", "a trait object collection", "mixed types on one shelf"],
                ],
            ),
            labelled(
                "EXPLANATION",
                "The `books` library is static: `Library<Book>` and one concrete `notify` per type. The \
                 `mixed` shelf is dynamic: one `Box<dyn Summary>` vector holding books and videos, with \
                 each `summarise` resolved through the vtable. Both coexist, because they answer \
                 different questions.",
            ),
            callout(
                "Every abstraction you have met, named",
                "Structs and enums to model data; generics to write it once; traits to name behaviour; \
                 bounds to require it; `Display` and `From` as the standard traits; and dispatch to \
                 choose the cost. Nothing here is a new feature — it is all the pieces, visible at \
                 once.",
            ),
        ],
    )
}

fn change_the_design() -> SeedLesson {
    SeedLesson::new(
        "Change the Design",
        vec![
            labelled(
                "CONCEPT",
                "Reading the program proves you can follow Rust. Changing it proves you can write it. \
                 Each of these asks you to extend the library, and each is a generics-or-traits \
                 decision.",
            ),
            steps(&[
                "Add an `Article` type with `headline`, `author`, and `words`, and implement `Summary` for it.",
                "Add a method to `Summary` with a default body, such as `short(&self) -> String` that truncates the summary.",
                "Add a generic `fn longest_title<T: Summary>(items: &[T]) -> Option<&T>` and decide the return type.",
                "Add `Article` to the mixed shelf and confirm nothing else needs changing.",
                "Implement `Display` for `Video` and explain why it cannot be derived.",
            ]),
            paragraphs(&[
                "The first two are trait work: a new implementor, and a provided method. The third is a \
                 bound decision — what does `longest_title` actually need `T` to do? The fourth tests \
                 whether you really understand trait objects: adding a type to `Box<dyn Summary>` \
                 should require no change to the loop, because the loop only knows the trait. The last \
                 is the derive-versus-implement line from the derives lessons.",
            ]),
            callout(
                "The two questions, one more time",
                "For every piece: is it 'all of these' (a struct), 'one of these' (an enum), or 'any \
                 type that can do this' (a trait)? Generics let you write the code once; traits name \
                 what the code requires. This module has been that idea, applied to behaviour.",
            ),
        ],
    )
}
