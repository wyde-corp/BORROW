//! Module 06 — Borrowing, lessons 11–16.
//!
//! Borrowing where it matters most: the function boundary. Signatures are
//! design decisions, and this file teaches the learner to read them — including
//! the one mistake that the compiler cannot let slide, returning a reference to
//! something that will not outlive the call.

use crate::content::types::{
    broken, callout, code, diagram, fixed, labelled, paragraphs, table, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        borrowing_in_functions(),
        ref_vs_mut_ref(),
        owner_vs_reference(),
        borrowing_vs_moving(),
        returning_references(),
        dangling_references(),
    ]
}

fn borrowing_in_functions() -> SeedLesson {
    SeedLesson::new(
        "Borrowing in Functions",
        vec![
            labelled(
                "THE IDEA",
                "The place you feel ownership most is the parameter list. A function that only reads \
                 its argument should say so in its signature, so every caller knows the value comes \
                 back to them.",
            ),
            paragraphs(&["Compare two versions of the same function. This one takes ownership:"]),
            code(
                "rust",
                "fn print_name(name: String) {\n    println!(\"{name}\");\n}\n\nfn main() {\n    let name = String::from(\"Gus\");\n    print_name(name);\n}",
            ),
            paragraphs(&["And this one borrows it:"]),
            code(
                "rust",
                "fn print_name(name: &String) {\n    println!(\"{name}\");\n}\n\nfn main() {\n    let name = String::from(\"Gus\");\n    print_name(&name);\n\n    println!(\"{name}\");\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The first version consumes the string: whatever `main` held is gone after the call. \
                 The second version lends it for the duration of the call and takes nothing. The \
                 signature is the only difference a caller needs to read — `name: String` means 'I \
                 keep this', `name: &String` means 'I only look'.",
            ),
            diagram(
                "Caller keeps the value",
                "main\n  │ owns String\n  │\n  ├── &name ──→ print_name\n  │                (reads only)\n  ↓\ncaller still owns it",
            ),
            paragraphs(&["The idiomatic form accepts `&str` rather than `&String`, which is more flexible because a `&String` coerces to `&str` and a string literal already is one:"]),
            code(
                "rust",
                "fn print_name(name: &str) {\n    println!(\"{name}\");\n}\n\nfn main() {\n    let owned = String::from(\"Gus\");\n    print_name(&owned);\n    print_name(\"a literal too\");\n}",
            ),
            callout(
                "A bridge, not the whole story",
                "`&str` is a borrowed string slice, and the next file in this module is entirely about \
                 it. For now, note only the direction of travel: prefer the least powerful parameter \
                 that does the job. If the function does not need ownership, do not ask for it.",
            ),
        ],
    )
}

fn ref_vs_mut_ref() -> SeedLesson {
    SeedLesson::new(
        "`&T` vs `&mut T`",
        vec![
            labelled(
                "THE IDEA",
                "The two reference forms answer two different requests. `&T` asks for permission to \
                 read. `&mut T` asks for permission to read and write, and promises exclusivity in \
                 exchange.",
            ),
            table(
                &["Type", "Grants", "Cost"],
                &[
                    &["`&T`", "read access", "none — many may coexist"],
                    &["`&mut T`", "read and write access", "exclusivity while active"],
                ],
            ),
            paragraphs(&["A reading function takes `&T`:"]),
            code(
                "rust",
                "fn length(s: &String) -> usize {\n    s.len()\n}\n\nfn main() {\n    let s = String::from(\"hello\");\n    println!(\"{}\", length(&s));\n}",
            ),
            paragraphs(&["A writing function takes `&mut T`:"]),
            code(
                "rust",
                "fn add_suffix(s: &mut String) {\n    s.push_str(\"!\");\n}\n\nfn main() {\n    let mut s = String::from(\"hello\");\n    add_suffix(&mut s);\n    println!(\"{s}\");\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`&mut T` does not mean 'the reference itself is mutable'. A reference variable is \
                 just a variable; if you wanted to reassign which value it points at you would write \
                 `let mut r`, and that is a separate topic. `&mut T` means *this reference grants \
                 mutable access to the borrowed value*.",
            ),
            diagram(
                "What the `mut` is attached to",
                "let mut r = &s;    → r (the binding) may point somewhere else\n&mut s             → the VALUE may be written through this borrow",
            ),
            callout(
                "Read the request, not the word",
                "When you see `&mut T` in a signature, translate it as 'I intend to modify the \
                 caller's value'. When you see `&T`, translate it as 'I only need to look'. The \
                 distinction is about the callee's intent, which is why it belongs in the signature.",
            ),
        ],
    )
}

fn owner_vs_reference() -> SeedLesson {
    SeedLesson::new(
        "Mutable Owner vs Mutable Reference",
        vec![
            labelled(
                "THE IDEA",
                "These two lines are different claims about different things. `let mut s` says the \
                 *binding* `s` may have its value modified or replaced. `&mut s` says a *reference* \
                 grants write access to whatever `s` currently refers to.",
            ),
            code(
                "rust",
                "fn main() {\n    let mut s = String::from(\"hello\");\n\n    let r = &mut s;\n    r.push_str(\" world\");\n\n    println!(\"{s}\");\n}",
            ),
            diagram(
                "Two permissions, two targets",
                "mut s\n  The owner is allowed to modify the value.\n\n&mut s\n  A mutable reference is created,\n  allowing mutation through that reference.",
            ),
            paragraphs(&["A mutable owner can still be rented out read-only. The borrow says what the borrower may do, not what the owner may do:"]),
            code(
                "rust",
                "fn main() {\n    let mut s = String::from(\"hello\");\n\n    let r = &s;\n    println!(\"{r}\");\n\n    // r is read-only even though s is mutable\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Why would you borrow a mutable value immutably? Because while that immutable borrow \
                 is active, no one may write through `s` — the shared borrow wins. The owner's `mut` is \
                 a capability, not an obligation, and it is temporarily suspended when a read-only \
                 borrow is outstanding.",
            ),
            table(
                &["Declaration", "Borrow", "Result"],
                &[
                    &["`let mut s`", "`&s`", "read-only view; cannot write while it lives"],
                    &["`let mut s`", "`&mut s`", "write access through the reference"],
                    &["`let s`", "`&mut s`", "rejected — owner never permitted mutation"],
                ],
            ),
            callout(
                "The `mut` on the owner is not a switch",
                "It does not turn every access into a write. It enables mutation; the specific borrow \
                 decides who is actually allowed to perform it. Keep the two permissions separate in \
                 your head and the error messages will read much more clearly.",
            ),
        ],
    )
}

fn borrowing_vs_moving() -> SeedLesson {
    SeedLesson::new(
        "Borrowing vs Moving",
        vec![
            labelled(
                "THE IDEA",
                "For any function call you make a design decision: hand over the value, lend it for \
                 reading, or lend it for writing. The three options are `T`, `&T`, and `&mut T`, and \
                 they are visible in the signature.",
            ),
            code(
                "rust",
                "fn use_string(s: String) {\n    println!(\"{s}\");\n}\n\nfn main() {\n    let s = String::from(\"hello\");\n    use_string(s);\n}",
            ),
            code(
                "rust",
                "fn inspect_string(s: &String) {\n    println!(\"{s}\");\n}\n\nfn main() {\n    let s = String::from(\"hello\");\n    inspect_string(&s);\n\n    println!(\"{s}\");\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Walk both calls through the same six questions. Before the call, `main` owns the \
                 string in each case. Inside `use_string`, the function owns it and `main` does not. \
                 Inside `inspect_string`, `main` still owns it and the function holds a borrow. After \
                 the call, `main` can use the value in the second program and cannot in the first. One \
                 moved; one borrowed.",
            ),
            diagram(
                "Translate between the three",
                "take ownership   →   fn f(v: T)\nborrow immutably →   fn f(v: &T)\nborrow mutably   →   fn f(v: &mut T)",
            ),
            table(
                &["Signature", "Owner after the call", "Can modify?"],
                &[
                    &["`fn f(v: T)`", "the function", "yes, and the caller cannot use it"],
                    &["`fn f(v: &T)`", "the caller", "no"],
                    &["`fn f(v: &mut T)`", "the caller", "yes, temporarily"],
                ],
            ),
            callout(
                "The signature is the contract",
                "Once you can translate a parameter list into these three answers, you can read any \
                 Rust API without opening its source. Part 27 turns this into a design procedure.",
            ),
        ],
    )
}

fn returning_references() -> SeedLesson {
    SeedLesson::new(
        "Returning References",
        vec![
            labelled(
                "THE IDEA",
                "A function may return a reference, but only to something that will still exist after \
                 it returns. The one safe source is the data the function borrowed from its caller.",
            ),
            code(
                "rust",
                "fn first_character(s: &str) -> &str {\n    &s[..1]\n}\n\nfn main() {\n    let text = String::from(\"hello\");\n    println!(\"{}\", first_character(&text));\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The returned `&str` points into the string the caller lent to the function. Because \
                 the caller still owns that string after the call, the reference remains valid. The \
                 rule of thumb is simple: a returned reference must come from an input, because inputs \
                 outlive the call by definition.",
            ),
            paragraphs(&["Now the tempting version that cannot work — returning a reference to a local value:"]),
            broken(
                "fn bad() -> &String {\n    let s = String::from(\"hello\");\n    &s\n}\n\nfn main() {\n    println!(\"{}\", bad());\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Inside `bad`, `s` owns a freshly created `String`. When the function returns, `s` goes \
                 out of scope and the string is freed. The reference the function tried to return would \
                 point at freed memory. The compiler rejects this before such a reference can be \
                 created, naming a missing lifetime because the function has no input to tie the \
                 return to.",
            ),
            callout(
                "The rule, in one line",
                "A reference cannot outlive the value it refers to. Returning a reference to a local is \
                 the most common way to violate that rule, and the error will look like lifetime \
                 trouble. It is not. It is a validity problem.",
            ),
        ],
    )
}

fn dangling_references() -> SeedLesson {
    SeedLesson::new(
        "Dangling References",
        vec![
            labelled(
                "THE IDEA",
                "A dangling reference is a reference to a value that no longer exists. Rust makes this \
                 impossible to create in safe code — not by tracking memory cleverly at runtime, but by \
                 refusing to compile programs that would need it.",
            ),
            broken(
                "fn dangling() -> &String {\n    let s = String::from(\"hello\");\n    &s\n}\n\nfn main() {\n    let r = dangling();\n    println!(\"{r}\");\n}",
            ),
            diagram(
                "What a language that allowed this would do",
                "function creates value\n       ↓\nvalue exists\n       ↓\nfunction ends\n       ↓\nvalue dropped\n       ↓\nreturned reference points nowhere",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Read the diagram top to bottom. The value is created, then the function ends, then the \
                 value is dropped, and only then does the caller try to use the reference. In C or C++ \
                 the last step would read freed memory — a use-after-free, and a classic security bug. \
                 Rust stops the program at step two by refusing to compile it.",
            ),
            fixed(
                "fn valid() -> String {\n    let s = String::from(\"hello\");\n    s\n}\n\nfn main() {\n    let r = valid();\n    println!(\"{r}\");\n}",
            ),
            paragraphs(&[
                "The fix is to return the value itself rather than a reference to it. Ownership moves \
                 out of the function into the caller, so there is always exactly one owner responsible \
                 for the value, and it never points at something freed.",
            ]),
            callout(
                "Not omniscience, a rule",
                "This is not 'Rust knows everything about memory'. The borrow checker enforces a \
                 specific rule — references may not outlive the values they refer to — and that rule is \
                 enough to make dangling references unrepresentable in safe code. Part 22 makes the \
                 enforcement procedure concrete.",
            ),
        ],
    )
}
