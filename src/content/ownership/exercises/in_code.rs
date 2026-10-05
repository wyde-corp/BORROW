//! Module 05 — Ownership, exercises for lessons 12–18.
//!
//! Ownership in assignment, function calls, return values, scope, drop,
//! shadowing, and collections. The recurring question is always the same:
//! who owns this value right now, and who frees it?

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // ── L12 Ownership and Assignment ───────────────────────────────
        SeedExercise::new(
            Some(11),
            ExerciseKind::PredictOwnership,
            "PREDICT OWNERSHIP — when is the \"hello\" buffer freed?",
            &[
                "At the end of main",
                "At the moment `s` is reassigned, before \"goodbye\" is created",
                "When `goodbye` is printed",
                "Never — it is leaked",
            ],
            1,
            "The moment ownership changes hands. Rust does not wait for end of scope to tidy up a \
             value the variable no longer owns, and it does not need a collector to notice. The old \
             buffer is released right there, which is something most languages only approximate.",
        )
        .code("let mut s = String::from(\"hello\");\ns = String::from(\"goodbye\");\nprintln!(\"{s}\");")
        .hints(&[
            "After the reassignment, does `s` still own \"hello\"?",
            "Then who releases it?",
        ]),
        SeedExercise::mcq(
            11,
            "Why does `let mut s` need `mut` when reassigning s never changes the *contents* of a String?",
            &[
                "Because `mut` controls who may read the value",
                "Because reassigning changes which value the binding owns — that is a change to the \
                 binding",
                "Because `String` is always mutable",
                "Because `mut` is required for any `let` with a non-primitive type",
            ],
            1,
            "`mut` is about the binding, not the contents. Reassigning changes which value `s` refers \
             to, and that counts as modifying the binding. This is why so much Rust that appears \
             immutable is never marked `mut` — if you never rebind, you never need it.",
        )
        .with_hints(&[
            "Ask what changes: the bytes, or which value the name points at?",
            "Try removing `mut` and reading the error.",
        ]),
        SeedExercise::new(
            Some(11),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE",
            &[
                "Yes — the second assignment is fine",
                "No — `y` is not declared `mut`, so it cannot be reassigned",
                "No — `x` was copied, so `y` does not own it",
                "No — reassigning always moves the old value out",
            ],
            1,
            "`y` was never declared `mut`, so rebinding it is refused. Note that this is a different \
             complaint from anything about ownership — it is the mutability system doing its job, not \
             the ownership rules.",
        )
        .code("fn main() {\n    let x = 5;\n    let y = x;\n    y = 10;\n}")
        .hints(&[
            "Is there a `mut` on the declaration of `y`?",
            "What kind of error is this — ownership, or mutability?",
        ]),
        SeedExercise::mcq(
            11,
            "Which single constraint makes reassignment simple in Rust?",
            &[
                "A variable may own at most one value at a time",
                "A variable may be reassigned at most once",
                "Values may only be reassigned inside a loop",
                "Reassignment requires the new value to have the same length",
            ],
            0,
            "One owner at a time. There is never a moment with two owners or zero, so reassignment \
             has an unambiguous meaning: the new value arrives, the old one is released, the same \
             variable is responsible throughout. Allow two owners and this becomes genuinely subtle.",
        )
        .with_hints(&[
            "Which of the three rules covers this?",
            "Think about what would be ambiguous with two owners.",
        ]),
        SeedExercise::new(
            Some(11),
            ExerciseKind::PredictOwnership,
            "PREDICT OWNERSHIP — does `push` move anything or free anything?",
            &[
                "It moves the contents into a new buffer",
                "Nothing — the contents change, but `v` still owns its buffer and frees it once at end \
                 of scope",
                "It frees the old buffer immediately",
                "It clones the whole Vec first",
            ],
            1,
            "Neither. `push` changes what the `Vec` contains without changing what it *owns* — the \
             same variable still owns the same buffer, or a bigger one, and frees it once. Ownership is \
             about the buffer, not the contents. This distinction matters a lot in the collections \
             module.",
        )
        .code("let mut v = vec![1];\nv.push(2);\nprintln!(\"{v:?}\");")
        .hints(&[
            "Does the variable `v` change which buffer it owns?",
            "How many times is anything freed at end of scope?",
        ]),

        // ── L13 Ownership and Functions ────────────────────────────────
        SeedExercise::new(
            Some(12),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — and where is the problem, exactly?",
            &[
                "Yes — the function borrowed the string temporarily",
                "No — the String parameter took ownership, so the caller's `s` is gone",
                "Yes — because the function returns nothing",
                "No — because the function prints the string",
            ],
            1,
            "The problem is in the signature, not the body. `s: String` means *takes ownership*, so the \
             call moves the value in. The fix is a reference, `s: &String`, which is the preview of \
             the entire next module.",
        )
        .code("fn consume(s: String) {\n    println!(\"{s}\");\n}\n\nfn main() {\n    let s = String::from(\"hello\");\n    consume(s);\n    println!(\"{s}\");\n}")
        .hints(&[
            "Look at the parameter type, not the body",
            "`String` versus `&String` — what is the difference?",
        ]),
        SeedExercise::new(
            Some(12),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — the same function but with an `i32`",
            &[
                "No — the same rule applies to all parameter types",
                "Yes, because i32 implements Copy so the argument is copied",
                "Yes, because the function does not return anything",
                "No, because a function cannot take ownership of a number",
            ],
            1,
            "Yes, and `x` survives. The function body is structurally identical to the String version; \
             the only difference is the type in the signature. That is the lesson: the signature tells \
             you what the function does to your ownership.",
        )
        .code("fn use_number(x: i32) {\n    println!(\"{x}\");\n}\n\nfn main() {\n    let x = 5;\n    use_number(x);\n    println!(\"{x}\");\n}")
        .hints(&[
            "Same body, different type",
            "Does `i32` implement `Copy`?",
        ]),
        SeedExercise::mcq(
            12,
            "In `fn f(s: String)` versus `fn f(s: &String)`, what does the ampersand change?",
            &[
                "Nothing — they behave identically",
                "The first takes ownership, the second borrows temporarily",
                "The first is faster",
                "The first allows mutation, the second does not",
            ],
            1,
            "Ownership versus borrowing — and that is the whole decoding. `T` in a parameter takes \
             ownership, `&T` borrows, `&mut T` borrows and may modify. Every signature in Rust obeys \
             this, and noticing it is most of what reading unfamiliar Rust code comes to.",
        )
        .with_hints(&[
            "The only difference is one character",
            "What does that character do to ownership?",
        ]),
        SeedExercise::new(
            Some(12),
            ExerciseKind::PredictOwnership,
            "PREDICT OWNERSHIP — a function that receives a String and does nothing but drop it. Who \
             frees the memory?",
            &[
                "The caller, after the function returns",
                "The function itself, when its scope ends",
                "Nobody — it leaks",
                "Both of them, which is the double-free bug Rust prevents",
            ],
            1,
            "The function. Ownership transferred in, so the function is now the owner, and it drops \
             the value at its closing brace. Cleanup follows ownership *wherever ownership is* — that \
             is what makes moving safe rather than merely efficient.",
        )
        .code("fn eat(s: String) {\n    // no println, no return — just takes it\n}\n\nfn main() {\n    let s = String::from(\"goodbye\");\n    eat(s);\n}")
        .hints(&[
            "Inside `eat`, who owns `s`?",
            "Whose scope does the cleanup happen in?",
        ]),
        SeedExercise::new(
            Some(12),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — calling a consuming function twice with the same String",
            &[
                "Yes — the first call consumed it, so the second uses a fresh value",
                "No — the first call moved `s`, so the second use is a use-after-move",
                "Yes — functions are allowed to consume a value more than once",
                "No — because the function returns nothing",
            ],
            1,
            "The first call moved `s` and the second tries to use it again. Same error as any other \
             use-after-move, just at a call site rather than an assignment. If you need to call twice, \
             the function should not be taking ownership.",
        )
        .code("fn shout(s: String) { println!(\"{}\", s.to_uppercase()); }\n\nfn main() {\n    let s = String::from(\"hey\");\n    shout(s);\n    shout(s);\n}")
        .hints(&[
            "Run the two calls in your head, one at a time",
            "After the first, does `s` still hold anything?",
        ]),
        SeedExercise::mcq(
            12,
            "A function only needs to read a String. What is the right signature?",
            &[
                "`fn f(s: String)` — simplest to write",
                "`fn f(s: &String)` — it borrows, so the caller keeps ownership",
                "`fn f(s: &mut String)` — it needs to modify it",
                "`fn f(s: String) -> String` — return it so nothing is lost",
            ],
            1,
            "A reference, because the function does not need to keep the value. Taking ownership would \
             force every caller to give up something it still wants, and that friction is exactly the \
             problem the next module removes properly. Answer B is correct here even though `&str` \
             would be even better — the `&String` version is what teaches you to read the ampersand.",
        )
        .with_hints(&[
            "Three fixes exist: clone, borrow, or return. Which costs nothing?",
            "Which one does not need the function to change what it does?",
        ]),

        // ── L14 Ownership and Return Values ─────────────────────────────
        SeedExercise::new(
            Some(13),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — and what was transferred?",
            &[
                "No — a function cannot return an owned value",
                "Yes — the String moved from the function to `s`",
                "Yes — a copy of the String was made on return",
                "No — returning requires `mut`",
            ],
            1,
            "It moves. A return is a transfer in the opposite direction from an argument, and it costs \
             exactly as much: nothing. The value the function had is the value the caller now has — \
             no intermediate copy.",
        )
        .code("fn create() -> String {\n    String::from(\"hello\")\n}\n\nfn main() {\n    let s = create();\n    println!(\"{s}\");\n}")
        .hints(&[
            "What happens to a value as a function returns?",
            "Is `s` holding a copy, or the original?",
        ]),
        SeedExercise::new(
            Some(13),
            ExerciseKind::PredictOwnership,
            "PREDICT OWNERSHIP — take a String, give it straight back. What happened?",
            &[
                "The value was copied out and the original freed",
                "Ownership went in and came straight back; nothing was copied",
                "The value was cloned, because returns duplicate",
                "The value was dropped and a new one created",
            ],
            1,
            "A round trip, for free. This looks like a pointless function and is actually a useful one: \
             it is a documented, reusable way of saying 'this value keeps its ownership but passes \
             through here'. You will write this pattern constantly once lifetimes appear.",
        )
        .code("fn take_and_return(s: String) -> String {\n    s\n}\n\nfn main() {\n    let s = String::from(\"hello\");\n    let s = take_and_return(s);\n    println!(\"{s}\");\n}")
        .hints(&[
            "What did the parameter hold, and what does the return expression hold?",
            "Same value both times?",
        ]),
        SeedExercise::new(
            Some(13),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — calling a String-returning function twice",
            &[
                "No — a function can only be called once",
                "No — the second call would move the first result",
                "Yes — each call creates a brand new String",
                "No — only Copy types can be returned twice",
            ],
            2,
            "Yes. Each call runs the function afresh and produces its own value, so there is nothing to \
             run out of. Returning is the *opposite* of consuming: a function that returns a value \
             never takes anything away from you.",
        )
        .code("fn make() -> String {\n    String::from(\"x\")\n}\n\nfn main() {\n    let a = make();\n    let b = make();\n    println!(\"{a} {b}\");\n}")
        .hints(&[
            "Is there shared state inside `make`?",
            "Each call is independent. What does that imply?",
        ]),
        SeedExercise::mcq(
            13,
            "Why is `fn take_and_return(s: String) -> String { s }` worth writing, rather than looking \
             pointless?",
            &[
                "It is a pointless function and should be deleted",
                "It is a documented, reusable, free ownership round trip",
                "It is required by the compiler in that situation",
                "It is faster than passing by reference",
            ],
            1,
            "It is a named, documented way of saying 'this value passes through here and keeps its \
             ownership'. It costs three words on the stack, it makes intent obvious at the call site, \
             and it is genuinely reusable. Calling it pointless would be like saying passing a value \
             around is pointless.",
        )
        .with_hints(&[
            "What does a reader learn from the function's name and signature?",
            "How much does it cost?",
        ]),
        SeedExercise::mcq(
            13,
            "A function returns a String it created. Who is responsible for the memory?",
            &[
                "The function, because it made it",
                "The caller, because ownership transferred with the return value",
                "Nobody — it is freed immediately",
                "Both, which is why returns are risky",
            ],
            1,
            "The caller. Whoever owns a value is responsible for it, and the return value arrives owned \
             by the caller. 'Who made it' and 'who owns it now' are two different questions, and \
             confusing them is how people end up imagining leaks that do not exist.",
        )
        .with_hints(&[
            "Apply rule 1 to the value after the function has returned",
            "Creation and ownership are separate questions.",
        ]),

        // ── L14b Ownership and Scope ────────────────────────────────────
        SeedExercise::new(
            Some(14),
            ExerciseKind::PredictOwnership,
            "PREDICT OWNERSHIP — nested scopes, inner declared second. What is the drop order?",
            &[
                "outer, then inner — creation order",
                "inner, then outer — reverse creation order",
                "Both at the end of main, in declaration order",
                "Only inner is dropped, because outer is borrowed",
            ],
            1,
            "Reverse creation order, guaranteed by the compiler. This is stack discipline showing \
             through: the most recently created value is the first to be finished. It holds even when \
             values are created inside conditionals or loops.",
        )
        .code("let outer = String::from(\"outer\");\n{\n    let inner = String::from(\"inner\");\n    println!(\"{outer} {inner}\");\n}\nprintln!(\"{outer}\");")
        .hints(&[
            "Which scope ends first?",
            "The compiler guarantees an order here. Which one?",
        ]),
        SeedExercise::new(
            Some(14),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE",
            &[
                "Yes — `inner` is still in scope in main",
                "No — `inner` was declared inside the block and that block has closed",
                "Yes — inner scopes are only for organisation",
                "No — you cannot print a String from an outer scope",
            ],
            1,
            "`inner` belongs to the inner block, and that block is closed. Out of scope means the name \
             does not exist — there is no stale value to read, which is the other half of why Rust has \
             no uninitialised-variable bugs.",
        )
        .code("fn main() {\n    let outer = String::from(\"outer\");\n    {\n        let inner = String::from(\"inner\");\n    }\n    println!(\"{inner}\");\n}")
        .hints(&[
            "Where was `inner` declared?",
            "Which brace closed that declaration?",
        ]),
        SeedExercise::mcq(
            14,
            "Beyond tidiness, what is a genuinely useful reason to create an inner block?",
            &[
                "To make the code look longer",
                "To end a value's lifetime early, so a borrow or lock does not outlive its useful region",
                "To let you redeclare the same variable name",
                "To make the compiler skip checks",
            ],
            1,
            "A block is a definite end, and a definite end is a tool. It is how you confine a lock to \
             the span that needs it, or end a borrow early enough that the borrow checker will then let \
             you mutate. Braces as lifetime control is a real tool, not a style preference.",
        )
        .with_hints(&[
            "What does a block give you that `let` does not?",
            "Think about holding a lock too long.",
        ]),
        SeedExercise::new(
            Some(14),
            ExerciseKind::PredictOwnership,
            "PREDICT OWNERSHIP — this drops a value early. Who frees it, and when?",
            &[
                "main, at the end of main",
                "The block's scope, as soon as the closing brace is reached",
                "Nobody — drop() does nothing",
                "Wherever the optimiser decides",
            ],
            1,
            "Immediately, at the closing brace. An explicit `drop` runs the same cleanup that end of \
             scope would have run, just earlier. The name is then unusable — Rust has no concept of \
             an in-scope but uninitialised variable.",
        )
        .code("fn main() {\n    let s = String::from(\"hello\");\n    drop(s);\n    // println!(\"{s}\");  ← gone\n}")
        .hints(&[
            "What does drop do that end-of-scope would not?",
            "It just does it sooner.",
        ]),

        // ── L15 Dropping Values ─────────────────────────────────────────
        SeedExercise::mcq(
            15,
            "A String is moved into a function, and the function never returns it. Where is it freed?",
            &[
                "In main, at the end of main",
                "Inside the function, when its scope ends",
                "Nowhere — it leaks",
                "Both places, which is a double free",
            ],
            1,
            "Inside the function, at its closing brace. Cleanup follows ownership wherever ownership \
             happens to be, which is precisely what makes moves safe. Nothing leaks and nothing is \
             freed twice — the value simply has a different owner than you might expect.",
        )
        .kind(ExerciseKind::PredictOwnership)
        .with_code("fn take(n: String) { println!(\"{n}\"); }\n\nfn main() {\n    let s = String::from(\"a\");\n    take(s);\n    println!(\"end of main\");\n}")
        .with_hints(&[
            "Inside `take`, who owns the value?",
            "Whose scope ends first?",
        ]),
        SeedExercise::new(
            Some(15),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — using a variable after an explicit `drop`",
            &[
                "Yes — it is empty now but still exists",
                "No — after drop, the binding is gone, just as at end of scope",
                "Yes — drop only frees memory, the name survives",
                "No — but only if the value was heap-allocated",
            ],
            1,
            "After `drop(s)` the name is unusable, exactly as it would be at end of scope. Rust has no \
             half-initialised variables, so there is no such thing as an empty String left behind. If \
             you want the name back, reassign it.",
        )
        .code("fn main() {\n    let s = String::from(\"a\");\n    drop(s);\n    println!(\"{s}\");\n}")
        .hints(&[
            "Is `drop` any different from the closing brace?",
            "Only in *when* it happens.",
        ]),
        SeedExercise::mcq(
            15,
            "What does implementing `Drop` for your own type let you do?",
            &[
                "Postpone cleanup to the end of the program",
                "Run your own cleanup code when the value goes out of scope, and control where the \
                 compiler places that call",
                "Prevent the value from being moved",
                "Make the type implement Copy",
            ],
            1,
            "You write cleanup logic that runs when the value's owner goes out of scope — opening a \
             file, releasing a lock, writing a log line — and the compiler inserts the call wherever \
             the value is finally dropped. That placement is often the more useful half: a `Drop` impl \
             can move the cleanup to exactly where the value dies.",
        )
        .with_hints(&[
            "You have used it on String without noticing",
            "Think about *when* it fires, not just what it does.",
        ]),
        SeedExercise::mcq(
            15,
            "Which of these is NOT a way a value gets dropped?",
            &[
                "At the end of the owner's scope",
                "When `drop(value)` is called explicitly",
                "When the process exits without running cleanup",
                "When ownership is moved to a new owner, at the new owner's end of scope",
            ],
            2,
            "A process exiting does not run destructors, and Rust does not pretend otherwise. This is \
             why `main` returning is the right way to finish — it is the return that unwinds and runs \
             the cleanups, not the process teardown.",
        )
        .with_hints(&[
            "One of these is about the operating system, not about Rust",
            "When exactly does cleanup run?",
        ]),

        // ── L16 Shadowing vs Moving ─────────────────────────────────────
        SeedExercise::new(
            Some(16),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — `let s = s;`",
            &[
                "No — you cannot assign a variable to itself",
                "Yes — the right-hand `s` is moved into a new binding that shadows it",
                "Yes — and it is a no-op",
                "No — String does not implement Copy",
            ],
            1,
            "It compiles, and it is not a no-op in the way it looks. The right-hand `s` is moved out \
             of the old binding, then a brand new binding named `s` takes it. The old binding is \
             shadowed rather than disarmed, which is why the compiler stays quiet about the move.",
        )
        .code("fn main() {\n    let s = String::from(\"hi\");\n    let s = s;\n    println!(\"{s}\");\n}")
        .hints(&[
            "Which `s` is on the right-hand side?",
            "Is the new binding a copy or the same value?",
        ]),
        SeedExercise::new(
            Some(16),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — the same operation to a new name",
            &[
                "Yes — the old `s` is still usable",
                "No — the old `s` was moved, so using it is an error",
                "Yes — a copy was made",
                "No — because you used a different variable name",
            ],
            1,
            "No. The difference from `let s = s;` is entirely about naming. Move to a *new* name and the \
             old binding stays in scope, so the compiler tells you it was moved. Shadow it and the old \
             binding is hidden, so it says nothing. Same operation, different diagnostics — and that \
             difference is a useful tell.",
        )
        .code("fn main() {\n    let s = String::from(\"hi\");\n    let t = s;\n    println!(\"{s}\");\n}")
        .hints(&[
            "Is the old `s` shadowed, or still visible?",
            "That is what decides whether the compiler mentions the move.",
        ]),
        SeedExercise::new(
            Some(16),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — shadowing changes the type",
            &[
                "No — a variable's type is fixed once declared",
                "Yes — shadowing creates a new binding, which may have a different type",
                "Yes — but only for primitives",
                "No — you need `mut` to change the type",
            ],
            1,
            "Yes. `let x = 5; let x = \"five\";` is legal, and it is one of the clearest demonstrations \
             that shadowing is not mutation — the first `x` is still there, an `i32`, untouched. The \
             `mut` version fails, and that is the comparison worth making yourself.",
        )
        .code("fn main() {\n    let x = 5;\n    let x = \"five\";\n    println!(\"{x}\");\n}")
        .hints(&[
            "Is the first `x` gone, or still there underneath?",
            "Compare with what `let mut x = 5; x = \"five\";` would do.",
        ]),
        SeedExercise::new(
            Some(16),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — the `mut` version of the same thing",
            &[
                "Yes — mut allows any change",
                "No — `mut` permits rebinding to the same type, not a different one",
                "Yes — the type is inferred from the new value",
                "No — because the variable was not `mut` in the original",
            ],
            1,
            "No. `mut` is about *which* value a binding refers to, not *what type* that value has. A \
             `mut` binding still has one type for its whole life, and assigning a `&str` to an `i32` \
             is a type error. Shadowing and mutation are genuinely different tools with different \
             rules.",
        )
        .code("fn main() {\n    let mut x = 5;\n    x = \"five\";\n    println!(\"{x}\");\n}")
        .hints(&[
            "Does `mut` change the binding's type?",
            "What does `mut` actually permit?",
        ]),
        SeedExercise::mcq(
            16,
            "Shadowing and moving produce the same picture afterwards. What is the actual difference?",
            &[
                "Shadowing copies; moving transfers",
                "Shadowing creates a new binding and hides the old one; moving transfers the value and \
                 disarms the old binding, which stays in scope",
                "They are the same operation with different syntax",
                "Shadowing only works for Copy types",
            ],
            1,
            "The consequence is what you can observe. With a move to a new name, the old name is still \
             there and the compiler tells you it is dead. With shadowing, the old binding is hidden and \
             the compiler says nothing. Same value, genuinely different history.",
        )
        .with_hints(&[
            "Which one produces a compiler message?",
            "That tells you which name is still in scope.",
        ]),

        // ── L17 Ownership With Collections ──────────────────────────────
        SeedExercise::new(
            Some(17),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE",
            &[
                "Yes — Vecs are cheap to copy",
                "No — a Vec owns a heap buffer, so assigning moves it",
                "Yes — because the elements are all i32",
                "No — because Vec does not implement Copy",
            ],
            1,
            "No. A `Vec` owns a heap buffer and is not `Copy`, so `other` takes the value and `numbers` \
             is left unusable. Nothing about this is special to `String` — it is the same three rules \
             applied to a different type, which is exactly the point of this lesson.",
        )
        .code("fn main() {\n    let numbers = vec![1, 2, 3];\n    let other = numbers;\n    println!(\"{numbers:?}\");\n}")
        .hints(&[
            "Does `Vec<i32>` implement Copy?",
            "Compare with a `Vec` of `Copy` elements — does that change anything?",
        ]),
        SeedExercise::new(
            Some(17),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — a function that sums a Vec by value",
            &[
                "Yes — the elements are Copy so the Vec is too",
                "No — the Vec moved into the function, so the caller cannot use it afterwards",
                "Yes — integers can always be passed freely",
                "No — because the function needs `&`",
            ],
            1,
            "No. `Vec` is not `Copy` regardless of its elements, so the vector moved in. This is the \
             third time this module that friction has appeared, and each time the answer is the same: \
             the function only needs to read it. That is the problem the next module solves.",
        )
        .code("fn total(values: Vec<i32>) -> i32 {\n    values.iter().sum()\n}\n\nfn main() {\n    let v = vec![1, 2, 3];\n    let t = total(v);\n    println!(\"{t} {v:?}\");\n}")
        .hints(&[
            "What does `values: Vec<i32>` say the function intends to do with it?",
            "It only reads. Is taking ownership necessary?",
        ]),
        SeedExercise::new(
            Some(17),
            ExerciseKind::MemoryDiagram,
            "DRAW THE MEMORY — a `Vec<String>` with one element. How many heap allocations does it own?",
            &[
                "One — the vector's buffer",
                "Two — the vector's buffer, plus one String buffer per element",
                "Three — one for the vector, one for the length, one for the contents",
                "Zero — it is all on the stack",
            ],
            1,
            "Two: the vector's own buffer of `String` headers, and one buffer for the string each header \
             points at. This is why cloning a `Vec<String>` is more interesting than cloning a \
             `Vec<i32>` — it copies the headers *and* every string they own.",
        )
        .code("let v = vec![String::from(\"a\")];")
        .hints(&[
            "How many layers of ownership are here?",
            "Each String inside the vector owns something of its own.",
        ]),
        SeedExercise::mcq(
            17,
            "A `Vec<i32>` clone copies one million integers. What does a `Vec<String>` clone copy?",
            &[
                "The same thing, since the element type does not matter",
                "The headers, plus every buffer each owned String pointed at — and it also clones each \
                 String",
                "Only the headers, because the Strings are shared",
                "Nothing, because Vecs are not cloned",
            ],
            1,
            "Everything, at two levels. The vector's buffer of `String` headers is copied, and each of \
             those Strings clones its own buffer too. `Vec<T>` is `Copy` when `T` is, and clone cost \
             scales with the *total* data owned, not the number of elements.",
        )
        .with_hints(&[
            "A String header is three words, and it points at more words",
            "Clone is defined by the element type.",
        ]),
        SeedExercise::new(
            Some(17),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — and is anything allocated a second time?",
            &[
                "Yes, and the clone allocates a second buffer",
                "Yes, and the assignment is O(1) with no new allocation",
                "No, because Vec is not Copy",
                "Yes, and both vectors point at the same buffer",
            ],
            0,
            "Yes, and `.clone()` allocates a genuinely second buffer and copies the contents. Compare \
             with the plain move, which is O(1) and allocates nothing at all. That is the entire \
             difference, and it is the difference between `let b = a;` and `let b = a.clone();`.",
        )
        .code("fn main() {\n    let numbers = vec![1, 2, 3];\n    let other = numbers.clone();\n    println!(\"{numbers:?} {other:?}\");\n}")
        .hints(&[
            "What does clone do to the allocation count?",
            "What does the move do?",
        ]),
    ]
}
