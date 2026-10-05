//! Module 05 — Ownership, exercises for lessons 1–8.
//!
//! Foundations and the move itself. The dominant exercise type here is
//! `PredictCompile` — the learner decides whether code builds, before anything
//! runs. Getting that habit installed early is the point of the module.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // ── L1 Why Ownership Exists ──────────────────────────────────────
        SeedExercise::mcq(
            0,
            "Rust needs to know who is responsible for freeing memory. What does it do with that \
             knowledge?",
            &[
                "It hands it to a background collector that runs occasionally",
                "It decides before the program runs, and checks it at compile time",
                "It makes the programmer remember to call free",
                "It allocates everything on the stack so nothing needs freeing",
            ],
            1,
            "This is the whole design in one sentence. The decisions are made statically, which is why \
             there is no collector and no manual bookkeeping. If you only remember one thing about why \
             Rust works this way, remember this one.",
        )
        .kind(ExerciseKind::PredictOwnership)
        .with_code(
            "fn main() {\n    let a = String::from(\"hello\");\n    let b = a;\n    println!(\"{b}\");\n}",
        )
        .with_hints(&[
            "Who is responsible for the bytes when main ends?",
            "There is no collector, so the compiler has to know.",
        ]),
        SeedExercise::mcq(
            0,
            "Which of these is a memory bug that ownership is specifically designed to make impossible?",
            &[
                "Using a variable before it has been initialised",
                "Freeing the same memory twice, because two things both believed they owned it",
                "Using more memory than you intended to",
                "Reading a variable that was never assigned a value",
            ],
            1,
            "Double free. Two owners of one buffer means two calls to the allocator's release, and the \
             second one hands live memory to someone else. Making that state unrepresentable is a \
             stronger guarantee than merely detecting it at runtime, which is what a collector would do.",
        )
        .with_hints(&[
            "One of these is caught by every language with any type system at all",
            "Ask what happens when two things both think they are responsible for the same resource",
        ]),
        SeedExercise::new(
            Some(0),
            ExerciseKind::Mcq,
            "THREE APPROACHES — a language lets you `free` memory manually. Another runs a collector at \
             runtime. Rust's approach is described as 'static'. What does that word mean here?",
            &[
                "Rust compiles faster than other languages",
                "The decisions about who owns what are made and checked before the program runs",
                "Rust only uses static memory, never the heap",
                "Rust's memory usage can be calculated at compile time",
            ],
            1,
            "Static means decided at compile time rather than at runtime. It has nothing to do with how \
             fast the compiler is, and Rust absolutely does use the heap. The word describes *when the \
             memory questions get answered*, which is the entire difference in behaviour you can observe.",
        )
        .hints(&[
            "When does a garbage collector answer 'who owns this'?",
            "The answer is 'while the program runs'. What is Rust's answer?",
        ]),

        // ── L2 Stack and Heap ───────────────────────────────────────────
        SeedExercise::new(
            Some(1),
            ExerciseKind::MemoryDiagram,
            "DRAW THE MEMORY — a `String` holding \"hi\" is created. What does the stack part of the \
             picture look like, and what does the heap part look like?",
            &[
                "Stack: a single pointer. Heap: the two bytes h and i",
                "Stack: pointer, length, capacity. Heap: the two bytes h and i",
                "Stack: the two bytes h and i. Heap: a pointer to the string",
                "Stack: nothing. Heap: pointer, length, capacity, and the bytes",
            ],
            1,
            "The `String` *value* is three words on the stack — pointer, length, capacity. The bytes it \
             points at are on the heap. This split is the single most useful picture in the module: a move \
             transfers the three stack words and leaves the heap bytes exactly where they were.",
        )
        .code("let s = String::from(\"hi\");")
        .hints(&[
            "A String is a struct. What three fields does it have?",
            "The struct itself has a fixed size, so it can live on the stack.",
            "The contents have a length nobody knew until runtime, so they cannot.",
        ]),
        SeedExercise::mcq(
            1,
            "Why is the stack described as 'last in, first out'?",
            &[
                "Because the newest memory is always freed first",
                "Because function calls nest, and they return in the reverse order they were made",
                "Because the operating system manages it that way",
                "Because it is the fastest allocation strategy",
            ],
            1,
            "It is a consequence of function calls nesting. `main` calls `foo`, which calls `bar`; `bar` \
             must finish before `foo`, which must finish before `main`. That is a stack. All the \
             bookkeeping the runtime would otherwise do is implied by the call structure.",
        )
        .with_hints(&[
            "Think about what happens when a function returns",
            "Can a function return before the function that called it?",
        ]),
        SeedExercise::new(
            Some(1),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — and more importantly, why?",
            &[
                "Yes, because tuples are a simple type",
                "No, because tuples always live on the heap",
                "Yes, because its size is known at compile time",
                "No, because it contains a f64",
            ],
            2,
            "A tuple of numbers has a fixed size determined entirely by its fields, so it is known at \
             compile time and needs no heap allocation. `Copy` follows: duplicating it is trivial. Note \
             that this is the answer that a 'simple versus complex type' rule gets wrong and a 'known \
             size' rule gets right.",
        )
        .code("fn main() {\n    let t = (1, 2.0, true);\n    let u = t;\n    println!(\"{:?} {:?}\", t, u);\n}")
        .hints(&[
            "Is there anything in there whose size we cannot know until the program runs?",
            "If there isn't, can the compiler just duplicate it?",
        ]),
        SeedExercise::mcq(
            1,
            "Which of these is the honest way to think about the stack/heap diagrams?",
            &[
                "They are a precise description of how every Rust program is laid out in memory",
                "They are a conceptual model that helps you reason about responsibility, not a spec",
                "They are only correct for types in the standard library",
                "They describe what happens in debug builds only",
            ],
            1,
            "Conceptual. The standard library may use different representations, and an optimiser may \
             elide allocations entirely. The diagrams are useful because the *reasoning* they support — \
             who is responsible for this value — is correct regardless. Hold the picture loosely, hold \
             the reasoning tightly.",
        )
        .with_hints(&[
            "Compilers optimise. Would an optimiser respect your diagram?",
            "The question to ask is whether the conclusion changes.",
        ]),

        // ── L3 What Is a Value? ─────────────────────────────────────────
        SeedExercise::new(
            Some(2),
            ExerciseKind::Mcq,
            "THE VARIABLE IS NOT THE VALUE — in `let s = String::from(\"hi\");`, what exactly is `s`?",
            &[
                "The String itself, stored in memory",
                "A name that currently refers to a String value",
                "A pointer to the characters h and i",
                "A copy of the characters h and i",
            ],
            1,
            "`s` is a name, and it refers to a value. The value is the three-word struct. Confusing the \
             name with the value is harmless for numbers and lethal for strings, because it leads you to \
             believe a move destroys something.",
        )
        .hints(&[
            "You can shadow a name. Can you shadow a value?",
            "If `s` were the value, what would `let t = s;` be doing to it?",
        ]),
        SeedExercise::mcq(
            2,
            "When a String moves from `s1` to `s2`, what happens to the bytes on the heap?",
            &[
                "They are copied to a new allocation",
                "They are freed, and new ones are allocated",
                "They stay exactly where they were",
                "They are moved to the stack",
            ],
            2,
            "They do not move at all. The three stack words — pointer, length, capacity — change hands, \
             and that is the entire move. If the bytes were relocated, moving a String would be a \
             catastrophically expensive operation and borrowing would be pointless.",
        )
        .kind(ExerciseKind::PredictOwnership)
        .with_code("let s1 = String::from(\"hi\");\nlet s2 = s1;")
        .with_hints(&[
            "What exactly is transferred in a move?",
            "It is small and fixed-size. That is a hint.",
        ]),
        SeedExercise::new(
            Some(2),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — and what does the answer tell you about the binding?",
            &[
                "Yes, because the name `s` is simply reused",
                "No, because you cannot assign a variable to itself",
                "Yes, and a brand new binding named `s` now owns the value",
                "No, because String does not implement Copy",
            ],
            2,
            "This compiles, and it is not a no-op. The right-hand `s` is evaluated first, moving the \
             value out of the old binding; then a fresh binding named `s` is created and owns it. The \
             old binding is shadowed, which is why the compiler does not bother reporting the move.",
        )
        .code("fn main() {\n    let s = String::from(\"hi\");\n    let s = s;\n    println!(\"{s}\");\n}")
        .hints(&[
            "Which s is on the right-hand side of the second let?",
            "Does the new binding hold the same value, or a copy?",
        ]),
        SeedExercise::mcq(
            2,
            "Two variables can absolutely refer to two separate values. What can never happen?",
            &[
                "Two variables holding equal but distinct Strings",
                "Two variables holding i32 values",
                "Two owners responsible for one heap buffer",
                "A variable holding a value that another variable also holds a copy of",
            ],
            2,
            "Multiple values are fine — that is what clone and Copy both produce. What is impossible is \
             two *owners* of one value, because that leaves the question of who frees it unanswerable. \
             Note the wording: it is ownership that is unique, not existence.",
        )
        .with_hints(&[
            "Ask which of these produces two distinct buffers",
            "Distinct is the key word, not multiple.",
        ]),

        // ── L4 Scope and Lifetime ───────────────────────────────────────
        SeedExercise::new(
            Some(3),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — the inner block prints `s`, and so does the code after it. Is that allowed?",
            &[
                "No — `s` ends at the inner closing brace",
                "Yes — the inner braces close only the inner scope; `s` belongs to `main`",
                "No — a String can only be printed once",
                "Yes, but only if `s` is marked `mut`",
            ],
            1,
            "This is the case people get wrong, so read it carefully. The inner braces create a scope \
             that ends early, but `s` was never declared *inside* it — `s` belongs to `main` and ends \
             with `main`. The inner block did not shorten `s`'s life; it just had its own, shorter one.",
        )
        .code(
            "fn main() {\n    let s = String::from(\"hello\");\n\n    {\n        println!(\"{s}\");\n    }\n\n    println!(\"{s}\");\n}",
        )
        .hints(&[
            "Where was `s` declared? Inside the inner block, or in main?",
            "Which closing brace belongs to the declaration of `s`?",
        ]),
        SeedExercise::mcq(
            3,
            "When a String's owner goes out of scope, when is the memory actually freed?",
            &[
                "At some later point, whenever a collector gets around to it",
                "At that exact point in the program, deterministically",
                "When the whole program exits",
                "Whenever something else allocates memory",
            ],
            1,
            "At that exact point, and deterministically. This is the whole payoff of the design: the \
             compiler knew the answer before the program started, so it could insert the cleanup \
             instructions there. There is no later, and no 'whenever'.",
        )
        .with_hints(&[
            "Is there a collector in Rust?",
            "If not, who does the work, and when?",
        ]),
        SeedExercise::new(
            Some(3),
            ExerciseKind::PredictOwnership,
            "PREDICT OWNERSHIP — when does the String below stop existing?",
            &[
                "When the inner block closes",
                "At the end of main",
                "When it is last printed",
                "It never gets freed",
            ],
            1,
            "The value is owned by `s`, which was declared in `main`, so it ends with `main`. The inner \
             block is a red herring — it has no owner of its own to clean up. This is the difference \
             between a value's scope and a block's scope.",
        )
        .code(
            "fn main() {\n    let s = String::from(\"hello\");\n    {\n        println!(\"{s}\");\n    }\n    println!(\"{s}\");\n}",
        )
        .hints(&[
            "Which variable owns the value?",
            "Which brace does that variable's declaration belong to?",
        ]),
        SeedExercise::mcq(
            3,
            "Does 'lifetime' in this module mean the same thing as a lifetime annotation?",
            &[
                "Yes, they are two names for the same mechanism",
                "No — here it means how long a value stays valid, which is scope; annotations are a \
                 separate system for references, taught later",
                "Yes, and you will need annotations after this lesson",
                "No — lifetime is only a term used in garbage-collected languages",
            ],
            1,
            "Related idea, different mechanism. Here 'lifetime' means nothing more precise than 'how \
             long is this value valid', and the answer is always 'until its scope ends'. The annotation \
             system — the `'a` you will meet in a few modules — is about how long a *reference* is \
             valid, and it is a much more subtle tool. If someone starts talking about annotations \
             now, they have jumped chapters.",
        )
        .with_hints(&[
            "One of these is about values, the other about references",
            "You have not met the annotation system yet. That is deliberate.",
        ]),

        // ── L5 The Three Rules ──────────────────────────────────────────
        SeedExercise::mcq(
            4,
            "The three ownership rules say each value has an owner, only one owner at a time, and \
             out-of-scope means dropped. What is the *reason* the second rule exists?",
            &[
                "To make the compiler faster",
                "To prevent two things both believing they must free the same memory",
                "To keep the code short",
                "To make the borrow checker simpler to implement",
            ],
            1,
            "Double free. If two bindings owned one buffer, both would free it at end of scope, and the \
             second free would either crash or corrupt memory that has since been handed to something \
             else. Rule 2 makes that state impossible to write rather than merely unlikely.",
        )
        .with_hints(&[
            "What happens at the end of scope, according to rule 3?",
            "Apply rule 3 to a value that has two owners.",
        ]),
        SeedExercise::new(
            Some(4),
            ExerciseKind::PredictOwnership,
            "PREDICT OWNERSHIP — trace all three lines and name the owner at the end.",
            &[
                "`s1` owns it — the moves did not really happen",
                "`s3` owns it; `s1` and `s2` are both unusable",
                "`s1` and `s2` both own it, `s3` is a copy",
                "Nobody owns it — ownership was duplicated",
            ],
            1,
            "Each `let` moved the value on: s1 to s2 to s3. The old names were not deleted, they were \
             disarmed — still in scope, still usable for nothing. Exactly one owner survives, which is \
             what makes the eventual single drop correct.",
        )
        .code(
            "let s1 = String::from(\"hello\");\nlet s2 = s1;\nlet s3 = s2;",
        )
        .hints(&[
            "Work one line at a time and ask who owns it *now*",
            "Nothing has been cloned. Where did the value go each time?",
        ]),
        SeedExercise::new(
            Some(4),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — and which rule is doing the work?",
            &[
                "Yes — String is a normal type",
                "No — rule 1, because the value would have no owner",
                "No — rule 2, because two bindings would own one value",
                "No — because String must be declared `mut`",
            ],
            2,
            "Rule 2 in its purest form. This is not expressible in safe Rust — you cannot write two \
             owners of one buffer — so the strongest version of rule 2 is the one you never have to \
             think about, because you cannot get it wrong.",
        )
        .code(
            "fn main() {\n    let s1 = String::from(\"hello\");\n    let s2 = s1;\n    let both = (s1, s2);\n    println!(\"{:?} {:?}\", s1, s2);\n}",
        )
        .hints(&[
            "s1 already moved into s2. What does the tuple try to do?",
            "Is there any way to write two owners of one String?",
        ]),
        SeedExercise::mcq(
            4,
            "Does `let x = 5;` involve ownership, given that nothing is ever freed?",
            &[
                "No — ownership only applies to heap data",
                "Yes — x is the owner of a value, it just needs no cleanup",
                "No — because `i32` is Copy",
                "Only if `x` is marked `mut`",
            ],
            1,
            "Yes, and this is the nuance most people never pick up. Ownership is Rust's general model \
             for responsibility and validity, not a synonym for 'who frees the memory'. `x` owns a \
             value, participates in scope, and is released at the closing brace — the machinery simply \
             had nothing to do.",
        )
        .kind(ExerciseKind::PredictOwnership)
        .with_code("fn main() {\n    let x = 5;\n    println!(\"{x}\");\n}")
        .with_hints(&[
            "Is `x` the owner of a value?",
            "'Ownership' and 'frees memory' are not synonyms.",
        ]),
        SeedExercise::mcq(
            4,
            "Two bindings owning one value would be a problem mainly at what moment?",
            &[
                "At assignment time, because it is confusing to read",
                "At end of scope, when both would try to free the same memory",
                "Immediately, because the compiler refuses to start",
                "At compile time, for no particular reason",
            ],
            1,
            "End of scope. Assignment and use would all work fine — the trouble only surfaces when \
             cleanup runs, which is usually long after the mistake. That is exactly why this class of \
             bug is so damaging in C and C++, where the error surfaces far from its cause.",
        )
        .with_hints(&[
            "When does rule 3 fire?",
            "Trace it forward to end of scope.",
        ]),

        // ── L6 String ───────────────────────────────────────────────────
        SeedExercise::new(
            Some(5),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — a string literal where a String was expected",
            &[
                "Yes — they print the same and both have `.len()`",
                "No — the expected type is `String` and a literal is a `&str`",
                "Yes — the compiler inserts a conversion automatically",
                "No — string literals are not allowed at all",
            ],
            1,
            "No automatic conversion. `&str` and `String` look interchangeable to a beginner and are \
             entirely different types — one borrows text that already existed, the other owns text it \
             allocated. This catches everyone once. The fix is `.to_string()`.",
        )
        .code("fn main() {\n    let s: String = \"hello\";\n    println!(\"{s}\");\n}")
        .hints(&[
            "What type is a bare string literal?",
            "Does Rust coerce between them silently?",
        ]),
        SeedExercise::mcq(
            5,
            "Why is `String` the right type to teach ownership with, rather than `i32`?",
            &[
                "Because it is the most common type in real programs",
                "Because it grows, owns heap data, and has cleanup work to do — i32 has none of those",
                "Because it is the only type that implements Clone",
                "Because it is easier to print",
            ],
            1,
            "It has every property that makes ownership necessary: dynamically sized, owns a heap \
             buffer, and needs cleanup at end of scope. `i32` has none of them, which is exactly why \
             `i32` is boring and `String` is the lesson.",
        )
        .with_hints(&[
            "List what `i32` lacks that `String` has",
            "Size known at compile time? Cleanup? Growth?",
        ]),
        SeedExercise::new(
            Some(5),
            ExerciseKind::MemoryDiagram,
            "DRAW THE MEMORY — `let owned = String::from(\"hello\");`. What is on the stack, what is on \
             the heap, and how many bytes got allocated at runtime?",
            &[
                "Stack: the 5 characters. Heap: a pointer. One allocation of 5 bytes",
                "Stack: pointer, length, capacity. Heap: the 5 characters. One allocation",
                "Stack: the String. Heap: a second String. One allocation",
                "Stack: 3 words. Heap: 5 words plus overhead. Two allocations",
            ],
            1,
            "One allocation, containing the five characters. The stack holds the three-word value that \
             points at it. The `String` struct itself is not on the heap — it is small and fixed-size, \
             and it is the *contents* that had to be allocated.",
        )
        .code("let owned = String::from(\"hello\");")
        .hints(&[
            "How many fields does a String have?",
            "One allocation. Which side of the stack/heap split is it on?",
        ]),
        SeedExercise::new(
            Some(5),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — two Strings, then a third assignment reusing the first name",
            &[
                "Yes — `c` gets a copy of `a`",
                "Yes — `c` gets `a` moved, and `a` is shadowed",
                "No — `a` is immutable so it cannot be reassigned",
                "No — a String can only be moved once in a program",
            ],
            1,
            "`let c = a;` is a new binding, so it shadows rather than reassigns, which makes it legal. \
             Note the trap in the wrong answers: shadowing and reassignment are different operations, \
             and only one of them is blocked here.",
        )
        .code(
            "fn main() {\n    let a = String::from(\"one\");\n    let b = String::from(\"two\");\n    let c = a;\n    println!(\"{b} {c}\");\n}",
        )
        .hints(&[
            "Is the third line `let c = ...` or `a = ...`?",
            "One of those is a new binding and the other is a reassignment.",
        ]),

        // ── L7 Move ─────────────────────────────────────────────────────
        SeedExercise::new(
            Some(6),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE",
            &[
                "Yes — s1 still holds a copy of the text",
                "No — s1 was moved into s2 and can no longer be used",
                "Yes — but s2 is empty afterwards",
                "No — because String cannot be assigned at all",
            ],
            1,
            "The canonical ownership error. `s1` was moved into `s2`; there is no copy, and `s1` is \
             disarmed. The compiler says `error[E0382]: borrow of moved value: s1` and annotates both \
             the move and the use.",
        )
        .code(
            "fn main() {\n    let s1 = String::from(\"hello\");\n    let s2 = s1;\n\n    println!(\"{s1}\");\n    println!(\"{s2}\");\n}",
        )
        .hints(&[
            "Did anything get copied?",
            "What is s1 pointing at after line 2?",
        ]),
        SeedExercise::new(
            Some(6),
            ExerciseKind::MemoryDiagram,
            "DRAW THE MEMORY — after `let s1 = String::from(\"hello\"); let s2 = s1;`, what do the stack \
             and the heap each look like?",
            &[
                "Stack: s1 and s2 both pointing at one heap buffer. Heap: one buffer",
                "Stack: s1 invalid, s2 pointing at one heap buffer. Heap: one buffer, unmoved",
                "Stack: s1 and s2 each with their own pointer, length, capacity. Heap: two buffers",
                "Stack: s2 with a pointer, length, capacity. Heap: one buffer. s1 does not exist",
            ],
            1,
            "One heap buffer, exactly where it always was. `s1` is marked invalid but still exists as a \
             name — it is disarmed, not deleted. Note how option three would be the clone diagram, and \
             option four over-corrects into 's1 does not exist'.",
        )
        .code("let s1 = String::from(\"hello\");\nlet s2 = s1;")
        .hints(&[
            "How many allocations happen in the whole program?",
            "Is `s1` gone, or just unable to be used?",
        ]),
        SeedExercise::mcq(
            6,
            "What is the actual cost of moving a String?",
            &[
                "Proportional to the length of the string",
                "Constant — three words on the stack change hands",
                "One heap allocation and a copy",
                "It depends on whether the string is longer than some threshold",
            ],
            1,
            "Constant. Pointer, length, capacity. The heap bytes do not move, so there is nothing \
             proportional to the length. This is the property that makes ownership affordable, and the \
             reason a move is not a deep copy in disguise.",
        )
        .with_hints(&[
            "What is a String *value*, physically?",
            "It is three words. Moving three words is O(1).",
        ]),
        SeedExercise::new(
            Some(6),
            ExerciseKind::PredictOwnership,
            "PREDICT OWNERSHIP — after these three lines, who owns the String and what happens at end \
             of scope?",
            &[
                "s1 and s2 both own it; freed twice",
                "s3 owns it; it is freed once, by s3",
                "Nobody owns it; it is leaked",
                "s1 owns it, because it was created first",
            ],
            1,
            "s3, and freed exactly once. Creation order gives no special status — the owner is whoever \
             currently holds the value, and that is s3. This is the entire model in one question.",
        )
        .code(
            "let s1 = String::from(\"hello\");\nlet s2 = s1;\nlet s3 = s2;",
        )
        .hints(&[
            "Follow the value, not the names",
            "Each line transfers it one step further along.",
        ]),
        SeedExercise::mcq(
            6,
            "Which of these best describes what happens to `s1` after a move?",
            &[
                "The variable is destroyed and its memory deleted",
                "The binding becomes unusable for that value; nothing is deleted or copied",
                "The value is moved back to s1 when s2 goes out of scope",
                "s1 becomes an empty String",
            ],
            1,
            "The binding becomes unusable. 'Your variable didn't disappear and Rust didn't delete your \
             data' is worth sitting with, because the mental image of deletion is what makes moves feel \
             magical. The bytes are untouched and s1 is merely no longer holding them.",
        )
        .with_hints(&[
            "Where are the heap bytes right now, compared to before?",
            "In safe Rust, can you even observe a moved-from variable?",
        ]),
        SeedExercise::new(
            Some(6),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — `s1.as_ptr()` after the move",
            &[
                "Yes — as_ptr only reads metadata",
                "No — reading metadata is still a use of a moved value",
                "Yes — because as_ptr takes a reference and references do not move",
                "No — because as_ptr requires `mut`",
            ],
            1,
            "No. `as_ptr(&self)` borrows, and borrowing a moved value is exactly the error. The general \
             rule: there is no operation at all you can perform on a moved-out variable, not even one \
             that would be harmless. That is deliberate.",
        )
        .code(
            "fn main() {\n    let s1 = String::from(\"hello\");\n    let s2 = s1;\n    println!(\"{}\", s1.as_ptr());\n}",
        )
        .hints(&[
            "What does as_ptr take?",
            "Taking a reference is still a use.",
        ]),

        // ── L8 Why Rust Moves ───────────────────────────────────────────
        SeedExercise::mcq(
            7,
            "Rust moves rather than deep-copying on assignment. Which of these is the *performance* \
             reason?",
            &[
                "Copying a String is slow, and a one-word line should not cost a megabyte",
                "Deep copies would use more total memory than the machine has",
                "Copies take longer than moves, and Rust cares about compile times",
                "The garbage collector would have to work harder",
            ],
            0,
            "It is about the cost being *invisible at the call site*. `let b = a;` looks free, and if it \
             silently copied a million bytes the cost would be real and completely hidden. Moves keep \
             assignment O(1) and predictable.",
        )
        .with_hints(&[
            "A Vec of a million i32s assigned to a new name",
            "How much work would a copy do, and would you know from reading the line?",
        ]),
        SeedExercise::mcq(
            7,
            "If Rust deep-copied on every assignment instead, what would break?",
            &[
                "Nothing — it would just be slower",
                "Double free would be solved, but the cost would be unbounded and invisible",
                "The borrow checker would not be needed",
                "Programs would not compile any faster",
            ],
            1,
            "Double free *would* be solved — copying is a legitimate way to avoid it. But you would \
             trade a memory-safety bug for a performance cliff that nobody can see in the source, and \
             that is a worse bargain. Rust chose to make duplication explicit instead.",
        )
        .with_hints(&[
            "Consider what 'two independent copies' does to the ownership question",
            "And what it does to the cost of every assignment.",
        ]),
        SeedExercise::new(
            Some(7),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — and is this assignment O(1) or O(n)?",
            &[
                "Yes, and it is O(1) — three words on the stack",
                "Yes, and it is O(n) — a million elements are copied",
                "No, because a Vec of a million i32s is too large",
                "No, because Vec does not implement Copy",
            ],
            0,
            "Yes, and O(1). Assignment moves the Vec's three-word header; the million elements sit on \
             the heap untouched. Compare with `.clone()`, which *is* O(n) and would allocate a second \
             million-element buffer.",
        )
        .code(
            "fn main() {\n    let data = vec![0u8; 1_000_000];\n    let copy = data;\n    println!(\"{}\", copy.len());\n}",
        )
        .hints(&[
            "How many words is a Vec's value?",
            "How many of those million bytes are touched by this line?",
        ]),
        SeedExercise::mcq(
            7,
            "The most common practical reason you will notice deep-copying being expensive is a large \
             value passed into a loop. What is the actual problem there?",
            &[
                "The value is copied on every iteration and you never wrote code that said so",
                "The compiler refuses to optimise it",
                "The loop runs too many iterations",
                "The value cannot fit in cache",
            ],
            0,
            "The invisibility. The cost is proportional to the data, repeated per iteration, and \
             nothing in the source suggests it. This is exactly the failure mode moves exist to prevent, \
             and it is why the fix is structural rather than a micro-optimisation.",
        )
        .with_hints(&[
            "Read `for x in big_vec { work(x); }` and ask what work receives",
            "What does a `String` parameter mean for a caller in a loop?",
        ]),
    ]
}
