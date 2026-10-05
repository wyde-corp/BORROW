//! Module 06 — Borrowing, lessons 1–4.
//!
//! The problem first, the mechanism second. Parts 01–04 take the learner from
//! "this function does not need ownership" to the syntax that expresses it, and
//! stop there — the rules are not introduced until the learner can already feel
//! the friction borrowing removes.

use crate::content::types::{
    broken, callout, code, diagram, fixed, labelled, paragraphs, table, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        the_problem(),
        references(),
        without_transfer(),
        dereferencing(),
    ]
}

fn the_problem() -> SeedLesson {
    SeedLesson::new(
        "The Problem Borrowing Solves",
        vec![
            labelled(
                "CONCEPT",
                "You have just spent a whole module learning that passing a `String` to a function \
                 moves it. That is correct, and it is also frequently the wrong thing to want. Most \
                 functions are not taking a value to keep it. They are taking it to look at it.",
            ),
            labelled(
                "EXPLANATION",
                "Read this program as a designer rather than as a compiler. What does \
                 `calculate_length` actually need? It needs to read the length of the string. It does \
                 not need to store it, free it, or hand it on. Yet the signature says 'give me the \
                 String and I will take it from here' — so the caller loses the value.",
            ),
            broken(
                "fn calculate_length(s: String) -> usize {\n    s.len()\n}\n\nfn main() {\n    let name = String::from(\"Gus\");\n\n    let length = calculate_length(name);\n\n    println!(\"{length}\");\n    println!(\"{name}\");\n}",
            ),
            paragraphs(&[
                "Compile that and the error points at the second `println!`. The string moved into \
                 `calculate_length`, the function ended, and the `String` was dropped with it. `name` \
                 is still in scope, but it no longer owns anything.",
                "The fix available to you at the end of the ownership module was to return the string \
                 as well — `fn calculate_length(s: String) -> (String, usize)` — or to clone it before \
                 the call. Both work. Both make the caller pay for a design that only wanted to read.",
            ]),
            diagram(
                "What actually happens",
                "main\n  │\n  │ owns String\n  ↓\ncalculate_length\n  │\n  │ takes ownership\n  ↓\nfunction ends\n  │\n  ↓\nString dropped",
            ),
            table(
                &["The function wanted", "The signature said", "The caller paid"],
                &[
                    &["to inspect the text", "take the String", "the value itself"],
                    &["to read the length", "keep the String forever", "a round-trip or a clone"],
                ],
            ),
            callout(
                "The question this module answers",
                "Ownership answers **who is responsible for this value**. Borrowing answers **who may \
                 temporarily use it without taking responsibility for it**. The rest of this module is \
                 the mechanism and the rules for that second question.",
            ),
            paragraphs(&[
                "Notice that nothing in the problem mentions `&`. That is deliberate. The syntax is the \
                 answer to a question you now have. If you learn `&` first, it is trivia; learned here, \
                 it is the obvious solution to a concrete problem.",
            ]),
        ],
    )
}

fn references() -> SeedLesson {
    SeedLesson::new(
        "References",
        vec![
            labelled(
                "CONCEPT",
                "A reference is Rust's mechanism for borrowing. It is a value that says 'the thing you \
                 want lives over there, and I am allowed to look at it' — without becoming an owner of \
                 that thing.",
            ),
            code(
                "rust",
                "fn main() {\n    let s = String::from(\"hello\");\n    let r = &s;\n\n    println!(\"{r}\");\n}",
            ),
            paragraphs(&[
                "Read the `&` as the word *borrow*. `let r = &s;` asks `s` for temporary access and \
                 stores a reference to the value `s` owns. Nothing moved. `s` is still the owner, and \
                 `s` is still usable after this line.",
            ]),
            labelled(
                "EXPLANATION",
                "The important facts, stated precisely. `s` owns the `String`. `r` does not own the \
                 `String`. `r` is a reference to the value owned by `s`. Using `r` does not transfer \
                 ownership. When `r` goes out of scope it does not drop the `String` — `r` was never an \
                 owner. `s` remains responsible for freeing the value.",
            ),
            diagram(
                "One owner, one reference",
                "STACK\n\ns ────────────────┐\n                  │\n                  ↓\n                HEAP\n             \"hello\"\n\nr ──────────────→ (points at the value s owns)",
            ),
            callout(
                "Do not over-read the diagram",
                "The picture shows the *conceptual* relationship — `s` owns the value, `r` refers to it. \
                 Do not take it to mean a reference has the same representation as a `String`. A `String` \
                 is pointer, length, and capacity; a reference is a different thing entirely. What \
                 matters is the ownership relationship, not the bytes.",
            ),
            table(
                &["Statement", "Who owns the value", "Is the original still usable?"],
                &[
                    &["`let s = String::from(\"hi\");`", "`s`", "—"],
                    &["`let r = &s;`", "`s` (unchanged)", "yes"],
                    &["`let t = s;`", "`t`", "no"],
                ],
            ),
        ],
    )
}

fn without_transfer() -> SeedLesson {
    SeedLesson::new(
        "Borrowing Without Ownership Transfer",
        vec![
            labelled(
                "CONCEPT",
                "The difference between moving a value and borrowing it is the difference between \
                 handing someone your only key and letting them look through the window. One transfers \
                 responsibility; the other does not.",
            ),
            paragraphs(&["Compare these two programs. Same first line, one character apart."]),
            broken(
                "fn main() {\n    let s1 = String::from(\"hello\");\n    let s2 = s1;\n\n    println!(\"{s2}\");\n    println!(\"{s1}\");\n}",
            ),
            fixed(
                "fn main() {\n    let s1 = String::from(\"hello\");\n    let s2 = &s1;\n\n    println!(\"{s2}\");\n    println!(\"{s1}\");\n}",
            ),
            diagram(
                "Two statements, two outcomes",
                "s2 = s1\n\n  ownership moves\n  s1 is no longer usable\n\n\ns2 = &s1\n\n  ownership does not move\n  s1 remains the owner\n  s2 merely borrows",
            ),
            labelled(
                "EXPLANATION",
                "In the first program there is exactly one `String`. After `let s2 = s1;` it belongs to \
                 `s2`. `s1` is not empty and not invalid — it is disarmed. The compiler rejects the \
                 attempt to read it because the value it used to own now belongs to someone else.",
            ),
            labelled(
                "EXPLANATION",
                "In the second program there is still exactly one `String`, owned by `s1`, and `s2` is a \
                 reference to it. Both names are usable, because only one of them is responsible for \
                 freeing anything. When the scope ends, `s2` disappears harmlessly and `s1` frees the \
                 buffer, once.",
            ),
            callout(
                "A reference is not another owner",
                "It is tempting to describe `s2` as 'also owning' the string. That is exactly the wrong \
                 mental model. Two owners means two frees, which is the double-free bug ownership exists \
                 to make impossible. A reference owns nothing; it only borrows.",
            ),
        ],
    )
}

fn dereferencing() -> SeedLesson {
    SeedLesson::new(
        "Dereferencing",
        vec![
            labelled(
                "CONCEPT",
                "`&` creates a reference. `*` goes the other way: it follows a reference back to the \
                 value it points at. One is the borrow; the other is how you use what you borrowed.",
            ),
            code(
                "rust",
                "fn main() {\n    let s = String::from(\"hello\");\n    let r = &s;\n\n    println!(\"{r}\");\n    println!(\"{}\", *r);\n}",
            ),
            diagram(
                "Following the reference",
                "s\n↓\nactual value\n\n\nr\n↓\nreference to that value\n\n\n*r\n↓\naccess the value through the reference",
            ),
            paragraphs(&[
                "Both `println!` lines print the same thing. `*r` says 'give me the value `r` points \
                 at', and formatting that value prints `hello`. `{r}` prints the same text without the \
                 explicit `*`.",
            ]),
            labelled(
                "EXPLANATION",
                "Why does `{r}` work without the `*`? Because formatting and many operators work through \
                 a reference automatically: the `Display` implementation for `String` can be reached \
                 through `&String`, so the compiler inserts the dereference for you. This is called \
                 *autoderef*, and it is why a reference usually behaves like the value it points at.",
            ),
            callout(
                "Enough dereferencing for now",
                "Autoderef and autoref go much deeper than this — method calls, operators, and coercion \
                 all participate. You do not need that machinery yet. For this module, hold onto the \
                 conceptual pair: `&` borrows, `*` reaches through the borrow to the value.",
            ),
        ],
    )
}
