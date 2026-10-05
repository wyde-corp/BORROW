//! Module 06 — Borrowing, lessons 17–20.
//!
//! Borrowing applied to containers and the views into them. Collections
//! reinforce that the model does not change with the type; slices then show
//! that a borrow can cover part of a value rather than all of it.

use crate::content::types::{
    broken, callout, code, diagram, labelled, paragraphs, table, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        borrowing_collections(),
        slices(),
        string_vs_str(),
        slice_ranges(),
    ]
}

fn borrowing_collections() -> SeedLesson {
    SeedLesson::new(
        "Borrowing and Collections",
        vec![
            labelled(
                "CONCEPT",
                "A `Vec<T>` owns its elements. Borrowing works exactly the same as it does for a \
                 `String`: you can take a reference to an element, and while that reference lives, the \
                 collection may not be mutated in a way that would move or invalidate it.",
            ),
            code(
                "rust",
                "fn main() {\n    let names = vec![\n        String::from(\"Gus\"),\n        String::from(\"Alice\"),\n        String::from(\"Bob\"),\n    ];\n\n    let first = &names[0];\n    println!(\"{first}\");\n}",
            ),
            labelled(
                "EXPLANATION",
                "`names` owns the three `String`s. `first` borrows the first element; it does not own \
                 it, and it cannot outlive the vector. This is the same owner/borrower relationship as \
                 a single `String`, just reached through an index.",
            ),
            paragraphs(&["The rule surfaces the moment you try to change the collection while a reference into it is live:"]),
            broken(
                "fn main() {\n    let mut names = vec![String::from(\"Gus\")];\n\n    let first = &names[0];\n\n    names.push(String::from(\"Alice\"));\n\n    println!(\"{first}\");\n}",
            ),
            labelling("Why the push conflicts", "Pushing may reallocate the vector's buffer and move every element to new memory. A reference taken before the push would then point into the old, freed buffer. This is the same outlive-the-value problem from Part 16, caused by a collection operation rather than a scope."),
            listing(&[
                "`names` owns the `String`s",
                "`first` borrows one element",
                "`first` does not own the element",
                "a reallocation would invalidate `first`",
            ]),
            callout(
                "The model does not change with the type",
                "Do not go looking for a separate set of 'collection borrowing rules'. There is one \
                 model. A vector can reallocate, which is a way for a borrowed value to stop existing \
                 — but the rule being enforced is still 'a reference may not outlive its value'.",
            ),
        ],
    )
}

fn slices() -> SeedLesson {
    SeedLesson::new(
        "Slices: Borrowing Part of a Value",
        vec![
            labelled(
                "CONCEPT",
                "A slice is a reference to a contiguous portion of a collection. It does not own the \
                 elements it covers; it borrows a range of them. For strings, that gives you the most \
                 common borrowed type in Rust: `&str`.",
            ),
            code(
                "rust",
                "fn main() {\n    let s = String::from(\"hello world\");\n\n    let hello = &s[0..5];\n    println!(\"{hello}\");\n}",
            ),
            labelled(
                "EXPLANATION",
                "`&s[0..5]` borrows the first five bytes of the string, from index 0 up to but not \
                 including 5. The slice refers to a part of the text `s` owns. It is a view, not a \
                 copy: no new characters were allocated, and the slice cannot outlive `s`.",
            ),
            diagram(
                "A window onto the string",
                "String\n\nh e l l o   w o r l d\n0 1 2 3 4 5 6 7 8 9 10\n\n        ┌───────────┐\n        ↓           ↓\n       [0 ........ 5]\n        ↓\n      \"hello\"",
            ),
            paragraphs(&[
                "The same idea applies to any sliceable collection. Here a slice covers part of a \
                 vector:",
            ]),
            code(
                "rust",
                "fn main() {\n    let numbers = vec![10, 20, 30, 40, 50];\n\n    let middle = &numbers[1..4];\n    println!(\"{middle:?}\");\n}",
            ),
            callout(
                "A slice is a borrow with a shape",
                "Think of a slice as a reference plus a length. It answers 'which part of the value?' \
                 where an ordinary reference answers only 'which value?'. Everything you know about \
                 borrows still applies: the slice is read-only here, it cannot outlive the collection, \
                 and mutating the collection while it lives is rejected.",
            ),
        ],
    )
}

fn string_vs_str() -> SeedLesson {
    SeedLesson::new(
        "`String` vs `&str`",
        vec![
            labelled(
                "CONCEPT",
                "`String` is an owned, growable UTF-8 string. `&str` is a borrowed string slice — a \
                 view onto text someone else owns. This is the single most common owned/borrowed pair \
                 in Rust, and the clearest illustration of the whole module.",
            ),
            table(
                &["Type", "Ownership", "Growable?", "Typical role"],
                &[
                    &["`String`", "owns its bytes", "yes", "stored, built, returned"],
                    &["`&str`", "borrows bytes", "no", "read, passed, viewed"],
                ],
            ),
            code(
                "rust",
                "fn main() {\n    let owned = String::from(\"hello\");\n    let borrowed: &str = &owned;\n\n    println!(\"{owned} {borrowed}\");\n}",
            ),
            labelled(
                "EXPLANATION",
                "`borrowed` does not own text. It points at the characters `owned` is responsible for. \
                 Because it is a slice, it can also point at *part* of that text — which is what makes \
                 functions like this possible:",
            ),
            code(
                "rust",
                "fn first_word(s: &str) -> &str {\n    match s.find(' ') {\n        Some(i) => &s[..i],\n        None => s,\n    }\n}\n\nfn main() {\n    let text = String::from(\"hello world\");\n    println!(\"{}\", first_word(&text));\n}",
            ),
            callout(
                "Why `&str` beats `&String` in parameters",
                "Accepting `&str` is more flexible. A `&String` coerces to `&str`, and a string literal \
                 is already a `&str`, so a `&str` parameter accepts both without giving up any safety. \
                 Prefer `&str` unless you genuinely need `String`-specific capabilities.",
            ),
            paragraphs(&[
                "This is not a UTF-8 lesson. The one fact to carry forward: slicing a string uses byte \
                 indices, and those indices must fall on character boundaries. Part 20 shows what \
                 happens when they do not.",
            ]),
        ],
    )
}

fn slice_ranges() -> SeedLesson {
    SeedLesson::new(
        "Slice Ranges and Borrowing",
        vec![
            labelled(
                "CONCEPT",
                "A range selects which part of the collection a slice covers. `start..end` runs from \
                 `start` up to but not including `end`, and the shorthand forms cover the obvious \
                 whole-prefix and whole-suffix cases.",
            ),
            code(
                "rust",
                "fn main() {\n    let s = String::from(\"hello world\");\n\n    let a = &s[0..5];\n    let b = &s[6..11];\n\n    println!(\"{a} / {b}\");\n}",
            ),
            paragraphs(&["The shorthands drop whichever end is obvious:"]),
            code(
                "rust",
                "fn main() {\n    let s = String::from(\"hello world\");\n\n    let whole = &s[..];\n    let beginning = &s[..5];\n    let ending = &s[6..];\n\n    println!(\"{whole} | {beginning} | {ending}\");\n}",
            ),
            table(
                &["Range", "Means"],
                &[
                    &["`start..end`", "from `start`, up to but not including `end`"],
                    &["`start..`", "from `start` to the end"],
                    &["`..end`", "from the start, up to but not including `end`"],
                    &["`..`", "the whole thing"],
                ],
            ),
            labelled(
                "EXPLANATION",
                "Every one of these is a borrow of `s`. `whole` covers all of it, `beginning` and \
                 `ending` cover parts, and all three obey the same rule: they may not outlive `s`, and \
                 `s` may not be mutated while they are live.",
            ),
            paragraphs(&["Now the failure that surprises newcomers. Rust strings are UTF-8, and a character can occupy more than one byte, so a range can cut through a character:"]),
            broken(
                "fn main() {\n    let text = String::from(\"héllo\");\n\n    let bad = &text[0..2];\n    println!(\"{bad}\");\n}",
            ),
            labelled(
                "EXPLANATION",
                "The `é` occupies two bytes, so index 2 is not a character boundary. Rust refuses to \
                 build a slice there and panics at runtime rather than handing back text that is not \
                 valid UTF-8. The fix is to slice on boundaries — using character-aware methods when \
                 the indexing is not obvious.",
            ),
            callout(
                "Bytes in, boundaries out",
                "Ranges are byte indices. For ASCII text bytes and characters line up, which is why \
                 every example until `héllo` looked simple. The moment non-ASCII appears, slice on \
                 known-good boundaries or use `char_indices`, `find`, and friends.",
            ),
        ],
    )
}

/// A short labelled paragraph for facts that deserve their own line but not a
/// whole callout.
fn labelling(label: &str, body: &str) -> serde_json::Value {
    labelled(label, body)
}

/// A bullet-ish list rendered as separate prose paragraphs.
fn listing(items: &[&str]) -> serde_json::Value {
    paragraphs(items)
}
