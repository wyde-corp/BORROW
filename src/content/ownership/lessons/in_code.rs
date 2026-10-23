//! Module 05 — Ownership, lessons 12–18.
//!
//! Ownership in the situations you actually meet it: assignment, function
//! calls, return values, scope, `drop`, shadowing, and collections.

use crate::content::types::{
    broken, broken_text, callout, checklist, code, diagram, fixed, labelled, table, text_block,
    SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        ownership_and_assignment(),
        ownership_and_functions(),
        ownership_and_return_values(),
        ownership_and_scope(),
        dropping_values(),
        shadowing_vs_moving(),
        ownership_with_collections(),
    ]
}

fn ownership_and_assignment() -> SeedLesson {
    SeedLesson::new(
        "Ownership and Assignment",
        vec![
            labelled(
                "THE IDEA",
                "Assignment is not \"put the value in another variable\". It is \"hand the value over, or \
                 duplicate it, depending on the type\" — and if the variable already owned something, \
                 reassigning drops that something first.",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "There are two separate questions hiding inside the word *assignment*, and beginners run \
                 them together. The first is what happens to the *new* value: does it copy, move, or \
                 clone? The second is what happens to the *old* value that the variable was already \
                 holding. The first is covered by the previous lesson. The second is the one that trips \
                 people up, because it is invisible — the old value is cleaned up quietly, in the middle \
                 of a line that appears to be doing something else.",
            ),
            code(
                "rust",
                "fn main() {\n    let mut s = String::from(\"hello\");\n    println!(\"{s}\");            // s owns \"hello\"\n\n    s = String::from(\"goodbye\");\n    //  └─ \"goodbye\" is created\n    //     \"hello\" is dropped, right here, because s used to own it\n    //     s now owns \"goodbye\"\n\n    println!(\"{s}\");\n}",
            ),
            diagram(
                "reassignment, step by step",
                "  after line 1        after line 4\n\n  ┌──────┐             ┌──────┐\n  │ s    │──> hello    │ s    │──> goodbye\n  └──────┘             └──────┘\n                         │\n                         └──> hello is freed. s no longer owned it.\n\n  the old buffer is released the moment ownership changes hands,\n  not when the variable is reassigned and not at end of scope.",
            ),
            text_block(
                "This is worth pausing on, because it is a genuine difference from most languages. In \
                 JavaScript, assigning over a string just drops the old reference and waits for a \
                 collector to notice. In Rust there is no noticing, no waiting, and no collector. The \
                 old value is released at the exact point where the variable stopped owning it, and the \
                 compiler already knows where that point is.",
            ),
            callout(
                "The rule that makes this safe",
                "A variable owns at most one value at a time. That single constraint is what makes \
                 reassignment simple: the new owner is the same variable, so there is never a moment with \
                 two owners or zero. If Rust allowed two owners, reassignment would have to either \
                 release the old value while the new variable still pointed at it, or leave a dangling \
                 pointer. One-owner-at-a-time removes the question.",
            ),
            code(
                "rust",
                "fn main() {\n    let x = 5;\n    let mut y = x;      // x copied; both live\n    y = 10;             // fine — y owned a number, nothing to drop\n\n    // y = 20;          // would not compile: y is not `mut`\n}",
            ),
            callout(
                "Why `mut` is required here even though nothing is being mutated",
                "`let mut s` is not about whether the *contents* change. It is about whether the \
                 *binding's referent changes*. Assigning `s = String::from(\"goodbye\")` is changing which \
                 value `s` owns, and that is a change to the binding. If you never reassign, you never \
                 need `mut` — which is why so much Rust that looks immutable is not marked as such. The \
                 compiler is asking about rebinding, not about editing bytes.",
            ),
            callout(
                "Practice — Count the drops",
                "For each, count how many times memory is released and at which line: (a) `let mut s = \
                 String::from(\"a\"); s = String::from(\"b\");` (b) the same plus a trailing \
                 `s = String::from(\"c\");` (c) `let t = s;` (d) `let mut v = vec![1]; v.push(2);` — \
                 does `push` drop anything, and why not? Item (d) is the interesting one; `push` changes \
                 the contents without changing what `v` owns.",
            ),
            callout(
                "Mini challenge — Predict the free order",
                "Write this: (a) `let mut a = String::from(\"1\"); let mut b = String::from(\"2\"); a = \
                 String::from(\"3\"); b = String::from(\"4\");` and then predict, in order, every moment a \
                 buffer is freed. Write the line numbers. Then run it with a type that prints on drop, or \
                 reason through the `Drop` implementation in lesson 16, and check. Getting the *order* \
                 right is the actual skill here.",
            ),
        ],
    )
}

fn ownership_and_functions() -> SeedLesson {
    SeedLesson::new(
        "Ownership and Functions",
        vec![
            labelled(
                "THE IDEA",
                "Parameters are just bindings. A `String` parameter takes ownership; a `i32` parameter \
                 takes a copy. After the call, the first leaves you with nothing and the second leaves \
                 you with everything.",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "This is where module 4 and module 5 collide, and the collision is instructive. In module \
                 4 you learned that `greet(name: &str)` borrows a string slice and hands back nothing. \
                 Now look at `consume(s: String)`. The parameter is declared as `String`, not `&String` — \
                 and that one character is the entire difference between a function that borrows and a \
                 function that takes the keys.",
            ),
            broken(
                "fn consume(s: String) {\n    println!(\"{s}\");\n}\n\nfn main() {\n    let s = String::from(\"hello\");\n    consume(s);          // s moves into the function\n    println!(\"{s}\");        // error[E0382] — you no longer own it\n}",
            ),
            broken_text(
                "error[E0382]: borrow of moved value: `s`\n  --> src/main.rs:8:14\n   |\n6  |     consume(s);\n   |              -- value moved here\n7  |     println!(\"{s}\");\n   |              ^^ value borrowed here after move",
            ),
            diagram(
                "the journey of a String through a call",
                "  main                         consume\n  ─────                         ───────\n  s owns \"hello\"\n       │\n       │  consume(s)\n       ▼\n                              parameter s now owns \"hello\"\n                                     │\n                                     │  println! borrows it — fine\n                                     │\n                                  function returns\n                                     │\n                                     ▼\n  nothing. main's s is empty-handed.\n  and \"hello\" was dropped at the closing brace of consume.",
            ),
            code(
                "rust",
                "fn use_number(x: i32) {\n    println!(\"{x}\");\n}\n\nfn main() {\n    let x = 5;\n    use_number(x);     // copied — i32 implements Copy\n    println!(\"{x}\");    // still fine\n}",
            ),
            text_block(
                "Two functions, two parameters, one takes your `String` and one takes your `i32` — and \
                 the difference in what you can do afterwards is total. Not because of the function body, \
                 which is identical in shape, but because of the *type in the signature*. Once you see \
                 ownership as something the signature decides, function signatures stop being boilerplate \
                 and start being the main tool you have.",
            ),
            callout(
                "The rule for reading any signature",
                "`T` in a parameter position means *takes ownership*. `&T` means *borrows*. `&mut T` means \
                 *borrows, and may modify*. That is the whole decoding, and it is true for every function \
                 you will read in Rust. Module 6 is devoted to the two borrowing forms; for now, the \
                 discipline of noticing the difference is the thing to build.",
            ),
            callout(
                "And the two escapes, in order of preference",
                "You have already met both. `clone()` when you genuinely want two independent owners: the \
                 caller's copy stays alive, at the cost of a duplicate allocation. And returning the value \
                 — which is the subject of the next lesson, and is almost always better than cloning, \
                 because it costs nothing.",
            ),
            callout(
                "Practice — Read the signatures",
                "Four signatures. For each, say whether the caller keeps ownership, and what the function \
                 may do to the value: (a) `fn f(s: String)` (b) `fn f(s: &String)` (c) `fn f(s: &mut \
                 String)` (d) `fn f(s: u32) -> String`. Then write a five-line `main` for each that would \
                 compile *and* use the variable afterwards if possible — and say which ones make that \
                 impossible.",
            ),
            callout(
                "Mini challenge — Fix it three ways",
                "Take the `consume` program at the top of this lesson. Make it compile three different \
                 ways: with `clone()`, with a reference, and by changing what the function returns. For \
                 each, write one sentence about the cost you are paying — a duplicate buffer, a borrow, \
                 or an ownership round trip. Then say which you would choose for a function that only \
                 needs to read the string, and defend it.",
            ),
        ],
    )
}

fn ownership_and_return_values() -> SeedLesson {
    SeedLesson::new(
        "Ownership and Return Values",
        vec![
            labelled(
                "THE IDEA",
                "Ownership moves out of functions exactly as it moves into them. A returned value is not \
                 a copy of what the function had — it *is* what the function had, handed back.",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "A return is a move in the opposite direction. When a function returns a `String` it had \
                 ownership of, that value does not get duplicated on the way out; the caller simply \
                 becomes the owner. This is the mirror image of the call, and once you see it as a \
                 mirror, the awkward part of Rust function design — a function that takes a value and \
                 gives it straight back — stops feeling like a wart and starts feeling like a free \
                 round trip.",
            ),
            code(
                "rust",
                "fn create_string() -> String {\n    String::from(\"hello\")   // created here, moved out here\n}\n\nfn main() {\n    let s = create_string();   // main owns it now\n    println!(\"{s}\");\n}",
            ),
            code(
                "rust",
                "fn take_and_return(s: String) -> String {\n    s    // moved in, moved straight back out. nothing was copied.\n}\n\nfn main() {\n    let s = String::from(\"hello\");\n    let s = take_and_return(s);   // s is fine after — it just owns it again\n    println!(\"{s}\");\n}",
            ),
            diagram(
                "ownership going out",
                "  create_string()\n        │\n        │  owns \"hello\"\n        ▼\n  return value  ─────►  main owns \"hello\"\n\n  there is no intermediate copy.\n  the value that existed inside the function\n  is the value the caller now holds.\n  one buffer, one owner, a handover.",
            ),
            text_block(
                "The second program is the one that reframes things. It looks like a function that does \
                 nothing — take a thing, give it straight back. It is not nothing. It is a documented, \
                 named, reusable *ownership round trip*, and it costs three words on the stack. Later, \
                 when you need a value to live slightly longer than the function that made it, that \
                 function is exactly what you write, and it is genuinely free.",
            ),
            code(
                "rust",
                "fn make() -> String {\n    String::from(\"x\")\n}\n\nfn main() {\n    let a = make();\n    let b = make();      // fine — two calls, two separate values\n\n    println!(\"{a} {b}\");\n}",
            ),
            text_block(
                "It is natural to expect something like `let s = make(); let t = s;` to work, or to be \
                 surprised that `make()` gives you a value you can only hold once. Neither surprise is \
                 real. `make()` runs fresh each time and produces a brand new `String` each time, so two \
                 calls mean two values, and there is no shared state to run out of. A function that \
                 *returns* a value never takes anything away from you — it is the opposite of a \
                 function that *consumes* one.",
            ),
            table(
                &["Situation", "What happens"],
                &[
                    &["Function returns a value it created", "Ownership transfers to the caller. Free."],
                    &["Function returns a parameter it received", "Ownership goes back to the caller. Free."],
                    &["Function creates a value and drops it, returning nothing", "The value is cleaned up at the closing brace, as always."],
                    &["Function is called twice", "Two separate values. Each call gets its own."],
                ],
            ),
            callout(
                "Practice — Trace the round trip",
                "Write (a) a function that takes a `String` and returns it with one character appended, \
                 (b) a function that takes a `Vec<i32>` and returns it sorted, (c) a function that \
                 creates a `String` from a `&str` argument. For each, state who owns the value at every \
                 line — before the call, inside, and after. None of them need `clone()`. Prove it by \
                 running them.",
            ),
            callout(
                "Mini challenge — Catch the wasteful version",
                "Someone writes `fn make_greeting(name: String) -> String { let owned = name.clone(); \
                 format!(\"Hello, {owned}\") }`. Rewrite it so that no clone happens, and explain in one \
                 sentence why the original needed none. Then find a case where a clone *is* the right \
                 answer and write that as well, so you have both examples side by side.",
            ),
        ],
    )
}

fn ownership_and_scope() -> SeedLesson {
    SeedLesson::new(
        "Ownership and Scope",
        vec![
            labelled(
                "THE IDEA",
                "Nested scopes are a scheduling tool. Values drop in reverse order of creation, innermost \
                 first, and a block is a way of saying *this thing is finished with now* rather than at \
                 some vague point later.",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Most languages treat a block as invisible — a place to put a temporary so the name does \
                 not clutter the function. Here a block is a scope with a definite end, and definite ends \
                 are meaningful. Entering a block can bring values into existence; leaving it guarantees \
                 they are cleaned up. That makes braces something you can use deliberately, rather than \
                 something the formatter uses.",
            ),
            code(
                "rust",
                "fn main() {\n    let outer = String::from(\"outer\");\n\n    {\n        let inner = String::from(\"inner\");\n        println!(\"{outer} {inner}\");   // both alive\n    }                                  // inner is dropped HERE\n\n    println!(\"{outer}\");              // outer is still fine\n}                                      // outer is dropped HERE",
            ),
            diagram(
                "drop order is the reverse of creation",
                "  fn main() {\n      let outer = ...        ← outer created\n      {\n        let inner = ...      ← inner created\n                            ← inner's scope ends, inner dropped\n      }\n                          ← outer's scope ends, outer dropped\n  }\n\n  created: outer, inner\n  dropped: inner, outer\n\n  the reverse. always. this is not a coincidence,\n  it is the stack discipline again — the most\n  recent thing is the first to be finished.",
            ),
            text_block(
                "The compiler guarantees that reverse order. It is not a convention you can get wrong, \
                 and it holds even when values are created inside conditionals or loops. That guarantee is \
                 what makes it reasonable to write a type whose `drop` does something visible.",
            ),
            code(
                "rust",
                "struct Noisy(&'static str);\n\nimpl Drop for Noisy {\n    fn drop(&mut self) {\n        println!(\"dropped {}\", self.0);\n    }\n}\n\nfn main() {\n    let _a = Noisy(\"a\");\n    let _b = Noisy(\"b\");\n    println!(\"end of main\");\n}\n\n// prints:\n//   end of main\n//   dropped b\n//   dropped a",
            ),
            callout(
                "A scope as a lifetime limiter",
                "When a value should stop existing before the end of a function, put it in a block. You \
                 do not need `Box::leak`, you do not need a closure, and you do not need to invent a \
                 lifetime. Braces. This is a real tool and it gets used in code that must hold a lock for \
                 a short span, or a value that is expensive to keep around, or a borrow you want to end \
                 early so the borrow checker will let you mutate afterwards.",
            ),
            callout(
                "Practice — Write the drop order",
                "Predict the output of each before running: (a) `let a = ...; let b = ...;` (b) \
                 `let a = ...; { let b = ...; let c = ...; }` (c) a value created in an `if` that is \
                 not taken (d) a value moved into a function — who drops it, the caller or the callee? \
                 Item (d) is the one that surprises people, and the answer is a genuinely new idea.",
            ),
            callout(
                "Mini challenge — Use a block on purpose",
                "Find a place in your own code where a value outlives the region where it is actually \
                 useful, and wrap that region in a block. Then write a comment above the block explaining \
                 what it is doing. The point is not the refactor — it is that you now have a tool, and a \
                 reason to reach for it, that you did not have yesterday.",
            ),
        ],
    )
}

fn dropping_values() -> SeedLesson {
    SeedLesson::new(
        "Dropping Values",
        vec![
            labelled(
                "THE IDEA",
                "Cleanup is automatic, deterministic, and nobody has to call it. When the owner goes out \
                 of scope the value is dropped — and if ownership moved, it is dropped by whoever ended \
                 up holding it.",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "This is the moment the whole design pays off. You have not written a single `free`, you \
                 will never write one, and yet nothing leaks. The reason is that 'when the owner goes out \
                 of scope' is a fact the compiler determined long before the program ran, and it is a \
                 fact it can act on without asking anyone. There is no unreachable cleanup code, because \
                 the cleanup is not code at all — it is the end of a scope.",
            ),
            diagram(
                "who drops a value, exactly",
                "  if the value stayed put          → its original owner drops it\n  when that owner's scope ends\n\n  if the value moved                 → the NEW owner drops it,\n                                        at the end of the NEW owner's scope\n\n  example:\n\n    let a = Noisy(\"a\");\n    take(a);          // ownership moves into take\n                        // take returns → a is dropped INSIDE take\n                        // not at the end of main",
            ),
            text_block(
                "That second bullet is worth its own exercise, because it is the piece that makes moves \
                 *safe* rather than merely efficient. Cleanup follows ownership, wherever ownership \
                 happens to be. A moved value is not a leaked value and it is not a double-dropped \
                 value — it simply gets cleaned up by its current owner, at that owner's end.",
            ),
            callout(
                "Drop is a trait, and you have met it without knowing",
                "You have been using `drop` on `String` since lesson 6. The compiler calls it for you at \
                 the end of scope. You can also call it yourself — `drop(s);` releases the value right \
                 there and makes `s` unusable from that line on. And you can implement `Drop` for your own \
                 types to run cleanup of your own, which is what the `Noisy` type in the previous lesson \
                 was doing. All three uses are the same mechanism. We are not going to write a custom \
                 `Drop` implementation in this module; you have seen enough to recognise one.",
            ),
            callout(
                "The part that still surprises people",
                "Calling `drop(s)` does not somehow destroy a variable that then lingers as a husk. After \
                 an explicit `drop(s)`, the name `s` is gone for the same reason it is gone at end of \
                 scope. Rust has no concept of an uninitialised, still-in-scope variable. If you want the \
                 name back afterwards, you reassign it — which is legal, and is how you model 'this is \
                 emptied and then refilled'.",
            ),
            checklist(
                "What you can now say about cleanup",
                &[
                    "Cleanup happens at the end of the owner's scope, not when you call anything.",
                    "Cleanup is deterministic — it is not 'at some point later'.",
                    "A moved value is dropped by its new owner, not its old one.",
                    "Values drop in reverse order of creation.",
                    "You will never write `free`, and you will not leak.",
                ],
            ),
            callout(
                "Practice — Predict the drop site",
                "For each, name the exact point the value is dropped — line number, and whether it is \
                 inside `main` or inside another function: (a) `let s = String::from(\"x\");` (b) \
                 `let s = String::from(\"x\"); take(s);` (c) `let s = give_me_a_string();` (d) \
                 `let s = String::from(\"x\"); drop(s);` (e) a value that is moved into a `Vec` — who \
                 drops it, and when? The last one is the interesting case and it has a surprising answer.",
            ),
            callout(
                "Mini challenge — Prove determinism",
                "Write a program that allocates ten large `String`s, prints a line, and lets them all go \
                 out of scope — and show, using something you can observe, that the memory came back at \
                 exactly the point you expected and not later. Then repeat it inside a loop for a \
                 million iterations and watch your program's memory stay flat. That flat line is the \
                 entire argument against needing a garbage collector.",
            ),
        ],
    )
}

fn shadowing_vs_moving() -> SeedLesson {
    SeedLesson::new(
        "Shadowing vs Moving",
        vec![
            labelled(
                "THE IDEA",
                "Shadowing creates a *new binding with the same name*. Moving creates a *new binding that \
                 takes the value*. They look identical in the source and mean different things, and \
                 confusing the two causes errors that make no sense until you can tell them apart.",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Here is the trap. These two lines look like they do the same thing, and in a sense they \
                 do — both end with a name `s` owning a `String`. But the mechanism is completely \
                 different, and so are the consequences. In the first, the original `s` is still there \
                 underneath, and the new `s` is a fresh name sitting on top of it. In the second, the \
                 original `s` was disarmed and handed its contents over. Same picture afterwards, very \
                 different history.",
            ),
            code(
                "rust",
                "fn main() {\n    // SHADOWING — a new binding, on top of the old one\n    let x = 5;\n    let x = x + 1;      // the right-hand side reads the OLD x\n    println!(\"{x}\");           // 6\n}",
            ),
            code(
                "rust",
                "fn main() {\n    // MOVING — the value transfers to a new name\n    let s = String::from(\"hello\");\n    let s = s;              // compiles! and s still works\n    println!(\"{s}\");\n}",
            ),
            text_block(
                "`let s = s;` is legal, and it is not a mistake. The right-hand `s` is evaluated first, \
                 moving the value out of the old binding; then a new binding named `s` is created and \
                 owns it. The old binding is shadowed, so you never see its disarmed state. Some people \
                 read this as a no-op, and it nearly is — but the type is still `String` and nothing was \
                 copied.",
            ),
            table(
                &["", "`let s = s;` (shadowing a move)", "`let t = s;` (plain move)"],
                &[
                    &["New name", "`s`", "`t`"],
                    &["Old binding", "Shadowed, hidden, no longer usable", "Still in scope, and disarmed"],
                    &["Number of copies", "One value, no duplication", "One value, no duplication"],
                    &["Can you still refer to the old name?", "No — it is shadowed", "Yes, and using it is an error"],
                    &["Reads as", "A rename, essentially", "A handover"],
                ],
            ),
            callout(
                "The difference you will actually feel",
                "In `let s = s;` the old `s` is shadowed, so the compiler does not bother telling you it \
                 was moved. In `let t = s;` it is still in scope, so using it later produces \
                 `error[E0382]: borrow of moved value: s` with a line number. Same operation, different \
                 diagnostics — and that is a useful tell. If you shadow, you get silence. If you move to a \
                 new name, you get told.",
            ),
            callout(
                "Shadowing is not mutation",
                "Another pair of words that get merged. `let mut x = 5; x = 6;` changes the *same* binding \
                 to a new value. `let x = 5; let x = 6;` creates a *new* binding, and the first one still \
                 exists, untouched, underneath. You can prove it: with shadowing, the old value is still \
                 droppable, and the types can differ — `let x = 5; let x = \"five\";` is legal, and it is \
                 not legal with `mut`.",
            ),
            callout(
                "Practice — Sort the pairs",
                "For each pair, say whether the second line compiles and if so what it means: (a) \
                 `let x = 5; let x = x + 1;` (b) `let x = 5; let y = x + 1; println!(\"{x}\");` \
                 (c) `let s = String::new(); let s = s;` (d) `let s = String::new(); let t = s; \
                 println!(\"{s}\");` (e) `let x = 5; let x = \"five\";` (f) `let mut x = 5; let x = \
                 \"five\";`. The last two are where the mut-versus-shadow distinction shows up, and only \
                 one of them is allowed.",
            ),
            callout(
                "Mini challenge — Find the accidental shadow",
                "Write a function where shadowing silently hides a bug: you meant to add to the original \
                 value, and instead you replaced it. Then rewrite it with distinct names and show that the \
                 compiler now catches the error. Shadowing is convenient right up until it is not, and \
                 this is the moment it stops being.",
            ),
        ],
    )
}

fn ownership_with_collections() -> SeedLesson {
    SeedLesson::new(
        "Ownership With Collections",
        vec![
            labelled(
                "THE IDEA",
                "Nothing about this is special to `String`. `Vec<T>`, and every other growable or owned \
                 type, follows the same three rules, because the rules are about ownership and not about \
                 any particular type.",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "It would be easy to come out of the last ten lessons thinking ownership is a `String` \
                 quirk with a workaround. It is not. A `Vec<T>` owns its buffer for exactly the same \
                 reason a `String` does, moves for exactly the same reason, and is dropped for exactly \
                 the same reason. If you can predict what happens to a `Vec`, you can predict what happens \
                 to any owned type you meet from here on.",
            ),
            broken(
                "fn main() {\n    let numbers = vec![1, 2, 3];\n    let other = numbers;     // the Vec moved\n\n    println!(\"{numbers:?}\");     // error[E0382]\n}",
            ),
            fixed(
                "fn main() {\n    let numbers = vec![1, 2, 3];\n    let other = numbers.clone();\n\n    println!(\"{numbers:?} {other:?}\");   // two independent vectors\n}",
            ),
            diagram(
                "a Vec is a String with more numbers",
                "STACK                        HEAP\n┌──────┐                    ┌───────────────┐\n│ ptr  │───────────────────>│  1   2   3    │\n│ len  │      3            │               │\n│ cap  │      3            └───────────────┘\n└──────┘\n\n  the same three words, the same rules.\n  a Vec<T> owns its buffer exactly as a String owns its bytes.\n  move it, clone it, or borrow it — nothing new is happening here.",
            ),
            text_block(
                "One difference is worth flagging, because it will confuse you in the collections module \
                 and it is better met now. `push` and `pop` change what a `Vec` *contains* without \
                 changing what it *owns* — the variable keeps the same buffer, or gives back a buffer it \
                 still owns. That is not a move, and no clone is involved. Ownership is about the buffer, \
                 not about the contents.",
            ),
            code(
                "rust",
                "fn total(values: Vec<i32>) -> i32 {\n    values.iter().sum()\n}\n\nfn main() {\n    let v = vec![1, 2, 3];\n    let t = total(v);        // v is moved — a Vec is not Copy\n    println!(\"{t}\");\n}",
            ),
            callout(
                "Borrowing preview, because you will hit it immediately",
                "`total(v)` above moves the vector and leaves `v` dead, even though the function only \
                 reads it. That is annoying, and it is the exact problem the next module solves. The fix \
                 is `fn total(values: &[i32]) -> i32` and calling `total(&v)`. You do not need to \
                 understand `&[i32]` yet — just notice that the friction is real, that it is friction \
                 you have now met three times, and that the next module exists to remove it. This is not \
                 a workaround to adopt. It is the problem statement.",
            ),
            callout(
                "Practice — Classify the containers",
                "For each, say whether it is `Copy`, whether assignment moves, and who frees the memory: \
                 `(1, 2)` a tuple of i32, `String`, `Vec<String>`, `&Vec<i32>`, `Option<i32>`, \
                 `Vec<Vec<u8>>`. The last one is the interesting one — a vector of vectors owns a buffer \
                 *of* buffers, so describe what a clone of it would actually copy, and how many \
                 allocations it involves.",
            ),
            callout(
                "Mini challenge — Predict the allocation count",
                "Write this and count heap allocations: `let a = vec![1,2,3]; let b = a.clone(); let c = \
                 a;` — and state which of the three variables is usable at the end. Then do the same for \
                 `let a = vec![String::from(\"x\")]; let b = a.clone();` and explain why the second clone \
                 is doing more work than the first. The answer is about how many allocations, not about \
                 how many bytes.",
            ),
        ],
    )
}
