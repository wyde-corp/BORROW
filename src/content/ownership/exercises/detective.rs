//! Module 05 — Ownership, exercises for lessons 19–23.
//!
//! Mental models, myths, compiler errors, deliberate prediction practice, and
//! the module recap. These lean on `PredictCompile` and `CompilerChallenge`
//! because by now the learner should be reading errors rather than reacting to
//! them.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // ── L18 Mental Models ───────────────────────────────────────────
        SeedExercise::mcq(
            18,
            "Which mental model is most useful when you have lost track of a value inside a function?",
            &[
                "The name tag — it shows you who is holding it",
                "The one responsible person — it shows you who has to clean it up",
                "Ownership travels — it shows you the current holder and how it got there",
                "Stack and heap — it shows you where the bytes are",
            ],
            2,
            "When a value has passed through several functions, the useful question is *who holds it \
             right now* — and that is what the 'ownership travels' model answers. The name tag is for \
             understanding a single move, and 'one responsible person' is for understanding why the \
             rules exist at all.",
        )
        .with_hints(&[
            "Diagnose first: is your problem about the current state, or about the mechanism?",
            "One model answers 'where is it now', another answers 'why does this rule exist'.",
        ]),
        SeedExercise::mcq(
            18,
            "The 'name tag' model says a move transfers a tag, not the value. Why is that distinction \
             worth keeping?",
            &[
                "Because the value is physically too large to move",
                "Because it explains why a move is cheap and why the heap bytes never move",
                "Because Rust does allow copying in that case",
                "Because it explains why String is not Copy",
            ],
            1,
            "It stops you imagining data flying around, which is what makes moves feel mysterious. \
             The value stays put; three words on the stack change hands; the operation is O(1) *because* \
             nothing is relocated. If moves relocated data, `String` would be unusable and borrowing \
             pointless.",
        )
        .with_hints(&[
            "What is a String *value*, physically?",
            "How many bytes does moving it touch?",
        ]),
        SeedExercise::new(
            Some(18),
            ExerciseKind::PredictOwnership,
            "PREDICT OWNERSHIP — the state that ownership rules forbid entirely",
            &[
                "Two bindings owning one value",
                "One binding owning two values",
                "No binding, with a value still on the heap",
                "A binding owning a value that has already been dropped",
            ],
            0,
            "Two owners of one value is the state the whole system exists to prevent, and Rust will not \
             let you write it. Note that the other three are all perfectly expressible: a binding can \
             own several values, a heap buffer can outlive every binding via `mem::forget`, and drop \
             order is fully determined. The design forbids exactly one thing.",
        )
        .code("// which of these can you actually write?")
        .hints(&[
            "Which single state does rule 2 forbid?",
            "The others are all legal Rust — try them.",
        ]),
        SeedExercise::new(
            Some(18),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — and what does 'no' prove?",
            &[
                "Yes — String is not Copy so it should move",
                "No — this is the one state rule 2 forbids: two owners of one value",
                "No — because String cannot be compared",
                "Yes — because a String can only have one owner anyway",
            ],
            1,
            "This is the state the whole design exists to prevent, and you cannot write it. The four \
             correct answers are not a coincidence: they are the evidence that Rust's ownership rules \
             forbid a single, specific thing rather than being generally restrictive.",
        )
        .code(
            "fn main() {\n    let s1 = String::from(\"hi\");\n    let s2 = s1;\n    let (a, b) = (s1, s2);\n    println!(\"{a} {b}\");\n}",
        )
        .hints(&[
            "What does `s1` hold at the third line?",
            "Rule 2 says only one owner at a time. Which state is this?",
        ]),

        // ── L19 Myths ───────────────────────────────────────────────────
        SeedExercise::mcq(
            19,
            "Myth-busting: 'Rust never copies.' Why is this wrong?",
            &[
                "Because it copies Strings, just very efficiently",
                "Because Copy types are copied on every assignment and every function call",
                "Because clone is just a copy in disguise",
                "Because the garbage collector copies at the end",
            ],
            1,
            "Rust copies constantly — every `i32`, every `bool`, every `&T` passed to a function. The \
             rule is that copying happens where copying is free and transferring happens where it is \
             not. 'Never copies' and 'always copies' are the same mistake: both assume the behaviour is \
             uniform rather than type-dependent.",
        )
        .with_hints(&[
            "What happens in `let y = x;` when `x` is an `i32`?",
            "Is the behaviour the same for every type?",
        ]),
        SeedExercise::mcq(
            19,
            "Myth-busting: 'a move means the data physically moved.' What actually happens?",
            &[
                "The heap allocation is copied to a new address",
                "The stack-resident representation transfers and the heap allocation stays put",
                "The data is freed and reallocated at the new location",
                "The value is duplicated and one copy discarded",
            ],
            1,
            "The heap allocation stays exactly where it is. What moves is the String's stack-resident \
             three-word representation — pointer, length, capacity. Moves are cheap *because* no data \
             is relocated, and this distinction is the reason borrowing is worth learning next.",
        )
        .with_hints(&[
            "What is a String value, physically?",
            "Count the bytes a move touches.",
        ]),
        SeedExercise::mcq(
            19,
            "Myth-busting: 'only heap values have ownership.' What is the accurate version?",
            &[
                "Ownership is a language-level model for responsibility and validity; memory cleanup is \
                 one consequence of it, not its definition",
                "Ownership applies to everything, but only matters for the heap",
                "Stack values are owned by the hardware",
                "Ownership only applies to String and Vec",
            ],
            0,
            "Ownership is a general model for responsibility and validity. `let x = 5;` has an owner, \
             participates in scope, and is released at the closing brace — the machinery just had \
             nothing to do. Reducing ownership to 'who frees the memory' makes every subsequent idea \
             harder to place.",
        )
        .with_hints(&[
            "Does `let x = 5;` have an owner?",
            "It needs no cleanup. Does that make it un-owned?",
        ]),
        SeedExercise::mcq(
            19,
            "Myth-busting: 'Rust has garbage collection.' Which is true?",
            &[
                "It has one, but it only runs at end of scope",
                "It has no collector — cleanup is compiled in at scope ends, with no tracing and no pause",
                "It has a collector for heap data and none for stack data",
                "It has a collector that can be disabled with a flag",
            ],
            1,
            "There is no collector and no pause, because there is nothing running that could pause. The \
             compiler inserts cleanup instructions at scope ends — the work is done ahead of time. If \
             you have never observed a GC pause in a Rust program, that is not luck.",
        )
        .with_hints(&[
            "What is doing the cleanup, and when did it decide to?",
            "A collector works at runtime. When does Rust's cleanup happen?",
        ]),
        SeedExercise::new(
            Some(19),
            ExerciseKind::PredictOwnership,
            "PREDICT OWNERSHIP — a myth this module warns about, as a real program",
            &[
                "They share one buffer, so exactly one is dropped",
                "Two separate buffers, each dropped by its own owner",
                "The clone is never dropped, because it is a copy",
                "Neither is dropped, because clones are exempt",
            ],
            1,
            "Two buffers, two owners, two cleanups — which is precisely why clones are safe and moved \
             values are not. The double-free diagram is two owners pointing at *one* buffer. Clone gives \
             you two of everything, not two handles on one thing.",
        )
        .code("let a = String::from(\"x\");\nlet b = a.clone();\nprintln!(\"{a} {b}\");")
        .hints(&[
            "How many allocations happened?",
            "Each owner is responsible for its own.",
        ]),

        // ── L20 Compiler Errors ─────────────────────────────────────────
        SeedExercise::new(
            Some(20),
            ExerciseKind::CompilerChallenge,
            "COMPILER DETECTIVE — read this error and say which rule it is enforcing",
            &[
                "Rule 1 — a value must have an owner",
                "Rule 2 — one owner at a time; `s` was moved",
                "Rule 3 — out of scope means dropped",
                "None — this is about mutability",
            ],
            1,
            "Rule 2, and the compiler says so in the annotation: *move occurs because `s` has type \
             `String`, which does not implement the `Copy` trait*. The move happened on line 3; the \
             illegal use is on line 4. Read the annotations rather than the error text and the rule \
             tells itself to you.",
        )
        .code(
            "error[E0382]: borrow of moved value: `s`\n --> src/main.rs:4:19\n  |\n2 |     let s = String::from(\"hello\");\n  |         -- move occurs because `s` has type `String`,\n  |            which does not implement the `Copy` trait\n3 |     let s2 = s;\n  |              -- value moved here\n4 |     println!(\"{s}\");\n  |                ^^ value borrowed here after move",
        )
        .hints(&[
            "Ignore the first line. Start at the `--` annotation.",
            "It names the type and the trait. What is the trait telling you?",
        ]),
        SeedExercise::new(
            Some(20),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — note that this is a different error code",
            &[
                "Yes — you can give a moved variable a new value",
                "No, and it is E0384: assigning to a value after it was moved",
                "No, and it is E0382 like every other move error",
                "Yes, but only because the new value is a String",
            ],
            1,
            "This is **E0384**, *assigning to a value after it was moved* — a different code from the \
             familiar E0382. You cannot resurrect a disarmed binding. The fix is to shadow: `let s = \
             String::from(\"goodbye\");` creates a new binding that legally takes a value, where `s = ...` \
             tries to reuse one that has nothing left.",
        )
        .code(
            "fn main() {\n    let s = String::from(\"hello\");\n    let moved = s;\n\n    s = String::from(\"goodbye\");\n\n    println!(\"{s} {moved}\");\n}",
        )
        .hints(&[
            "Reassigning is not the same as rebinding",
            "Which spelling creates a new binding?",
        ]),
        SeedExercise::new(
            Some(20),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — the fix for the previous error",
            &[
                "No — the original `s` is still moved",
                "Yes — shadowing creates a fresh binding, which is legal",
                "Yes — because String is now allowed to be copied",
                "No — you must clone instead",
            ],
            1,
            "Shadowing sidesteps the problem entirely: the moved binding is simply hidden, and a new \
             one legally takes a fresh value. This is why E0384's fix is almost always a `let` rather \
             than an `=` — a distinction worth having in your fingers, not just your head.",
        )
        .code(
            "fn main() {\n    let s = String::from(\"hello\");\n    let moved = s;\n\n    let s = String::from(\"goodbye\");\n\n    println!(\"{s} {moved}\");\n}",
        )
        .hints(&[
            "Compare the two spellings carefully",
            "One hides the old binding; one tries to reuse it.",
        ]),
        SeedExercise::new(
            Some(20),
            ExerciseKind::CompilerChallenge,
            "COMPILER DETECTIVE — what exactly is the compiler telling you?",
            &[
                "That `s` is uninitialised",
                "That `s` is out of scope here",
                "That `s` was moved, and the reason is that String is not Copy",
                "That `println!` cannot take a String",
            ],
            2,
            "Everything you need is in the annotation: the type is `String`, `String` does not \
             implement `Copy`, therefore this is a move. That is the decision procedure from lesson 11, \
             spelled out by the compiler in two lines. Rust does not merely report the symptom; it \
             names the rule.",
        )
        .code(
            "error[E0382]: borrow of moved value: `s`\n --> src/main.rs:4:19\n  |\n2 |     let s = String::from(\"hello\");\n  |         -- move occurs because `s` has type `String`,\n  |            which does not implement the `Copy` trait\n3 |     let s2 = s;\n  |              -- value moved here\n4 |     println!(\"{s}\");\n  |                ^^ value borrowed here after move",
        )
        .hints(&[
            "The annotation says the type and the missing trait",
            "That pair is the whole decision procedure.",
        ]),
        SeedExercise::new(
            Some(20),
            ExerciseKind::Mcq,
            "The compiler suggests `.clone()`. Why is that not always the right fix?",
            &[
                "Because clone is deprecated",
                "Because it duplicates the data — the other options are to reorder, or to borrow",
                "Because clone only works for small Strings",
                "Because it will not silence this particular error",
            ],
            1,
            "It silences the error, and it is one of three legitimate answers — but it is the one that \
             costs memory. The alternatives are reordering your code or borrowing instead. The compiler \
             cannot know which of the three you meant, which is why its suggestion is only a hint.",
        )
        .with_hints(&[
            "List every way to stop this error, not just the cheapest one",
            "What does clone cost that the alternatives do not?",
        ]),

        // ── L21 Debugging ───────────────────────────────────────────────
        SeedExercise::new(
            Some(21),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — run the nine-question checklist on this",
            &[
                "Yes — the returned value is still owned by the function",
                "No — the function returned a new String, and `name` was moved in and not moved back",
                "Yes — functions can return values they were given",
                "No — because `to_uppercase` is not a function",
            ],
            1,
            "The parameter `s: String` took ownership, and the function built a *new* String from it \
             rather than handing `s` back. So the value the caller passed is gone, and the caller's \
             `name` is disarmed. The nine questions point at this: Q3 (is String Copy? no), Q6 (is a \
             value entering a function? yes), Q7 (did it return the same value? no).",
        )
        .code(
            "fn shout_it(s: String) -> String {\n    s.to_uppercase()\n}\n\nfn main() {\n    let name = String::from(\"Gus\");\n    let shout = shout_it(name);\n    println!(\"{name} {shout}\");\n}",
        )
        .hints(&[
            "Does the return expression hand `s` back, or build something new?",
            "Work backwards: which value does `shout` own?",
        ]),
        SeedExercise::new(
            Some(21),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — this is the version that borrows instead of consuming",
            &[
                "Yes — and `name` is still usable afterwards",
                "No — `&String` still takes ownership of the characters",
                "No — a reference cannot be passed to a function that returns a String",
                "No — because `to_uppercase` needs to modify the original",
            ],
            0,
            "Yes, and both `name` and `shout` survive. The signature changed from `String` to \
             `&String`, so the function borrows instead of consuming, and it still produces a *new* \
             String to return. This is the free fix — the previous exercise's version returned a new \
             value and consumed the input, and this one gives up ownership without giving up the result.",
        )
        .code(
            "fn shout_it(s: &String) -> String {\n    s.to_uppercase()\n}\n\nfn main() {\n    let name = String::from(\"Gus\");\n    let shout = shout_it(&name);\n    println!(\"{name} {shout}\");\n}",
        )
        .hints(&[
            "What changed between this signature and the previous one?",
            "A borrow does not consume. Does the original survive?",
        ]),
        SeedExercise::mcq(
            21,
            "A function must keep a value alive after it returns. Which fix will not work?",
            &[
                "Returning the value so ownership travels back to the caller",
                "Taking it by value and giving it straight back",
                "Taking a reference and returning it",
                "Cloning it inside the function and returning the clone",
            ],
            2,
            "Returning a reference is the one that cannot work — the reference would outlive the data it \
             points at, and the compiler will not let you. This is a preview of lifetimes, and the rule \
             emerging here is: a function cannot hand back a reference to something it does not own. \
             Borrowing is fine for *reading* and not for *keeping*.",
        )
        .with_hints(&[
            "What happens to the data a reference points at when the function returns?",
            "Reading is fine. Keeping is not. Which fix is 'keeping'?",
        ]),
        SeedExercise::new(
            Some(21),
            ExerciseKind::PredictOwnership,
            "PREDICT OWNERSHIP — which fix costs an allocation?",
            &[
                "Returning the value so it travels back",
                "Borrowing with `&String`",
                "Cloning before the call",
                "Reordering so the print happens first",
            ],
            2,
            "Cloning. Returning and reordering are both free, and borrowing is free — it just creates a \
             reference rather than transferring anything. Clone is the only one of the four that \
             allocates and copies. Being able to name which fix is free is more useful than memorising \
             which fix compiles.",
        )
        .code("let shouted = shout_it(name.clone());   // allocates")
        .hints(&[
            "Three of these are free. Which is not?",
            "What does clone do that the others do not?",
        ]),
        SeedExercise::new(
            Some(21),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — a control case that looks like a move problem",
            &[
                "No — a moved number cannot be reassigned",
                "No — but the reason is mutability, not ownership",
                "Yes — because `i32` is Copy, so the move is harmless",
                "No — and the fix is to clone the value",
            ],
            1,
            "It fails, but for an entirely different reason: `x` is not declared `mut`, so it cannot be \
             reassigned. `Copy` means you get a second independent number, so there is no move and no \
             moved-value error — `x` is still perfectly alive. This is the control case that shows why \
             you have to read the error code, not pattern-match on 'it didn't compile'.",
        )
        .code("fn main() {\n    let n = 5;\n    let m = n;\n    n = 7;\n    println!(\"{n} {m}\");\n}")
        .hints(&[
            "Is there a `mut` on `n`?",
            "What error code would you expect — E0382, or something else?",
        ]),

        // ── Open-ended ──────────────────────────────────────────────────
        SeedExercise::new(
            None,
            ExerciseKind::BuildIt,
            "BUILD IT — write a program that proves the three rules to yourself, with the compiler as \
             the referee",
            &[
                "Write four separate programs, one per rule, each small enough to read at a glance",
                "For each, write in a comment which rule it demonstrates and what the expected error is",
                "Include at least one case the compiler ACCEPTS that a beginner expects it to reject",
                "Paste the real compiler output next to your prediction",
            ],
            0,
            "The accepted case is the valuable one. Everyone predicts the use-after-move; almost nobody \
             considers that `let s = s;` compiles. Write a program you expected to fail, note the \
             surprise, and the model gets sharper than more reading would have made it.",
        )
        .hints(&[
            "Rule 1: an i32 still has an owner even though nothing is freed",
            "Rule 2: the forbidden state is two owners of ONE value",
            "Rule 3: try a Drop impl that prints, and read the order",
        ]),
        SeedExercise::new(
            None,
            ExerciseKind::BuildIt,
            "BUILD IT — a memory-model lab",
            &[
                "Create a String, a Vec, and an i32 in the same function",
                "Print `as_ptr()` for the String and the raw slice pointer for the Vec",
                "Move each of them once and print the addresses again",
                "Write two comments: what changed, and what did not",
            ],
            0,
            "This is the empirical version of 'a move transfers three words, not the data'. The \
             addresses staying identical across a move is the proof, and proving it yourself is worth \
             more than being told it. The comment is not optional — writing what changed is the \
             exercise.",
        )
        .hints(&[
            "`as_ptr()` on String, `as_ptr()` on the Vec's slice",
            "Compare the values before and after each move",
        ]),
        SeedExercise::new(
            None,
            ExerciseKind::BuildIt,
            "BUILD IT — a deliberate-ownership exercise",
            &[
                "Pick one small program you wrote in an earlier module",
                "For every String, Vec, and Box: write down whether it should be moved, copied, or cloned",
                "Add a comment at each site saying which and WHY",
                "Then check whether the compiler agrees with your reasoning at every point",
            ],
            0,
            "This is the skill the module is for. Not recognising the error after you have written it, \
             but deciding in advance — and then finding the places where your decision was wrong. \
             Expect to be wrong about at least one, and read the compiler's reason carefully when it \
             happens.",
        )
        .hints(&[
            "Start with the places where a value is used more than once",
            "Used once and then dead? Move. Needed twice? Think about why.",
        ]),
    ]
}
