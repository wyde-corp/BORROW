//! Module 05 — Ownership, exercises for lessons 9–11.
//!
//! Copy, Clone, and the decision procedure that ties them to Move. This is
//! where the learner starts predicting rather than recognising.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // ── L9 Copy Types ───────────────────────────────────────────────
        SeedExercise::new(
            Some(8),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — and is `x` still usable afterwards?",
            &[
                "Yes, and `x` is still usable because i32 implements Copy",
                "Yes, but `x` is moved and unusable",
                "No, because i32 is not Copy",
                "Yes, because integers are small",
            ],
            0,
            "It compiles and `x` survives. `i32` implements `Copy`, which is the actual reason — not \
             that it is small. A 256-byte array of `Copy` elements would behave identically.",
        )
        .code("fn main() {\n    let x = 5;\n    let y = x;\n\n    println!(\"{x} {y}\");\n}")
        .hints(&[
            "Check the trait, not the size",
            "Does `i32` implement `Copy`?",
        ]),
        SeedExercise::new(
            Some(8),
            ExerciseKind::Mcq,
            "WHICH TYPES IMPLEMENT COPY — pick the correct set",
            &[
                "i32, String, &T, Vec<i32>",
                "i32, bool, char, &T, [u8; 4]",
                "i32, bool, &mut i32, String",
                "All of them — everything is Copy except references",
            ],
            1,
            "Fixed-size types with no cleanup are `Copy`, and shared references are `Copy` because \
             copying one cannot let anyone free anything. `&mut T` deliberately is not — two writable \
             handles to one value is exactly the aliasing problem ownership exists to prevent.",
        )
        .hints(&[
            "For each, ask: can duplicating it cause a double free?",
            "What is special about `&mut T` compared to `&T`?",
        ]),
        SeedExercise::mcq(
            8,
            "Why can a type NOT implement both `Copy` and `Drop`?",
            &[
                "Because the compiler cannot decide which one runs first",
                "Because two copies would mean two cleanups, and cleanup must happen exactly once",
                "Because `Drop` is only allowed on structs",
                "Because `Copy` is only allowed on primitives",
            ],
            1,
            "If a type needs custom cleanup, then two copies would mean two cleanups — and the whole \
             reason `Copy` exists is that duplicating the type is free and harmless. The two traits are \
             mutually exclusive by construction, and the compiler enforces it.",
        )
        .kind(ExerciseKind::PredictCompile)
        .with_code(
            "struct Noisy;\nimpl Drop for Noisy {\n    fn drop(&mut self) { println!(\"bye\"); }\n}\nimpl Copy for Noisy {}\n",
        )
        .with_hints(&[
            "Try it and read the error",
            "What would two Noisy values do when dropped?",
        ]),
        SeedExercise::new(
            Some(8),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — a 1024-byte array of u8",
            &[
                "No — it is too large to be Copy",
                "Yes, and both are usable afterwards, because `[u8; 1024]` is Copy",
                "Yes, but only because arrays are special-cased",
                "No — arrays always live on the heap",
            ],
            1,
            "Yes, and both survive. Its size is known at compile time, it has no cleanup, and every \
             element is `Copy` — so the whole array is `Copy`. This is the example that kills the \
             'stack types copy, heap types move' rule as a usable mental model: nothing here is small.",
        )
        .code(
            "fn main() {\n    let a = [0u8; 1024];\n    let b = a;\n    println!(\"{} {}\", a.len(), b.len());\n}",
        )
        .hints(&[
            "Size known at compile time?",
            "Does it need cleanup?",
        ]),
        SeedExercise::new(
            Some(8),
            ExerciseKind::PredictOwnership,
            "PREDICT OWNERSHIP — this compiles. Does that mean the value was copied, moved, or cloned?",
            &[
                "Copied — the compiler inserted an implicit clone",
                "Copied — because `&str` implements Copy",
                "Moved — the reference itself is the value being transferred",
                "Cloned — nothing else would compile",
            ],
            1,
            "`&str` implements `Copy` because it is just a pointer and a length — duplicating it does \
             not duplicate the text, it duplicates the *address*. This is a good thing to internalise: \
             `Copy` is about the value, and the value here is small.",
        )
        .code("fn main() {\n    let a = \"hello\";\n    let b = a;\n    println!(\"{a} {b}\");\n}")
        .hints(&[
            "What is a `&str` physically?",
            "Does copying it copy the characters?",
        ]),
        SeedExercise::mcq(
            8,
            "What is wrong with the rule 'stack types copy, heap types move'?",
            &[
                "Nothing — it is a useful simplification",
                "It is close but the real rule is whether the type implements Copy",
                "It is exactly right, just informally worded",
                "It is wrong because everything is on the stack",
            ],
            1,
            "It gets the intuition and mangles the rule. `String` contains stack-resident data and does \
             not implement `Copy`; a 1 KB `[u8; 1024]` never touches the heap and does. Stack versus \
             heap is a good *reason*; the trait is the *rule*.",
        )
        .with_hints(&[
            "Is String on the stack?",
            "Does a 1 KB array ever reach the heap?",
        ]),
        SeedExercise::new(
            Some(8),
            ExerciseKind::Mcq,
            "A type implements `Copy` when duplicating it is cheap and what else?",
            &[
                "Safe — duplicating it cannot cause cleanup problems",
                "It is a primitive type",
                "It is stored on the stack",
                "It has no methods",
            ],
            0,
            "Cheap *and* safe. Cheap because copying must not be a hidden cost; safe because a copy \
             cannot lead to a double free. The second half is why `Copy` and `Drop` are mutually \
             exclusive, and why a type with cleanup work can never be `Copy`.",
        )
        .hints(&[
            "Both halves matter, and they exclude each other",
            "What would cleanup mean if there were two copies?",
        ]),

        // ── L10 Clone ───────────────────────────────────────────────────
        SeedExercise::new(
            Some(9),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — and what exists after?",
            &[
                "Yes, but s1 is empty afterwards",
                "Yes, and both are live: s1 still owns its buffer, s2 owns a separate one",
                "Yes, and both point at the same buffer",
                "No, because String does not implement Clone",
            ],
            1,
            "Both live, two genuinely separate buffers. `clone()` duplicates the contents, so the two \
             values are *equal* but not *identical* — mutating one cannot affect the other.",
        )
        .code(
            "fn main() {\n    let s1 = String::from(\"hello\");\n    let s2 = s1.clone();\n\n    println!(\"{s1}\");\n    println!(\"{s2}\");\n}",
        )
        .hints(&[
            "How many heap allocations does clone perform?",
            "Are the two Strings the same buffer?",
        ]),
        SeedExercise::new(
            Some(9),
            ExerciseKind::MemoryDiagram,
            "DRAW THE MEMORY — after `let s1 = String::from(\"hi\"); let s2 = s1.clone();`, how many \
             heap buffers exist and who owns them?",
            &[
                "One buffer, owned by s2 — s1 was consumed",
                "Two buffers, one owned by s1 and one by s2",
                "Two buffers, both owned by s2",
                "One buffer, owned jointly by s1 and s2",
            ],
            1,
            "Two buffers, two independent owners. That is the defining difference from a move: a move \
             leaves one buffer and one owner, a clone leaves two of each. It is also the only reason \
             both variables can be used at once.",
        )
        .code("let s1 = String::from(\"hi\");\nlet s2 = s1.clone();")
        .hints(&[
            "Count the allocations",
            "What does each variable own afterwards?",
        ]),
        SeedExercise::mcq(
            9,
            "Is every `.clone()` call necessarily a byte-for-byte deep copy of everything the type holds?",
            &[
                "Yes — that is what the Clone trait specifies",
                "No — Clone is a trait, and each implementation defines what duplication means",
                "Yes, for String specifically, but not for other types",
                "No — most clones are actually just moves",
            ],
            1,
            "`Clone` is a trait and each implementation decides what duplication means. For `String` it \
             does duplicate the owned bytes. For a type you write yourself it could share large data \
             behind a reference count. The precise phrasing: *clone produces another value the type \
             considers independent.*",
        )
        .with_hints(&[
            "Who decides what clone does for a given type?",
            "The type's own Clone implementation does.",
        ]),
        SeedExercise::new(
            Some(9),
            ExerciseKind::Mcq,
            "When is `clone()` the right call?",
            &[
                "Never — it is always a code smell",
                "Whenever you want two independent owners and the duplication cost is acceptable",
                "Whenever the compiler complains about a moved value",
                "Only for small types",
            ],
            1,
            "When you actually want two independent owners and can afford the cost. What is *not* fine \
             is using it reflexively because it makes the error go away, and then never returning to \
             decide whether ownership was really the right choice. Intention versus reflex.",
        )
        .with_hints(&[
            "One of these answers is about a decision, the others about a habit",
            "Think about a cache you keep after handing the original away.",
        ]),
        SeedExercise::new(
            Some(9),
            ExerciseKind::PredictOwnership,
            "PREDICT OWNERSHIP — after `let b = a.clone();` and end of scope, how many times is the \
             memory freed?",
            &[
                "Once — there is only one buffer",
                "Twice — one for each owned buffer",
                "Never — clones are never freed",
                "It depends on the string's length",
            ],
            1,
            "Twice, once for each buffer, by each owner. This is the reason clones are safe and moved \
             values are not: two owners means two *separate* things to clean up, not one thing cleaned \
             up twice.",
        )
        .code("let a = String::from(\"hi\");\nlet b = a.clone();")
        .hints(&[
            "How many buffers?",
            "How many owners, and who drops each?",
        ]),
        SeedExercise::new(
            Some(9),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — cloning a `&str`",
            &[
                "No — `&str` does not implement Clone",
                "Yes, and the result is a new heap allocation",
                "Yes, and it copies the pointer and length, not the text",
                "No — you must call to_string() first",
            ],
            2,
            "Yes, and it duplicates the *reference* — two pointers to the same text. Nothing was \
             allocated. It is a nice illustration that `clone()` clones the value, and for `&str` the \
             value is a pointer and a length. Use `to_string()` when you want a real owned copy.",
        )
        .code("fn main() {\n    let a = \"hello\";\n    let b = a.clone();\n    println!(\"{a} {b}\");\n}")
        .hints(&[
            "What is the value of a `&str`?",
            "Is the text owned by the reference?",
        ]),

        // ── L11 Move vs Copy vs Clone ───────────────────────────────────
        SeedExercise::new(
            Some(10),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — a tuple containing a String and an i32",
            &[
                "Yes — tuples are Copy when their contents are",
                "Yes — the i32 makes the whole tuple Copy",
                "No — the tuple contains a String, so it moves",
                "No — tuples cannot own heap data",
            ],
            2,
            "No — the tuple moves, because a `Copy` field does not make the whole tuple `Copy`. \
             `(String, i32)` is `Copy` only if *every* field is, and `String` is not. This is a good \
             moment to notice that types compose: ownership is a property of the whole, not of the \
             parts.",
        )
        .code("fn main() {\n    let t = (String::from(\"a\"), 1);\n    let u = t;\n    println!(\"{:?} {:?}\", t, u);\n}")
        .hints(&[
            "For a tuple to be Copy, what must be true of every field?",
            "Is String Copy?",
        ]),
        SeedExercise::new(
            Some(10),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — is the whole thing copied, moved, or cloned?",
            &[
                "Copied — every element is Copy, so the Vec is Copy",
                "Moved — a Vec is not Copy and there is no clone",
                "Cloned — Vecs clone on assignment",
                "Copied — because i32 is Copy",
            ],
            1,
            "Moved. The elements being `Copy` is irrelevant — the `Vec` itself is not `Copy`, so the \
             header transfers and the heap buffer stays put. `Vec<T>` is `Copy` only when `T` is `Copy`, \
             and even then the standard library declines, because a `Vec` always owns a heap buffer.",
        )
        .code("fn main() {\n    let v = vec![1, 2, 3];\n    let w = v;\n    println!(\"{w:?}\");\n}")
        .hints(&[
            "Ask whether `Vec<i32>` implements Copy, not whether `i32` does",
            "A Vec owns a heap buffer. Does that matter?",
        ]),
        SeedExercise::new(
            Some(10),
            ExerciseKind::PredictOwnership,
            "PREDICT OWNERSHIP — decide for each line: copied, moved, or cloned?",
            &[
                "(a) copied  (b) moved  (c) cloned",
                "(a) moved  (b) cloned  (c) copied",
                "(a) copied  (b) cloned  (c) moved",
                "(a) cloned  (b) moved  (c) copied",
            ],
            2,
            "`i32` is `Copy` so (a) copies. `Vec` is not `Copy` and has no `.clone()`, so (b) moves. \
             `String` with an explicit `.clone()` is (c) cloned. The decision procedure is mechanical: \
             check `Copy`, then look for `.clone()`, then it moved.",
        )
        .code("let x = 10;\nlet y = x;\n\nlet v = vec![1];\nlet w = v;\n\nlet s = String::from(\"a\");\nlet t = s.clone();")
        .hints(&[
            "Apply the three-question procedure to each line separately",
            "Copy is a property of the type, not the value.",
        ]),
        SeedExercise::new(
            Some(10),
            ExerciseKind::MemoryDiagram,
            "DRAW THE MEMORY — `let a = String::from(\"x\"); let b = a.clone();` versus \
             `let a = String::from(\"x\"); let b = a;`. How many buffers in each?",
            &[
                "Clone: 1 buffer. Move: 2 buffers",
                "Clone: 2 buffers. Move: 1 buffer",
                "Clone: 2 buffers. Move: 2 buffers",
                "Clone: 0 buffers. Move: 1 buffer",
            ],
            1,
            "Clone duplicates the allocation, so two buffers and two owners. Move transfers the \
             existing one, so one buffer and one owner. Everything else in this module — why double \
             free is impossible, why moves are cheap, why clones cost money — falls out of this \
             picture.",
        )
        .code("let a = String::from(\"x\");\nlet b = a.clone();   // two buffers\nlet a = String::from(\"x\");\nlet b = a;            // one buffer")
        .hints(&[
            "Count allocations in each",
            "A clone allocates. A move does not.",
        ]),
        SeedExercise::new(
            Some(10),
            ExerciseKind::Mcq,
            "Why is there no 'silent deep copy' option in the table of outcomes?",
            &[
                "Because the compiler does not optimise it away",
                "Because assignment of an owning value either copies (Copy) or transfers — duplication is \
                 always explicit",
                "Because it was removed for performance in a recent version",
                "Because it only applies to function calls, not assignment",
            ],
            1,
            "Because `let b = a;` has exactly two possible outcomes and the type decides which. If you \
             want duplication you write `.clone()`. Borrowing is not a third outcome — it is a way of \
             avoiding the question entirely, and it is next.",
        )
        .with_hints(&[
            "How many rows should the table have, and what decides which one?",
            "Duplication has a name. Use it.",
        ]),
        SeedExercise::new(
            Some(10),
            ExerciseKind::PredictCompile,
            "WILL IT COMPILE — the third case is the interesting one",
            &[
                "Yes — a move and a clone can be chained",
                "No — `a` was moved into `b`, so it cannot be cloned afterwards",
                "Yes — clone works on a moved value",
                "No — you cannot clone and move in the same program",
            ],
            1,
            "No. Once `a` has been moved there is no value left to clone — the binding is disarmed, so \
             there is nothing for `clone()` to duplicate. Order matters: clone first, then move, if you \
             want both.",
        )
        .code(
            "fn main() {\n    let a = String::from(\"x\");\n    let b = a;\n    let c = a.clone();\n    println!(\"{b} {c}\");\n}",
        )
        .hints(&[
            "Which line moves `a`?",
            "After that, is there still a value in `a` to clone?",
        ]),
    ]
}
