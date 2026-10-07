//! Exercises for Module 03 — Control Flow.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // ── L1 Thinking in control flow ──────────────────────────────
        SeedExercise::mcq(
            0,
            "Which of these is a decision, rather than a sequence or a repetition?",
            &[
                "Adding two numbers",
                "Printing the first ten items in a list",
                "Choosing a different message when health is zero",
                "Assigning a value to a variable",
            ],
            2,
            "Deciding *which* of several paths to take is control flow. Adding is a calculation, printing ten items is repetition, and assignment is neither.",
        ),
        SeedExercise::mcq(
            0,
            "Which three tools cover essentially all of Rust's control flow?",
            &[
                "if, loop, for",
                "if, match, loop",
                "match, break, continue",
                "while, for, range",
            ],
            1,
            "`if` for binary decisions, `match` for many cases, and the loop family (loop/while/for) for repetition. Everything else is a variation on those.",
        ),

        // ── L2 if ─────────────────────────────────────────────────────
        SeedExercise::mcq(
            1,
            "What type must an `if` condition produce?",
            &["i32", "bool", "String", "Option<bool>"],
            1,
            "Only `bool`. Rust will not treat a nonzero integer as true — a condition that reads as a number is almost always a bug waiting to happen.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    let logged_in = true;\n    let has_key = false;\n\n    if logged_in && has_key {\n        println!(\"in\");\n    }\n\n    println!(\"out\");\n}")
        .with_hints(&["Does the if body run?", "The last line is never inside the if"]),
        SeedExercise::mcq(
            1,
            "Why does `if 1 { }` fail to compile?",
            &[
                "Because 1 is not a keyword",
                "Because the condition must be a bool, and 1 is an integer",
                "Because if needs an else branch",
                "Because 1 is too small",
            ],
            1,
            "Rust requires an explicit bool. It will not guess that you meant 'non-zero is true', because that convention has caused a lot of bugs in other languages.",
        )
        .kind(ExerciseKind::FixCode)
        .with_code("fn main() {\n    if 1 {\n        println!(\"yes\");\n    }\n}")
        .with_hints(&["What type does the condition need to be?", "Write the comparison out"]),
        SeedExercise::mcq(
            1,
            "What does `if 3 > 2 { }` do?",
            &[
                "Fails — 3 > 2 is not allowed in an if",
                "Runs the block, because 3 > 2 evaluates to a bool",
                "Fails — only variables are allowed",
                "Runs the block twice",
            ],
            1,
            "The comparison itself produces a bool, which is exactly what an `if` wants. Rust is happy as long as the *result* of the condition is a bool.",
        ),

        // ── L3 else / else if ─────────────────────────────────────────
        SeedExercise::mcq(
            2,
            "In an `if / else if / else` chain, when does the chain stop?",
            &[
                "After the else if",
                "At the first branch whose condition is true",
                "At the end of the program",
                "It always runs every branch",
            ],
            1,
            "Branches are tested in order and the first true one wins. This is why you order them most-specific-first — a broad condition early will shadow everything after it.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    let temp = 45;\n\n    if temp > 40 {\n        println!(\"a\");\n    } else if temp > 30 {\n        println!(\"b\");\n    } else {\n        println!(\"c\");\n    }\n}")
        .with_hints(&["Which condition is checked first?", "45 > 40 is true"]),
        SeedExercise::mcq(
            2,
            "What is the bug in this chain?\n\nif score > 50 { \"pass\" }\nelse if score > 90 { \"excellent\" }",
            &[
                "Nothing, it is fine",
                "A score of 95 can never reach the \"excellent\" branch",
                "It will not compile",
                "The else is missing",
            ],
            1,
            "A score of 95 satisfies `score > 50` first, so the second branch is dead code. Order conditions from most specific to most general.",
        )
        .kind(ExerciseKind::Refactor)
        .with_code("// Put these branches in an order where every one is reachable")
        .with_hints(&["Check the most extreme condition first"]),
        SeedExercise::mcq(
            2,
            "What does `&&` mean and when does its right side get evaluated?",
            &[
                "Logical OR, right side always evaluated",
                "Logical AND, right side only if the left is true",
                "Logical AND, right side always evaluated",
                "Bitwise AND, always evaluated",
            ],
            1,
            "AND short-circuits: if the left is false, the right is never evaluated. That is occasionally a performance win and often a safety tool when the right would panic.",
        )
        .with_hints(&["Short-circuit evaluation"]),

        // ── L4 if as expression ───────────────────────────────────────
        SeedExercise::mcq(
            3,
            "Why must an `if` used for a value have an `else`?",
            &[
                "For readability",
                "Because otherwise the if might not produce any value at all",
                "Because Rust requires it in all cases",
                "To make the branches the same type",
            ],
            1,
            "If the condition is false and there is no `else`, the `if` has nothing to produce. The compiler will not guess a default for you.",
        ),
        SeedExercise::mcq(
            3,
            "What is the type of this expression?\n\nlet x = if flag { 10 } else { 20 };",
            &["()", "i32", "&str", "bool"],
            1,
            "Both branches produce an integer, so the whole `if` is an i32. With the `else` branch present, Rust infers the type from it.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    let flag = false;\n    let x = if flag { 10 } else { 20 };\n    println!(\"{}\", x);\n}")
        .with_hints(&["The condition is false, so which branch runs?"]),
        SeedExercise::mcq(
            3,
            "Why is this rejected?\n\nlet v = if c { 10 } else { \"ten\" };",
            &[
                "The branches produce different types — i32 and &str",
                "if expressions cannot hold numbers",
                "It needs a semicolon",
                "The else branch is too long",
            ],
            0,
            "An `if` used for a value has to produce one consistent type. The `else` branch set the expectation as `&str`, and the number does not match.",
        )
        .kind(ExerciseKind::FixCode)
        .with_code("fn main() {\n    let c = true;\n    let v = if c { 10 } else { \"ten\" };\n    println!(\"{}\", v);\n}")
        .with_hints(&["Both branches must be the same kind of value"]),
        SeedExercise::mcq(
            3,
            "What is the most common real use of an `if` expression?",
            &[
                "Printing different messages",
                "Choosing between two values to assign or return",
                "Looping until a condition holds",
                "Matching on an enum",
            ],
            1,
            "Assigning or returning one of several possible values. That is how you write a function that 'returns A or B depending on something' without a mutable flag variable.",
        ),

        // ── L5 match intro ────────────────────────────────────────────
        SeedExercise::mcq(
            4,
            "What does the `_` pattern do in a match?",
            &[
                "Matches any value and binds it to a variable named _",
                "Matches anything not covered by earlier arms, and binds nothing",
                "Ignores the match entirely",
                "Is required to be the first arm",
            ],
            1,
            "`_` is the catch-all. It matches whatever the arms above did not, and because it binds no name, you cannot use the value inside the arm.",
        )
        .kind(ExerciseKind::CompilerChallenge)
        .with_code("error[E0004]: non-exhaustive patterns: `i32` not covered\n --> src/main.rs:2:5\n  |\n2 |     match n {\n  |           ^ not covered\n  |           _ => println!(\"other\"),\n  |           ^^^^^^^^^^^^^^^^^^^ this pattern is unhandled")
        .with_hints(&["Rust wants every possible value handled", "What is the type of n?"]),
        SeedExercise::mcq(
            4,
            "Why does Rust require matches to be exhaustive?",
            &[
                "For performance",
                "So a value you forgot to handle is a compile error, not a silent fallthrough",
                "Because matches must have a fixed number of arms",
                "To make the code shorter",
            ],
            1,
            "This is the compiler refusing to let a bug ship. A missing case in an `else if` chain silently does nothing; a missing match arm does not compile.",
        ),
        SeedExercise::mcq(
            4,
            "Which is true of `_` versus a named binding in the final arm?",
            &[
                "They are identical",
                "`_` discards the value; `other` lets you use it in the arm body",
                "`other` is faster",
                "`_` can only be used in matches, not ifs",
            ],
            1,
            "Write `n => ...` when you need the value inside the arm. Write `_ => ...` when you genuinely do not care about it.",
        ),

        // ── L6 match values ───────────────────────────────────────────
        SeedExercise::mcq(
            5,
            "Which values does `1..=5` match in a pattern?",
            &["1, 2, 3, 4", "1, 2, 3, 4, 5", "2, 3, 4, 5", "0 through 5"],
            1,
            "The `=` makes it inclusive on both ends. `1..5` would be 1,2,3,4 — which is why ranges in patterns are almost always `..=`.",
        )
        .kind(ExerciseKind::SpotDifference)
        .with_code("match n {\n    1..=5 => println!(\"low\"),\n    _ => println!(\"high\"),\n}")
        .with_code_b("match n {\n    1..5 => println!(\"low\"),\n    _ => println!(\"high\"),\n}")
        .with_hints(&["One character difference changes which end is included"]),
        SeedExercise::mcq(
            5,
            "What does `1 | 2 | 3 => ...` mean?",
            &[
                "Bitwise or of 1, 2, 3",
                "Matches any of 1, 2, or 3",
                "Matches 1 through 3",
                "A syntax error",
            ],
            1,
            "In pattern position, `|` is an 'or' between alternative patterns. It matches any one of them, and the same code runs for each.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    let d = 4;\n    match d {\n        1 | 2 => println!(\"low\"),\n        3 | 4 => println!(\"mid\"),\n        5 | 6 => println!(\"high\"),\n        _ => println!(\"not a die\"),\n    }\n}")
        .with_hints(&["Which arm does 4 fall into?"]),
        SeedExercise::mcq(
            5,
            "What does `1 | 2..=5 | 9` match?",
            &["1, 2, 3, 4, 5, 9", "1 through 9", "Only 1, 2, and 9", "A syntax error"],
            0,
            "You can freely mix single-value alternatives with ranges in one arm. Each piece is a separate accepted pattern.",
        ),

        // ── L7 match enums ────────────────────────────────────────────
        SeedExercise::mcq(
            6,
            "In `Message::Text(text) => ...`, what is `text`?",
            &[
                "A comparison against the string 'text'",
                "The String carried by the Text variant, bound to that name",
                "A new field being created",
                "A match guard",
            ],
            1,
            "That is a binding pattern. It matches the Text variant and pulls its String out, so you can use it in the arm body. The compiler knows the type because the variant declares it.",
        )
        .kind(ExerciseKind::TypeDetective)
        .with_code("enum Message {\n    Text(String),\n    Number(i32),\n    Quit,\n}\n\nfn main() {\n    let m = Message::Number(42);\n    match m {\n        Message::Text(t) => println!(\"{}\", t),\n        Message::Number(n) => println!(\"{}\", n),\n        Message::Quit => println!(\"bye\"),\n    }\n}")
        .with_hints(&["What does the Number variant carry?", "The pattern binds it to a name"]),
        SeedExercise::mcq(
            6,
            "Why does matching all three variants of an enum need no `_` arm?",
            &[
                "Because enums are always exhaustive",
                "Because the compiler knows the complete set of variants from the type",
                "Because `_` is not allowed with enums",
                "Because the match is inside a loop",
            ],
            1,
            "This is exhaustiveness doing real work. The type tells Rust every possible value, so handling all of them satisfies the check without a catch-all.",
        ),
        SeedExercise::mcq(
            6,
            "What does the `Quit` variant look like in a match pattern?",
            &[
                "Quit() — empty parens required",
                "Message::Quit — no data, so nothing to bind",
                "Message::Quit(_) — a wildcard for the empty case",
                "It cannot be matched",
            ],
            1,
            "A variant with no data needs no binding. `Message::Quit => ...` is the whole pattern. Adding parens or a wildcard would be a compile error.",
        ),

        // ── L8 catch-all ──────────────────────────────────────────────
        SeedExercise::mcq(
            7,
            "In `Event::Paste(_) => println!(\"pasted something\")`, what does `_` do?",
            &[
                "Binds the pasted text to a name",
                "Matches the Paste variant and discards the String it carries",
                "Skips the whole arm",
                "Is a syntax error",
            ],
            1,
            "Inside a destructuring pattern, `_` ignores the data. Use it when you care that it was a Paste but not what was pasted.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    let v = 5;\n    match v {\n        0 => println!(\"zero\"),\n        n => println!(\"got {}\", n),\n    }\n}")
        .with_hints(&["The named binding is used inside the arm"]),
        SeedExercise::mcq(
            7,
            "Why can you not use the value from a `_` arm?",
            &[
                "Because `_` is a keyword",
                "Because `_` binds no name, so there is nothing to refer to",
                "Because the value was consumed",
                "Because Rust forbids it",
            ],
            1,
            "The underscore does not create a binding. That is the whole difference between `_` and a name like `other`.",
        ),
        SeedExercise::mcq(
            7,
            "What is a pattern, in one sentence?",
            &[
                "A comparison operator",
                "A description of the value or shape you are looking for",
                "A function that returns bool",
                "A type annotation",
            ],
            1,
            "Literals, wildcards, variant patterns, and tuples are all patterns. A pattern describes what you are looking for; matching asks whether a value fits.",
        ),

        // ── L9 match guards ───────────────────────────────────────────
        SeedExercise::mcq(
            8,
            "In `n if n > 10 => ...`, when is the arm's body run?",
            &[
                "Whenever the pattern matches, regardless of the guard",
                "When the pattern matches AND the guard is true",
                "Only when the guard is true",
                "When the pattern does not match",
            ],
            1,
            "The binding happens first, then the guard is evaluated, then the body runs. If the guard is false, matching continues to the next arm.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    let n = 15;\n    match n {\n        n if n > 10 => println!(\"big\"),\n        n if n < 0 => println!(\"negative\"),\n        n => println!(\"small {}\", n),\n    }\n}")
        .with_hints(&["15 > 10 is true", "The first matching arm wins"]),
        SeedExercise::mcq(
            8,
            "How is a guard different from a range pattern?",
            &[
                "They are identical",
                "A range is checked against the value; a guard runs arbitrary code",
                "A guard is faster",
                "Ranges cannot be used with numbers",
            ],
            1,
            "Rust can reason about whether a range covers the type. It cannot reason about arbitrary guard code, so a match that is exhaustive on patterns may still need a catch-all.",
        )
        .kind(ExerciseKind::FillGap)
        .with_code("let grade = match score {\n    n if n >= 90 => 'A',\n    n if n >= 80 => 'B',\n    ____\n};\nprintln!(\"{}\", grade);")
        .with_hints(&["The last arm should catch everything below 80"]),

        // ── L10 nested match ──────────────────────────────────────────
        SeedExercise::mcq(
            9,
            "In `(Color::Red, 0) => ...`, how many values must match for this arm to run?",
            &[
                "One",
                "Two — the colour and the number",
                "Three",
                "It matches everything",
            ],
            1,
            "A tuple pattern matches position by position. Each position can have its own pattern, including a literal like `0`.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("enum C { R, G }\n\nfn main() {\n    let p = (C::R, 5);\n    match p {\n        (C::R, 0) => println!(\"dark red\"),\n        (C::R, n) => println!(\"red {}\", n),\n        (_, 0) => println!(\"pure\"),\n        _ => println!(\"other\"),\n    }\n}")
        .with_hints(&["Which arm matches (Red, 5)?"]),
        SeedExercise::mcq(
            9,
            "What does `..` mean in a struct-like pattern?",
            &[
                "Range operator",
                "Match any remaining fields and ignore them",
                "Required field",
                "A rest pattern that is invalid",
            ],
            1,
            "`Move { from, .. }` matches a variant with named fields, binds `from`, and ignores everything else. Useful when a variant has more data than you care about.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("enum A {\n    Move { from: u8, to: u8, speed: u8 },\n    Pass,\n}\n\nfn main() {\n    let a = A::Move { from: 1, to: 2, speed: 9 };\n    match a {\n        A::Move { from, .. } => println!(\"from {}\", from),\n        A::Pass => println!(\"pass\"),\n    }\n}")
        .with_hints(&["The `..` ignores to and speed"]),

        // ── L11 loop ──────────────────────────────────────────────────
        SeedExercise::mcq(
            10,
            "Why is `loop { }` with no `break` described as 'infinite'?",
            &[
                "Because it is slow",
                "Because `loop` repeats until broken, and there is no break here",
                "Because it never compiles",
                "Because it allocates memory",
            ],
            1,
            "`loop` is defined as infinite. The only exit is `break`. Without one, the program runs until you interrupt it.",
        )
        .kind(ExerciseKind::FixCode)
        .with_code("fn main() {\n    let mut n = 0;\n\n    loop {\n        n += 1;\n        // the loop never stops\n    }\n}")
        .with_hints(&["What makes a `loop` stop?", "Check n against a limit"]),
        SeedExercise::mcq(
            10,
            "When is `loop` a better fit than `while`?",
            &[
                "Always",
                "When the stopping condition is discovered inside the loop body",
                "When the loop runs zero times",
                "When you need an index",
            ],
            1,
            "`while` checks its condition before each iteration, so the condition must be known up front. `loop` exits on a `break`, which can be anywhere.",
        ),
        SeedExercise::mcq(
            10,
            "What makes a `while` loop run forever?",
            &[
                "Using `continue`",
                "The body never changing the variable the condition reads",
                "A missing `else`",
                "Using `mut`",
            ],
            1,
            "The condition is re-evaluated every iteration. If nothing inside the body changes what it reads, it stays true forever. `continue` can also cause this if it skips the update.",
        ),

        // ── L12 break / continue ──────────────────────────────────────
        SeedExercise::mcq(
            11,
            "What does `continue` do?",
            &[
                "Stops the loop entirely",
                "Skips the rest of this iteration and starts the next",
                "Restarts the loop from the beginning",
                "Returns a value from the loop",
            ],
            1,
            "`continue` jumps to the next iteration. `break` is the one that stops the loop.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    let n = [1, 0, 3, 0, 5];\n    let mut sum = 0;\n    for v in n {\n        if v == 0 {\n            continue;\n        }\n        sum += v;\n    }\n    println!(\"{}\", sum);\n}")
        .with_hints(&["Which values get added?", "1 + 3 + 5"]),
        SeedExercise::mcq(
            11,
            "Why can `continue` cause an infinite `while` loop?",
            &[
                "Because `continue` is a macro",
                "If it skips the line that updates the loop's condition variable",
                "Because `while` does not support `continue`",
                "It cannot",
            ],
            1,
            "Classic trap: `while n < 5 { if skip { continue; } n += 1; }` never advances `n`. With `loop` this is impossible, since there is no pre-checked condition.",
        )
        .kind(ExerciseKind::SpotDifference)
        .with_code("while n < 5 {\n    if skip { continue; }\n    n += 1;\n}")
        .with_code_b("loop {\n    if n >= 5 { break; }\n    if skip { continue; }\n    n += 1;\n}")
        .with_hints(&["Which one can update n after the continue?"]),
        SeedExercise::mcq(
            11,
            "What happens if you use both `break` and `continue` in the same loop?",
            &[
                "It is a compile error",
                "It usually means the loop is doing two jobs and should be split",
                "The loop runs twice",
                "`break` is ignored",
            ],
            1,
            "Not an error, just a smell. Two keywords with opposite meanings in one loop body usually means the loop is doing two things. Splitting it makes each half obvious.",
        ),

        // ── L13 loop values ───────────────────────────────────────────
        SeedExercise::mcq(
            12,
            "What does `break 42;` do inside a `loop`?",
            &[
                "Breaks out and makes the whole loop evaluate to 42",
                "Breaks out and returns 42 from the function",
                "Panics",
                "Breaks out of the innermost `if`",
            ],
            0,
            "The value attaches to the `break` and becomes the value of the whole `loop` expression. That is why `let x = loop { break 42; };` works.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    let mut n = 0;\n    let result = loop {\n        n += 1;\n        if n == 3 {\n            break n * 10;\n        }\n    };\n    println!(\"{}\", result);\n}")
        .with_hints(&["What is n when the break runs?", "n * 10"]),
        SeedExercise::mcq(
            12,
            "Why must every exit path from a value-producing `loop` have a `break` with a value?",
            &[
                "For readability",
                "Because otherwise the loop might finish without producing anything",
                "Because Rust requires it syntactically",
                "It does not matter",
            ],
            1,
            "If the loop could fall out the bottom, the value would never be produced. The compiler checks every path. This turns a confusing runtime bug into a compile error.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    let nums = [1, 3, 8, 4];\n    let first_even = loop {\n        for n in nums {\n            if n % 2 == 0 {\n                break n;\n            }\n        }\n        break -1;\n    };\n    println!(\"{}\", first_even);\n}")
        .with_hints(&["The outer break is never reached here, but it is still required"]),

        // ── L14 while ─────────────────────────────────────────────────
        SeedExercise::mcq(
            13,
            "What is the difference between `while c { body }` and `loop { if !c { break; } body }`?",
            &[
                "They behave differently",
                "None — they are equivalent, but `while` states the intent more directly",
                "`while` is faster",
                "`loop` cannot use a condition",
            ],
            1,
            "They compile to the same thing. Rust has `while` because writing the second form when you mean the first is noise.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    let mut c = 3;\n    while c > 0 {\n        print!(\"{} \", c);\n        c -= 1;\n    }\n    println!();\n}")
        .with_hints(&["c goes 3, 2, 1, then stops"]),
        SeedExercise::mcq(
            13,
            "A `while` loop with `continue` hangs. Why does the same code in a `loop` not hang?",
            &[
                "`loop` ignores `continue`",
                "Because `loop` has no pre-checked condition that the `continue` can skip",
                "Because `loop` is faster",
                "It would hang too",
            ],
            1,
            "With `while`, the condition is checked at the top, so skipping the update leaves it true forever. With `loop`, the exit is a `break`, and `continue` just moves to the next iteration of the same body — the update still runs.",
        )
        .kind(ExerciseKind::FixCode)
        .with_code("fn main() {\n    let mut n = 0;\n    while n < 5 {\n        if n == 2 {\n            n += 1;\n            continue;\n        }\n        println!(\"{}\", n);\n    }\n}")
        .with_hints(&["Does this actually hang, or is it fine? Trace it carefully."]),

        // ── L15 while let ─────────────────────────────────────────────
        SeedExercise::mcq(
            14,
            "What does `while let Some(x) = stack.pop()` do?",
            &[
                "Loops while the pop returns Some, binding it to x",
                "Loops forever",
                "Pops the whole stack at once",
                "Is a syntax error",
            ],
            0,
            "The pattern matches while pop returns `Some`. The moment it returns `None` the pattern stops matching and the loop ends — no manual emptiness check needed.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    let mut v = vec![1, 2, 3];\n    while let Some(top) = v.pop() {\n        print!(\"{} \", top);\n    }\n    println!();\n}")
        .with_hints(&["vec::pop returns from the end"]),
        SeedExercise::mcq(
            14,
            "When should you reach for `while let` rather than `while`?",
            &[
                "Whenever the loop is short",
                "When the condition is a pattern match, usually on an Option or Result",
                "When you need an index",
                "Never",
            ],
            1,
            "`while` takes a bool. `while let` takes a pattern. If your exit condition is 'this pattern stopped matching', `while let` is the right tool.",
        )
        .kind(ExerciseKind::FillGap)
        .with_code("fn main() {\n    let mut v = vec![10, 20, 30];\n    while let Some(____) = v.pop() {\n        println!(\"{}\", x);\n    }\n}")
        .with_hints(&["What do you call the value that comes out of pop?"]),

        // ── L16 for ───────────────────────────────────────────────────
        SeedExercise::mcq(
            15,
            "Why is `for` usually safer than an index-based `while` loop?",
            &[
                "It is faster",
                "It cannot forget to update a condition, run forever, or go out of bounds",
                "It uses less memory",
                "It compiles faster",
            ],
            1,
            "There is no counter to get wrong, no condition to forget to update, and no index to overrun. The sequence tells you when it is done.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    let names = [\"Gus\", \"Alex\", \"Sam\"];\n    for (i, name) in names.iter().enumerate() {\n        println!(\"{}: {}\", i, name);\n    }\n}")
        .with_hints(&["enumerate adds an index to a for loop"]),
        SeedExercise::mcq(
            15,
            "What does `for x in 1..=5` print?",
            &["1 2 3 4", "1 2 3 4 5", "0 1 2 3 4 5", "1 2 3 4 5 6"],
            1,
            "`1..=5` is inclusive on both ends. This is the first thing to check when a for loop prints one fewer or one more than you expected.",
        )
        .with_hints(&["..= means inclusive"]),

        // ── L17 ranges ────────────────────────────────────────────────
        SeedExercise::mcq(
            16,
            "What does `for n in 5..0` do?",
            &[
                "Counts down from 5 to 0",
                "Prints nothing, because the range is empty",
                "Panics",
                "Prints 5 and then stops",
            ],
            1,
            "A range where start exceeds end is empty, not an error and not descending. For descending, use `(1..=5).rev()`.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    for n in 5..0 {\n        print!(\"{} \", n);\n    }\n    println!(\"done\");\n}")
        .with_hints(&["Can a range go backwards?"]),
        SeedExercise::mcq(
            16,
            "How do you iterate from 5 down to 1?",
            &["`for n in 5..1`", "`for n in (1..=5).rev()`", "`for n in 5..=1`", "`for n in 1..5`"],
            1,
            "Build a forward range and call `.rev()`. A descending range literal does not work in Rust.",
        )
        .kind(ExerciseKind::FillGap)
        .with_code("fn main() {\n    for n in (1..=10).____(3) {\n        println!(\"{}\", n);\n    }\n}")
        .with_hints(&["You want to take every third number", "There is a method for stepping"]),
        SeedExercise::mcq(
            16,
            "What does `0..5` include?",
            &["0 through 5", "0 through 4", "1 through 5", "5 only"],
            1,
            "Ranges are half-open by default: the start is included, the end is not. `..=` opts into including the end.",
        ),

        // ── L18 collections ───────────────────────────────────────────
        SeedExercise::mcq(
            17,
            "What does `for x in &items` do differently from `for x in items`?",
            &[
                "Nothing",
                "It borrows instead of taking ownership, so `items` is usable afterwards",
                "It iterates in reverse",
                "It only reads the first element",
            ],
            1,
            "Without the `&`, the collection is consumed and emptied. With it, you walk the values and leave the collection intact.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    let mut v = vec![1, 2, 3];\n    for x in &mut v {\n        *x *= 2;\n    }\n    println!(\"{:?}\", v);\n}")
        .with_hints(&["&mut gives mutable access to each element", "The star dereferences"]),
        SeedExercise::mcq(
            17,
            "Why does `*x += 5` need the star?",
            &[
                "For style",
                "Because iterating `&mut` gives a reference, and you must dereference to modify the value behind it",
                "Because the compiler requires it",
                "It does not need it",
            ],
            1,
            "With `for x in &mut v`, `x` is a `&mut i32`. The star dereferences it to reach the actual `i32` to change.",
        )
        .kind(ExerciseKind::CompilerChallenge)
        .with_code("error[E0308]: mismatched types\n --> src/main.rs:3:20\n  |\n3 |     for x in &mut v {\n  |                   ^ expected `i32`, found `&mut {integer}`")
        .with_hints(&["x is a reference, not a value", "How do you get to the thing behind a reference?"]),
        SeedExercise::mcq(
            17,
            "Which is equivalent to `for x in &coll`?",
            &["`for x in coll`", "`for x in coll.iter()`", "`for x in coll.iter_mut()`", "`for x in coll.into_iter()`"],
            1,
            "The borrow checker treats `&coll` and `coll.iter()` identically. `.iter_mut()` is the mutable version, `.into_iter()` consumes.",
        ),

        // ── L19 ownership while looping ───────────────────────────────
        SeedExercise::mcq(
            18,
            "Why does this fail?\n\nlet names = vec![String::from(\"a\")];\nfor n in names { }\nprintln!(\"{:?}\", names);",
            &[
                "names is empty after the loop",
                "The loop moved `names`, so using it afterwards is an error",
                "The loop modified the strings",
                "It compiles fine",
            ],
            1,
            "Iterating without `&` takes ownership. The vector is gone. Adding `&` fixes it.",
        )
        .kind(ExerciseKind::FixCode)
        .with_code("fn main() {\n    let names = vec![String::from(\"Gus\"), String::from(\"Alex\")];\n\n    for name in names {\n        println!(\"{}\", name);\n    }\n\n    println!(\"{:?}\", names);\n}")
        .with_hints(&["The error is `borrow of moved value`", "One character fixes it"]),
        SeedExercise::mcq(
            18,
            "Why does the same loop over `Vec<i32>` compile without any `&`?",
            &[
                "Integers are special-cased",
                "Because i32 is Copy — cheap to duplicate, so it is copied rather than moved",
                "Because the compiler is lenient with numbers",
                "It does not compile either",
            ],
            1,
            "Small integers are `Copy`, so giving one away is a silent copy. `String` owns heap memory, so giving one away is a real move. That distinction is the seed of the ownership module.",
        ),

        // ── L20 nested loops + labels ─────────────────────────────────
        SeedExercise::mcq(
            19,
            "In nested loops, when does the inner loop advance?",
            &[
                "After every outer iteration",
                "After it runs through all of its own iterations",
                "They run in parallel",
                "It never advances",
            ],
            1,
            "The inner loop completes entirely before the outer loop moves to its next value. That is what makes nested loops produce grids.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    'outer: loop {\n        for n in 0..3 {\n            if n == 2 {\n                break 'outer;\n            }\n            print!(\"{} \", n);\n        }\n    }\n}")
        .with_hints(&["What does the label let you break out of?"]),
        SeedExercise::mcq(
            19,
            "What does `break 'outer;` do inside nested loops?",
            &[
                "Breaks only the inner loop",
                "Leaves both loops, because 'outer names the one to exit",
                "Breaks the program",
                "Is only valid on a for loop",
            ],
            1,
            "The label names which loop to leave. Without it, a plain `break` only exits the innermost.",
        )
        .kind(ExerciseKind::FixCode)
        .with_code("fn main() {\n    'outer: for r in 1..=3 {\n        for c in 1..=3 {\n            if r * c > 6 {\n                break 'outer;\n            }\n        }\n    }\n}")
        .with_hints(&["Rust cannot break out of a for loop", "You need a loop for labelled breaks"]),

        // ── L21 blocks ────────────────────────────────────────────────
        SeedExercise::mcq(
            20,
            "What does a block produce as its value?",
            &[
                "The first statement",
                "The final expression, if it has no trailing semicolon",
                "The last statement, always",
                "Nothing",
            ],
            1,
            "The final expression without a semicolon is the block's value. Add the semicolon and the block produces `()` — nothing.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    let x = {\n        let a = 10;\n        let b = 20;\n        a + b\n    };\n    println!(\"{}\", x);\n}")
        .with_hints(&["The last line has no semicolon"]),
        SeedExercise::mcq(
            20,
            "Why do idiomatic Rust functions rarely use `return`?",
            &[
                "`return` is deprecated",
                "Because the function body is a block, and its final expression is the return value",
                "Because it is slower",
                "It is a style choice with no technical reason",
            ],
            1,
            "A function body is a block. If the last line is an expression with no semicolon, that is what the function returns.",
        )
        .kind(ExerciseKind::FillGap)
        .with_code("fn main() {\n    let result = {\n        let x = 10;\n        let y = 20;\n\n        x ____ y\n    };\n\n    println!(\"{}\", result);\n}")
        .with_hints(&["Both are numbers", "One character, and it changes the type of the block"]),

        // ── L22 expressions vs statements ─────────────────────────────
        SeedExercise::mcq(
            21,
            "What is the type of `{ 5; }`?",
            &["i32", "()", "bool", "&str"],
            1,
            "The semicolon makes `5;` a statement whose value is discarded, so the block yields `()` — the empty type.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    let a = { 5 };\n    let b = { 5; };\n    println!(\"{}\", a);\n    // println!(\"{}\", b);\n}")
        .with_hints(&["One of these will not compile"]),
        SeedExercise::mcq(
            21,
            "What does the compiler's phrase \"this expression has type `()`\" mean?",
            &[
                "There is a syntax error",
                "That expression produced no value, and something wanted one",
                "The expression is empty",
                "It means null",
            ],
            1,
            "`()` is the type of 'nothing'. When you see it in an error, look at where the caret points — it is almost always a semicolon on an expression you wanted to keep.",
        )
        .kind(ExerciseKind::FixCode)
        .with_code("fn main() {\n    let v = { 10 + 5; };\n    println!(\"{}\", v);\n}")
        .with_hints(&["What is the block producing?", "Delete one character"]),
        SeedExercise::mcq(
            21,
            "Which of these is a statement rather than an expression?",
            &[
                "`5`",
                "`x + 1`",
                "`if c { 1 } else { 2 }`",
                "`let x = 5;`",
            ],
            3,
            "A `let` with a semicolon does the work and produces nothing. The other three all evaluate to values.",
        ),

        // ── L23 Option ────────────────────────────────────────────────
        SeedExercise::mcq(
            22,
            "What are the two possible shapes of `Option<T>`?",
            &["`Some(t)` and `None`", "`Ok(t)` and `Err(e)`", "`true` and `false`", "`Left` and `Right`"],
            0,
            "Either there is a value, or there is not. There is no third state, which is the entire point — absence is part of the type, not a surprise.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    let name: Option<String> = Some(String::from(\"Gus\"));\n\n    if let Some(n) = name {\n        println!(\"hi {}\", n);\n    }\n\n    let none: Option<String> = None;\n    if let Some(n) = none {\n        println!(\"hi {}\", n);\n    }\n}")
        .with_hints(&["Only one of these has a value"]),
        SeedExercise::mcq(
            22,
            "When should you use `if let` instead of `match` on an Option?",
            &[
                "Never — match is always better",
                "When one case is interesting and the other is trivial or can be an empty else",
                "When you need to modify the value",
                "When there are more than two cases",
            ],
            1,
            "`if let` is shorter when you only care about the `Some` case. If both cases do real work, use `match` — the exhaustiveness check is worth having.",
        )
        .kind(ExerciseKind::SpotDifference)
        .with_code("match name {\n    Some(n) => println!(\"{}\", n),\n    None => println!(\"none\"),\n}")
        .with_code_b("if let Some(n) = name {\n    println!(\"{}\", n);\n}")
        .with_hints(&["One is exhaustive, the other is not"]),

        // ── L24 Result ────────────────────────────────────────────────
        SeedExercise::mcq(
            23,
            "What does `input.parse::<i32>()` return?",
            &["An i32", "A Result<i32, ParseIntError>", "An Option<i32>", "A String"],
            1,
            "A `Result` — either the parsed number, or an error explaining why parsing failed.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    match \"42\".parse::<i32>() {\n        Ok(n) => println!(\"ok {}\", n),\n        Err(e) => println!(\"err {}\", e),\n    }\n\n    match \"abc\".parse::<i32>() {\n        Ok(n) => println!(\"ok {}\", n),\n        Err(e) => println!(\"err {}\", e),\n    }\n}")
        .with_hints(&["One of these strings is a number"]),
        SeedExercise::mcq(
            23,
            "In `Ok(n) if n >= 0 => ...`, what does the guard add?",
            &[
                "Nothing — it is decoration",
                "An extra condition that must also hold for the arm to run",
                "A type conversion",
                "It makes the arm exhaustive",
            ],
            1,
            "The pattern matches Ok, binds n, then the guard checks n is non-negative. Both must hold for the arm to run.",
        )
        .kind(ExerciseKind::FillGap)
        .with_code("fn main() {\n    let r: Result<i32, String> = Ok(5);\n\n    match r {\n        Ok(n) if n > 3 => println!(\"big\"),\n        Ok(n) => println!(\"small {}\", n),\n        ____ => println!(\"failed\"),\n    }\n}")
        .with_hints(&["The other variant of Result", "It carries an error value"]),

        // ── L25 putting it together ───────────────────────────────────
        SeedExercise::mcq(
            24,
            "You need to sum only the even numbers in a Vec, and leave the Vec usable afterwards. What do you write?",
            &[
                "`for x in v { if x % 2 == 0 { sum += x; } }`",
                "`for x in &v { if x % 2 == 0 { sum += x; } }`",
                "`let mut sum = 0; for x in &mut v { ... }`",
                "`while let Some(x) = v.pop() { ... }`",
            ],
            1,
            "`for x in &v` borrows, so the vector survives the loop. The first option consumes it. The third borrows mutably and would not compile without `*x`.",
        )
        .kind(ExerciseKind::Refactor)
        .with_code("// Convert this to a match expression that produces a &str\nfn grade(n: u32) -> &'static str {\n    if n >= 90 { \"A\" } else if n >= 80 { \"B\" } else { \"F\" }\n}")
        .with_hints(&["A match with guards does the same job flatly"]),
        SeedExercise::mcq(
            24,
            "Why does this need `&`?\n\nfor name in &names {\n    println!(\"{}\", name);\n}\nprintln!(\"{:?}\", names);",
            &[
                "To make it faster",
                "Without `&` the loop would move `names` and the final println would be using a moved value",
                "To print the length",
                "It does not need it — it would work either way",
            ],
            1,
            "Borrowing keeps the collection alive for the final println. Consuming it in the loop would leave nothing to print.",
        ),
        SeedExercise::mcq(
            24,
            "Which tool would you reach for to search a collection and return the first match?",
            &[
                "`for` with a `break` — but that only exits the loop, not with a value",
                "`loop` with `break value`",
                "`while` with a flag variable",
                "`match` with a guard",
            ],
            1,
            "`loop { ... break Some(x); }` is the idiomatic search-and-return shape. It is exactly why loops can produce values.",
        ),

        // ── Reference answers for earlier lessons ─────────────────────
        SeedExercise::mcq(
            3,
            "Complete the discount: a member gets 20, everyone else gets 0.\n\nlet discount = if is_member { 20 } else { ____ };",
            &["0", "0u8", "none"],
            0,
            "Both branches must be the same type, and the `if` branch is an integer literal, so 0 is right.",
        )
        .kind(ExerciseKind::FillGap)
        .with_code("fn main() {\n    let is_member = false;\n\n    let discount = if is_member {\n        20\n    } else {\n        ____\n    };\n\n    println!(\"{}% off\", discount);\n}")
        .with_hints(&["Both branches need to be the same type"]),

        SeedExercise::mcq(
            9,
            "What does `_` mean inside a tuple pattern like `(_, 0)`?",
            &[
                "Nothing special",
                "The first position can be anything; the second must be exactly 0",
                "Both positions are ignored",
                "It is invalid inside tuples",
            ],
            1,
            "A wildcard works at any position in a pattern. The rest of the tuple still has to match its own patterns.",
        )
        .kind(ExerciseKind::Refactor)
        .with_code("// Flatten this into a single match with two arms\nmatch (a, b) {\n    (0, 0) => println!(\"both zero\"),\n    (0, _) => println!(\"a is zero\"),\n    (_, 0) => println!(\"b is zero\"),\n    _ => println!(\"neither\"),\n}")
        .with_hints(&["You can write a pattern that captures both into one arm"]),

        SeedExercise::mcq(
            12,
            "A function returns `Result<i32, String>`. What must you do before using the value?",
            &[
                "Nothing — it is just an i32 with extra steps",
                "Handle it with match or `?` — the compiler forces you to look at both cases",
                "Call `.unwrap()` — always safe",
                "Convert it to Option",
            ],
            1,
            "This is the whole point. The type says the operation can fail, and the compiler will not let you use the value without acknowledging the failure.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn parse(s: &str) -> Result<i32, String> {\n    s.parse::<i32>().map_err(|e| e.to_string())\n}\n\nfn main() {\n    match parse(\"7\") {\n        Ok(n) => println!(\"got {}\", n),\n        Err(e) => println!(\"bad: {}\", e),\n    }\n}")
        .with_hints(&["\"7\" parses fine"]),

        SeedExercise {
            lesson_index: None,
            kind: ExerciseKind::BuildIt,
            prompt: "BUILD IT — A health-bar system that reports its own state.".to_string(),
            code: None,
            code_b: None,
            options: vec![
                "Store a current health value and a maximum, both as u32".to_string(),
                "Write a `describe` function that returns a &str describing the health band using a match with ranges: critical (0-20), low (21-50), healthy (51-100), full (above the max)".to_string(),
                "Write an `apply_damage` function that takes current and amount, returns None if the amount is zero, and otherwise returns the reduced value — never below zero".to_string(),
                "Write a `heal` function that takes current, max, and amount, and never returns above the max".to_string(),
                "Call them in sequence to simulate: take 30 damage, heal 10, then report the final state".to_string(),
            ],
            correct_answer: 0,
            explanation: "There is one important subtlety in apply_damage: if amount is zero you cannot \
subtract, and if the damage exceeds current health you would underflow a u32 and panic in debug. \
Returning Option forces the caller to decide what a zero-damage hit means, and clamping is a real design \
decision rather than an afterthought."
                .to_string(),
            xp_reward: 60,
            hints: vec![
                "Match ranges on health bands is a natural fit — you learned ranges in patterns for this.".to_string(),
                "A u32 underflow panics in debug builds. That is what the Option return is protecting you from.".to_string(),
                "Try writing apply_damage without the guard and pass amount = 0. See what happens.".to_string(),
            ],
        },
    ]
}
