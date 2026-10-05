//! Module 06 — Borrowing, lessons 8–10.
//!
//! Borrows have a duration, and that duration is decided by use, not by the
//! surrounding braces. These three parts install the liveness mental model that
//! makes `&` and `&mut` stop looking arbitrary.

use crate::content::types::{
    broken, callout, code, diagram, labelled, paragraphs, table, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![borrowing_and_scope(), multiple_immutable(), exclusivity()]
}

fn borrowing_and_scope() -> SeedLesson {
    SeedLesson::new(
        "Borrowing and Scope",
        vec![
            labelled(
                "CONCEPT",
                "A borrow does not last forever, and it does not necessarily last to the end of its \
                 enclosing `{}`. It lasts from the point it is created to the last point it is *used*. \
                 After that final use, the borrow is over and the owner is free again.",
            ),
            code(
                "rust",
                "fn main() {\n    let mut s = String::from(\"hello\");\n\n    {\n        let r = &mut s;\n        r.push_str(\" world\");\n    }\n\n    println!(\"{s}\");\n}",
            ),
            paragraphs(&[
                "Here the mutable borrow lives inside an inner block. When the block closes, `r` goes \
                 out of scope and the borrow ends — so `s` is readable again on the last line. Nothing \
                 surprising yet; that version of the rule has existed since the beginning.",
            ]),
            paragraphs(&["The interesting case is when there is no inner block at all:"]),
            code(
                "rust",
                "fn main() {\n    let mut s = String::from(\"hello\");\n\n    let r = &mut s;\n\n    println!(\"{r}\");\n\n    let r2 = &mut s;\n    r2.push('!');\n\n    println!(\"{s}\");\n}",
            ),
            labelled(
                "EXPLANATION",
                "This compiles, even though `r` and `r2` both mutably borrow `s` in the same outer \
                 scope. They do not overlap: `r` is last used by the `println!`, and `r2` is created \
                 after that. The first borrow is finished before the second begins. Modern Rust \
                 reasons about the region in which a reference is *actually needed*, not the region in \
                 which it is in scope.",
            ),
            callout(
                "Do not learn the old oversimplification",
                "Never tell yourself 'a reference lives until the end of its lexical scope'. That was \
                 true of early Rust and is false today. A borrow ends at its last use, which is why \
                 re-borrowing works so often. If you see code that compiles and 'should not', this is \
                 almost always why.",
            ),
            table(
                &["Question", "Wrong answer", "Right answer"],
                &[
                    &["When does a borrow end?", "at the closing brace", "at its last use"],
                    &["Can borrows overlap?", "no, never", "only if their used regions overlap"],
                ],
            ),
        ],
    )
}

fn multiple_immutable() -> SeedLesson {
    SeedLesson::new(
        "Multiple Immutable Borrows",
        vec![
            labelled(
                "CONCEPT",
                "Several immutable borrows are always allowed to coexist. The moment that becomes a \
                 problem is not when the references *exist*, but when a write is attempted while a \
                 reader is still going to be used.",
            ),
            code(
                "rust",
                "fn main() {\n    let s = String::from(\"hello\");\n\n    let a = &s;\n    let b = &s;\n    let c = &s;\n\n    println!(\"{a}\");\n    println!(\"{b}\");\n    println!(\"{c}\");\n}",
            ),
            paragraphs(&["Three readers, no writer, no conflict. Now add a write while the readers are still live:"]),
            broken(
                "fn main() {\n    let mut s = String::from(\"hello\");\n\n    let a = &s;\n    let b = &s;\n\n    s.push_str(\"!\");\n\n    println!(\"{a}\");\n    println!(\"{b}\");\n}",
            ),
            labelled(
                "EXPLANATION",
                "The compiler rejects this because `a` and `b` are immutable borrows of `s`, and the \
                 direct mutation `s.push_str(\"!\")` needs mutable access to `s`. The borrows are still \
                 *used* by the `println!` calls after the mutation, so their regions overlap the write \
                 and cannot be finished early.",
            ),
            paragraphs(&["Move the final uses of the borrows above the mutation and the same program compiles:"]),
            code(
                "rust",
                "fn main() {\n    let mut s = String::from(\"hello\");\n\n    let a = &s;\n    let b = &s;\n\n    println!(\"{a}\");\n    println!(\"{b}\");\n\n    s.push_str(\"!\");\n\n    println!(\"{s}\");\n}",
            ),
            callout(
                "The problem is use, not existence",
                "`let a = &s;` on its own is harmless. What matters is whether `a` is used after the \
                 write. This is why the same two lines of setup can compile or fail depending entirely \
                 on what comes later. Trace *uses*, not declarations.",
            ),
        ],
    )
}

fn exclusivity() -> SeedLesson {
    SeedLesson::new(
        "Mutable Borrowing and Exclusivity",
        vec![
            labelled(
                "CONCEPT",
                "A mutable borrow is exclusive while it is active. Two mutable references to the same \
                 value cannot both be live and used at once — that would be two writers over one value, \
                 which is exactly the aliasing the rule forbids.",
            ),
            broken(
                "fn main() {\n    let mut s = String::from(\"hello\");\n\n    let r1 = &mut s;\n    let r2 = &mut s;\n\n    r1.push('!');\n    r2.push('?');\n\n    println!(\"{s}\");\n}",
            ),
            labelled(
                "EXPLANATION",
                "`r1` and `r2` both claim mutable access to `s`, and both are used after the second is \
                 created. There is no point at which only one of them is live and the other finished, \
                 so the compiler cannot make either of them exclusive. The error names the second \
                 borrow, because that is the one that broke exclusivity.",
            ),
            paragraphs(&["Two mutable borrows are fine when they do not overlap. The trick is ending the first before the second begins:"]),
            code(
                "rust",
                "fn main() {\n    let mut s = String::from(\"hello\");\n\n    {\n        let r1 = &mut s;\n        r1.push('!');\n    }\n\n    let r2 = &mut s;\n    r2.push('?');\n\n    println!(\"{s}\");\n}",
            ),
            diagram(
                "Sequential, not simultaneous",
                "ONE MUTABLE BORROW\n      ↓\n    use it\n      ↓\n  borrow ends\n      ↓\nANOTHER MUTABLE BORROW",
            ),
            table(
                &["Pattern", "Allowed?", "Why"],
                &[
                    &["two `&mut` created, both used together", "no", "two writers at once"],
                    &["two `&mut` created, first used and finished first", "yes", "regions do not overlap"],
                    &["`&mut` then `&mut` in nested scopes", "yes", "inner borrow ends at the brace"],
                ],
            ),
            callout(
                "Exclusive means no other view",
                "While the mutable borrow `r2` is live, nothing else — not another `&mut`, not a `&`, \
                 not even the owner `s` directly — may touch the value. Exclusive is the whole point; \
                 the writer is the only observer while it runs.",
            ),
        ],
    )
}
