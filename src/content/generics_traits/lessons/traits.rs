//! Module 10 — Generics & Traits, lessons 8–15.
//!
//! Traits. The shared behaviour that has no shared type, defining a trait,
//! implementing it, calling its methods, default methods, bounds, bounds on
//! `impl` blocks, and multiple bounds.

use crate::content::types::{
    callout, code, gap_fill, guess_run, labelled, step_through, table, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        missing_shared_behaviour(),
        defining_a_trait(),
        implementing_a_trait(),
        calling_trait_methods(),
        default_methods(),
        trait_bounds(),
        bounds_on_impls(),
        multiple_bounds(),
    ]
}

fn missing_shared_behaviour() -> SeedLesson {
    SeedLesson::new(
        "The Missing Shared Behaviour",
        vec![
            labelled(
                "CONCEPT",
                "Every item in the media library can be summarised — a book by title and pages, a video \
                 by title and length. But there is no type that is 'a book, an article, and a video', \
                 so the summarising code gets written three times.",
            ),
            code(
                "rust",
                r#"#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32,
}

#[derive(Debug)]
struct Video {
    title: String,
    creator: String,
    minutes: u32,
}

fn summarise_book(book: &Book) -> String {
    format!("{} by {} ({} pages)", book.title, book.author, book.pages)
}

fn summarise_video(video: &Video) -> String {
    format!("{} by {} ({} min)", video.title, video.creator, video.minutes)
}

fn main() {
    let book = Book { title: String::from("Dune"), author: String::from("Herbert"), pages: 412 };
    let video = Video { title: String::from("Rust intro"), creator: String::from("Gus"), minutes: 18 };

    println!("{}", summarise_book(&book));
    println!("{}", summarise_video(&video));
}"#,
            ),
            labelled(
                "EXPLANATION",
                "A generic function cannot help here the way `largest<T>` did, because the two types \
                 have no shared field — a `Book` has `pages`, a `Video` has `minutes`. What they share \
                 is not their *data*; it is a *behaviour*: both can be summarised. Generics cover \
                 'many types, same code'; traits cover 'many types, same promise'.",
            ),
            callout(
                "Behaviour, not shape",
                "A trait names something types can *do*, and lets you write code against that promise \
                 instead of against a specific type. 'Anything that can be summarised' is a trait. It is \
                 the missing piece between `Book` and `Video`.",
            ),
        ],
    )
}

fn defining_a_trait() -> SeedLesson {
    SeedLesson::new(
        "Defining a Trait",
        vec![
            labelled(
                "CONCEPT",
                "A trait lists method signatures that implementing types must provide. `trait Summary` \
                 with `fn summarise(&self) -> String` says: any type that claims to be `Summary` must \
                 have a `summarise` method returning a `String`.",
            ),
            gap_fill(
                "Declare the trait and its required method. Fill in the keyword and the receiver.",
                r#"____ Summary {
    fn summarise(____) -> String;
}"#,
                &[
                    ("t1", "trait", "The keyword that defines shared behaviour."),
                    ("r1", "&self", "The method borrows the value it summarises."),
                ],
            ),
            code(
                "rust",
                r#"trait Summary {
    fn summarise(&self) -> String;
}

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32,
}

fn main() {
    // The trait exists, but no type implements it yet.
}"#,
            ),
            labelled(
                "EXPLANATION",
                "The method body is just a semicolon — this is a *requirement*, not an implementation. \
                 A trait is a contract: 'any implementor must provide this method'. Nothing compiles \
                 against it until at least one type implements it.",
            ),
            callout(
                "A trait is a promise, not a type",
                "You cannot make a `Summary` by itself — there is no value of type `Summary`. A trait \
                 describes what implementing types can do; the values are still `Book`s and `Video`s.",
            ),
        ],
    )
}

fn implementing_a_trait() -> SeedLesson {
    SeedLesson::new(
        "Implementing a Trait",
        vec![
            labelled(
                "CONCEPT",
                "`impl Summary for Book { ... }` provides the method for one type. The `for` is what \
                 connects a trait to a type, and you write one impl block per type.",
            ),
            gap_fill(
                "Implement the trait for `Book`. Fill in the keywords and the return type.",
                r#"____ Summary ____ Book {
    fn summarise(&self) -> ____ {
        format!("{} by {} ({} pages)", self.title, self.author, self.pages)
    }
}"#,
                &[
                    ("i1", "impl", "Introduces the implementation."),
                    ("f1", "for", "Connects the trait to the type."),
                    ("s1", "String", "The declared return type."),
                ],
            ),
            code(
                "rust",
                r#"trait Summary {
    fn summarise(&self) -> String;
}

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Summary for Book {
    fn summarise(&self) -> String {
        format!("{} by {} ({} pages)", self.title, self.author, self.pages)
    }
}

fn main() {
    let book = Book { title: String::from("Dune"), author: String::from("Herbert"), pages: 412 };
    println!("{}", book.summarise());
}"#,
            ),
            labelled(
                "EXPLANATION",
                "The impl block gives `Book` a `summarise` method. The body uses `self`, just like a \
                 normal method, because that is what it is — a method that happens to be required by a \
                 trait. Now `book.summarise()` works.",
            ),
            callout(
                "One impl per type",
                "Each type gets its own `impl Summary for ...`. The signatures must match the trait \
                 exactly; only the bodies differ. That is precisely the duplication you set out to \
                 remove — but now the compiler checks that every implementor kept the promise.",
            ),
        ],
    )
}

fn calling_trait_methods() -> SeedLesson {
    SeedLesson::new(
        "Calling Trait Methods",
        vec![
            labelled(
                "CONCEPT",
                "Once a type implements a trait, the trait's methods are available on values of that \
                 type with ordinary dot syntax. One condition: the trait must be *in scope* where you \
                 call it.",
            ),
            code(
                "rust",
                r#"trait Summary {
    fn summarise(&self) -> String;
}

#[derive(Debug)]
struct Video {
    title: String,
    minutes: u32,
}

impl Summary for Video {
    fn summarise(&self) -> String {
        format!("{} ({} min)", self.title, self.minutes)
    }
}

fn main() {
    let video = Video { title: String::from("Rust intro"), minutes: 18 };
    println!("{}", video.summarise());
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`video.summarise()` looks just like calling a method defined on `Video`, and that is \
                 the point — trait methods feel native. Behind the scenes the compiler looks up the \
                 trait implementation, but you never write anything different.",
            ),
            guess_run(
                "The trait is defined in another module and not imported. Predict what happens when the \
                 method is called.",
                r#"mod media {
    pub trait Summary {
        fn summarise(&self) -> String;
    }
}

struct Video { title: String }

impl media::Summary for Video {
    fn summarise(&self) -> String {
        self.title.clone()
    }
}

fn main() {
    let v = Video { title: String::from("x") };
    println!("{}", v.summarise());
}"#,
                &[
                    "It compiles — the method is on the type",
                    "It does not compile — `media::Summary` is not in scope for the call",
                ],
                1,
                "To call a trait method with dot syntax, the trait itself must be in scope. The impl \
                 exists, but the call site cannot see `Summary`, so the method is 'not found'. Import \
                 the trait (`use media::Summary;`) and it works. When you `use` a library trait like \
                 `std::fmt::Display`, this is the same rule.",
            ),
            callout(
                "The impl is not enough; the name must be visible",
                "A confusing error for beginners is 'method not found' when the type clearly has the \
                 method via a trait. The fix is usually a missing `use` of the trait. The method \
                 exists; the compiler just needs to be told about the trait at the call site.",
            ),
        ],
    )
}

fn default_methods() -> SeedLesson {
    SeedLesson::new(
        "Default Methods",
        vec![
            labelled(
                "CONCEPT",
                "A trait can provide a method body as a default. Implementors get that behaviour for \
                 free, and can override it when they need something different.",
            ),
            step_through(
                "A default that the trait provides",
                r#"trait Summary {
    fn summarise(&self) -> String;
    fn author(&self) -> String {
        String::from("unknown")
    }
}

impl Summary for Book {
    fn summarise(&self) -> String {
        format!("{} ({} pages)", self.title, self.pages)
    }
}"#,
                &[
                    (&[2][..], "`summarise` is required — no body, so every type must write it."),
                    (&[3][..], "`author` has a default body."),
                    (&[4][..], "The default returns a placeholder."),
                    (&[8][..], "`Book` implements only `summarise`."),
                    (&[8][..], "It still has `author` — the default is inherited."),
                ],
            ),
            code(
                "rust",
                r#"trait Summary {
    fn summarise(&self) -> String;
    fn author(&self) -> String {
        String::from("unknown")
    }
}

struct Book { title: String, author: String, pages: u32 }
struct Video { title: String, creator: String, minutes: u32 }

impl Summary for Book {
    fn summarise(&self) -> String {
        format!("{} ({} pages)", self.title, self.pages)
    }
    fn author(&self) -> String {
        self.author.clone()
    }
}

impl Summary for Video {
    fn summarise(&self) -> String {
        format!("{} ({} min)", self.title, self.minutes)
    }
}

fn main() {
    let book = Book { title: String::from("Dune"), author: String::from("Herbert"), pages: 412 };
    let video = Video { title: String::from("Rust intro"), creator: String::from("Gus"), minutes: 18 };

    println!("{} — {}", book.summarise(), book.author());
    println!("{} — {}", video.summarise(), video.author());
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`Book` overrides `author` to return the real one; `Video` does not, so it uses the \
                 default `\"unknown\"`. A default method can also call other trait methods — that is \
                 how you build rich behaviour on top of a single required method.",
            ),
            callout(
                "Required versus provided",
                "A trait method without a body is required; one with a body is provided and optional to \
                 override. Most traits have a small required core and several provided helpers — which \
                 is why implementing a trait is often just one method.",
            ),
        ],
    )
}

fn trait_bounds() -> SeedLesson {
    SeedLesson::new(
        "Trait Bounds",
        vec![
            labelled(
                "CONCEPT",
                "A bound on a generic says what the type must be able to do. `fn notify<T: Summary>` \
                 accepts any `T` that implements `Summary`, and inside the function you may call \
                 `summarise` on it.",
            ),
            gap_fill(
                "Bound the generic to types that can be summarised. Fill in the bound.",
                r#"fn notify<T: ____>(item: &T) {
    println!("{}", item.summarise());
}"#,
                &[("b1", "Summary", "The trait that provides `summarise`.")],
            ),
            code(
                "rust",
                r#"trait Summary {
    fn summarise(&self) -> String;
}

struct Book { title: String, pages: u32 }
struct Video { title: String, minutes: u32 }

impl Summary for Book {
    fn summarise(&self) -> String {
        format!("{} ({} pages)", self.title, self.pages)
    }
}

impl Summary for Video {
    fn summarise(&self) -> String {
        format!("{} ({} min)", self.title, self.minutes)
    }
}

fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarise());
}

fn main() {
    let book = Book { title: String::from("Dune"), pages: 412 };
    let video = Video { title: String::from("Rust intro"), minutes: 18 };

    notify(&book);
    notify(&video);
}"#,
            ),
            labelled(
                "EXPLANATION",
                "Without the bound, `item.summarise()` would be an error — the compiler cannot know \
                 that `T` has such a method. `T: Summary` guarantees it. Now `notify` works for any \
                 current or future type that implements `Summary`, and the compiler checks every call \
                 site.",
            ),
            callout(
                "The bound is the function's requirement",
                "The signature `fn notify<T: Summary>` reads: 'give me any type that can be \
                 summarised'. That is the whole contract, and it is visible to every caller. A type \
                 that does not implement `Summary` gets a clear 'trait bound not satisfied' error, \
                 naming the type and the trait.",
            ),
        ],
    )
}

fn bounds_on_impls() -> SeedLesson {
    SeedLesson::new(
        "Bounds on `impl` Blocks",
        vec![
            labelled(
                "CONCEPT",
                "A generic type's `impl` block can require a bound too. `impl<T: Summary> Library<T>` \
                 means: these methods exist only for libraries whose items can be summarised.",
            ),
            step_through(
                "A library of summarisable things",
                r#"struct Library<T> {
    items: Vec<T>,
}

impl<T: Summary> Library<T> {
    fn add(&mut self, item: T) {
        self.items.push(item);
    }

    fn summary(&self) -> String {
        self.items
            .iter()
            .map(|item| item.summarise())
            .collect::<Vec<_>>()
            .join("\n")
    }
}"#,
                &[
                    (&[1][..], "`Library<T>` is generic over its item type."),
                    (&[5][..], "The `impl` block requires `T: Summary`."),
                    (&[6][..], "So `add` is available for any `Library<T>` — no bound needed on it."),
                    (&[12][..], "`summary` may call `item.summarise()` because every `T` is `Summary`."),
                ],
            ),
            code(
                "rust",
                r#"trait Summary {
    fn summarise(&self) -> String;
}

struct Book { title: String }
impl Summary for Book {
    fn summarise(&self) -> String {
        format!("book: {}", self.title)
    }
}

struct Library<T> {
    items: Vec<T>,
}

impl<T: Summary> Library<T> {
    fn add(&mut self, item: T) {
        self.items.push(item);
    }

    fn summary(&self) -> String {
        self.items.iter().map(|i| i.summarise()).collect::<Vec<_>>().join("\n")
    }
}

fn main() {
    let mut shelf = Library { items: Vec::new() };
    shelf.add(Book { title: String::from("Dune") });
    shelf.add(Book { title: String::from("Neuromancer") });
    println!("{}", shelf.summary());
}"#,
            ),
            labelled(
                "EXPLANATION",
                "The bound sits on the block, not on each method, so every method may use `T`'s \
                 `Summary` abilities. `Library<Book>` works; a `Library<i32>` would exist as a type but \
                 would have no methods, because `i32` is not `Summary` — and the compiler says so the \
                 moment you try to call one.",
            ),
            callout(
                "Bound where it applies",
                "Bound a method when only that method needs it; bound the block when the whole set of \
                 methods does. `Library<T>` is the second case — everything it does rests on being able \
                 to summarise its items.",
            ),
        ],
    )
}

fn multiple_bounds() -> SeedLesson {
    SeedLesson::new(
        "Multiple Bounds",
        vec![
            labelled(
                "CONCEPT",
                "A type can be required to satisfy more than one trait. `T: Summary + Clone` needs both \
                 abilities, and the `+` joins them.",
            ),
            gap_fill(
                "Require both summarising and cloning. Fill in the bound.",
                r#"fn duplicate<T: Summary ____ Clone>(item: &T) -> (String, T) {
    (item.summarise(), item.clone())
}"#,
                &[("b1", "+", "Joins two bounds.")],
            ),
            code(
                "rust",
                r#"trait Summary {
    fn summarise(&self) -> String;
}

#[derive(Clone)]
struct Book { title: String }
impl Summary for Book {
    fn summarise(&self) -> String {
        format!("book: {}", self.title)
    }
}

fn duplicate<T: Summary + Clone>(item: &T) -> (String, T) {
    (item.summarise(), item.clone())
}

fn main() {
    let book = Book { title: String::from("Dune") };
    let (text, copy) = duplicate(&book);
    println!("{text} / {}", copy.title);
}"#,
            ),
            table(
                &["Bound", "Means"],
                &[
                    &["`T: Summary`", "has `summarise`"],
                    &["`T: Summary + Clone`", "has `summarise` and can be cloned"],
                    &["`T: Summary + Clone + PartialEq`", "all three"],
                ],
            ),
            labelled(
                "EXPLANATION",
                "`duplicate` both summarises and clones, so it needs both bounds. The `+` is cumulative \
                 — each bound narrows the set of types that qualify. With several bounds, a `where` \
                 clause (from the generics lessons) is usually easier to read.",
            ),
            callout(
                "Bounds stack",
                "Every method you call on a generic value needs a corresponding bound. Add a `.clone()` \
                 and you must add `Clone`; compare with `==` and you add `PartialEq`. The compiler \
                 errors point at exactly the missing ability.",
            ),
        ],
    )
}
