//! Module 05 — Ownership, lessons 6–11.
//!
//! `String` as the teaching tool, then the three ways a value can be handled on
//! assignment: move, copy, clone.

use crate::content::types::{
    breakdown, broken, callout, code, diagram, fixed, labelled, table, text_block, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        string_first_example(),
        the_move(),
        why_move_instead_of_copying(),
        copy_types(),
        clone_(),
        move_vs_copy_vs_clone(),
    ]
}

fn string_first_example() -> SeedLesson {
    SeedLesson::new(
        "String: Our First Ownership Example",
        vec![
            labelled(
                "CONCEPT",
                "`String` is the type that makes ownership visible. It owns heap memory whose size is not \
                 known until runtime, which means someone has to be responsible for freeing it. That \
                 someone is the owner. Watch what happens to two slightly different ways of saying \
                 `\"hello\"`.",
            ),
            labelled(
                "EXPLANATION",
                "`\"hello\"` on its own is not a value with a size decided by your program — it is a \
                 chunk of text sitting inside your compiled binary. It will be there until the process \
                 exits, and nobody has to clean it up, because nothing was ever allocated at runtime. \
                 `String::from(\"hello\")` takes those same five bytes, copies them somewhere the program \
                 owns, and now has to give them back. That new responsibility is the entire reason `String` \
                 behaves differently from everything else you have used so far.",
            ),
            code(
                "rust",
                "fn main() {\n    let literal: &str = \"hello\";            // a view into the binary\n    let owned = String::from(\"hello\");   // data copied to the heap, we own it\n\n    println!(\"{literal} {owned}\");\n    println!(\"{}\", literal.len() + owned.len());   // both behave like text\n}",
            ),
            breakdown(
                "the difference in one picture",
                "let literal = \"hello\";         let owned = String::from(\"hello\");\n\n  no allocation                ┌───────────────┐\n  lives in your binary        │ ptr  ─────────────────────┐\n  never freed                  │ len           5 │        │\n  fixed size                   │ cap           5 │        │\n                              └───────────────┘        │\n                                                       ▼\n                                            ┌───────────────────┐\n                                            │  h e l l o         │\n                                            └───────────────────┘\n                                             allocated at runtime\n                                             owned by `owned`\n                                             freed when `owned` dies",
                &[
                    "`\"hello\"` is a `&str` — a pointer and a length pointing into data that already existed.",
                    "`String::from(\"hello\")` is a value that *owns* a heap buffer it allocated itself.",
                    "That is why the second one can be moved, cloned, and eventually dropped, and the first one cannot.",
                ],
            ),
            text_block(
                "`String` is the right type to learn ownership on, and it is worth saying why rather than \
                 just asserting it. It grows: `push_str` makes it longer, so its size is genuinely not \
                 known when your program is compiled. It owns a buffer. It has cleanup work to do at the \
                 end. Every property that makes ownership necessary is present in it and absent from \
                 `i32`.",
            ),
            callout(
                "A detail that will bite you later",
                "`let s: String = \"hello\";` does not compile. The literal is a `&str` and `String` is not \
                 `&str`, even though both print the same and both have `.len()`. You will meet the \
                 conversion (`\"hello\".to_string()`) in the next lesson. For now just notice: the two \
                 types *look* interchangeable to a beginner and are not interchangeable at all.",
            ),
            diagram(
                "the properties that make String the right teacher",
                "  does it need to grow?          yes   → size unknown at compile time\n  does it own heap data?         yes   → bytes must be handed back\n  does cleanup need a decision?   yes   → who?\n  is it cheap to duplicate?       no    → copying a million bytes is real work\n\n  i32 has none of these. that is why i32 is boring.\n  String has all four. that is why String is the lesson.",
            ),
            callout(
                "Practice — Predict before you run",
                "Write these four lines and, *before running anything*, answer in writing: (1) does this \
                 compile? (2) how many heap allocations happen? (3) who is responsible for each one? \
                 `let a = String::from(\"one\"); let b = String::from(\"two\"); let c = a; let d = a;` \
                 Then run it. The gap between your prediction and reality is the thing to pay attention \
                 to — not whether you were right.",
            ),
            callout(
                "Mini challenge — Find the odd one out",
                "Of `i32`, `bool`, `char`, `f64`, `String`, `Vec<i32>`, `&str`, and `(i32, bool)` — which \
                 ones implement `Copy`, and what property predicts it? Write your rule down as a single \
                 sentence before you look it up. If your sentence is wrong, your mental model has a hole \
                 in it and now is the time to find it.",
            ),
        ],
    )
}

fn the_move() -> SeedLesson {
    SeedLesson::new(
        "Move",
        vec![
            labelled(
                "CONCEPT",
                "Assigning a `String` does not copy it. Ownership transfers. The old binding becomes \
                 unusable, and the new one is responsible for the bytes. This is the single behaviour that \
                 makes Rust's memory safety possible.",
            ),
            labelled(
                "EXPLANATION",
                "Your variable did not disappear. Rust did not reach in and delete anything. The heap \
                 buffer is sitting in exactly the same place it was a microsecond ago, byte for byte. \
                 What happened is that responsibility for it moved from one name to another, and the old \
                 name lost the permission that came with it. It is the difference between a name being \
                 *destroyed* and a name being *disarmed*.",
            ),
            broken(
                "fn main() {\n    let s1 = String::from(\"hello\");\n    let s2 = s1;              // ownership moves to s2\n\n    println!(\"{s1}\");         // error[E0382] — s1 no longer owns anything\n    println!(\"{s2}\");         // fine\n}",
            ),
            diagram(
                "before the move",
                "STACK                              HEAP\n┌──────┐                          ┌─────────┐\n│ s1   │──────────────────────────>│  hello  │\n└──────┘                          └─────────┘\n  owns it\n\none owner. one buffer. no ambiguity.",
            ),
            diagram(
                "after the move",
                "STACK                              HEAP\n┌──────┐                          ┌─────────┐\n│ s1   │ - - - - - - - - - - - - -  │  hello  │\n└──────┘   INVALID                └─────────┘\n  no longer owns                  still exactly where\n  it, may not be used              it always was. nothing\n                                  was copied or deleted.\n┌──────┐\n│ s2   │──────────────────────────>│         │\n└──────┘                          └─────────┘\n  owns it",
            ),
            text_block(
                "The heap allocation did not move. That is the misconception to kill first, because \
                 everything else follows from it. Moving a `String` is a three-word operation on the \
                 stack — the pointer, the length, and the capacity change hands. The bytes themselves \
                 stay put. If they *did* move, `String` would be a hugely expensive operation and \
                 borrowing would be pointless. The cheapness of a move is not an accident; it is the \
                 design that makes everything else affordable.",
            ),
            callout(
                "What 'invalid' actually means",
                "It does not mean the memory is poisoned, or that `s1` now reads garbage, or that some \
                 runtime flag got flipped. `s1` is simply a name that no longer refers to a value, and \
                 the compiler treats using it as a type error — the same error you would get for \
                 referencing a variable that was never declared. There is no way to observe a moved-from \
                 `String` in safe Rust at all, which is a deliberate design choice.",
            ),
            text_block(
                "Now the payoff. Because there is exactly one owner, the cleanup question has exactly \
                 one answer, and Rust can answer it statically:",
            ),
            diagram(
                "the entire point, in three lines",
                "  s1  →  s2\n    ↓\n  one value, one owner, one drop\n\n  versus, if copying were allowed:\n\n  s1 ──┐\n       ├── same bytes\n  s2 ──┘\n\n  two owners of one buffer. both freed it? neither?\n  there is no good answer, so Rust will not let you write it.",
            ),
            callout(
                "A move is not a copy, and not a deletion",
                "Three separate ideas that get collapsed into the word \"move\", so keep them apart. \
                 Nothing is copied — there is no second buffer. Nothing is deleted — the buffer is \
                 untouched. What changed is **which name is responsible for it**. If you find yourself \
                 imagining data flying across the screen, stop and redraw the diagram: the arrow from \
                 the stack box to the heap does not change. Only the label on the box does.",
            ),
            callout(
                "Practice — Track the owner",
                "For each line, name the current owner of the `String` and mark every variable that is no \
                 longer usable: (a) `let a = String::from(\"1\");` (b) `let b = a;` (c) `let c = b;` \
                 (d) `let d = c.clone();` (e) end of scope. Then answer: at end of scope, how many times \
                 is memory freed, and by whom? This exercise looks trivial and it is the foundation for \
                 everything else in the module.",
            ),
            callout(
                "Mini challenge — Prove nothing moved",
                "Write a program that moves a `String` around five times and never clones it. Print the \
                 address of the underlying data before and after — `s.as_ptr()` gives it to you. What do \
                 you observe, and what does that prove about what a move actually costs? Then explain why \
                 that matters for the module after this one.",
            ),
        ],
    )
}

fn why_move_instead_of_copying() -> SeedLesson {
    SeedLesson::new(
        "Why Rust Moves Instead of Copying",
        vec![
            labelled(
                "CONCEPT",
                "The alternative to moving was copying everything, silently, on every assignment. That is \
                 a real engineering decision with a real cost, and Rust decided against it.",
            ),
            labelled(
                "EXPLANATION",
                "Nobody designed moves to be confusing. Moves are the *result* of a much more boring \
                 argument about performance. If assignment silently duplicated every value, then the cost \
                 of an assignment would depend on how big the value was — and for a vector of a million \
                 numbers, that cost would be enormous, and completely invisible at the call site.",
            ),
            table(
                &["", "Silently deep-copying", "Moving"],
                &[
                    &["What assignment does", "Allocates, copies every byte, now two owners exist", "Transfers three words on the stack"],
                    &["Cost of `let b = a;`", "O(size of the value)", "O(1), always"],
                    &["Surprise factor", "High — a one-word line can cost a megabyte", "None — it is visible in the type"],
                    &["Cleanup", "Now genuinely ambiguous", "Unambiguous: exactly one owner"],
                    &["What you must write to duplicate", "nothing", "`clone()` — and it is right there on the line"],
                ],
            ),
            code(
                "rust",
                "fn main() {\n    let data = vec![0u8; 1_000_000];   // a megabyte, on the heap\n\n    let copy = data;                 // O(1). transfers the Vec header.\n                                    // no million bytes were touched.\n\n    println!(\"{}\", copy.len());    // `data` is gone — this is a move, not a copy\n}",
            ),
            text_block(
                "Now imagine the deep-copying version of that same line. A million bytes copied, every \
                 single time, because you assigned a variable. No `memcpy` in the source. No hint that \
                 anything expensive happened. Your program gets slower because of a line that looks like \
                 it costs nothing, and profiling takes an afternoon to find.",
            ),
            callout(
                "The other half of the argument",
                "Performance is the reason Rust chooses moves; *safety* is the reason it cannot also allow \
                 copies to be implicit. Deep-copying every assignment would sidestep double-free entirely \
                 — but only by making the copying invisible, unbounded, and usually unwanted. Rust would \
                 rather be explicit at the call site than clever about it. A line that says `.clone()` \
                 costs what it looks like it costs.",
            ),
            callout(
                "The rule of thumb that survives contact with real code",
                "Assignment of an owning, heap-backed value transfers ownership. It never duplicates \
                 silently. If you want duplication, you ask for it by name. If you want to *avoid* \
                 transfer, that is borrowing, and it is the next module — but the module after that. For \
                 now, transfer and duplicate are the only two options, and they are both explicit.",
            ),
            callout(
                "Practice — Cost estimation",
                "For each, estimate whether the assignment is O(1) or O(n), and say why: (a) \
                 `let y = 5;` (b) `let s2 = s1;` where `s1: String` holds 10 bytes (c) `let v2 = v1;` \
                 where `v1: Vec<f64>` holds 1,000,000 elements (d) `let s2 = s1.clone();` with the same \
                 10-byte string (e) `let n2 = n1;` where `n1: u128`. Then find the one that surprised you \
                 and explain why.",
            ),
            callout(
                "Mini challenge — Argue the other side",
                "Write the strongest honest case *for* implicit deep copying. Who would benefit, and what \
                 would it cost them? Then say why it lost. Being able to state the losing argument well is \
                 how you find out whether you actually understand the design — a rule you cannot argue both \
                 sides of is a rule you have only memorised.",
            ),
        ],
    )
}

fn copy_types() -> SeedLesson {
    SeedLesson::new(
        "Copy Types",
        vec![
            labelled(
                "CONCEPT",
                "Some types are cheap and safe to duplicate, so assignment copies them and both variables \
                 stay usable. This is not a property of \"primitives\" — it is a property of the type \
                 declaring that it implements `Copy`.",
            ),
            labelled(
                "EXPLANATION",
                "If duplicating a value cannot cause a double free, cannot leak, and costs nothing \
                 measurable, then there is no safety reason to forbid making two of them. `5` copied into \
                 `y` is not two owners of one resource — it is two identical numbers. Nothing has to be \
                 freed, so nothing has to be freed exactly once, so the whole ownership apparatus simply \
                 does not apply. `Copy` is the type saying *I am safe to duplicate*.",
            ),
            code(
                "rust",
                "fn main() {\n    let x = 5;\n    let y = x;        // copied — the number 5 has no resource\n\n    println!(\"{x} {y}\");   // both fine\n}",
            ),
            broken(
                "fn main() {\n    let s1 = String::from(\"hello\");\n    let s2 = s1;\n\n    println!(\"{s1}\");   // String does NOT implement Copy\n}",
            ),
            fixed(
                "fn main() {\n    let x = 5;\n    let y = x;        // copied\n    println!(\"{x} {y}\");\n\n    let s1 = String::from(\"hello\");\n    let s2 = s1.clone();   // duplicated, explicitly\n    println!(\"{s1} {s2}\");\n}",
            ),
            table(
                &["Type", "Implements `Copy`?", "Why"],
                &[
                    &["`i32`, `u64`, `f64`, `bool`, `char`", "Yes", "Fixed size, no heap, no cleanup"],
                    &["`&T` (shared reference)", "Yes", "Copying a reference copies the pointer, not the target"],
                    &["`[T; N]` where `T: Copy`", "Yes", "Fixed size, and every element is itself copyable"],
                    &["`String`", "No", "Owns a heap buffer that must be freed exactly once"],
                    &["`Vec<T>`", "No", "Same, for a growable buffer"],
                    &["`&mut T`", "No", "Copying it would alias two owners of one mutable value"],
                ],
            ),
            text_block(
                "That last row is the interesting one. A shared reference `&T` is `Copy` — handing \
                 someone a second `&T` cannot let them free anything, because they never owned it. But a \
                 mutable reference `&mut T` deliberately is not. If two bindings could both write through \
                 the same reference, you would have two owners of one piece of state and no way to stop \
                 them disagreeing. One rule, `Copy`, quietly encodes that distinction.",
            ),
            callout(
                "The rule people teach wrong",
                "\"Stack types copy, heap types move\" is memorable, nearly right, and will eventually \
                 bite you. The accurate statement is one question long: **does this type implement \
                 `Copy`?** That is a decision the type's author made, declared in the type definition, and \
                 visible in the docs. A `[u8; 1024]` is entirely stack-resident and is `Copy`. A `String` \
                 containing only three stack words is not. Stack versus heap is a good *reason*; the trait \
                 is the *rule*.",
            ),
            callout(
                "What `Copy` actually requires of a type",
                "It is a promise, and the compiler checks the promise in one specific way: you cannot \
                 implement `Copy` for a type that also implements `Drop`. If a type needs custom cleanup, \
                 then it must be possible for it to be destroyed, and two copies would mean two cleanups. \
                 The two traits are mutually exclusive by construction, not by convention.",
            ),
            callout(
                "Practice — Sort the types",
                "For each of these, decide `Copy` or not, and give the one-sentence reason: `u8`, `String`, \
                 `bool`, `Vec<u8>`, `char`, `&str`, `(i32, u8)`, `[u8; 4]`, `Box<i32>`, `Option<char>`. \
                 Two of them are meant to be tricky — `&str` and `Option<char>`. Work out why each is \
                 what it is before guessing.",
            ),
            callout(
                "Mini challenge — Write a `Copy` type",
                "Define your own struct that implements `Copy` and `Clone`, hold only a `u32`, and confirm \
                 that assigning it leaves the original usable. Then try to add `Copy` to a version of it \
                 that also implements `Drop`, and read the compiler's refusal carefully. Quote it in a \
                 comment. That error message is a better explanation of `Copy` than any paragraph.",
            ),
        ],
    )
}

fn clone_() -> SeedLesson {
    SeedLesson::new(
        "Clone",
        vec![
            labelled(
                "CONCEPT",
                "`clone()` asks for a second, independent, fully-owned value. It is explicit, it may be \
                 expensive, and for a `String` it means genuinely duplicating the bytes. That is a tool, \
                 not a mistake.",
            ),
            labelled(
                "EXPLANATION",
                "Every design has a moment where it forces you to be explicit about something you would \
                 rather leave automatic, and this is Rust's. When you write `.clone()`, you are making a \
                 statement: *I intend for two owners of this kind of thing to exist, and I accept what \
                 that costs.* A reader sees it on the line. You see it in review. You can grep for it. \
                 That is the entire appeal.",
            ),
            code(
                "rust",
                "fn main() {\n    let s1 = String::from(\"hello\");\n    let s2 = s1.clone();\n\n    println!(\"{s1} still works\");\n    println!(\"{s2} works too\");\n}",
            ),
            diagram(
                "clone — two owners, two buffers",
                "STACK                    HEAP\n┌──────┐                ┌─────────┐\n│ s1   │────────────────>│  hello  │  ← allocation A\n└──────┘                └─────────┘\n┌──────┐                ┌─────────┐\n│ s2   │────────────────>│  hello  │  ← allocation B, a different one\n└──────┘                └─────────┘\n\n  two owners. two values. two frees. no ambiguity.\n  note they are equal in *content* and distinct in *identity*.",
            ),
            text_block(
                "That last distinction is worth a moment. `s1 == s2` is true — they compare by contents. \
                 But they are not the same buffer, and mutating one cannot affect the other. `clone()` \
                 produces a value that is equal, not identical.",
            ),
            callout(
                "Clone is a trait, and what it does is up to the implementor",
                "Do not assume every `clone()` is a byte-for-byte deep copy of everything, uniformly. \
                 `Clone` is a trait; each implementation defines what duplication means for that type. \
                 For `String` it does duplicate the owned bytes — that is what you want here. For a type \
                 you write yourself, `clone()` might share something behind a reference count, or copy a \
                 small handle while leaving large data shared. The honest phrasing is: *`clone()` creates \
                 another value the type considers independent, and for `String` that means another heap \
                 buffer.*",
            ),
            callout(
                "How expensive is clone?",
                "Proportional to the data. A `String` of 5 bytes is trivial. A `String` of five million \
                 bytes is five million bytes allocated and written, and it will not be optimised away in \
                 a way you should count on. `Vec<i32>` holding a million elements is a million elements \
                 copied. So the question is never *\"is clone bad\"* — it is *\"how much am I cloning, and \
                 do I need to\"*. For small values, freely. For large ones, think.",
            ),
            callout(
                "Do not memorise \"never clone\"",
                "The advice you will hear — *\"clone until it compiles, then clean up later\"* — is \
                 terrible as a habit, because it teaches you to reach for the tool that makes the error go \
                 away rather than the tool that expresses your intent. But the opposite advice is also \
                 wrong. Sometimes you genuinely want two independent owners: a cache you keep after \
                 handing the original away, a value stored in two places, a test fixture you must not \
                 mutate. Clone when that is what you mean.",
            ),
            text_block(
                "There is a third option coming, and it is the one that will usually be right: **do not \
                 take ownership at all.** If you only need to read a `String`, taking a reference lets you \
                 skip both the move and the clone. That is borrowing, it is the entire subject of the next \
                 module, and it is worth knowing now only so you stop treating clone as the answer to \
                 every ownership complaint.",
            ),
            callout(
                "Practice — Count the allocations",
                "For each snippet, write down how many heap allocations occur, and whether the original \
                 survives: (a) `let b = a.clone();` (b) `let b = a;` (c) `let b = a; let c = a;` \
                 (d) `let b = a.clone(); let c = b;` (e) `let b = a.clone(); let c = a.clone();`. For (e), \
                 work out whether `b` and `c` could ever end up pointing at the same buffer.",
            ),
            callout(
                "Mini challenge — Justify every clone",
                "Take any program you have written in the last few modules and find every clone in it. \
                 For each one, write a sentence: *I am cloning because …, and the cost is …*. If you \
                 cannot complete that sentence honestly, you have found either a clone you do not need or \
                 an ownership decision you made without thinking. Both are worth knowing about.",
            ),
        ],
    )
}

fn move_vs_copy_vs_clone() -> SeedLesson {
    SeedLesson::new(
        "Move vs Copy vs Clone",
        vec![
            labelled(
                "CONCEPT",
                "Three outcomes for `let b = <something involving a>;`. One is decided by the type, one \
                 is decided by you. Once you can predict which is which from the left-hand side alone, you \
                 will never be surprised by an ownership error again.",
            ),
            labelled(
                "EXPLANATION",
                "This is the single most useful thing to memorise in the module, and it is a decision \
                 procedure rather than a fact. Look at the type. Ask whether it implements `Copy`. If yes, \
                 you are done — it copied, both names are live, nothing else can have happened. If it does \
                 not, look for `.clone()` on the right-hand side. If you find one, you are done. If you find \
                 neither, ownership moved. Three questions, and the answer is determined before you run \
                 anything.",
            ),
            table(
                &["Concept", "Trigger", "Original usable?", "New owned value?", "Cost"],
                &[
                    &["Copy", "implicit, type is `Copy`", "Yes — it is still whole", "Yes, trivially", "Fixed and tiny"],
                    &["Move", "implicit, type is not `Copy`", "No — it is disarmed", "Yes — it is the same value", "O(1), constant"],
                    &["Clone", "explicit `.clone()`", "Yes", "Yes — a genuinely separate one", "O(size of the data)"],
                ],
            ),
            code(
                "rust",
                "fn main() {\n    // 1. COPY — i32 is Copy, so both survive\n    let a = 10;\n    let b = a;\n    println!(\"{a} {b}\");        // fine\n\n    // 2. MOVE — String is not Copy, and there is no clone()\n    let s1 = String::from(\"x\");\n    let s2 = s1;\n    println!(\"{s2}\");            // fine\n    // println!(\"{s1}\");          // error: moved\n\n    // 3. CLONE — asked for explicitly, two real values\n    let t1 = String::from(\"y\");\n    let t2 = t1.clone();\n    println!(\"{t1} {t2}\");        // fine\n}",
            ),
            callout(
                "Notice what is *not* on this table",
                "There is no \"silent deep copy\" row, and no \"borrow\" row. Borrowing is not a fourth \
                 way that assignment behaves — it is a way to avoid the question entirely, and it comes \
                 next. The absence of a fourth row is the point: `let b = a;` has exactly two possible \
                 outcomes, and the type on the left decides which.",
            ),
            diagram(
                "the decision procedure",
                "  let b = <expr involving a>;\n         │\n         ▼\n  does the type of `a` implement Copy?\n         │\n    ┌────┴────┐\n   yes         no\n    │           │\n    ▼           ▼\n  COPY       is there a .clone() in <expr>?\n  both live      │\n            ┌───┴───┐\n           yes      no\n            │        │\n            ▼        ▼\n          CLONE     MOVE\n        two values  one value, new owner\n        old lives   old name is dead",
            ),
            callout(
                "Practice — Predict the outcome",
                "For each line, answer all four: copied, moved, or cloned? Is the original still usable? \
                 How many owners at the end? (a) `let y = x;` with `x: i32` (b) `let s2 = s1;` with \
                 `s1: String` (c) `let s2 = s1.clone();` (d) `let v2 = v1;` with `v1: Vec<i32>` \
                 (e) `let c = b;` with `b: char` (f) `let t = (a, b);` where `a: String, b: i32`. \
                 Item (f) is not in the table above — work out what it does and say why.",
            ),
            callout(
                "Mini challenge — Break the procedure",
                "Find a type that implements `Copy` where `.clone()` still does something non-trivial, or \
                 prove that no such type exists. Then find a line of code where the answer to \"copied, \
                 moved, or cloned?\" cannot be determined from the type alone, and explain why that is \
                 still acceptable design. There is a real answer to both, and it is more interesting than \
                 the exercise sounds.",
            ),
        ],
    )
}
