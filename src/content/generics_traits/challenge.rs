//! Module 10 — the three module challenges.
//!
//! Derive-or-implement decisions, collapsing duplicated functions into one
//! generic function, and choosing a dispatch form. The walkthrough is the last
//! hint.

use crate::content::types::{ExerciseKind, SeedExercise};

const DERIVE_BRIEF: &str = r#"// For each type below, decide whether `#[derive]` suffices or a
// manual `impl` is required, and write the missing impls.
//
// 1. `struct Tag(String)` needs `==` and `{:?}`.
// 2. `struct Money { cents: u64 }` needs `{}` to print "$1.50".
// 3. `struct Version { major: u32, minor: u32 }` should sort with `<`.
// 4. `struct User { name: String, email: String }` needs to be a `HashMap` key.
// 5. `struct Range { start: u32, end: u32 }` needs `{}` to print "3..7".
// 6. `struct Session { id: String, token: String }` should be cloneable.
//
// The decisions are the deliverable; write the code for the manual ones.
"#;

const COLLAPSE: &str = r#"// These three functions do the same job for three types.
// Replace them with ONE generic function plus a trait.

struct Book { title: String, pages: u32 }
struct Video { title: String, minutes: u32 }
struct Podcast { title: String, episodes: u32 }

fn describe_book(b: &Book) -> String { format!("{} ({} pages)", b.title, b.pages) }
fn describe_video(v: &Video) -> String { format!("{} ({} min)", v.title, v.minutes) }
fn describe_podcast(p: &Podcast) -> String { format!("{} ({} eps)", p.title, p.episodes) }

fn main() {
    let b = Book { title: String::from("Dune"), pages: 412 };
    println!("{}", describe_book(&b));
}"#;

const DISPATCH_BRIEF: &str = r#"// For each scenario, choose static dispatch (generics / impl Trait)
// or dynamic dispatch (dyn Trait), and defend it in one sentence.
//
// 1. A function that prints any one `Summary`, called in a hot loop.
// 2. A `Vec` holding books, videos, and articles together.
// 3. A struct field that holds "a formatter" chosen at start-up.
// 4. A function over a single `Vec<Book>` that sums page counts.
// 5. A registry of handlers, each with a different concrete type.
"#;

pub const DERIVE_WALKTHROUGH: &str = r#"The dividing line is whether Rust has one obvious
implementation.

  1. Tag { == , {:?} }        -> derive(Debug, PartialEq)
  2. Money { {} as "$1.50" }  -> manual Display  (no obvious format)
  3. Version { < }            -> derive(PartialOrd, Ord, PartialEq, Eq)
  4. User { HashMap key }     -> derive(Hash, PartialEq, Eq)
  5. Range { {} as "3..7" }   -> manual Display
  6. Session { cloneable }    -> derive(Clone)

The two manual impls are the `Display` cases, and they are manual for the same
reason: the user-facing text is a choice. A `Money` value could be "$1.50" or
"150 cents"; a `Range` could be "3..7" or "from 3 to 7". Rust cannot guess, so
you write it. Everything with a single obvious meaning — equality, ordering,
hashing, cloning, debug printing — is a derive."#;

pub const COLLAPSE_WALKTHROUGH: &str = r#"The trait names the shared behaviour:

  trait Describe {
      fn describe(&self) -> String;
  }

  impl Describe for Book {
      fn describe(&self) -> String { format!("{} ({} pages)", self.title, self.pages) }
  }
  // ... and the same for Video and Podcast.

  fn describe<T: Describe>(item: &T) -> String {
      item.describe()
  }

Three near-identical functions become one bounded function plus three small
impls. That is the whole point: the *shape* of the code is written once, and
each type supplies the part that genuinely differs.

The bound you chose matters. `T: Describe` is exactly what `describe` needs.
If you also wrote `T: Clone`, you added a requirement the body never uses, and
every caller now has to satisfy it for nothing. Bounds should name the abilities
used, and no more."#;

pub const DISPATCH_WALKTHROUGH: &str = r#"  1. Hot loop over one type        -> static. The type is known; direct calls
                                      and inlining, no run-time lookup.
  2. Vec of books/videos/articles  -> dynamic. Three concrete types cannot
                                      share one Vec<T>; Box<dyn Summary> can.
  3. A field holding "a formatter" -> dynamic. The concrete type is chosen
                                      later, so it cannot be a type parameter.
  4. Summing over Vec<Book>        -> static. One type, no reason to erase it.
  5. A registry of handlers        -> dynamic. The handler types are different
                                      and not known together.

The pattern: choose dynamic dispatch when the concrete type is *not known*
where the code is written — because it is chosen later, or because several
types must sit in one collection. Choose static when there is a single type,
both for speed and because the compiler can do more with it."#;

pub fn challenges() -> Vec<SeedExercise> {
    vec![
        SeedExercise::new(
            None,
            ExerciseKind::Challenge,
            "DERIVE OR IMPLEMENT — For each of the six types, decide whether `#[derive]` suffices or a \
             manual impl is required, and write the manual ones. The decisions are the deliverable.",
            &[
                "For each type, ask whether Rust has one obvious implementation",
                "Derive where the meaning is unambiguous",
                "Write a manual `impl` where the format is a choice",
                "Say why each manual impl cannot be derived",
                "Do not derive `Copy` on anything with owned data",
            ],
            0,
            DERIVE_WALKTHROUGH,
        )
        .code(DERIVE_BRIEF)
        .xp(150)
        .hints(&[
            "Equality, ordering, hashing, cloning and debug printing all have one obvious meaning.",
            "The user-facing format (`{}`) never does.",
            "A manual impl for a standard trait looks just like implementing your own trait.",
            "The walkthrough sorts all six and explains the split.",
        ]),
        SeedExercise::new(
            None,
            ExerciseKind::Challenge,
            "ONE FUNCTION, MANY TYPES — Three near-identical functions handle a book, a video, and a \
             podcast. Replace them with one generic function plus a trait, and justify the bound you \
             chose.",
            &[
                "Name the shared behaviour and turn it into a trait",
                "Implement the trait for each type",
                "Write one bounded function that calls the trait method",
                "Choose the smallest bound the body needs",
                "Explain why a caller with an unrelated type now gets a clear error",
            ],
            0,
            COLLAPSE_WALKTHROUGH,
        )
        .code(COLLAPSE)
        .xp(170)
        .hints(&[
            "The differing part is how each type formats itself.",
            "A trait method with `&self` fits all three.",
            "A bound should name the abilities the body uses — no more.",
            "The walkthrough shows the trait and the single function.",
        ]),
        SeedExercise::new(
            None,
            ExerciseKind::Challenge,
            "STATIC OR DYNAMIC? — For five scenarios, choose generics/`impl Trait` or `dyn Trait`, and \
             defend each choice. No code required; the reasoning is the deliverable.",
            &[
                "For each scenario, ask whether the concrete type is known where the code is written",
                "If one type is known: static",
                "If several types share a collection or the type is chosen later: dynamic",
                "Give one sentence per choice",
                "Name any scenario where the choice is genuinely close, and why",
            ],
            0,
            DISPATCH_WALKTHROUGH,
        )
        .code(DISPATCH_BRIEF)
        .xp(150)
        .hints(&[
            "A `Vec` holding different types cannot use a single `Vec<T>`.",
            "A field whose type is chosen later cannot be a type parameter.",
            "One type, known at compile time, is the static case.",
            "The walkthrough gives a choice and a reason for all five.",
        ]),
    ]
}
