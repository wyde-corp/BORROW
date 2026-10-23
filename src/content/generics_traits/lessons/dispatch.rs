//! Module 10 — Generics & Traits, lessons 19–23.
//!
//! Dispatch. `impl Trait` parameters, the mixed-shelf problem, trait objects,
//! static versus dynamic dispatch, and how to choose.

use crate::content::types::{
    callout, code, dispatch, gap_fill, guess_run, labelled, step_through, table, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        impl_trait_parameters(),
        mixed_shelf(),
        dyn_trait(),
        static_vs_dynamic(),
        choosing_dispatch(),
    ]
}

fn impl_trait_parameters() -> SeedLesson {
    SeedLesson::new(
        "`impl Trait` Parameters",
        vec![
            labelled(
                "THE IDEA",
                "`fn notify<T: Summary>(item: &T)` has a shorter alias in parameter position: `fn \
                 notify(item: &impl Summary)`. Same meaning, less angle-bracket noise.",
            ),
            gap_fill(
                "Accept any type that can be summarised, using the shorthand. Fill in the two blanks.",
                r#"fn notify(item: &____ ____) {
    println!("{}", item.summarise());
}"#,
                &[
                    ("i1", "impl", "The shorthand for an anonymous type parameter."),
                    ("t1", "Summary", "The trait it must satisfy."),
                ],
            ),
            code(
                "rust",
                r#"trait Summary {
    fn summarise(&self) -> String;
}

struct Book { title: String, pages: u32 }
impl Summary for Book {
    fn summarise(&self) -> String {
        format!("{} ({} pages)", self.title, self.pages)
    }
}

fn notify(item: &impl Summary) {
    println!("{}", item.summarise());
}

fn main() {
    notify(&Book { title: String::from("Dune"), pages: 412 });
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`&impl Summary` means 'a reference to some type that implements `Summary`'. The caller \
                 writes `notify(&book)` exactly as before. The difference from the generic form is \
                 cosmetic in this case, but it matters when you need the shorthand's limits — you \
                 cannot name the type parameter, so `impl Trait` is for parameters only here.",
            ),
            callout(
                "Two spellings, one meaning",
                "`fn f<T: Summary>(x: &T)` and `fn f(x: &impl Summary)` compile to the same thing. Use \
                 whichever reads better; reach for the generic form when you need to name `T` (for \
                 example, to return it or to mention it twice).",
            ),
        ],
    )
}

fn mixed_shelf() -> SeedLesson {
    SeedLesson::new(
        "The Mixed Shelf Problem",
        vec![
            labelled(
                "THE IDEA",
                "A `Vec<Book>` holds books and a `Vec<Video>` holds videos, but a real shelf mixes them. \
                 A `Vec` holds one type, so no `Vec<Book>` can also hold a `Video` — even though both \
                 can be summarised.",
            ),
            code(
                "rust",
                r#"trait Summary {
    fn summarise(&self) -> String;
}

struct Book { title: String }
struct Video { title: String }

impl Summary for Book {
    fn summarise(&self) -> String { self.title.clone() }
}
impl Summary for Video {
    fn summarise(&self) -> String { self.title.clone() }
}

fn main() {
    let books: Vec<Book> = vec![Book { title: String::from("Dune") }];
    let videos: Vec<Video> = vec![Video { title: String::from("Rust intro") }];

    // There is no `Vec<Book or Video>` — the element type must be one type.
    println!("{} {}", books.len(), videos.len());
}"#,
            ),
            guess_run(
                "You try to push a `Video` onto a `Vec<Book>`. Predict the result.",
                r#"struct Book { title: String }
struct Video { title: String }

fn main() {
    let mut shelf: Vec<Book> = Vec::new();
    shelf.push(Book { title: String::from("Dune") });
    shelf.push(Video { title: String::from("Rust intro") });
}"#,
                &[
                    "It compiles — both have a `title`",
                    "It does not compile — `Video` is not `Book`",
                ],
                1,
                "A `Vec<Book>` holds `Book`s exactly. `Video` is a different type, so `push` rejects it \
                 even though both have a `title`. The shared field does not make them the same type; \
                 the shared *behaviour* is real, but there is no single concrete type to store. That is \
                 what trait objects solve.",
            ),
            callout(
                "Generics need one concrete type",
                "`Vec<T>` is homogeneous: every element is a `T`. To hold values of *different* types \
                 with a shared behaviour, you need a type that stands for 'any type implementing this \
                 trait'. That is `dyn Summary`.",
            ),
        ],
    )
}

fn dyn_trait() -> SeedLesson {
    SeedLesson::new(
        "`dyn Trait`",
        vec![
            labelled(
                "THE IDEA",
                "`Box<dyn Summary>` is a box holding *any* value that implements `Summary`. A `Vec<Box<dyn \
                 Summary>>` can hold books, videos, and articles side by side, because every element is \
                 the same type: a boxed trait object.",
            ),
            step_through(
                "A shelf of different types",
                r#"let shelf: Vec<Box<dyn Summary>> = vec![
    Box::new(Book { title: String::from("Dune"), pages: 412 }),
    Box::new(Video { title: String::from("Rust intro"), minutes: 18 }),
];

for item in &shelf {
    println!("{}", item.summarise());
}"#,
                &[
                    (&[1][..], "`Vec<Box<dyn Summary>>` — every element is a boxed trait object."),
                    (&[2][..], "A `Book` is boxed; the box erases the concrete type `Book`."),
                    (&[3][..], "A `Video` is boxed the same way. Different types, one vector type."),
                    (&[7][..], "`item.summarise()` calls the right method at runtime, via the vtable."),
                ],
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

fn main() {
    let shelf: Vec<Box<dyn Summary>> = vec![
        Box::new(Book { title: String::from("Dune"), pages: 412 }),
        Box::new(Video { title: String::from("Rust intro"), minutes: 18 }),
    ];

    for item in &shelf {
        println!("{}", item.summarise());
    }
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`dyn Summary` is a *trait object*: a value that can be any implementor of `Summary`, \
                 with the concrete type erased. `Box::new(book)` moves the book onto the heap and hands \
                 back a `Box<dyn Summary>`. Inside the loop, `item.summarise()` finds the right method \
                 at run time, because the box carries a pointer to the implementation.",
            ),
            table(
                &["Type", "Holds", "Chosen"],
                &[
                    &["`Vec<Book>`", "books only", "at compile time"],
                    &["`Vec<Box<dyn Summary>>`", "any `Summary` implementor", "at run time"],
                ],
            ),
            callout(
                "One rust standard type, all your media",
                "This is the trick behind heterogeneous collections. The box owns the value, the `dyn` \
                 records the behaviour, and the `Vec` sees a single element type. The next lesson is \
                 about what that erasure costs.",
            ),
        ],
    )
}

fn static_vs_dynamic() -> SeedLesson {
    SeedLesson::new(
        "Static vs Dynamic Dispatch",
        vec![
            labelled(
                "THE IDEA",
                "When you call a trait method on a generic or `impl Trait` value, the compiler knows the \
                 concrete type and can call the method directly — *static* dispatch. On a `dyn Trait`, \
                 it cannot, so it looks the method up in a table at run time — *dynamic* dispatch. Both \
                 work; they trade differently.",
            ),
            dispatch(
                "The same call, two dispatch forms",
                r#"trait Summary { fn summarise(&self) -> String; }

fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarise());
}

fn notify_dyn(item: &dyn Summary) {
    println!("{}", item.summarise());
}"#,
                &[
                    (
                        &[3][..],
                        "`notify::<Book>` is compiled specifically for `Book`.",
                        "`notify_dyn` is compiled once, taking any `&dyn Summary`.",
                    ),
                    (
                        &[4][..],
                        "The call to `summarise` resolves to `Book`'s method at compile time.",
                        "The call goes through a table of method pointers stored with the value.",
                    ),
                    (
                        &[4][..],
                        "No lookup at run time. No size cost beyond the extra code.",
                        "One pointer lookup per call. The box is also larger: data pointer plus table pointer.",
                    ),
                ],
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Static dispatch knows the type, so it can inline and optimise. Dynamic dispatch knows \
                 only the trait, so it keeps a table (a vtable) and jumps through it. For a mixed shelf \
                 that is exactly what you want; for a hot loop over one type, the generic version is \
                 faster and the compiler can do more with it.",
            ),
            table(
                &["", "Static (`T: Summary`, `impl Summary`)", "Dynamic (`dyn Summary`)"],
                &[
                    &["Type known", "at compile time", "at run time"],
                    &["Call", "direct", "through a table"],
                    &["Can hold mixed types", "no", "yes"],
                    &["Run-time cost", "none", "one lookup per call"],
                    &["Binary size", "grows per type", "one copy"],
                ],
            ),
            callout(
                "There is no free lunch, but the cost is small",
                "Dynamic dispatch's cost is tiny and usually irrelevant. Choose it when you *need* \
                 heterogeneous storage, not to avoid 'slowness'. Reach for generics when you have a \
                 single type and want the compiler's full optimisation.",
            ),
        ],
    )
}

fn choosing_dispatch() -> SeedLesson {
    SeedLesson::new(
        "Choosing",
        vec![
            labelled(
                "THE IDEA",
                "The rule is small: use generics or `impl Trait` when all the values are the same type; \
                 use `dyn Trait` when they are not. Everything else is a refinement.",
            ),
            table(
                &["Situation", "Reach for", "Why"],
                &[
                    &["a function over one type at a time", "`impl Trait` / `<T: Trait>`", "direct calls, optimised"],
                    &["a `Vec` of mixed implementors", "`Box<dyn Trait>`", "the element type must be one type"],
                    &["a field that may be one of several", "`Box<dyn Trait>`", "the concrete type is chosen later"],
                    &["a hot loop over a single type", "generics", "no run-time lookup"],
                    &["a plugin-style registry", "`Box<dyn Trait>`", "types are not known together"],
                ],
            ),
            code(
                "rust",
                r#"trait Summary { fn summarise(&self) -> String; }
struct Book { title: String }
impl Summary for Book {
    fn summarise(&self) -> String { self.title.clone() }
}

// Static: one type, direct call.
fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarise());
}

// Dynamic: any implementor, table lookup.
fn notify_any(item: &dyn Summary) {
    println!("{}", item.summarise());
}

fn main() {
    let book = Book { title: String::from("Dune") };
    notify(&book);
    notify_any(&book);
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The same `book` can be passed to either. `notify` is generic and monomorphised; \
                 `notify_any` takes a trait object. When you have a `Vec<Book>` the generic form is \
                 natural; when you have a `Vec<Box<dyn Summary>>` the dynamic form is the only one that \
                 accepts it.",
            ),
            callout(
                "Start generic, switch when you must mix",
                "Most code should use generics or `impl Trait`, because the compiler can do more with a \
                 known type. You reach for `dyn` at the specific moment you need to store or return \
                 different concrete types together, and that moment is easy to recognise.",
            ),
        ],
    )
}
