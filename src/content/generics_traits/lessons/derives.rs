//! Module 10 — Generics & Traits, lessons 16–18.
//!
//! Derives. What `#[derive]` really does, which derives are worth knowing, and
//! when the generated implementation is not what you want.

use crate::content::types::{callout, code, gap_fill, labelled, step_through, table, SeedLesson};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        what_derive_does(),
        the_common_derives(),
        when_derive_is_not_enough(),
    ]
}

fn what_derive_does() -> SeedLesson {
    SeedLesson::new(
        "What `#[derive]` Does",
        vec![
            labelled(
                "THE IDEA",
                "`#[derive(Debug)]` is not magic. It tells the compiler to write an `impl Debug for \
                 ...` on your behalf, using a sensible default: print the type name and every field.",
            ),
            step_through(
                "The derive and the impl it stands for",
                r#"#[derive(Debug)]
struct Book {
    title: String,
    pages: u32,
}

// What the compiler writes for you (approximately):
impl std::fmt::Debug for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Book")
            .field("title", &self.title)
            .field("pages", &self.pages)
            .finish()
    }
}"#,
                &[
                    (&[1][..], "The attribute asks the compiler to generate an impl."),
                    (&[7][..], "The generated impl implements `Debug` for `Book`."),
                    (&[10][..], "It prints the type name and each field, in order."),
                    (&[10][..], "That is why `{:?}` shows `Book { title: \"…\", pages: … }`."),
                ],
            ),
            code(
                "rust",
                r#"#[derive(Debug)]
struct Book {
    title: String,
    pages: u32,
}

fn main() {
    let book = Book { title: String::from("Dune"), pages: 412 };
    println!("{book:?}");
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The derive saves you from writing that impl by hand for every type. It only works when \
                 every field also implements the trait — you cannot `derive(Debug)` for a struct \
                 containing a type that is not `Debug`. When it fails, the error names the field that \
                 is not printable.",
            ),
            callout(
                "Derive is a shortcut, not a separate feature",
                "Everything `#[derive(X)]` produces, you could write yourself as `impl X for Type`. It \
                 is a macro that writes boilerplate. Knowing that is what lets you decide when to \
                 derive and when to write your own.",
            ),
        ],
    )
}

fn the_common_derives() -> SeedLesson {
    SeedLesson::new(
        "The Common Derives",
        vec![
            labelled(
                "THE IDEA",
                "A handful of derives cover almost everything a data type needs. Each one gives the type \
                 a specific ability, and each requires every field to have the same ability.",
            ),
            table(
                &["Derive", "Gives the type", "Requires each field to be"],
                &[
                    &["`Debug`", "`{:?}` printing", "`Debug`"],
                    &["`Clone`", "`.clone()`", "`Clone`"],
                    &["`Copy`", "implicit copy on assignment", "`Copy` (no heap data)"],
                    &["`PartialEq`", "`==` and `!=`", "`PartialEq`"],
                    &["`Eq`", "total equality", "`Eq`"],
                    &["`PartialOrd` / `Ord`", "`<`, `>`, sorting", "the same ordering trait"],
                    &["`Default`", "`Type::default()`", "`Default`"],
                    &["`Hash`", "usable as a `HashMap` key", "`Hash`"],
                ],
            ),
            gap_fill(
                "Give the type equality and printing. Fill in the two derives.",
                r#"#[derive(____, ____)]
struct Book {
    title: String,
    pages: u32,
}"#,
                &[
                    ("d1", "Debug", "Printing with `{:?}`."),
                    ("d2", "PartialEq", "Comparison with `==`."),
                ],
            ),
            code(
                "rust",
                r#"#[derive(Debug, PartialEq, Clone)]
struct Book {
    title: String,
    pages: u32,
}

fn main() {
    let a = Book { title: String::from("Dune"), pages: 412 };
    let b = a.clone();

    println!("{a:?}");
    println!("equal: {}", a == b);
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Each derive is independent, and you list the ones you need. `Debug` and `Clone` are the \
                 most common; `PartialEq` comes next; `Hash` only when the type becomes a map key. \
                 Deriving a trait you do not use adds a little compile time and binary size, nothing \
                 more — but naming only what you need keeps intent clear.",
            ),
            callout(
                "`Copy` is the one to think about",
                "Most derives are harmless. `Copy` changes assignment semantics — a `Copy` type is \
                 duplicated instead of moved — so only derive it for small, heap-free types. A struct \
                 holding a `String` cannot be `Copy` at all, because `String` is not.",
            ),
        ],
    )
}

fn when_derive_is_not_enough() -> SeedLesson {
    SeedLesson::new(
        "When Derive Is Not Enough",
        vec![
            labelled(
                "THE IDEA",
                "Derive gives a sensible default, but sometimes you want a specific one. A `Summary` \
                 trait has no derive at all — you must write the impl — and even for standard traits, \
                 you can write your own instead of deriving.",
            ),
            code(
                "rust",
                r#"struct Book {
    title: String,
    author: String,
    pages: u32,
}

// The condition the summary should show depends on the book,
// so the body is ours to write. No derive exists for this.
impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.pages > 300 {
            write!(f, "{} by {} — a long read", self.title, self.author)
        } else {
            write!(f, "{} by {} — a quick read", self.title, self.author)
        }
    }
}

fn main() {
    let book = Book { title: String::from("Dune"), author: String::from("Herbert"), pages: 412 };
    println!("{book}");
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "There is no `#[derive(Display)]` because Rust cannot guess how you want a type shown \
                 to users. `Debug` has an obvious default; `Display` does not. So `Display` — and your \
                 own traits like `Summary` — are always written by hand. Derive only exists for traits \
                 with a single sensible implementation.",
            ),
            table(
                &["Trait", "Derivable?", "Why"],
                &[
                    &["`Debug`", "yes", "there is one obvious debug format"],
                    &["`Clone`", "yes", "clone every field"],
                    &["`Display`", "no", "only you know the user-facing format"],
                    &["`Summary`", "no", "it is your trait; you write it"],
                ],
            ),
            callout(
                "Derive when the default is what you want",
                "If 'print all the fields' or 'compare all the fields' is right, derive. If you would \
                 write a condition or a specific format, write the impl. The two are not in conflict — a \
                 type can derive `Debug` and hand-write `Display` side by side.",
            ),
        ],
    )
}
