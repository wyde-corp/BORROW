//! Module 06 — Borrowing, lessons 21–24.
//!
//! From rules to reasoning. Part 21 has the learner reconstruct the system,
//! Part 22 gives them the borrow checker's procedure, Part 23 teaches them to
//! read its output, and Part 24 collects the ten mistakes that most often mean
//! the model has not fully landed.

use crate::content::types::{
    broken, broken_text, callout, diagram, fixed, labelled, paragraphs, steps, table, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        rules_as_a_system(),
        the_borrow_checker(),
        reading_errors(),
        common_mistakes(),
    ]
}

fn rules_as_a_system() -> SeedLesson {
    SeedLesson::new(
        "Borrowing Rules as a System",
        vec![
            labelled(
                "CONCEPT",
                "Stop here and reconstruct the rules yourself before reading on. No new syntax — just \
                 the system the previous twenty parts have been building, stated once as a whole.",
            ),
            steps(&[
                "Who owns the value, and which name is responsible for freeing it?",
                "How many immutable borrows exist, and are any of them still used?",
                "Is there a mutable borrow, and is anything else touching the value while it lives?",
                "Do any two regions overlap in a way that puts a reader with a writer?",
            ]),
            diagram(
                "The system",
                "OWNER\n  │\n  ├── owns value\n  │\n  ├── can create immutable borrows\n  │       └── many can coexist\n  │\n  └── can create mutable borrow\n          └── exclusive access",
            ),
            diagram(
                "Many readers OR one writer",
                "        IMMUTABLE BORROWS\n        ┌────┬────┬────┐\n        ↓    ↓    ↓\n        READ READ READ\n\n\nOR\n\n\n        MUTABLE BORROW\n              ↓\n            WRITE",
            ),
            labelled(
                "EXPLANATION",
                "The rule exists to prevent one thing: unsafe aliasing and mutation. Two views of the \
                 same memory are fine while both only read. The trouble begins when at least one of \
                 them writes, because then the other view can observe a value changing underneath it, \
                 or a reallocation can strand a reference. Every part of the system is in service of \
                 keeping 'read' and 'write' from overlapping.",
            ),
            callout(
                "Reconstruct, do not recite",
                "If you can rebuild this diagram from the concepts — owner, borrow, region, reader, \
                 writer — you do not need to memorise any of it. If you cannot, that is the part to \
                 revisit before moving on. The borrow checker rewards a real model and punishes a \
                 half-remembered rule list.",
            ),
        ],
    )
}

fn the_borrow_checker() -> SeedLesson {
    SeedLesson::new(
        "The Borrow Checker",
        vec![
            labelled(
                "CONCEPT",
                "The borrow checker is the part of the compiler that enforces the borrowing rules. You \
                 do not need to know its internals. You need its procedure, so you can run it in your \
                 head before asking the compiler to.",
            ),
            steps(&[
                "Who owns the value?",
                "Where is each reference created?",
                "Is each reference immutable or mutable?",
                "How long is each reference used?",
                "Are there other references active over the same value?",
                "Is the owner also being accessed directly?",
                "Is anything being mutated while a reader is still live?",
                "Does any reference outlive the value it refers to?",
            ]),
            paragraphs(&["Apply the procedure to a program that looks innocent at a glance:"]),
            broken(
                "fn main() {\n    let mut s = String::from(\"hello\");\n\n    let r = &s;\n\n    s.push_str(\"!\");\n\n    println!(\"{r}\");\n}",
            ),
            labelled(
                "EXPLANATION",
                "Step one: `s` owns the `String`. Step two: `r` is created before the push. Step \
                 three: it is immutable. Step four: `r` is used by the `println!` *after* the push, so \
                 its region covers the write. Step five: there is an active reader. Step seven: the \
                 push mutates the owner while that reader is live. The checker rejects it at exactly \
                 that step.",
            ),
            paragraphs(&["Fix the ordering so the reader finishes first, and the same program is accepted:"]),
            fixed(
                "fn main() {\n    let mut s = String::from(\"hello\");\n\n    let r = &s;\n    println!(\"{r}\");\n\n    s.push_str(\"!\");\n\n    println!(\"{s}\");\n}",
            ),
            callout(
                "The goal is prediction",
                "The point is not to memorise the borrow checker. The point is to be able to look at a \
                 program and say 'this will be rejected, and here is the overlap' before compiling. \
                 That skill is what the final challenge tests.",
            ),
        ],
    )
}

fn reading_errors() -> SeedLesson {
    SeedLesson::new(
        "Reading Borrow Checker Errors",
        vec![
            labelled(
                "CONCEPT",
                "Borrow-checker errors are not noise. Each one names a first borrow, a second access \
                 that conflicts, and the places where the borrows are used. Reading those three facts \
                 tells you the whole story.",
            ),
            labelled(
                "The program",
                "A mutable string, an immutable borrow, then a mutable borrow, and both printed at the \
                 end. Commit to a prediction before reading the error below.",
            ),
            broken(
                "fn main() {\n    let mut s = String::from(\"hello\");\n\n    let r1 = &s;\n    let r2 = &mut s;\n\n    println!(\"{r1}\");\n    println!(\"{r2}\");\n}",
            ),
            broken_text(
                "error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable\n --> main.rs:5:14\n  |\n4 |     let r1 = &s;\n  |               - immutable borrow occurs here\n5 |     let r2 = &mut s;\n  |              ^^^^^^^^ mutable borrow occurs here\n6 |\n7 |     println!(\"{r1}\");\n  |               ---- immutable borrow later used here\n  |\n  = note: some errors include an explanation code; run `rustc --explain E0502`",
            ),
            steps(&[
                "Find the first borrow: `r1 = &s`, line 4, immutable.",
                "Find the second access: `r2 = &mut s`, line 5, mutable.",
                "Find the use that keeps `r1` alive: the `println!` on line 7, after the mutable borrow.",
                "The conflict is that `r1`'s region extends past the point where exclusive access was requested.",
            ]),
            labelled(
                "EXPLANATION",
                "The error is almost a proof. It points at the immutable borrow, the mutable borrow, \
                 and the later use that forces the overlap. The fix is to end the immutable borrow \
                 before taking the mutable one — reorder the prints, or drop `r1` sooner.",
            ),
            paragraphs(&["The second shape of error comes from returning a reference whose source is a local:"]),
            broken(
                "fn bad() -> &String {\n    let s = String::from(\"hello\");\n    &s\n}\n\nfn main() {\n    println!(\"{}\", bad());\n}",
            ),
            broken_text(
                "error[E0106]: missing lifetime specifier\n --> main.rs:1:14\n  |\n1 | fn bad() -> &String {\n  |              ^ expected named lifetime parameter\n  |\n  = help: this function's return type contains a borrowed value,\n          but there is no value for it to be borrowed from",
            ),
            callout(
                "Lifetime words, validity meaning",
                "The error mentions a lifetime because that is the compiler's vocabulary for 'how long \
                 a reference is valid'. You do not need lifetime annotation syntax yet. The meaning is \
                 plain: there is no input for the return to borrow from, so the reference has nothing \
                 to point at. Return the value instead.",
            ),
        ],
    )
}

fn common_mistakes() -> SeedLesson {
    SeedLesson::new(
        "Common Borrowing Mistakes",
        vec![
            labelled(
                "CONCEPT",
                "These ten mistakes account for almost every wrong mental model about borrowing. Read \
                 them as a checklist against your own understanding, not as trivia.",
            ),
            table(
                &["#", "Mistake", "The correction"],
                &[
                    &["1", "thinking borrowing transfers ownership", "the owner keeps it; a borrow owns nothing"],
                    &["2", "thinking `&mut` means the reference is mutable", "it grants write access to the value"],
                    &["3", "thinking `mut` allows every mutation", "the borrow still has to be `&mut`"],
                    &["4", "thinking many mutable references are harmless", "a mutable borrow is exclusive"],
                    &["5", "thinking one `&` locks out all future mutation", "the owner may mutate once the borrow ends"],
                    &["6", "thinking a reference owns the value", "the original owner remains responsible"],
                    &["7", "returning a reference to a local value", "return the value itself"],
                    &["8", "using `.clone()` to dodge a borrow error", "decide whether a second owner is really wanted"],
                    &["9", "using `&String` everywhere", "prefer `&str` unless `String` is needed"],
                    &["10", "thinking a borrow ends at the `{}`", "it ends at its last use"],
                ],
            ),
            labelled("Mistake 7 in code — a reference to a local", "Commit to a prediction, then read the fix."),
            broken(
                "fn local_ref() -> &String {\n    let s = String::from(\"hello\");\n    &s\n}\n\nfn main() {\n    println!(\"{}\", local_ref());\n}",
            ),
            fixed(
                "fn owned() -> String {\n    String::from(\"hello\")\n}\n\nfn main() {\n    println!(\"{}\", owned());\n}",
            ),
            paragraphs(&[
                "The corrected version has no borrow to get wrong. `owned` creates a `String` and moves \
                 it to the caller, so the value and the name that owns it arrive together.",
            ]),
            labelled(
                "Mistake 8 in code — clone instead of design",
                "A borrow error often means the design asked for ownership it did not need. Cloning \
                 silences the error by creating a second value, which changes the ownership design \
                 rather than fixing it. Before cloning, ask whether a borrow would do.",
            ),
            broken(
                "fn longest(a: String, b: String) -> String {\n    if a.len() >= b.len() { a } else { b }\n}\n\nfn main() {\n    let x = String::from(\"short\");\n    let y = String::from(\"longer\");\n\n    let best = longest(x.clone(), y.clone());\n\n    println!(\"{best} {x} {y}\");\n}",
            ),
            labelled(
                "EXPLANATION",
                "This compiles, and the clones are a warning sign. `longest` never needs to own either \
                 string. If it returned a reference it could borrow both and clone nothing. The clone \
                 is not wrong — it is a decision the code does not explain.",
            ),
            callout(
                "Use the list in reverse",
                "When a borrow error appears, find which of these ten it is. Nine times in ten the \
                 answer is here, and the correction is the sentence that follows it. That is faster \
                 and more reliable than guessing.",
            ),
        ],
    )
}
