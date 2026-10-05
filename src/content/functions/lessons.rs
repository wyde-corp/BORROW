//! Lessons for Module 04 — Functions.

use crate::content::types::{
    breakdown, broken, broken_text, callout, checklist, code, diagram, fixed, labelled, paragraphs,
    steps, table, text_block, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        why_functions_exist(),
        defining_calling(),
        reading_a_signature(),
        parameters_arguments(),
        return_values(),
        unit_type(),
        expressions_inside(),
        explicit_return(),
        scope(),
        ownership_across_calls(),
        mutable_parameters(),
        functions_calling_functions(),
        designing_good_functions(),
        recursion(),
        things_you_will_break(),
        putting_it_together(),
    ]
}

fn why_functions_exist() -> SeedLesson {
    SeedLesson::new(
        "Why Functions Exist",
        vec![
            labelled(
                "CONCEPT",
                "A function is basically you admitting that this piece of logic deserves a name. That is \
                 the whole idea. Everything else is detail.",
            ),
            text_block(
                "Here is a program that works. It is also the kind of program you stop being able to work \
                 with after about forty more lines.",
            ),
            code(
                "rust",
                "fn main() {\n    // get user input\n    let raw = \"42\";\n\n    // validate input\n    let value = match raw.parse::<i32>() {\n        Ok(n) => n,\n        Err(_) => { println!(\"bad input\"); return; }\n    };\n\n    // calculate result\n    let doubled = value * 2;\n    let taxed = doubled as f64 * 1.2;\n\n    // format result\n    let message = format!(\"Result: {}\", taxed);\n\n    // print result\n    println!(\"{}\", message);\n}",
            ),
            labelled(
                "EXPLANATION",
                "Every one of those comments marks a job. Get input. Validate it. Calculate. Format. \
                 Print. Five distinct behaviours, tangled together, sharing variables in a scope that keeps \
                 growing. To change the tax rate you have to read past the validation logic. To reuse the \
                 calculation you have to copy it out. And none of it has a name.",
            ),
            diagram(
                "the same program, named",
                "  main()\n   │\n   ├── get_input()      → 42\n   ├── validate_input()  → true\n   ├── calculate_total() → 100.8\n   └── print_result()    → prints it\n\n  each piece has a name you can use",
            ),
            text_block(
                "Now change the tax rate. You edit one function, not a paragraph. Now reuse the calculation \
                 somewhere else. You call it. Now want to prove the validation works. You test one \
                 function, not a program that also does six other things.",
            ),
            table(
                &["What functions give you", "What it looks like in practice"],
                &[
                    &["A name for behaviour", "calculate_total() says what it does"],
                    &["Reuse without copying", "call it from three places"],
                    &["Decomposition", "five small problems instead of one big one"],
                    &["Isolation", "a bug in printing does not touch validation"],
                    &["Testability", "feed one function one input, check one output"],
                ],
            ),
            callout(
                "A good function does not merely save lines of code",
                "It gives a piece of behaviour a meaningful name. `calculate_total()` is not shorter than \
                 the three lines it replaced. It is *clearer*, and that is the trade you want.",
            ),
            callout(
                "The naming test",
                "If you cannot name a function without reaching for a verb like `do_stuff` or `handle_thing`, \
                 the function is doing too much. That is not a style complaint — an unnameable function is \
                 usually several functions pretending to be one.",
            ),
            callout(
                "Practice — Try this now",
                "Take the monolith above and mark every distinct job with a comment. Count the jobs. Then \
                 write a function name for each one. Do not write the function bodies yet — just the names, \
                 in a comment block. This is the first half of decomposition and it is the harder half.",
            ),
            callout(
                "Mini challenge",
                "Write a `main` that prints a receipt. It needs to look up an item's price, work out a \
                 discount based on quantity, and format the total. Leave it as one block. Then write the \
                 three function names you would extract. Read your two lists side by side and notice which \
                 one is easier to hand to someone else.",
            ),
        ],
    )
}

fn defining_calling() -> SeedLesson {
    SeedLesson::new(
        "Defining and Calling Functions",
        vec![
            labelled(
                "CONCEPT",
                "Defining a function tells Rust what the name means. Calling it does the thing. Keeping \
                 those two apart is the only thing this lesson is about.",
            ),
            breakdown(
                "fn greet() — the smallest useful function",
                "fn greet() {\n    println!(\"Hello!\");\n}",
                &[
                    "`fn` — the keyword that says a function definition starts here",
                    "`greet` — the name you choose. This is the part that matters; the rest is structure.",
                    "`()` — the parameter list. Empty means it takes no input.",
                    "`{ }` — the body. Everything between the braces is what the function does.",
                    "Defining it does **not** run it. It writes the name into the program's vocabulary.",
                ],
            ),
            text_block(
                "To actually run it you call it by name, with parentheses and a semicolon:",
            ),
            code(
                "rust",
                "fn greet() {\n    println!(\"Hello!\");\n}\n\nfn main() {\n    greet();\n    greet();\n}",
            ),
            diagram(
                "define vs call",
                "  DEFINE                CALL\n  \"here is what          \"do that thing\n   this does\"            now\"\n\n  repeated freely        runs, then control\n                         returns to the line\n                         after the call",
            ),
            labelled(
                "EXPLANATION",
                "The semicolon on `greet();` matters for a reason you already know from the control flow \
                 module. A bare `greet` is an expression that produces a value — here, the unit type — and a \
                 `greet()` followed by a semicolon is a statement that throws that value away. For a function \
                 that does nothing useful, either is fine, but the semicolon is what you almost always want.",
            ),
            callout(
                "Congratulations. You have written a function that does absolutely nothing useful.",
                "That is genuinely fine. Every programmer starts here. `fn main` itself is a function that \
                 currently does nothing but house other functions.",
            ),
            code(
                "rust",
                "fn beep() {\n    println!(\"beep\");\n}\n\nfn main() {\n    beep();\n    beep();\n    beep();\n}\n\n// prints:\n// beep\n// beep\n// beep",
            ),
            text_block(
                "Three calls, three lines of output. If you had to write `println!(\"beep\")` three times you \
                 would get the same output — but if the beep needed to change, you would have to find all \
                 three. That is the maintenance argument, and it is the one that keeps paying.",
            ),
            callout(
                "Functions can live anywhere",
                "In Rust you do not have to define a function before you call it, or above it, or in a \
                 particular order. The compiler collects them all first. That is unusual enough that people \
                 assume otherwise — try defining `main` at the bottom of the file and watch it still work.",
            ),
            callout(
                "Practice — Try this now",
                "Write `fn beep()` and call it five times. Then change the function body to print something \
                 different and run it again — one edit, five changed outputs. Then move `fn main` to the \
                 bottom of the file and confirm it still runs.",
            ),
            callout(
                "Mini challenge",
                "Write `fn line()` that prints a horizontal rule, and `fn heading(text: &str)` that prints \
                 the text between two rules. Then write `fn main` that prints a title and three sections, \
                 using those two functions. Count how many times you could change the rule character by \
                 editing one line.",
            ),
        ],
    )
}

fn reading_a_signature() -> SeedLesson {
    SeedLesson::new(
        "Reading a Function Signature",
        vec![
            labelled(
                "CONCEPT",
                "This is the single most valuable skill in this module. If you can read a signature, you \
                 can use a function you have never seen. If you cannot, every function in the standard \
                 library is a wall of noise.",
            ),
            labelled(
                "EXPLANATION",
                "A signature is a label on a jar. You have all labelled jars in your kitchen without reading \
                 the recipe inside. `flour, 500g` tells you what is in there and whether you can use it, \
                 without telling you how to bake. A Rust signature is exactly that: the name says what the \
                 thing is called, the parentheses say what goes in, and the arrow says what comes out. That \
                 is the whole contract. Everything after this lesson is detail on how to write one that \
                 keeps its promise.",
            ),
            diagram(
                "the shape of a signature",
                "  fn calculate_price(price: f64, quantity: u32) -> f64\n     │\n     │          │          │              │\n     │          │          │              └── what it produces\n     │          │          └── second parameter, u32\n     │          └── first parameter, f64\n     └── the function's name",
            ),
            breakdown(
                "fn add(a: i32, b: i32) -> i32",
                "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}",
                &[
                    "`fn` — tells Rust we are defining a function",
                    "`add` — the function's name. You call it as `add(10, 20)`.",
                    "`a: i32` — a parameter named `a`, and it must be an i32",
                    "`b: i32` — a second parameter, also i32. Commas separate parameters.",
                    "`-> i32` — the function promises to produce an i32",
                    "`a + b` — the final expression, with no semicolon, and that is what it produces",
                ],
            ),
            text_block(
                "Read that breakdown once more. Every Rust function signature is exactly this shape: a name, \
                 zero or more named parameters each with a type, and optionally a return type after the arrow. \
                 Nothing else is happening. The curly braces and the body are a separate concern — the \
                 signature is the contract, the body is the implementation.",
            ),
            table(
                &["Signature", "Read it as"],
                &[
                    &["fn greet()", "takes nothing, produces nothing useful"],
                    &["fn greet(name: &str)", "takes a name, produces nothing useful"],
                    &["fn calculate(x: i32, y: i32) -> i32", "takes two i32s, produces an i32"],
                    &["fn find_user(id: u64) -> Option<String>", "takes a u64, produces maybe-a-String"],
                    &["fn save(u: &User) -> Result<(), DbError>", "takes a borrowed User, produces success or an error"],
                ],
            ),
            callout(
                "You do not need to understand those last two yet",
                "`Option<String>` and `Result<(), DbError>` get their own modules. The point of showing them \
                 now is purely shape: the arrow says 'produces', and whatever follows the arrow is the type \
                 it produces. Recognise the shape now, understand it later. That is the curriculum working \
                 properly — not a gap.",
            ),
            code(
                "rust",
                "fn find_user(id: u64) -> Option<String> {\n    if id == 1 {\n        Some(String::from(\"Gus\"))   // this is the Some case\n    } else {\n        None                            // and this is the None case\n    }\n}\n\nfn main() {\n    match find_user(1) {\n        Some(name) => println!(\"found {}\", name),\n        None => println!(\"no such user\"),\n    }\n}",
            ),
            text_block(
                "Read the signature: *takes a u64, produces an Option of a String.* Then read the body: it \
                 produces `Some(\"Gus\")` for id 1 and `None` for anything else. That is the entire function. \
                 The `match` in `main` is not part of the function at all — it is the caller deciding what to \
                 do with whatever came back.",
            ),
            callout(
                "The arrow is a promise",
                "If the signature says `-> i32` and the body produces a string, the program does not compile. \
                 Rust checks that the function keeps its own contract, which means you can trust a signature \
                 you have read without reading the body.",
            ),
            callout(
                "Practice — Try this now",
                "Read these five signatures out loud, in words, without looking back at this lesson: \
                 `fn is_empty(s: &str) -> bool`, `fn split(s: &str) -> (&str, &str)`, \
                 `fn first_char(s: &str) -> Option<char>`, `fn area(w: f64, h: f64) -> f64`, \
                 `fn reset(state: &mut Vec<i32>)`. If you can do that, you have the skill.",
            ),
            callout(
                "Mini challenge",
                "Write four function signatures before writing a single body. `double(x: i32) -> ?`, \
                 `is_empty(s: &str) -> ?`, `average(scores: &[u32]) -> ?`, and `describe(n: i32) -> ?`. \
                 Fill in the return types yourself, then implement all four. Guessing the type first is \
                 genuinely how experienced Rust developers start a function.",
            ),
        ],
    )
}

fn parameters_arguments() -> SeedLesson {
    SeedLesson::new(
        "Parameters and Arguments",
        vec![
            labelled(
                "CONCEPT",
                "A function that cannot receive information cannot do anything interesting. Parameters \
                 are the named, typed slots in the signature. Arguments are what you put in them at the \
                 call site.",
            ),
            labelled(
                "EXPLANATION",
                "A parameter is a labelled box the function owns while it works. `greet(name: &str)` sets \
                 one box aside, labelled `name`, and the function is not allowed to look anywhere else for \
                 it. When you call `greet(\"Gus\")` you are deciding what goes in that box. The parameter \
                 is the box, the argument is the thing in it. Separating the two is what lets one function \
                 work for every caller, and it is why a function that reaches out into `main` for what it \
                 needs is a function nobody can reason about.",
            ),
            diagram(
                "the vocabulary",
                "  fn greet(name: &str) { }\n            │\n            └── parameter: the named slot,\n                declared with a type\n\n  greet(\"Gus\");\n        │\n        └── argument: the actual value\n            you supply at the call",
            ),
            code(
                "rust",
                "fn greet(name: &str) {\n    println!(\"Hello, {}\", name);\n}\n\nfn main() {\n    greet(\"Gus\");\n    greet(\"Alex\");\n}",
            ),
            text_block(
                "Multiple parameters work the same way. Order matters, because there are no labels at the \
                 call site — the compiler matches arguments to parameters purely by position.",
            ),
            code(
                "rust",
                "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n\nfn main() {\n    let sum = add(10, 20);   // a = 10, b = 20\n    println!(\"{}\", sum);   // 30\n\n    add(20, 10);   // same types, same result — order is invisible here\n}",
            ),
            callout(
                "Swap two same-typed arguments and the compiler cannot help you",
                "`add(10, 20)` and `add(20, 10)` are both perfectly valid and both give 30. The compiler has \
                 no opinion because there is nothing to be wrong about. When a function takes several \
                 values of the same type, give the function a name that says what each one is — or take a \
                 struct, which you will meet in module 07.",
            ),
            text_block(
                "Types must match, though, and the compiler is quite clear about it:",
            ),
            broken("fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n\nfn main() {\n    let sum = add(10, \"20\");\n    println!(\"{}\", sum);\n}"),
            code(
                "text",
                "error[E0308]: mismatched types\n  --> src/main.rs:8:23\n   |\n8  |     let sum = add(10, \"20\");\n   |                            ^^^^ expected `i32`, found `&str`\n   |                            \n   |                            arguments to this\n   |                            function call",
            ),
            text_block(
                "In plain terms: the second parameter is an `i32`, and you handed it a string. The arrow \
                 pointing at `\"20\"` is the compiler showing you exactly which argument is wrong, and the \
                 note at the bottom is telling you where that requirement came from. That second part is \
                 the useful one — it points back at the signature, which is the thing you would otherwise \
                 have to go hunting for.",
            ),
            callout(
                "How to read a mismatch error in a call",
                "Find the caret. It marks the offending argument. Then find the note — it names the parameter. \
                 The first number is which parameter position, so `arguments to this function call` with a \
                 `1` means the second one. You will not need to guess again.",
            ),
            callout(
                "Practice — Try this now",
                "For each call below, name the function, then say which argument binds to which parameter: \
                 `divide(12, 4)`, `greet(\"Gus\")`, `move_to(3.5, 7.0)`, `is_open(9, false)`. Then write \
                 the four signatures those calls imply.",
            ),
            callout(
                "Mini challenge",
                "Write `fn rectangle_area(width: f64, height: f64) -> f64` and `fn perimeter(w: f64, h: f64) \
                 -> f64`. Call both with the same two values, and compute the ratio of area to perimeter. \
                 Then call `rectangle_area` with the arguments reversed — the types still match, so the \
                 compiler will not stop you. Notice that this one is actually fine, and think about when \
                 the reversed call would have been a real bug.",
            ),
        ],
    )
}

fn return_values() -> SeedLesson {
    SeedLesson::new(
        "Return Values",
        vec![
            labelled(
                "CONCEPT",
                "So far your functions have printed things. That is a one-way street — the value goes to \
                 the screen and is gone. The arrow in a signature opens the other direction.",
            ),
            code(
                "rust",
                "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n\nfn main() {\n    let result = add(10, 20);\n    println!(\"{}\", result);   // 30\n}",
            ),
            diagram(
                "where the value goes",
                "  add(10, 20)\n     │\n     ▼\n  a + b  evaluates to 30\n     │\n     ▼\n  30 is returned out of the function\n     │\n     ▼\n  stored in result\n     │\n     ▼\n  you decide what to do with it next",
            ),
            labelled(
                "EXPLANATION",
                "There is no `return` keyword here, and that is not an oversight. A function's body is a \
                 block, and a block's value is its final expression when that expression has no semicolon. \
                 You met this in the control flow module. Functions just use it more.",
            ),
            table(
                &["", "No semicolon", "With semicolon"],
                &[
                    &["`a + b`", "is the block's value → returned", "is a statement → discarded"],
                    &["Function produces", "the i32 it promised", "the unit type ()"],
                    &["Compiles with `-> i32`?", "yes", "no — expected i32, found ()"],
                ],
            ),
            broken("fn add(a: i32, b: i32) -> i32 {\n    a + b;\n}"),
            code(
                "text",
                "error[E0308]: mismatched types\n --> src/main.rs:2:30\n  |\n2 |     a + b;\n  |        ^   expected `i32`, found `()`\n  |        |\n  |        this expression has type `()`\n  |\nnote: expected `i32`, found `()`\nhelp: if this is a `return` expression, remove the semicolon",
            ),
            text_block(
                "That is one of the friendliest errors Rust produces. It says what it expected, what it \
                 found, which characters are responsible, and — in the help line — exactly what to do. You \
                 have now seen this same error twice: once in the control flow module, once here. It is the \
                 single most common early stumble in Rust, and it is the one the compiler goes out of its \
                 way to make obvious.",
            ),
            callout(
                "Rust keeps repeating this idea: expressions produce values",
                "Blocks produce values. `if` produces values. `loop` produces values with `break value`. \
                 Function bodies are blocks, so they produce values too. This is not three separate rules — \
                 it is one rule you keep meeting in different clothes.",
            ),
            code(
                "rust",
                "fn classify(n: i32) -> &'static str {\n    if n < 0 {\n        \"negative\"\n    } else if n == 0 {\n        \"zero\"\n    } else {\n        \"positive\"\n    }\n}\n\nfn main() {\n    for n in [-5, 0, 12] {\n        println!(\"{} is {}\", n, classify(n));\n    }\n}",
            ),
            callout(
                "Practice — Try this now",
                "Write `fn square(x: i32) -> i32 { x * x }` and call it with 5, printing the result. Then add \
                 a semicolon to the body and read the error — you should be able to predict every word of it \
                 now. Then remove the return type and watch the error change shape.",
            ),
            callout(
                "Mini challenge",
                "Write `fn max_of(a: i32, b: i32) -> i32` that returns the larger of two numbers, using an \
                 `if` expression with no semicolon. Then write `fn clamped(n: i32, low: i32, high: i32) -> \
                 i32` that returns n, or low if n is too small, or high if n is too large — nested blocks, \
                 no `if` statements. Call both and print a table of inputs and outputs.",
            ),
        ],
    )
}

fn unit_type() -> SeedLesson {
    SeedLesson::new(
        "The Unit Type ()",
        vec![
            labelled(
                "CONCEPT",
                "A function that returns nothing still returns *something*. That something is `()`, called \
                 the unit type. It is Rust's way of saying 'this ran, and there is no value here'.",
            ),
            labelled(
                "EXPLANATION",
                "In most languages a function that gives you nothing back returns a null, an undefined, or \
                 nothing at all, and you have to guess which. Rust refuses the guess. Every function \
                 produces a value, always, and when there is no interesting value the value is `()` — the \
                 same idea as an empty present. It is why `let result = setup();` compiles and why \
                 `{:?}` on it prints `()`. It is a small idea that removes an entire category of confusion.",
            ),
            code(
                "rust",
                "fn greet(name: &str) {\n    println!(\"Hello, {}\", name);\n}\n\nfn main() {\n    let result = greet(\"Gus\");\n    println!(\"{:?}\", result);   // ()\n}",
            ),
            text_block(
                "`greet` has no `-> T` in its signature, which means it produces `()`. The `println!` \
                 inside it returns `()` too, and that `()` is the last expression, so it becomes the \
                 function's value. You have been receiving unit values since lesson 1 without noticing.",
            ),
            table(
                &["Produces", "Meaning", "Used for"],
                &[
                    &["nothing / no arrow", "the function does something with no result", "most functions"],
                    &["()", "explicitly nothing, an empty tuple", "when the signature must say it"],
                    &["()", "when something *did* happen but there is no value", "fns that print or mutate"],
                ],
            ),
            callout(
                "Why it exists at all",
                "So a signature can be honest about producing nothing. Without a unit type, a function with \
                 no return value would have no return type, and then you could not put it in a position \
                 where a value is required. The unit type makes 'returns nothing' a real, expressible, \
                 checkable thing rather than an absence.",
            ),
            code(
                "rust",
                "fn log_event(name: &str) -> () {\n    println!(\"event: {}\", name);\n}\n\nfn main() {\n    log_event(\"login\");\n    let nothing = log_event(\"logout\");\n    println!(\"still fine: {:?}\", nothing);\n}",
            ),
            callout(
                "You will see a stranger-looking () very soon",
                "The shape `Result<(), Error>` means 'this operation might fail, and if it succeeds it \
                 produced no value, just a confirmation.' It looks odd and it is not — the `()` is the \
                 success payload. We will meet it properly in the error handling module. For now, recognise \
                 it and move on.",
            ),
            text_block(
                "The practical rule: if a function's only job is to *do* something — print, mutate, write to \
                 a file — it produces `()`, and you should usually leave the arrow off the signature. If it \
                 produces something, put the arrow there. Do not write `-> ()` out of habit; it is noise.",
            ),
            callout(
                "Practice — Try this now",
                "Call three different functions and bind each result to a variable, then print all three \
                 with `{:?}`. All three will be `()`. Then write one function with an explicit `-> ()` and \
                 one without, and confirm both compile identically.",
            ),
            callout(
                "Mini challenge",
                "Write `fn assert_even(n: i32) -> ()` that prints a message if n is even and panics if not. \
                 Call it with 4, 6, and 7 — the last one will stop the program. Then write \
                 `fn is_even(n: i32) -> bool` that returns the answer instead, call it with all three, and \
                 print the results. Compare a function that complains with one that answers.",
            ),
        ],
    )
}

fn expressions_inside() -> SeedLesson {
    SeedLesson::new(
        "Expressions Inside Functions",
        vec![
            labelled(
                "CONCEPT",
                "Function bodies are blocks, and blocks are expressions. So a function body can be built \
                 out of the same expression machinery you have been using for three modules.",
            ),
            labelled(
                "EXPLANATION",
                "Think of a function body as the last line of a sum written on a receipt. Everything above \
                 it is working out; whatever is left at the bottom is what you walk away with. A semicolon \
                 is a full stop that says 'I am done with this, it produced nothing, keep going'. No \
                 semicolon means the opposite — 'this is the answer'. That is the entire rule. Every weird \
                 thing about returning values from functions falls out of it, including the mistakes.",
            ),
            breakdown(
                "a function body is a block that produces a value",
                "fn calculate() -> i32 {\n    let x = 10;\n    let y = 20;\n\n    x + y\n}",
                &[
                    "`let x = 10;` — a statement. Does the work, produces nothing.",
                    "`let y = 20;` — another statement.",
                    "blank line — style, and it genuinely helps here",
                    "`x + y` — the final expression, no semicolon, so it is the block's value",
                    "the whole block is the function's value, and the signature promised i32",
                ],
            ),
            text_block(
                "An `if` expression drops straight into a return position, because it produces a value:",
            ),
            code(
                "rust",
                "fn choose_number(condition: bool) -> i32 {\n    if condition {\n        10\n    } else {\n        20\n    }\n}\n\nfn main() {\n    println!(\"{}\", choose_number(true));    // 10\n    println!(\"{}\", choose_number(false));   // 20\n}",
            ),
            text_block(
                "And a block can produce a value that another block then uses, which is how you build up a \
                 result in stages without leaking temporary names into the function scope:",
            ),
            code(
                "rust",
                "fn calculate() -> i32 {\n    let result = {\n        let a = 10;\n        let b = 20;\n\n        a + b\n    };\n\n    result\n}",
            ),
            callout(
                "This is not a collection of unrelated rules",
                "Statements, expressions, blocks, `if`, `match`, loops — they all obey the same two rules: \
                 an expression produces a value, and a trailing semicolon turns that value into nothing. Once \
                 you see it as one mechanism rather than five, Rust's syntax stops feeling arbitrary.",
            ),
            code(
                "rust",
                "fn report(score: u32) -> String {\n    let band = match score {\n        90..=100 => \"excellent\",\n        70..=89 => \"good\",\n        _ => \"needs work\",\n    };\n\n    format!(\"score {}: {}\", score, band)\n}",
            ),
            callout(
                "Practice — Try this now",
                "Write `fn stats(values: &[i32]) -> (i32, i32)` that returns the min and max as a tuple. \
                 Build the tuple with a block expression, compute min and max with `if` expressions, and \
                 leave no semicolons on any value-producing line. Then add one semicolon by accident and \
                 predict the error before running.",
            ),
            callout(
                "Mini challenge",
                "Write `fn classify_temperature(celsius: f64) -> &'static str` using only a `match` \
                 expression with ranges as the function's final expression — no `let` bindings at all. \
                 Then write a second version that precomputes with `let` bindings and returns a `format!` \
                 string instead. Same logic, two shapes. Notice which one you can read faster.",
            ),
        ],
    )
}

fn explicit_return() -> SeedLesson {
    SeedLesson::new(
        "Explicit Return and Early Returns",
        vec![
            labelled(
                "CONCEPT",
                "You do not need `return` — the final expression handles it. Sometimes you want it anyway, \
                 because an early exit makes a long function readable. Both styles are legitimate. This is \
                 not a style rule, it is a judgement call.",
            ),
            labelled(
                "EXPLANATION",
                "This is a bouncer's rule. Someone who is not on the list does not walk to the back of the \
                 queue to be turned away there — they are stopped at the door, and the queue behind them \
                 never forms. `return` is the door. Every check you pass lets the next person through; the \
                 first one that fails is out immediately and nothing below it runs. That is why early \
                 returns keep a long function flat: each guard sits at the left margin instead of nesting \
                 one level deeper than the last.",
            ),
            code(
                "rust",
                "fn check_age(age: u32) -> bool {\n    if age < 18 {\n        return false;\n    }\n\n    true\n}",
            ),
            text_block(
                "The final expression is `true`, so the function returns `true` — but only if we did not \
                 already leave. `return false;` exits immediately, skipping everything below. That is an \
                 *early return*, and it is the reason the word exists.",
            ),
            code(
                "rust",
                "fn check_age(age: u32) -> bool {\n    if age < 18 {\n        false\n    } else {\n        true\n    }\n}",
            ),
            paragraphs(&[
                "Both versions return the same thing for every input. Neither is more correct. The first \
                 reads better once the function grows past a few lines, because the main path is not buried \
                 at the bottom of an else. The second is flat and shows every case side by side.",
            ]),
            table(
                &["", "Early return", "Final expression"],
                &[
                    &["Reads as", "'get this out of the way, then continue'", "'here are all the cases'"],
                    &["Scales to many conditions", "well — each is one flat block", "poorly — nested else"],
                    &["Return value visible", "at every return", "once, at the end"],
                    &["Common in Rust for", "guards, bail-outs, validation", "short exhaustive matches"],
                ],
            ),
            callout(
                "Neither is universally correct",
                "You will meet people who insist one or the other is idiomatic. Both are. The useful \
                 question is: *does this early exit make the remaining code easier to read?* If yes, use it. \
                 If the function is three lines, it does not matter.",
            ),
            code(
                "rust",
                "fn first_even(numbers: &[i32]) -> Option<i32> {\n    for n in numbers {\n        if n % 2 == 0 {\n            return Some(*n);      // found it — leave now\n        }\n    }\n    None                          // fell off the end\n}",
            ),
            text_block(
                "This shape — loop, early `return` the moment you find what you are looking for, fall \
                 through to a default at the end — is the single most common function shape you will write \
                 in Rust. It is worth having in your fingers.",
            ),
            callout(
                "Every path must produce a value",
                "If a function promises `-> i32` and there is any route through it that falls off the end, \
                 the compiler stops you. That is the exhaustiveness check from `match`, applied to control \
                 flow. It is why the compiler can trust a signature.",
            ),
            callout(
                "Practice — Try this now",
                "Write `fn classify(n: i32) -> &'static str` twice: once with early returns, once with a \
                 final `match`. Then write a third version that is thirty lines long with a guard near the \
                 top and a single return at the bottom. Notice which version is easiest to modify.",
            ),
            callout(
                "Mini challenge",
                "Write `fn validate_username(name: &str, age: u32) -> Result<&str, &'static str>` using \
                 early returns: bail with one error if empty, another if too short, another if age is \
                 under 13. Count how many `else` blocks you avoided. Then rewrite it with a `match` on a \
                 tuple and compare.",
            ),
        ],
    )
}

fn scope() -> SeedLesson {
    SeedLesson::new(
        "Scope",
        vec![
            labelled(
                "CONCEPT",
                "A variable belongs to the place it was created. Inside that place, you can use its name. \
                 Outside, the name simply does not mean anything.",
            ),
            labelled(
                "EXPLANATION",
                "A name is a label on a thing that only works in the room the thing is in. Write it on the \
                 whiteboard in the meeting, and everyone in that meeting knows exactly what it means. Step \
                 into the next room and the label is meaningless — not because it is wrong, but because \
                 nothing there is pointing at the thing. Scope is just which room you are standing in. A \
                 function body is a room with walls, and that is why a function cannot see `main`'s \
                 variables, and why you pass things in instead.",
            ),
            code(
                "rust",
                "fn main() {\n    let x = 10;\n\n    {\n        let y = 20;\n\n        println!(\"{} {}\", x, y);   // both visible\n    }\n\n    println!(\"{}\", x);            // x still fine\n}",
            ),
            diagram(
                "nested scopes",
                "  main's scope\n   │\n   ├── x          ← declared here, visible here and below\n   │\n   └── inner scope\n       │\n       └── y      ← declared here, visible here only\n                             and nowhere after the closing brace",
            ),
            broken("fn main() {\n    let x = 10;\n\n    {\n        let y = 20;\n        println!(\"{} {}\", x, y);\n    }\n\n    println!(\"{}\", y);   // y is out of scope\n}"),
            code(
                "text",
                "error[E0425]: cannot find value `y` in this scope\n --> src/main.rs:8:21\n  |\n8 |     println!(\"{}\", y);\n  |                    ^ not found in this scope\n  |\nhelp: if you intended to capture a variable from the\n      enclosing scope, you can list it in a closure",
            ),
            text_block(
                "In plain terms: there is no `y` here. It existed inside those braces, and when the braces \
                 closed it stopped existing. Rust says 'not found in this scope' rather than 'undefined', \
                 because in Rust a name that is out of scope does not exist at all — there is no garbage \
                 value left lying around to read by accident.",
            ),
            text_block(
                "Function boundaries are scope boundaries too, and this is the one that surprises people:",
            ),
            broken("fn main() {\n    let name = \"Gus\";\n\n    greet();\n}\n\nfn greet() {\n    println!(\"Hello, {}\", name);   // name does not exist in here\n}"),
            text_block(
                "Some languages would reach across the boundary and hand you the value. Rust will not, and \
                 the reason is a good one: a function should be able to be read on its own. If `greet` needed \
                 something from `main`, you would have to know what `main` contains to understand `greet`. \
                 Passing it in as an argument makes the dependency visible, which makes the function easier \
                 to reason about and to reuse.",
            ),
            callout(
                "Scope is not ownership",
                "They are related — both are about how long a value is available — but they are different \
                 questions. Scope asks *where can I use this name?* Ownership asks *who is responsible for \
                 freeing this value, and when?* You can have an in-scope variable you have moved, which is \
                 exactly what the next lesson is about. Two concepts, one module apart.",
            ),
            callout(
                "Practice — Try this now",
                "Write `let x = 1;` in main. Open a block, shadow it with `let x = 2;`, print it, close the \
                 block, print it again. The shadowing is scoped, so the outer `x` is untouched. Then try \
                 printing the inner one after the block closes and read the error.",
            ),
            callout(
                "Mini challenge",
                "Write `fn shadow_demo() -> i32` with a variable in three nested scopes, each doubling it, \
                 and return the outermost value — which should be unchanged. Then write a version using \
                 shadowing instead of nesting, where the final value is 8, and explain in one sentence why \
                 the two versions produce different results.",
            ),
        ],
    )
}

fn ownership_across_calls() -> SeedLesson {
    SeedLesson::new(
        "Ownership Across Function Calls",
        vec![
            labelled(
                "CONCEPT",
                "Function parameters are ordinary bindings, and they obey every rule you have already met. \
                 Including the one that is about to get its own module: when a value moves, the original \
                 stops being usable.",
            ),
            labelled(
                "EXPLANATION",
                "Handing someone a `String` in Rust is not lending it, it is giving it away. They now have \
                 the only copy, and when their function ends it is dropped — gone, for both of you. That \
                 sounds harsh until you notice the alternative: if they might keep it, you get to keep it \
                 too. Passing `&String` is the polite version, a quick look rather than a handover. \
                 Nothing about the function's logic changes, only who owns the thing when it is done. The \
                 rule is simple — if the function only reads, borrow; if it needs to keep it, take it.",
            ),
            code(
                "rust",
                "fn consume_name(name: String) {\n    println!(\"{}\", name);\n}\n\nfn main() {\n    let name = String::from(\"Gus\");\n\n    consume_name(name);   // name is moved into the function\n\n    println!(\"{}\", name); // ← does not compile\n}",
            ),
            code(
                "text",
                "error[E0382]: borrow of moved value: `name`\n --> src/main.rs:9:20\n   |\n7  |     consume_name(name);\n   |                 ----- value moved here\n8  |\n   |     println!(\"{}\", name);\n   |                    ^^^^ value borrowed here after move",
            ),
            text_block(
                "Two things in that message matter. The first is `value moved here`, pointing at the call — \
                 that is where your variable went. The second is `value borrowed here after move`, pointing \
                 at the use. Rust is drawing you a line between the two. The value is not gone; it belongs \
                 to the function now, and the function threw it away at the end of its scope.",
            ),
            text_block(
                "If the function only needed to *look* at the name, borrow it instead:",
            ),
            code(
                "rust",
                "fn borrow_name(name: &String) {\n    println!(\"{}\", name);\n}\n\nfn main() {\n    let name = String::from(\"Gus\");\n\n    borrow_name(&name);    // lends it, keeps it\n\n    println!(\"{}\", name); // fine — we still own it\n}",
            ),
            diagram(
                "move vs borrow at the call boundary",
                "  fn f(name: String)      fn f(name: &String)\n        │                            │\n        ▼                            ▼\n  the String is MOVED          the String is BORROWED\n  caller loses it               caller keeps it\n  f is now responsible         f is a guest, must return it\n  for freeing it",
            ),
            callout(
                "What this means for how you write functions",
                "Take `String` by value when the function needs to keep or consume it — store it, return it, \
                 put it in a struct. Take `&String` when it only needs to read it, which is most of the \
                 time. This one decision is the difference between code that compiles and code that fights \
                 you.",
            ),
            callout(
                "We are not going further here",
                "Ownership and borrowing each get a full module — they are the heart of Rust and they deserve \
                 proper treatment. Today the only thing you need is the shape: parameters participate in \
                 ownership, passing a `String` can take it away from you, and `&` means 'I am only looking'.",
            ),
            code(
                "rust",
                "fn consume(s: String) {\n    println!(\"got {}\", s);\n}   // s is dropped at the end of this function\n\nfn borrow(s: &String) {\n    println!(\"saw {}\", s);\n}   // s is a reference, nothing to drop\n\nfn main() {\n    let s = String::from(\"hello\");\n\n    borrow(&s);    // still ours\n    consume(s);    // ours no longer\n}",
            ),
            callout(
                "Practice — Try this now",
                "Write `fn shout(s: String)` that takes ownership and prints the name in caps. Call it with \
                 a `String`, then try to print the original. Read the error. Then change the parameter to \
                 `&String` and confirm it works. Then try `String` parameters with an `i32` argument and see \
                 what the compiler says about the copy.",
            ),
            callout(
                "Mini challenge",
                "Write `fn take_name(name: String) -> String` that returns the name in caps, so the value \
                 comes back rather than being dropped. Write `fn peek(name: &String) -> &str` that borrows \
                 and returns a slice. Call both in an order where `peek`'s result is used after \
                 `take_name` has run, and explain why that works for one and not the other.",
            ),
        ],
    )
}

fn mutable_parameters() -> SeedLesson {
    SeedLesson::new(
        "Mutable Parameters",
        vec![
            labelled(
                "CONCEPT",
                "Sometimes a function needs to change something. It cannot change it by magic — the caller \
                 has to hand over permission, and the parameter has to ask for it. Two keywords, both \
                 required.",
            ),
            code(
                "rust",
                "fn double(value: &mut i32) {\n    *value *= 2;\n}\n\nfn main() {\n    let mut number = 10;\n\n    double(&mut number);\n\n    println!(\"{}\", number);   // 20\n}",
            ),
            labelled(
                "EXPLANATION",
                "Three things have to line up here, and if any one is missing the program will not compile. \
                 That is a good property. It means a function that mutates something always *looks* like it \
                 mutates something.",
            ),
            steps(&[
                "`let mut number = 10;` — the caller declares the variable mutable. Without `mut` there is nothing mutable to lend.",
                "`fn double(value: &mut i32)` — the parameter is a mutable reference. `&mut` means 'a reference I may write through'.",
                "`*value *= 2;` — the `*` dereferences, so this changes the caller's variable, not a copy.",
                "`double(&mut number);` — the caller passes the reference. `&mut` here grants that permission.",
            ]),
            broken("fn double(value: &mut i32) {\n    *value *= 2;\n}\n\nfn main() {\n    let number = 10;      // no mut\n\n    double(&mut number);  // ← error\n\n    println!(\"{}\", number);\n}"),
            code(
                "text",
                "error[E0596]: cannot borrow `number` as mutable, as it is not declared as mutable\n --> src/main.rs:7:12\n  |\n7 |     double(&mut number);\n  |            ^^^^^^ cannot borrow as mutable\n  |\nhelp: consider changing this to be mutable\n  |\n7 |     let mut number = 10;\n  |         +++\nhelp: consider changing the type of `number`\n  |         &mut i32",
            ),
            callout(
                "The mental model: permission",
                "`&mut` is a permission slip, and both sides have to ask for it. The caller says 'you may \
                 change this' with `&mut` at the call, and the function says 'I intend to change this' with \
                 `&mut` in the signature. A function that takes `&mut i32` cannot be called with something \
                 immutable, even by accident, and a function that takes `&i32` can never change it.",
            ),
            callout(
                "The star is not decoration",
                "`value` inside the function is an `&mut i32` — a reference. `*value` is the `i32` it points \
                 at. Without the star you would be trying to do arithmetic on a reference, and the error \
                 will mention `&mut {integer}` where it expected an integer. It is a good error, but only \
                 after you have written the star.",
            ),
            code(
                "rust",
                "fn reset(list: &mut Vec<i32>) {\n    list.clear();\n}\n\nfn add_item(list: &mut Vec<i32>, item: i32) {\n    list.push(item);\n}\n\nfn main() {\n    let mut scores = vec![90, 85];\n\n    add_item(&mut scores, 95);\n    println!(\"{:?}\", scores);   // [90, 85, 95]\n\n    reset(&mut scores);\n    println!(\"{:?}\", scores);   // []\n}",
            ),
            callout(
                "We are not going further here",
                "Rust's borrowing rules — how many mutable references you can hold at once, and why — are \
                 the subject of module 06. For now, one rule covers everything you need: if a function \
                 changes what you pass it, your variable must be `mut` and you must pass `&mut`. Everything \
                 else is detail for later.",
            ),
            callout(
                "Practice — Try this now",
                "Write `fn increment(counter: &mut i32)` and call it three times on the same variable, \
                 printing after each. Then remove the `mut` from the variable declaration and read the error. \
                 Then remove the `&mut` from the parameter — it will compile but do nothing, which is worse.",
            ),
            callout(
                "Mini challenge",
                "Write `fn normalise(values: &mut Vec<i32>)` that replaces every negative number with its \
                 absolute value, and call it on a vector you did not declare `mut`. Read the error. Then \
                 declare it `mut` and add a second function `fn is_normalised(values: &[i32]) -> bool` that \
                 checks without being allowed to change anything. Two functions, one borrows mutably and \
                 one borrows immutably, over the same data.",
            ),
        ],
    )
}

fn functions_calling_functions() -> SeedLesson {
    SeedLesson::new(
        "Functions Calling Functions",
        vec![
            labelled(
                "CONCEPT",
                "This is where functions stop being a tidiness habit and start being how you actually build \
                 things. Every non-trivial program is a graph of small functions calling each other.",
            ),
            code(
                "rust",
                "fn calculate_total(price: f64, quantity: u32) -> f64 {\n    price * quantity as f64\n}\n\nfn print_total(total: f64) {\n    println!(\"Total: ${:.2}\", total);\n}\n\nfn main() {\n    let total = calculate_total(10.0, 3);\n    print_total(total);\n}",
            ),
            diagram(
                "the call chain",
                "  main()\n   │\n   ├── calculate_total(10.0, 3)\n   │      │\n   │      └── returns 30.0\n   │\n   └── print_total(30.0)\n          │\n          └── returns ()\n\n  main orchestrates; the helpers do one job each",
            ),
            labelled(
                "EXPLANATION",
                "Notice what `main` does here: it calls two things and stitches the result between them. \
                 It does no arithmetic and no formatting itself. That is the shape to aim for — `main` reads \
                 like a summary of the program, and every detail lives somewhere it can be found, named, and \
                 tested.",
            ),
            code(
                "rust",
                "fn apply_discount(price: f64) -> f64 {\n    price * 0.9\n}\n\nfn add_tax(price: f64) -> f64 {\n    price * 1.2\n}\n\nfn checkout(subtotal: f64) -> f64 {\n    let discounted = apply_discount(subtotal);\n    add_tax(discounted)\n}\n\nfn main() {\n    println!(\"{:.2}\", checkout(100.0));   // 108.00\n}",
            ),
            text_block(
                "Read `checkout` and you can see the whole business rule in two lines: discount first, then \
                 tax. You do not have to hold the arithmetic in your head. And if the tax rate changes, you \
                 edit one tiny function and the error is guaranteed to be local.",
            ),
            callout(
                "Each function should be readable on its own",
                "A good test: can you read one function without having read the ones above it? `add_tax` \
                 above tells you exactly what it does in one line, with no context needed. If a function \
                 only makes sense once you have understood three others, the boundaries are in the wrong \
                 place.",
            ),
            callout(
                "Watch the argument types across the boundary",
                "In `checkout`, `apply_discount` takes an `f64` and produces an `f64`, which is exactly what \
                 `add_tax` needs. When the types line up, functions compose without friction. When they do \
                 not, you get a conversion in the middle — and conversions are where mistakes hide.",
            ),
            callout(
                "Practice — Try this now",
                "Trace `checkout(100.0)` by hand before running it: what is the value after each of the two \
                 calls? Then run it and check. If your hand-trace disagreed with the output, find out why \
                 before moving on — that gap is where the bugs live.",
            ),
            callout(
                "Mini challenge",
                "Build a four-function pipeline: `parse_price(text: &str) -> f64`, `apply_tax(price: f64) -> \
                 f64`, `format_currency(amount: f64) -> String`, and `receipt(price: f64) -> String` that \
                 composes the other three. Then add a fifth function `process(text: &str) -> String` that \
                 takes raw input and produces the final line, so `main` is a single call. Count the total \
                 lines. Compare that to writing it all inline.",
            ),
        ],
    )
}

fn designing_good_functions() -> SeedLesson {
    SeedLesson::new(
        "Designing Good Functions",
        vec![
            labelled(
                "CONCEPT",
                "Everything so far has been syntax. This is the part where you start making decisions, and \
                 where programming stops being a language problem and becomes a design problem.",
            ),
            labelled(
                "EXPLANATION",
                "A compiler will happily accept a function that does six things badly. Nothing in Rust is \
                 going to stop you, which means the only thing standing between a working program and a \
                 readable one is your judgement about where the seams go. The test is embarrassingly \
                 simple: if you cannot say what a function does without using the word 'and', you have not \
                 found the seam yet. You are not hunting for the smallest function. You are hunting for \
                 the one with a name you can say out loud.",
            ),
            code(
                "rust",
                "fn process_order() {\n    // validate user\n    // check inventory\n    // calculate subtotal\n    // calculate tax\n    // print receipt\n    // update inventory\n    // send confirmation\n}",
            ),
            callout(
                "How many different jobs is this function doing?",
                "Seven. And it takes no arguments, so you cannot test a single one of them. This is the \
                 shape of code that looks fine at twenty lines and becomes hostile at two hundred.",
            ),
            text_block(
                "The instinct is to split it. Do that, and you get something you can hold in your head:",
            ),
            diagram(
                "the same behaviour, divided",
                "  fn validate_order(user: &User) -> Result<(), Error>\n  fn check_inventory(items: &[Item]) -> Result<(), Error>\n  fn calculate_subtotal(items: &[Item]) -> f64\n  fn calculate_tax(subtotal: f64) -> f64\n  fn print_receipt(total: f64)\n  fn update_inventory(items: &mut [Item])\n  fn send_confirmation(user: &User, total: f64)\n\n  then process_order() orchestrates them",
            ),
            table(
                &["Property", "What it buys you"],
                &[
                    &["Single responsibility", "one name describes one thing, so searching finds it"],
                    &["Meaningful names", "the call site reads as a summary"],
                    &["Clear inputs", "you can see what the function needs, without reading the body"],
                    &["Clear outputs", "you can see what you get back before reading the body"],
                    &["Small size", "you can hold it in your head; you can test it alone"],
                ],
            ),
            callout(
                "Single responsibility is a heuristic, not a law",
                "Do not split a function until it hurts. A four-line function that does two things is \
                 usually clearer as one function. The signal to split is not line count — it is that you \
                 cannot describe what the function does without using the word 'and'.",
            ),
            callout(
                "The 'and' test",
                "Try to describe a function in a single sentence without the word 'and'. If you cannot, it is \
                 doing more than one thing. `checkout` applies a discount and then adds tax — two things. \
                 `add_tax` applies tax — one thing. That is the whole heuristic.",
            ),
            code(
                "rust",
                "// Hard to name: it does two things\nfn process(data: &str) {\n    let cleaned = data.trim();\n    println!(\"{}\", cleaned.to_uppercase());\n}\n\n// Easy to name: each does one thing\nfn clean(data: &str) -> &str {\n    data.trim()\n}\n\nfn shout(data: &str) -> String {\n    data.to_uppercase()\n}",
            ),
            callout(
                "Practice — Try this now",
                "Take the `process_order` comment list and write a real function signature for each job — \
                 parameters and return types, no bodies. Then ask of each one: can I name it without 'and'? \
                 Two of them probably cannot, and that is worth noticing before you write the bodies.",
            ),
            callout(
                "Mini challenge",
                "Write a function that is deliberately overstuffed: it reads a list of prices, applies a \
                 discount to some, adds tax, formats a report, and prints it. Then split it until every \
                 function passes the 'and' test. Do not go past that — resist the urge to make eight \
                 one-line functions. Stop when splitting no longer improves the names.",
            ),
        ],
    )
}

fn recursion() -> SeedLesson {
    SeedLesson::new(
        "Recursion",
        vec![
            labelled(
                "CONCEPT",
                "A function that calls itself. That is the whole definition. The interesting part is not \
                 the calling — it is knowing when to stop.",
            ),
            code(
                "rust",
                "fn countdown(n: i32) {\n    if n == 0 {\n        return;\n    }\n\n    println!(\"{}\", n);\n    countdown(n - 1);\n}\n\nfn main() {\n    countdown(3);\n}",
            ),
            labelled(
                "EXPLANATION",
                "Every recursive function needs two things. A **base case** — the condition that stops the \
                 recursion, here `n == 0`. And a **recursive case** — the part that calls itself, here \
                 `countdown(n - 1)`. Get either wrong and you have either an infinite loop or a function \
                 that does nothing.",
            ),
            diagram(
                "the call stack unwinding",
                "  countdown(3)\n   │  not 0, print 3, recurse\n   ▼\n  countdown(2)\n   │  not 0, print 2, recurse\n   ▼\n  countdown(1)\n   │  not 0, print 1, recurse\n   ▼\n  countdown(0)\n   │  base case — return\n   ▼\n  back up the stack, nothing left to do",
            ),
            callout(
                "Watch out — the base case must be reachable",
                "Delete the `if n == 0` line and this function will recurse until the stack runs out, then \
                 panic. The counter decreases toward zero and the base case is waiting there, so it stops. \
                 If your base case can never be reached, you have written a stack overflow with extra steps.",
            ),
            code(
                "rust",
                "fn factorial(n: u64) -> u64 {\n    if n <= 1 {\n        return 1;          // base case\n    }\n    n * factorial(n - 1)     // recursive case\n}\n\nfn main() {\n    println!(\"{}\", factorial(5));   // 120\n}",
            ),
            callout(
                "Recursion is not the Rust way, the way, or even usually the best way",
                "Many problems that are textbook recursion in other languages are more naturally loops or \
                 iterators in Rust — the borrow checker actively pushes you away from recursion, because \
                 recursive functions on owned data are awkward. Reach for recursion when the data is \
                 naturally shaped like it, such as a tree. Use a loop for everything else.",
            ),
            code(
                "rust",
                "struct Node {\n    children: Vec<Node>,\n}\n\n// Naturally recursive — a tree is shaped like this\nfn count_nodes(node: &Node) -> usize {\n    1 + node.children.iter().map(count_nodes).sum::<usize>()\n}\n\n// Naturally a loop — a flat list\nfn sum(values: &[i32]) -> i32 {\n    let mut total = 0;\n    for v in values {\n        total += v;\n    }\n    total\n}\n\nfn main() {\n    let leaf = Node { children: vec![] };\n    let root = Node {\n        children: vec![Node { children: vec![leaf] }],\n    };\n\n    println!(\"{} nodes\", count_nodes(&root));\n    println!(\"sum: {}\", sum(&[1, 2, 3, 4]));\n}",
            ),
            callout(
                "Practice — Try this now",
                "Write `fn countdown_loop(n: i32)` that prints the same numbers using a `for` loop. Then \
                 remove the base case from the recursive version and run it. Read the stack overflow message \
                 — the wording tells you exactly what filled up.",
            ),
            callout(
                "Mini challenge",
                "Write a recursive function that reverses a string by returning a new one — \
                 `fn reverse(s: &str) -> String` — using slicing: `format!(\"{}{}\", &s[1..], &s[..1])`. The \
                 base case is the empty string. Then write the same function iteratively with a loop, and \
                 time both on a 10,000-character input. One will be dramatically worse than the other, and \
                 knowing which one is the actual lesson.",
            ),
        ],
    )
}

fn things_you_will_break() -> SeedLesson {
    SeedLesson::new(
        "Things You Will Probably Break",
        vec![
            labelled(
                "CONCEPT",
                "Seven mistakes that account for most of what beginners hit in their first week of \
                 functions. None of them are stupid. Each one has a shape, and once you see the shape you \
                 stop making them.",
            ),
            labelled(
                "EXPLANATION",
                "The good news about compiler errors is that there are only so many of them. Rust does not \
                 invent a new way to be confused each week — it has about seven, and every function bug \
                 you are ever going to write is one of them wearing a different variable name. So this is \
                 less a list of traps than a lookup table. The first time you meet each one, memorise the \
                 shape. The second time you meet it, you will recognise it before you finish scrolling \
                 the message.",
            ),
            callout(
                "Mistake 1 — Calling with the wrong number of arguments",
                "`add(10)` when `add` takes two. Rust will not fill in defaults for you the way some \
                 languages do, because a function that sometimes receives one argument and sometimes two \
                 is a function with a hidden precondition.",
            ),
            broken("fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n\nfn main() {\n    let sum = add(10);\n    println!(\"{}\", sum);\n}"),
            broken_text(
                "error[E0061]: this function takes 2 arguments but 1 argument was supplied\n --> src/main.rs:6:17\n   |\n6  |     let sum = add(10);\n   |                 ^^^ -- argument to this function\n   |                 |\n   |                 expected 2 arguments",
            ),
            text_block(
                "In plain terms: the call site promised two values and gave one. The `^^^` marker counts the \
                 arguments, and `expected 2 arguments` is the count it wanted. Notice Rust also underlined \
                 the function name — it is telling you the call is the problem, not `add` itself.",
            ),
            fixed("fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n\nfn main() {\n    let sum = add(10, 0);   // or supply a real second value\n    println!(\"{}\", sum);\n}"),
            callout(
                "Mistake 2 — Wrong argument type",
                "`add(\"10\", 20)`. The same mismatch you have seen before, but now it points at a call \
                 rather than a variable.",
            ),
            broken_text(
                "error[E0308]: mismatched types\n  --> src/main.rs:6:23\n   |\n6  |     let sum = add(\"10\", 20);\n   |                            ^^^^ expected `i32`, found `&str`\n   |                            |\n   |                            arguments to this function call",
            ),
            fixed("fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n\nfn main() {\n    let sum = add(10, 20);\n    println!(\"{}\", sum);\n}"),
            callout(
                "Mistake 3 — Wrong return type",
                "The function promises one type and the body produces another. This is the signature \
                 contract being enforced.",
            ),
            broken("fn add() -> i32 {\n    \"hello\"\n}"),
            broken_text(
                "error[E0308]: mismatched types\n --> src/main.rs:2:5\n  |\n2 |     \"hello\"\n  |     ^^^^^^^ expected `i32`, found `&str`",
            ),
            fixed("fn message() -> &'static str {\n    \"hello\"\n}\n\nfn main() {\n    println!(\"{}\", message());\n}"),
            callout(
                "Mistake 4 — The accidental unit return",
                "The one you have now met twice. A semicolon on the final expression turns the value into \
                 nothing, and the signature is left unfulfilled.",
            ),
            broken_text(
                "error[E0308]: mismatched types\n --> src/main.rs:2:5\n  |\n2 |     10;\n  |     ^^ expected `i32`, found `()`\n  |     |\n  |     this expression has type `()`\nhelp: if this is a `return` expression, remove the semicolon",
            ),
            fixed("fn ten() -> i32 {\n    10          // no semicolon\n}\n\nfn main() {\n    println!(\"{}\", ten());\n}"),
            callout(
                "Mistake 5 — Using a variable out of scope",
                "The name belongs to the block it was created in, including the block that is a function \
                 body.",
            ),
            broken_text(
                "error[E0425]: cannot find value `name` in this scope\n --> src/main.rs:8:27\n   |\n8  |     println!(\"Hello, {}\", name);\n   |                           ^^^^ not found in this scope\n   |                           |\n   |                           maybe you meant to use a variable defined elsewhere?",
            ),
            fixed("fn greet(name: &str) {\n    println!(\"Hello, {}\", name);\n}\n\nfn main() {\n    let name = \"Gus\";\n    greet(name);   // pass it in instead of reaching for it\n}"),
            callout(
                "Mistake 6 — Moving a value into a function, then using it",
                "A `String` parameter takes ownership. The caller loses it unless it borrows instead.",
            ),
            broken_text(
                "error[E0382]: borrow of moved value: `name`\n --> src/main.rs:7:9\n   |\n7  |     consume_name(name);\n   |                 ----- value moved here\n8  |     println!(\"{}\", name);\n   |                    ^^^^ value borrowed here after move",
            ),
            fixed("fn consume_name(name: &String) {\n    println!(\"{}\", name);\n}\n\nfn main() {\n    let name = String::from(\"Gus\");\n    consume_name(&name);   // borrow, do not consume\n    println!(\"{}\", name); // still ours\n}"),
            callout(
                "Mistake 7 — Borrowing mutably from a non-mut variable",
                "You cannot lend out write permission for something you never claimed to be writable.",
            ),
            broken_text(
                "error[E0596]: cannot borrow `number` as mutable, as it is not declared as mutable\n --> src/main.rs:6:12\n  |\n6  |     double(&mut number);\n  |            ^^^^^^ cannot borrow as mutable\n  |\nhelp: consider changing this to be mutable\n  |\n6  |     let mut number = 10;",
            ),
            fixed("fn double(value: &mut i32) {\n    *value *= 2;\n}\n\nfn main() {\n    let mut number = 10;\n    double(&mut number);\n    println!(\"{}\", number);\n}"),
            callout(
                "Practice — Try this now",
                "Write all seven broken versions into one file, uncommented. Run it. The compiler stops at \
                 the first error, so fix one, run, fix the next, repeat. Seven runs, seven fixes. Reading \
                 the errors in order is faster than staring at all of them at once.",
            ),
            callout(
                "Mini challenge",
                "Fix mistake 3 in the *opposite* direction: change the function's return type to match the \
                 body instead of changing the body to match the signature. Both are legitimate fixes. Write \
                 one sentence about which one you would choose when the function is a public API and which \
                 you would choose when it is private.",
            ),
        ],
    )
}

fn putting_it_together() -> SeedLesson {
    SeedLesson::new(
        "Putting It All Together",
        vec![
            labelled(
                "CONCEPT",
                "You can now define functions, pass them information, get values back, understand where \
                 their variables live, and know what happens to a value when it crosses a call boundary. \
                 That is the vocabulary. This lesson is the sentence.",
            ),
            code(
                "rust",
                "struct Order {\n    subtotal: f64,\n    member: bool,\n}\n\nfn discount_rate(member: bool) -> f64 {\n    if member { 0.10 } else { 0.0 }\n}\n\nfn discounted(subtotal: f64, rate: f64) -> f64 {\n    subtotal * (1.0 - rate)\n}\n\nfn with_tax(amount: f64) -> f64 {\n    amount * 1.2\n}\n\nfn describe(order: &Order) -> String {\n    let base = discounted(order.subtotal, discount_rate(order.member));\n    let total = with_tax(base);\n    format!(\"total {:.2} (member: {})\", total, order.member)\n}\n\nfn main() {\n    let order = Order { subtotal: 100.0, member: true };\n    println!(\"{}\", describe(&order));\n}",
            ),
            labelled(
                "EXPLANATION",
                "Four functions, each doing one job, each named for what it does. `describe` reads as a \
                 summary of the whole calculation. Note the two shapes of return: `discount_rate` uses a \
                 bare `if` expression as its final value, while `describe` builds a `String` through several \
                 steps and returns the last. Both are normal. Both are the same idea.",
            ),
            checklist(
                "What You Can Do Now",
                &[
                    "Define and call functions",
                    "Read Rust function signatures",
                    "Work with parameters and arguments",
                    "Return values from functions",
                    "Understand implicit returns",
                    "Understand explicit return",
                    "Understand the unit type ()",
                    "Understand function scope",
                    "Understand how ownership interacts with calls",
                    "Pass references into functions",
                    "Pass mutable references",
                    "Compose functions together",
                    "Break large problems into smaller functions",
                    "Understand basic recursion",
                    "Debug common function errors",
                    "Design a small program around functions",
                ],
            ),
            diagram(
                "where this goes next",
                "  Functions        ← you are here\n     │\n     ▼\n  Ownership        module 05\n     │\n     ▼\n  Borrowing        module 06\n     │\n     ▼\n  Structs & Enums  module 07   methods arrive here\n     │\n     ▼\n  Traits           module 10\n     │\n     ▼\n  Error handling   module 09",
            ),
            paragraphs(&[
                "Everything from here builds on what you just learned. Ownership is about what happens to \
                 values as they move between functions. Structs give your functions something meaningful to \
                 take as a parameter instead of five loose arguments. Methods are functions with a \
                 `self` parameter. Traits are how you say 'any of these types can do this' about a function.",
                "So the two things worth carrying forward: give behaviour a name, and make the data a \
                 function needs visible in its signature. Everything else follows.",
            ]),
            callout(
                "Practice — Try this now",
                "Add a `fn shipping_cost(member: bool, subtotal: f64) -> f64` that is free for members and \
                 `subtotal * 0.05` otherwise, and wire it into `describe` between the discount and the \
                 tax. Three functions now, composed in one. Do not edit the others.",
            ),
            callout(
                "Mini challenge — the gate",
                "Write `fn busiest_slot(slots: &[u32]) -> usize` that returns the index of the largest \
                 value using a `for` loop and an `if` expression to track the best so far, returning 0 for \
                 an empty slice. Then call it twice: once normally, and once passing a `&Vec<u32>` to see \
                 the coercion. If you reach for `mut` anywhere, check whether you actually needed it.",
            ),
        ],
    )
}
