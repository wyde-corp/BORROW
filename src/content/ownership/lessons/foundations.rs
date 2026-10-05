//! Module 05 — Ownership, lessons 1–5.
//!
//! Foundations: the problem ownership solves, where values live, what a value
//! actually is, what scope does, and the three rules themselves.

use crate::content::types::{
    broken, broken_text, callout, checklist, code, diagram, fixed, labelled, paragraphs, table,
    text_block, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        why_ownership_exists(),
        stack_and_heap(),
        what_is_a_value(),
        scope_and_lifetime(),
        the_three_rules(),
    ]
}

fn why_ownership_exists() -> SeedLesson {
    SeedLesson::new(
        "Why Ownership Exists",
        vec![
            labelled(
                "CONCEPT",
                "Every running program needs memory. Not some of it — all of it. Every number you add, \
                 every string you hold, every struct you build lives somewhere in RAM. And that somewhere \
                 has to be handed back when you are finished, or your program slowly eats the machine alive.",
            ),
            labelled(
                "EXPLANATION",
                "There are exactly two honest answers to the question 'what happens when we are done with \
                 this memory?'. The first is 'someone works it out while the program runs' — a garbage \
                 collector walks the heap, finds what nothing points at any more, and frees it. You pay for \
                 that with pauses you cannot predict and work you did not ask for. The second is 'the \
                 programmer remembers' — malloc and free by hand, or new and delete. You pay for that with \
                 bugs that only appear when the timing is wrong. Rust refuses both, and that refusal is \
                 what this module is about.",
            ),
            paragraphs(&[
                "So where does the third option come from? From refusing to treat ownership as a runtime \
                 question at all. If the rules of who-owns-what are written down, and the compiler can check \
                 them, then the memory decisions are settled before your program ever starts running. No \
                 collector. No manual bookkeeping. No pause you did not ask for.",
                "That is a genuinely different bet from the one C and C++ made, and it is worth being clear \
                 about what you are buying and what you are paying. You buy: memory safety with no runtime \
                 cost, and errors that show up at compile time with an exact line number. You pay: a set of \
                 rules the compiler will not let you ignore.",
            ]),
            callout(
                "The two bugs this exists to prevent",
                "A **use-after-free**: your program keeps reading memory it no longer owns, and the contents \
                 change underneath you while you read. A **double free**: two things both believe they own \
                 the same memory, so the allocator is asked to release it twice, and that second release \
                 hands your heap to someone else entirely. Neither is exotic. Between them they account for \
                 a large share of security advisories in C and C++ codebases for four decades. Rust makes \
                 both unrepresentable, and the rules you are about to learn are the price of admission.",
            ),
            table(
                &["Approach", "Who frees the memory", "What you pay for it"],
                &[
                    &[
                        "Garbage collection",
                        "A collector, at runtime",
                        "Predictable code, unpredictable pauses, and work you never requested",
                    ],
                    &[
                        "Manual management",
                        "You, in every code path",
                        "Fast, and a whole category of bugs that only fire under the right timing",
                    ],
                    &[
                        "Ownership (this one)",
                        "The compiler, before the program runs",
                        "Rules to follow — checked once, never again at runtime",
                    ],
                ],
            ),
            text_block(
                "The phrase you will hear most in this module is *static* versus *dynamic*. Dynamic means \
                 decided while running, which is what a collector does. Static means decided before running, \
                 which is what a compiler does. Rust is trying to move as much of the memory question as \
                 possible from the first column to the second. Not all of it — there are things genuinely \
                 easier at runtime — but ownership is the biggest single chunk of that move.",
            ),
            diagram(
                "the shape of the problem before we solve it",
                "  you write code                  the machine needs answers\n  ─────────────────                ──────────────────────────────\n  let s = ...;\n  use s\n  forget s                        who frees the bytes?\n                                   when?\n                                   what if I use it after?\n\n  three possible parties to ask:\n\n    the runtime        the programmer        the compiler",
            ),
            callout(
                "Practice — Ask the question out loud",
                "Before you touch a single line of Rust, answer this in plain English for any program you \
                 write: *who is responsible for freeing the memory this allocation uses?* Write the answer \
                 next to it. If your honest answer is \"nobody, we just hope\", that is not a moral \
                 failure — it is the exact situation Rust was built to end. Notice how uncomfortable the \
                 question feels. That discomfort is the feature.",
            ),
            callout(
                "Mini challenge — The uncomfortable question",
                "Some memory is not *allocated* at all: a `u32` is four bytes sitting in a register or a \
                 stack slot, and there is nothing to hand back. If that is true, does it need an owner at \
                 all? Hold that question — it is not as obvious as it sounds, and the answer is the reason \
                 `Copy` exists. Write down your guess now so you can check it in lesson 9.",
            ),
        ],
    )
}

fn stack_and_heap() -> SeedLesson {
    SeedLesson::new(
        "Memory: Stack and Heap",
        vec![
            labelled(
                "CONCEPT",
                "Two regions, two different jobs. The stack is a stack of fixed-size slots tied to the \
                 call stack — cheap, automatic, rigid. The heap is general-purpose storage you can ask for \
                 at runtime — flexible, manual, and the reason ownership exists at all.",
            ),
            labelled(
                "EXPLANATION",
                "Imagine a clerk's counter with a spike of forms behind it. Every form you take goes on top \
                 of the pile, and you hand forms back in exactly the reverse order — last one out, first \
                 one in. Nothing is labelled, nothing is searched for, and nothing is ever out of order. \
                 That is the stack. Now imagine a warehouse across town. You ask for a crate, it arrives \
                 whenever it arrives, and when you are done with it someone has to phone the warehouse to \
                 say the crate is free. That is the heap. Programs use both, constantly, and most of what \
                 confuses people about memory is really a mix-up between them.",
            ),
            paragraphs(&[
                "The stack is organised around one fact: **the size of everything on it is known before \
                 the program starts**. A function's parameters and locals occupy a region of fixed size, \
                 handed out in last-in-first-out order as calls nest and unwind. That is why it needs no \
                 bookkeeping — the frame itself is the bookkeeping.",
                "The heap exists for data whose size is not known until the program is running. A string \
                 typed by a user. A vector that grows. A linked list whose length depends on input. You ask \
                 for space, you get a handle to it, and *you* are responsible for saying when it is done.",
            ]),
            diagram(
                "the stack grows and unwinds with the call stack",
                "TOP OF STACK\n┌───────────────┐\n│ main's locals │  ← in scope\n├───────────────┤\n│   foo locals  │  ← called from main\n├───────────────┤\n│    bar locals │  ← called from foo\n└───────────────┘\nBOTTOM\n\nas calls return, frames pop off the top\nas calls nest, new frames are pushed on\n\nno searching, no allocation, no free list",
            ),
            text_block(
                "Now the part that actually matters for ownership. A `String` is not *on* the heap. A \
                 `String` is a small fixed-size value — three machine words — that *points at* heap data. \
                 The value itself sits on the stack. What it owns is somewhere else entirely.",
            ),
            diagram(
                "what a String really is",
                "STACK                          HEAP\n┌────────────────────┐          ┌─────────────────────┐\n│ ptr      ───────────────────────>│  h  e  l  l  o       │\n│ length       5  │           └─────────────────────┘\n│ capacity     5  │\n└────────────────────┘\n   ^ the String value:            ^ the bytes it owns\n     24 bytes, fixed size,            sized at runtime,\n     trivially copyable               must be freed",
            ),
            text_block(
                "Read that diagram carefully, because most ownership confusion collapses once it lands. \
                 When we later say \"`s1` was moved\", we do **not** mean the letters moved. They are still \
                 on the heap, in exactly the same place. What moved is the three-word box on the stack — \
                 the pointer, the length, and the capacity. Ownership is about the box. The box is the \
                 only thing that can be copied or transferred, and it is the only thing Rust tracks.",
            ),
            callout(
                "A warning about this model",
                "These diagrams are a *conceptual* model, not a map of every implementation detail of every \
                 Rust type. The standard library may use different internal representations, and a compiler \
                 is free to optimise aggressively — a `String` that never escapes might not touch the heap \
                 at all in an optimised build. The diagrams are here because they help you reason about \
                 *who is responsible for what*, and that reasoning is correct regardless of how the code \
                 was actually laid out. Hold the model loosely and the reasoning tightly.",
            ),
            callout(
                "The rule people teach wrong",
                "You will see this everywhere: *primitive types live on the stack, complex types live on \
                 the heap.* It is memorable and it is not true. `String` contains stack data. A tuple of \
                 four `u64`s is entirely stack-resident and is not a \"primitive type\" by any definition. \
                 The question is not primitive-versus-complex, it is **does this type need to allocate, and \
                 can its size be known in advance**. That is the question to ask.",
            ),
            table(
                &["", "Stack", "Heap"],
                &[
                    &["Size known at compile time", "Yes", "No"],
                    &["Allocation cost", "Essentially free — move a pointer", "Real: ask the allocator"],
                    &["Order", "Last in, first out", "No order"],
                    &["Freed by", "End of scope, automatically", "Someone must decide — this is the whole problem"],
                    &["Grows during execution", "No", "Yes"],
                ],
            ),
            callout(
                "Practice — Predict the layout",
                "For each of these, say whether the value needs the heap, and what its stack representation \
                 is: (a) `let x = 42;` (b) `let s = String::from(\"hi\");` (c) `let v = vec![1, 2, 3];` \
                 (d) `let t = (1, 2.0, true);` (e) `let n: Option<Box<i32>>;`. For each, also answer: if I \
                 assign it to another variable, is anything allocated a second time? Writing these down \
                 forces you to commit to an answer before checking yourself.",
            ),
            callout(
                "Mini challenge — Break the model",
                "`String` needs the heap because its length is not known in advance. Is that the *only* \
                 reason? Find a type that allocates on the heap despite having a size known at compile \
                 time. (Think about a fixed-size type that still has to own something variable.) Then find \
                 a type with no heap allocation at all whose size is *not* known at compile time, if you \
                 can. Both exist. Write down what they are and what they teach you about the limits of \
                 the stack/heap split.",
            ),
        ],
    )
}

fn what_is_a_value() -> SeedLesson {
    SeedLesson::new(
        "What Is a Value?",
        vec![
            labelled(
                "CONCEPT",
                "The variable is not the data. It is a name that currently refers to a value. Once you \
                 separate the two, moves stop being mysterious — because only the *value* gets transferred, \
                 and the name is just a label that comes off it.",
            ),
            labelled(
                "EXPLANATION",
                "You have written `let x = 5;` a hundred times and read it as \"x is five\". That is fine \
                 shorthand and it will quietly mislead you the moment the value is a `String`. Think of a \
                 mug. `s` is not the coffee and it is not the mug — `s` is the label stuck to the side of \
                 the mug. Rip the label off and stick it on a different mug and the coffee has not moved. \
                 What changed is which mug the name refers to.",
            ),
            diagram(
                "name, binding, value",
                "  s                        the name you typed\n  │\n  ▼\n  binding                   the fact that s currently refers to something\n  │\n  ▼\n  value                     the actual thing: {ptr, len, cap} on the stack,\n  │                          plus 5 bytes on the heap\n  ▼\n  owned data                the bytes s is responsible for\n\nRust's job is tracking which name owns which value.\nWhen ownership moves, the value does not change.\nThe binding does.",
            ),
            text_block(
                "Three words, and the difference between them is where every remaining confusion lives. \
                 **Name** is what you typed. **Binding** is the fact that this name currently refers to \
                 something, and it is created by `let`. **Value** is the thing itself — the actual data in \
                 memory. A `String` value is those three words. A `u32` value is four bytes. An \
                 `i64` value is eight.",
            ),
            code(
                "rust",
                "fn main() {\n    let x = 5;              // x is a name; 5 is the value\n    let y = x;              // y is a new name for the same value\n\n    // The name x and the name y now both refer to the value 5.\n    // For an i32 that is harmless, because 5 owns nothing.\n    // For a String it is not harmless at all.\n}",
            ),
            table(
                &["Term", "What it is", "Can you have two of them?"],
                &[
                    &[
                        "Name",
                        "The identifier you typed",
                        "Yes — twice in different scopes, or shadowed",
                    ],
                    &[
                        "Binding",
                        "The link between a name and a value",
                        "One name binds to one value at a time",
                    ],
                    &[
                        "Value",
                        "The data itself, in memory",
                        "Yes — as many as exist, but then something owns each one",
                    ],
                ],
            ),
            text_block(
                "That last row is the one that will keep you honest. Two variables can absolutely refer to \
                 two separate values — that is what `clone()` gives you, and what `Copy` gives you for free. \
                 What can never happen is two *owners* for one value. Multiple names pointing at separate \
                 copies is fine. Two names both claiming to be responsible for freeing the same bytes is \
                 the thing Rust refuses.",
            ),
            callout(
                "The question that fixes the mental model",
                "Whenever a move confuses you, ask: **did the value change, or did the binding change?** \
                 For a `String` the answer is always the binding. The heap bytes are sitting in exactly the \
                 same place they were. Rust transferred responsibility for them, and invalidated the name \
                 that no longer holds it. Nothing was copied. Nothing was deleted. One label came off and \
                 went somewhere else.",
            ),
            callout(
                "Practice — Say what each word refers to",
                "Take this and label every noun: *In `let s = String::from(\"hi\");` the name `s` is a \
                 binding to a `String` value, whose pointer and length live on the stack while its bytes \
                 live on the heap, and `s` is the owner of those bytes.* Do the same for \
                 `let v = vec![1, 2, 3];` and for `fn f() -> String { String::from(\"x\") }`. If you cannot \
                 produce a sentence that precise without hesitating, that is the gap to close.",
            ),
            callout(
                "Mini challenge — Write your own glossary",
                "Open a file and write three definitions in your own words, in this order: *value*, \
                 *binding*, *ownership*. No peeking at the lesson. Then write one sentence that is \
                 deliberately wrong but plausible — the misconception you had an hour ago. Keep that one too. \
                 Reading it in three months should be embarrassing, which means it was useful.",
            ),
        ],
    )
}

fn scope_and_lifetime() -> SeedLesson {
    SeedLesson::new(
        "Scope and the Lifetime of a Variable",
        vec![
            labelled(
                "CONCEPT",
                "A variable's lifetime starts where it is declared and ends where its scope ends. Rust \
                 knows exactly when that is, which means Rust knows exactly when you are finished with a \
                 value — without you telling it.",
            ),
            labelled(
                "EXPLANATION",
                "Scope is not a Rust idea, it is a programming idea. What is new is that Rust *uses* it. \
                 In most languages the end of a scope means the compiler stops caring about a name. Here \
                 it means the value is cleaned up, right there, on that line, in that order, with no \
                 runtime check needed. Scope became a cleanup schedule.",
            ),
            code(
                "rust",
                "fn main() {\n    {\n        let s = String::from(\"hello\");\n        println!(\"{s}\");\n    }\n\n    // s is no longer accessible here.\n    // println!(\"{s}\");   ← error[E0425]: cannot find value `s`\n}",
            ),
            diagram(
                "the lifecycle of a binding",
                "  let s = ...      s enters scope\n      │              s may be used\n      ▼\n      │              ...\n      │\n  closing brace    scope ends\n      │              s may never be used again\n      ▼\n  (dropped)        the value is cleaned up HERE\n\n  the compiler knows this moment statically.\n  no runtime check. no collector. it simply happens.",
            ),
            broken(
                "fn main() {\n    {\n        let s = String::from(\"hello\");\n        println!(\"{s}\");\n    }\n\n    println!(\"{s}\");   // error[E0425]: s ended at the inner brace\n}",
            ),
            fixed(
                "fn main() {\n    let s = String::from(\"hello\");\n\n    {\n        println!(\"{s}\");\n    }\n\n    println!(\"{s}\");   // both fine — s belongs to main, not the block\n}",
            ),
            text_block(
                "The fix is not about extending a lifetime. It is about where the variable was declared. \
                 In the first version `s` belongs to the inner block, and that block closes. In the \
                 second, `s` is declared in `main` and ends with `main` — the inner braces did not \
                 shorten its life, they just had their own, shorter one. Nesting scopes is how you \
                 control *when* values die without controlling *whether* they die — and that turns out \
                 to be a genuinely useful tool, not just a curiosity.",
            ),
            callout(
                "A word we are not using yet",
                "You will hear *lifetime* constantly in Rust, and in this module it means only the \
                 conceptual thing above: how long a value stays valid, measured by its scope. Rust also \
                 has a formal lifetime *annotation* system — the `'a` you will meet in a few modules — \
                 which is a way of telling the compiler how long a **reference** stays valid. Different \
                 thing, related idea, much later. If someone starts talking about lifetime annotations \
                 today, they have skipped ahead.",
            ),
            text_block(
                "For now, hold the version you can verify by reading braces: a value is valid from the \
                 `let` that creates it until the end of the block it was declared in. Everything about \
                 ownership follows from that, because it means Rust always knows whose job it is to clean \
                 up, and exactly when.",
            ),
            callout(
                "Practice — Name the end point",
                "For each snippet, say exactly which line the value stops existing, and which variables \
                 are still alive on the line after it: (a) `let x = 5;` in `main` (b) `let s = String::new();` \
                 inside an inner block (c) a `String` bound as a function parameter (d) a `String` that a \
                 function returned. For (d), you may need to think about where the value came from.",
            ),
            callout(
                "Mini challenge — Make scope do something",
                "Write a function that prints two things and guarantees the first is cleaned up before the \
                 second starts, using scope rather than any explicit free call. Then write the version \
                 that does it with nested blocks only, and say which one you would rather read in six \
                 months. There is no wrong answer — but you should be able to justify yours.",
            ),
        ],
    )
}

fn the_three_rules() -> SeedLesson {
    SeedLesson::new(
        "The Three Ownership Rules",
        vec![
            labelled(
                "CONCEPT",
                "Three rules. One owner per value. Ownership can move. Out of scope means gone. Every \
                 behaviour in this module is a consequence of these three sentences, and the compiler \
                 never enforces anything else about ownership.",
            ),
            labelled(
                "EXPLANATION",
                "These read like trivia and they are not. They are the *whole* system. Rust does not have \
                 a hidden fourth rule about ownership that it will spring on you in module 6 — it has \
                 these three, and everything else you will learn is detail about how they apply. If a \
                 piece of code surprises you, one of these three is why.",
            ),

            // Rule 1
            labelled(
                "RULE 1 — Each value has an owner",
                "Every value in Rust has exactly one owner. For a variable, that owner is the binding. For \
                 a value living inside a struct, it is the struct. There is no anonymous pool of values \
                 waiting for someone to claim them.",
            ),
            code(
                "rust",
                "fn main() {\n    let s = String::from(\"hello\");   // s owns this String\n    let n = 42;                   // n owns this i32\n    let v = vec![1, 2, 3];         // v owns this Vec, and through it the heap\n\n    println!(\"{s} {n} {v:?}\");\n}",
            ),
            text_block(
                "Notice that ownership is not limited to heap data. `n` owns an `i32` that lives in four \
                 bytes of stack and requires no cleanup whatsoever — and it is still an owned value taking \
                 part in the ownership model. This is worth holding onto, because \"ownership\" is often \
                 mistaken for a euphemism for \"who frees the memory\". It is not. Ownership is the \
                 language's general model for **responsibility and validity**, and memory cleanup is one \
                 of the things that falls out of it.",
            ),
            callout(
                "Rule 1, violated",
                "You cannot write the violation, which is the point. To have two owners of one value you \
                 would need two bindings pointing at one allocation with no rule about which of them frees \
                 it. Rust simply has no syntax for that, so there is no version to get wrong.",
            ),

            // Rule 2
            labelled(
                "RULE 2 — Only one owner at a time",
                "There can be exactly one owner. When ownership moves, the previous owner is no longer \
                 allowed to use the value. Not 'warned about' — refused.",
            ),
            broken(
                "fn main() {\n    let s1 = String::from(\"hello\");\n    let s2 = s1;    // s2 is now the owner\n\n    println!(\"{s1}\");   // error[E0382]: borrow of moved value: `s1`\n    println!(\"{s2}\");\n}",
            ),
            broken_text(
                "error[E0382]: borrow of moved value: `s1`\n --> src/main.rs:4:19\n  |\n2 |     let s1 = String::from(\"hello\");\n  |         -- move occurs because `s1` has type `String`,\n  |            which does not implement the `Copy` trait\n3 |     let s2 = s1;\n  |              -- value moved here\n4 |     println!(\"{s1}\");\n  |                ^^ value borrowed here after move",
            ),
            fixed(
                "fn main() {\n    let s1 = String::from(\"hello\");\n    let s2 = s1.clone();   // a second, independent value\n\n    println!(\"{s1}\");    // fine: s1 still owns its own copy\n    println!(\"{s2}\");\n}",
            ),
            text_block(
                "Read the compiler's annotation again, because it is unusually generous here: *move \
                 occurs because `s1` has type `String`, which does not implement the `Copy` trait*. It just \
                 told you which rule decided the outcome and exactly what would change it. Compare it to \
                 the messages you get from a C compiler for a use-after-free, which typically says nothing \
                 at all. This is what compile-time reasoning buys you.",
            ),

            // Rule 3
            labelled(
                "RULE 3 — Out of scope means dropped",
                "When the owner goes out of scope, the value is dropped. Not maybe — deterministically, at \
                 that exact point in the program, whether or not you ever asked.",
            ),
            code(
                "rust",
                "fn main() {\n    let a = String::from(\"first\");\n    let b = String::from(\"second\");\n    println!(\"{a} {b}\");\n}   // b is dropped here, then a — reverse order\n// nothing to call, nothing to free, nothing to check at runtime",
            ),
            diagram(
                "why one owner, and not two",
                "  ALLOWED — one owner, freed exactly once\n\n      s2 ──┐\n           ├──>  \"hello\"  ──> freed once when s2 dies\n      s1 ──┘  (moved, so not an owner)\n\n  NOT ALLOWED — two owners\n\n      s1 ──┐\n           ├──>  \"hello\"  ──> who frees it? both? neither?\n      s2 ──┘\n\n  the second version has no good answer,\n  so Rust refuses to let you write it.",
            ),
            callout(
                "Why rule 2 exists at all",
                "The double-free diagram above is the whole argument. If `s1` and `s2` both owned the same \
                 heap buffer, then at end of scope the allocator would be told to release it twice. The \
                 first release succeeds. The second one either crashes, or — worse, and far more often — \
                 succeeds in freeing memory that has since been handed to something else, and now two \
                 unrelated objects believe they own the same bytes. Rule 2 makes that state unrepresentable \
                 rather than merely unlikely.",
            ),
            checklist(
                "The three rules, one more time",
                &[
                    "Each value in Rust has an owner.",
                    "There can only be one owner at a time.",
                    "When the owner goes out of scope, the value is dropped.",
                ],
            ),
            callout(
                "Practice — Apply all three at once",
                "Track ownership through this, naming who owns the String at each step and which variables \
                 are still usable: (a) `let s = String::from(\"x\");` (b) `let t = s;` (c) \
                 `let u = t;` (d) end of scope. Then do the same for: `let s = String::from(\"x\"); let \
                 t = s.clone();` followed by end of scope — and be precise about how many times the bytes \
                 are freed and by whom.",
            ),
            callout(
                "Mini challenge — Break a rule on purpose",
                "Write three tiny programs, each of which violates exactly one of the three rules, and \
                 record the compiler's complaint for each. One will not be expressible — find out which, \
                 and say why that rule is stronger than the other two. Then find a *fourth* thing Rust \
                 refuses that is not in this list, and work out which rule it comes from.",
            ),
        ],
    )
}
