//! Exercises for Module 04 — Functions.

use crate::content::types::{ExerciseKind, SeedExercise};

pub fn exercises() -> Vec<SeedExercise> {
    vec![
        // ── L1 Why functions exist ────────────────────────────────────
        SeedExercise::mcq(
            0,
            "What is the main benefit of naming a piece of behaviour?",
            &[
                "It makes the program run faster",
                "It gives a piece of behaviour a meaningful, reusable name",
                "It reduces the file size",
                "It lets Rust skip some checks",
            ],
            1,
            "A function does not save lines of code — `calculate_total()` is the same number of lines as the arithmetic it replaces. It saves *comprehension*, and that compounds every time the code is read.",
        ),
        SeedExercise::mcq(
            0,
            "In `fn process_order()`, what is the strongest sign it is doing too much?",
            &[
                "Its name is too short",
                "Describing it requires using the word 'and'",
                "It contains comments",
                "It takes no arguments",
            ],
            1,
            "The 'and' test: if you cannot describe a function in one sentence without 'and', it is doing more than one thing. That is a far better signal than line count.",
        )
        .kind(ExerciseKind::SpotDifference)
        .with_code("fn main() {\n    let price = 19.99;\n    let qty = 3;\n    let total = price * qty;\n    let tax = total * 0.08;\n    println!(\"{:.2}\", total + tax);\n}")
        .with_code_b("fn subtotal(price: f64, qty: u32) -> f64 {\n    price * qty as f64\n}\n\nfn tax(amount: f64) -> f64 {\n    amount * 0.08\n}\n\nfn main() {\n    let total = subtotal(19.99, 3);\n    println!(\"{:.2}\", total + tax(total));\n}")
        .with_hints(&[
            "Both produce the same output. That is the point.",
            "Count the number of named things in each. Which one can you point at in a conversation?",
        ]),
        SeedExercise::mcq(
            0,
            "A function is worth extracting when its name improves the reader's guess. Which name is best?",
            &[
                "fn part_a()",
                "fn do_the_thing(x: i32) -> i32",
                "fn is_prime(n: u32) -> bool",
                "fn helper()",
            ],
            2,
            "`is_prime` tells you its input, its output, and its question all at once. `part_a` and `helper` push the work onto the reader — they are names that admit no understanding.",
        )
        .kind(ExerciseKind::BuildIt)
        .with_hints(&[
            "A good name answers: what comes in, what comes out, and what question is being asked",
            "If you need a comment to say what the function does, the name is not doing its job",
        ]),

        // ── L2 Defining and calling ────────────────────────────────────
        SeedExercise::mcq(
            1,
            "What does defining a function do?",
            &[
                "Runs the function body once",
                "Registers the name and what it does; the body does not run",
                "Prints the function name",
                "Allocates memory for the body",
            ],
            1,
            "Definition writes the name into the program's vocabulary. Only a call executes the body. That is why you can define a function and never call it without any complaint about missing output.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn beep() {\n    println!(\"beep\");\n}\n\nfn main() {\n    beep();\n    beep();\n    beep();\n}")
        .with_hints(&["Count the calls, not the definitions"]),
        SeedExercise::mcq(
            1,
            "How many times does this print anything?\n\nfn shout() {\n    println!(\"HI\");\n}\n\nfn main() {\n    shout();\n}",
            &["Zero", "One", "Two", "It does not compile"],
            1,
            "One call, one execution. Defining `shout` printed nothing at all — only the call did.",
        )
        .kind(ExerciseKind::FillGap)
        .with_code("fn line() {\n    println!(\"---\");\n}\n\nfn main() {\n    ____\n    line();\n    line();\n}")
        .with_hints(&["Same as the line above it", "The name, then parentheses, then a semicolon"]),
        SeedExercise::mcq(
            1,
            "Does a function have to be defined before it is called?",
            &[
                "Yes — Rust reads top to bottom, so the call must come first in the file only",
                "Yes — the definition must be textually above the call",
                "No — a call can appear before the definition in the same file",
                "No — but only inside a module",
            ],
            2,
            "Within a file, item order does not matter. `main` at the bottom calling a function defined above it is the convention, not a requirement. This is a difference from C, where a forward declaration is needed.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    countdown(3);\n}\n\nfn countdown(n: u32) {\n    if n == 0 { return; }\n    println!(\"{}\", n);\n    countdown(n - 1);\n}")
        .with_hints(&["The definition comes after main here", "So the order in the file cannot be the constraint"]),
        SeedExercise::mcq(
            1,
            "What is wrong with calling a function that was never defined?",
            &[
                "Nothing — Rust will generate it",
                "The compiler cannot find the name and reports it",
                "It runs and returns null",
                "It is a warning, not an error",
            ],
            1,
            "An unknown name is a compile error, not a warning. Rust will not guess at an implementation. If a name is not in scope, the program does not build.",
        )
        .kind(ExerciseKind::CompilerChallenge)
        .with_code("error[E0425]: cannot find function `greet` in this scope\n --> src/main.rs:2:5\n  |\n2 |     greet();\n  |     ^^^^ not found in this scope")
        .with_hints(&["Not found in this scope", "Is `greet` spelled the same in both places?"]),
        SeedExercise::mcq(
            1,
            "What does this print?\n\nfn main() {\n    never_called();\n}\n\nfn never_called() {\n    println!(\"surprise\");\n}",
            &["surprise", "Nothing", "An error", "surprise twice"],
            1,
            "`never_called` is defined but not called, so its body never executes. A definition on its own is completely silent — which is why an accidentally uncalled function is such a common bug.",
        )
        .kind(ExerciseKind::BuildIt)
        .with_hints(&[
            "Call it. Then remove the call and watch the output disappear",
            "This is the cheapest way to prove definition and call are different events",
        ]),

        // ── L3 Reading a signature ─────────────────────────────────────
        SeedExercise::new(
            Some(2),
            ExerciseKind::SignatureReading,
            "READ THE SIGNATURE — `fn calculate(x: i32, y: i32) -> i32`. Name the function, both parameters, and the return type.",
            &[
                "Function `calculate`, parameters `x: i32` and `y: i32`, returns `i32`",
                "Function `calculate(x, y)`, parameters `i32` and `i32`, returns `x: i32`",
                "Function `x`, parameters `calculate` and `y`, returns `i32`",
                "Takes two i32s and returns nothing",
            ],
            0,
            "Read it as: fn says definition, `calculate` is the name, `(x: i32, y: i32)` is the named parameter list, `-> i32` is what it produces. Every Rust signature is this shape — name, typed parameters, optional return.",
        )
        .code("fn calculate(x: i32, y: i32) -> i32")
        .hints(&[
            "Ignore `fn` and the braces. What is the identifier before the parenthesis?",
            "Each comma-separated item inside the parentheses is `name: type`.",
            "The `->` introduces the return type and nothing else does.",
        ]),
        SeedExercise::new(
            Some(2),
            ExerciseKind::SignatureReading,
            "READ THE SIGNATURE — `fn find_user(id: u64) -> Option<String>`. What does this function produce?",
            &[
                "A u64",
                "An Option that is either Some(String) or None",
                "A String, always",
                "A Result that is either Ok or Err",
            ],
            1,
            "The arrow introduces the return type, and here it is `Option<String>` — a value that is either `Some(String)` or `None`. You do not need to know how Option works yet; recognising the shape is the whole point of this lesson.",
        )
        .code("fn find_user(id: u64) -> Option<String>")
        .hints(&[
            "What comes after the arrow is always the return type.",
            "Option<T> means 'a T, or nothing'. It is not Result.",
        ]),
        SeedExercise::new(
            Some(2),
            ExerciseKind::SignatureReading,
            "READ THE SIGNATURE — `fn save_user(user: &User) -> Result<(), DatabaseError>`. What is the function's name?",
            &["`save_user`", "`user`", "`User`", "`Result`"],
            0,
            "The name is the identifier immediately after `fn` and before the parenthesis. `user` is a parameter, `User` is a type, and `Result` is the return type.",
        )
        .code("fn save_user(user: &User) -> Result<(), DatabaseError>")
        .hints(&["Look at the very first word after `fn`."]),
        SeedExercise::new(
            Some(2),
            ExerciseKind::SignatureReading,
            "READ THE SIGNATURE — `fn is_valid(name: &str, age: u8) -> bool`. How many parameters does it take, and what type is the first one?",
            &[
                "One, and it is `&str`",
                "Two, and the first is `&str`",
                "Two, and the first is `bool`",
                "Three, including the return type",
            ],
            1,
            "Two parameters: `name: &str` and `age: u8`. The `-> bool` is the return type, not a third parameter — parameters live inside the parentheses, the return type lives after the arrow.",
        )
        .code("fn is_valid(name: &str, age: u8) -> bool")
        .hints(&[
            "Count the comma-separated items inside the parentheses only.",
            "The `-> bool` sits outside the parentheses entirely.",
        ]),
        SeedExercise::new(
            Some(2),
            ExerciseKind::SignatureReading,
            "READ THE SIGNATURE — `fn greet(name: &str)`. Does it produce a value?",
            &[
                "Yes — it produces a String",
                "No — there is no `->`, so it produces the unit type ()",
                "Yes — it produces a &str",
                "Only if it is called",
            ],
            1,
            "A signature with no arrow produces `()` — the unit type. The parameter `name` is an input, not an output, which is a common thing to misread.",
        )
        .code("fn greet(name: &str)")
        .hints(&["Parameters are inside the parentheses. The return type is after the arrow."]),
        SeedExercise::new(
            Some(2),
            ExerciseKind::SignatureReading,
            "READ THE SIGNATURE — `fn first_char(s: &str) -> Option<char>`. What does the function take?",
            &[
                "An Option<char>",
                "A borrowed string slice, `&str`",
                "A String",
                "Nothing",
            ],
            1,
            "Inside the parentheses is `s: &str` — a borrowed string slice. The `Option<char>` is what comes back out. Reading the two halves separately is the whole trick.",
        )
        .code("fn first_char(s: &str) -> Option<char>")
        .hints(&["Look only at what is inside the parentheses."]),
        SeedExercise::new(
            Some(2),
            ExerciseKind::SignatureReading,
            "READ THE SIGNATURE — `fn reset(state: &mut Vec<i32>)`. What is `&mut Vec<i32>`?",
            &[
                "A mutable reference to a vector of i32",
                "A vector that the function creates",
                "A vector that the function returns",
                "A function pointer",
            ],
            0,
            "`&` means reference — the function borrows. `mut` means it may write through it. `Vec<i32>` is a growable vector of i32. Together: it may modify the caller's vector in place.",
        )
        .code("fn reset(state: &mut Vec<i32>)")
        .hints(&["Read the symbols left to right: & then mut then the type."]),
        SeedExercise::new(
            Some(2),
            ExerciseKind::SignatureReading,
            "READ THE SIGNATURE — `fn area(w: f64, h: f64) -> f64`. What is the return type?",
            &["&f64", "f64", "(&f64, &f64)", "Nothing — there is no arrow"],
            0,
            "The `-> f64` is the return type. The `f64`s inside the parentheses are the parameter types, and they are a different thing entirely.",
        )
        .code("fn area(w: f64, h: f64) -> f64")
        .hints(&["What comes after the arrow?"]),
        SeedExercise::mcq(
            2,
            "What does the arrow in `-> i32` mean?",
            &[
                "The function accepts an i32",
                "The function produces an i32",
                "The function returns early at i32",
                "The function loops i32 times",
            ],
            1,
            "Parameters live inside the parentheses, with `name: type`. The arrow introduces the return type and only the return type.",
        ),

        // ── L4 Parameters and arguments ────────────────────────────────
        SeedExercise::new(
            Some(3),
            ExerciseKind::SignatureReading,
            "IDENTIFY THE PIECES — in `greet(\"Gus\")`, what is `\"Gus\"`?",
            &[
                "A parameter",
                "An argument — the value supplied at the call site",
                "The return type",
                "A macro",
            ],
            1,
            "Parameters are the named slots in the declaration (`name: &str`). Arguments are the actual values at the call. The parameter is a bucket; the argument is what you put in it.",
        )
        .code("fn greet(name: &str) {\n    println!(\"Hello, {}\", name);\n}\n\nfn main() {\n    greet(\"Gus\");\n}")
        .hints(&["Is this in the definition or the call?"]),
        SeedExercise::mcq(
            3,
            "In `fn add(a: i32, b: i32)`, how many parameters are there?",
            &["One", "Two", "Three", "It depends on the call"],
            1,
            "Two: `a: i32` and `b: i32`. The comma separates them. There is no third parameter for the return type — that lives after the arrow.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n\nfn main() {\n    println!(\"{}\", add(10, 20));\n}")
        .with_hints(&["a = 10, b = 20, so what is a + b?"]),
        SeedExercise::mcq(
            3,
            "Why does `add(10, \"20\")` fail to compile?",
            &[
                "Because you cannot pass two arguments",
                "Because the second parameter is i32 and you gave it a string",
                "Because add returns i32",
                "Because strings cannot be arguments",
            ],
            1,
            "Types must match at the call site. The error points at `\"20\"` and names the parameter, because the signature promised an i32 there.",
        )
        .kind(ExerciseKind::FixCode)
        .with_code("fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n\nfn main() {\n    let sum = add(10, \"20\");\n    println!(\"{}\", sum);\n}")
        .with_hints(&["What type does the second parameter promise?"]),
        SeedExercise::mcq(
            3,
            "Does Rust let you call `add(20, 10)` when both parameters are i32?",
            &[
                "No — arguments must be in ascending order",
                "Yes — the types match, so it compiles and gives the same result",
                "No — parameters are positional and order is fixed",
                "Only if the function is marked immutable",
            ],
            1,
            "Rust cannot tell you probably meant to swap them, because with two identical types there is nothing technically wrong. The danger is when order carries meaning the types do not.",
        ),

        // ── L5 Return values ───────────────────────────────────────────
        SeedExercise::mcq(
            4,
            "Why does this fail?\n\nfn add() -> i32 {\n    10 + 5;\n}",
            &[
                "The body is too short",
                "The semicolon makes the block produce (), not the i32 promised",
                "You need parentheses around the expression",
                "The function name is wrong",
            ],
            1,
            "A trailing semicolon turns an expression into a statement and discards its value. The block then produces `()`, and the signature wanted `i32`. The compiler's help line says exactly this.",
        )
        .kind(ExerciseKind::FixCode)
        .with_code("fn add() -> i32 {\n    10 + 5;\n}\n\nfn main() {\n    println!(\"{}\", add());\n}")
        .with_hints(&["One character, on the second line"]),
        SeedExercise::mcq(
            4,
            "In `let result = add(10, 20);` where does the value 30 come from?",
            &[
                "It is printed by add",
                "It is the final expression of add's body, returned and then bound to result",
                "It is the default value of result",
                "It is computed twice",
            ],
            1,
            "add's body ends with `a + b` and no semicolon, so the block produces 30, the function produces 30, and the `let` stores it.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn add_one(x: i32) -> i32 {\n    x + 1\n}\n\nfn main() {\n    let x = add_one(5);\n    println!(\"{}\", x);\n}")
        .with_hints(&["What is 5 + 1?"]),
        SeedExercise::mcq(
            4,
            "What is the difference between these two?\n\nfn a() -> i32 { 10 }\nfn b() -> i32 { 10; }",
            &["No difference", "a returns 10, b returns () and does not compile", "a does not compile", "b returns 10"],
            1,
            "The semicolon is the entire difference. `10` is an expression and becomes the block's value. `10;` is a statement and produces nothing, so the signature is unfulfilled.",
        ),
        SeedExercise::mcq(
            4,
            "Why must every route through a value-returning function produce the value?",
            &[
                "For readability",
                "Because otherwise the function could fall off the end with nothing, and the compiler checks this",
                "Because Rust requires exhaustive matches",
                "It does not — the function can just return nothing",
            ],
            1,
            "If any path could fall off the end without producing a value, the function would be lying about its signature. The compiler catches that before you ship it.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn first_even(n: &[i32]) -> Option<i32> {\n    for n in n {\n        if n % 2 == 0 {\n            return Some(*n);\n        }\n    }\n    None\n}\n\nfn main() {\n    match first_even(&[1, 3, 8]) {\n        Some(n) => println!(\"{}\", n),\n        None => println!(\"none\"),\n    }\n}")
        .with_hints(&["Where does the early return fire?"]),
        SeedExercise::mcq(
            4,
            "A function returns `()`, and you write `let x = f();`. What is x?",
            &["the return value of the last println", "()", "a String", "It does not compile"],
            1,
            "Everything returns something. A function that produces no value produces `()`, and that is what gets bound. The call is perfectly legal.",
        )
        .kind(ExerciseKind::FillGap)
        .with_code("fn triple(x: i32) -> i32 {\n    x * 3\n}\n\nfn main() {\n    let result = triple(____);\n    println!(\"{}\", result);\n}")
        .with_hints(&["What argument gives you 21?"]),

        // ── L6 Unit type ───────────────────────────────────────────────
        SeedExercise::mcq(
            5,
            "What is the type of the result of `let x = greet();` where greet only prints?",
            &["String", "i32", "()", "bool"],
            2,
            "greet has no return type, so it produces `()` — the unit type. It is Rust's way of saying 'this ran, and there is no value here'.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn greet(name: &str) {\n    println!(\"Hello, {}\", name);\n}\n\nfn main() {\n    let result = greet(\"Gus\");\n    println!(\"{:?}\", result);\n}")
        .with_hints(&["{:?} on a function that returns nothing"]),
        SeedExercise::mcq(
            5,
            "Why does `fn log(s: &str) -> ()` compile even though nothing is returned?",
            &[
                "The arrow is optional when there is a semicolon",
                "`()` explicitly declares 'this produces nothing', which is legal",
                "The compiler ignores return types on logging functions",
                "It does not compile",
            ],
            1,
            "`-> ()` is just an explicit spelling of the default. It is legal and it compiles. You should still usually omit it, because it is noise — but the compiler accepts it.",
        )
        .kind(ExerciseKind::FillGap)
        .with_code("fn setup() ____ {\n    println!(\"ready\");\n}\n\nfn main() {\n    setup();\n}")
        .with_hints(&["The function genuinely does produce something — the unit type", "Same thing, named"]),
        SeedExercise::mcq(
            5,
            "What does `Result<(), Error>` mean?",
            &[
                "It returns no value at all",
                "It returns either success with no payload, or an error",
                "It returns an optional Error",
                "It returns a tuple of two Errors",
            ],
            1,
            "The `()` is the success payload — 'it worked, and there is no data'. It looks odd and it is not. We will meet it properly in the error handling module.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn setup() -> () {\n    println!(\"ready\");\n}\n\nfn main() {\n    let x = setup();\n    println!(\"{:?}\", x);\n}")
        .with_hints(&["{:?} prints the value that was bound"]),

        // ── L7 Expressions inside ──────────────────────────────────────
        SeedExercise::mcq(
            6,
            "What is the value of this function?\n\nfn calculate() -> i32 {\n    let x = 10;\n    let y = 20;\n    x + y\n}",
            &["0", "30", "()", "It does not compile"],
            1,
            "The two `let` lines are statements. The final `x + y` has no semicolon, so it is the block's value, and the block is the function's value.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn choose(condition: bool) -> i32 {\n    if condition {\n        10\n    } else {\n        20\n    }\n}\n\nfn main() {\n    println!(\"{}\", choose(true));\n    println!(\"{}\", choose(false));\n}")
        .with_hints(&["if is an expression — which branch matches?"]),
        SeedExercise::mcq(
            6,
            "What does this function return?\n\nfn calculate() -> i32 {\n    let result = {\n        let a = 10;\n        a + 5\n    };\n    result\n}",
            &["15", "()", "10", "It does not compile"],
            0,
            "The inner block ends with `a + 5` and no semicolon, so it produces 15. That is bound to `result`, and `result` is the final expression of the function.",
        )
        .kind(ExerciseKind::FillGap)
        .with_code("fn calculate() -> i32 {\n    let a = 10;\n\n    a ____ 5\n}")
        .with_hints(&["The function promises an i32", "No semicolon on the last line"]),
        SeedExercise::mcq(
            6,
            "Why does this not compile?\n\nfn get_value() -> i32 {\n    1 + 1;\n}",
            &[
                "The addition is wrong",
                "The semicolon makes the block produce (), not i32",
                "A function cannot return an integer literal",
                "It compiles fine",
            ],
            1,
            "The `;` turns `1 + 1` into a statement, so the block's value becomes `()`. Delete the semicolon and it compiles. This is the same error as the 'expected i32, found ()' case in module 02 — it just wears different clothes.",
        )
        .kind(ExerciseKind::SpotDifference)
        .with_code("fn get_value() -> i32 {\n    1 + 1;\n}")
        .with_code_b("fn get_value() -> i32 {\n    1 + 1\n}")
        .with_hints(&["One character is different", "The compiler will tell you which"]),
        SeedExercise::mcq(
            6,
            "What is the value of `x`?\n\nfn main() {\n    let x = {\n        let y = 2;\n        y * y;\n    };\n    println!(\"{}\", x);\n}",
            &["4", "()", "2", "It does not compile"],
            1,
            "The semicolon on `y * y` discards the value, so the block produces `()`. `x` is bound to `()`. Remove the semicolon and you get 4.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn classify(n: i32) -> &'static str {\n    if n % 2 == 0 {\n        \"even\"\n    } else {\n        \"odd\"\n    }\n}\n\nfn main() {\n    println!(\"{}\", classify(4));\n    println!(\"{}\", classify(7));\n}")
        .with_hints(&["if is an expression, so it can be a value", "Two branches, one string each"]),

        // ── L8 Explicit return ─────────────────────────────────────────
        SeedExercise::mcq(
            7,
            "What does `return false;` do inside `if age < 18 { }`?",
            &[
                "Sets a flag for later",
                "Exits the function immediately with the value false, skipping everything below",
                "Returns false from the if, then continues",
                "Loops until the condition is false",
            ],
            1,
            "An early return leaves the whole function, not just the if. That is exactly what makes it useful for skipping the rest of a long function.",
        )
        .kind(ExerciseKind::SpotDifference)
        .with_code("fn check(age: u32) -> bool {\n    if age < 18 {\n        return false;\n    }\n    true\n}")
        .with_code_b("fn check(age: u32) -> bool {\n    if age < 18 {\n        false\n    } else {\n        true\n    }\n}")
        .with_hints(&["Both return the same values — what is actually different?"]),
        SeedExercise::mcq(
            7,
            "Which style is the better choice for a 40-line validation function?",
            &[
                "Always the final-expression style",
                "Always the early-return style",
                "Early returns, because the main path stays flat and unindented",
                "Neither — split it into more functions",
            ],
            2,
            "Not a law — a judgement call. Early returns stop each guard from nesting another level, which is what keeps long functions readable.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn f(n: i32) -> &'static str {\n    if n < 0 {\n        return \"neg\";\n    }\n    if n == 0 {\n        return \"zero\";\n    }\n    \"pos\"\n}\n\nfn main() {\n    for n in [-1, 0, 1] {\n        println!(\"{}\", f(n));\n    }\n}")
        .with_hints(&["Each return exits immediately"]),
        SeedExercise::mcq(
            7,
            "What does this print?\n\nfn f() -> &'static str {\n    return \"a\";\n}",
            &["a", "Nothing — the value is discarded", "An error", "()"],
            1,
            "`return \"a\";` exits the function with the value `a`, and then the function has nothing left to do. There is no final expression after it, so the function body evaluates to `()` where `&str` was promised — which is a type error. The `return` is fine; the missing final expression is not.",
        )
        .kind(ExerciseKind::FixCode)
        .with_code("fn double(n: i32) -> i32 {\n    if n < 0 {\n        return -n;\n    };\n\n    n * 2\n}")
        .with_hints(&[
            "Compare the early-return arm with the expression that follows the if",
            "One arm uses return, the other uses the block's value. Pick one style for the whole function",
        ]),
        SeedExercise::mcq(
            7,
            "A function that returns `Option<&str>` — which call sites need a `?` or an `if let`?",
            &[
                "None of them; the Option can be used directly",
                "Every site that uses the value, because the Option may hold None",
                "Only the sites that print",
                "Only inside a match",
            ],
            1,
            "The return type is a contract, and this one says the answer might not exist. A caller that assumes a value has to deal with the None case — that is the entire reason Option is in the signature.",
        )
        .kind(ExerciseKind::BuildIt)
        .with_hints(&[
            "Write the signature with -> Option<&str> first",
            "Then find every place the value is used and ask what None means there",
        ]),

        // ── L9 Scope ───────────────────────────────────────────────────
        SeedExercise::mcq(
            8,
            "Why does `println!(\"{}\", y)` fail after the inner block closes?",
            &[
                "y was moved",
                "y belongs to the inner block's scope and no longer exists outside it",
                "y is shadowed by x",
                "println! cannot read block variables",
            ],
            1,
            "A variable belongs to the scope where it was created. In Rust an out-of-scope name does not exist at all, so there is no garbage value to read by accident.",
        )
        .kind(ExerciseKind::FixCode)
        .with_code("fn main() {\n    let x = 10;\n    {\n        let y = 20;\n        println!(\"{} {}\", x, y);\n    }\n    println!(\"{}\", y);\n}")
        .with_hints(&["Where is y declared?", "Either move the print, or make y visible"]),
        SeedExercise::mcq(
            8,
            "Why can `greet()` not use a `name` declared in `main`?",
            &[
                "Because name is a String",
                "Because a function body's scope is separate — the function must be passed what it needs",
                "Because main runs first",
                "Because functions cannot read variables",
            ],
            1,
            "A function should be readable on its own. If it reached into main's scope, you would need to know what main contains to understand greet. Passing it in makes the dependency visible.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    let x = 1;\n    {\n        let x = 2;\n        println!(\"{}\", x);\n    }\n    println!(\"{}\", x);\n}")
        .with_hints(&["Shadowing inside a block does not escape the block"]),
        SeedExercise::mcq(
            8,
            "What does this print?\n\nfn main() {\n    let x = 10;\n    {\n        let x = x + 5;\n        println!(\"{}\", x);\n    }\n    println!(\"{}\", x);\n}",
            &["15 then 10", "15 then 15", "10 then 15", "It does not compile"],
            0,
            "The inner `let x = x + 5` creates a *new* x for the inner block, and the right-hand side reads the outer one — 10 plus 5. The inner block ends, the new x dies with it, and the outer x is still 10.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    let data = vec![1, 2, 3];\n    {\n        let first = &data[0];\n        println!(\"{}\", first);\n    }\n    println!(\"{:?}\", data);\n}")
        .with_hints(&[
            "first is a borrow, and the borrow ends at the close of the block",
            "So data is free to be printed afterwards",
        ]),
        SeedExercise::new(
            Some(8),
            ExerciseKind::PredictOwnership,
            "PREDICT OWNERSHIP — what must the caller of `fn shout(message: String)` own?",
            &[
                "A `mut` binding, because println! writes to it",
                "The only thing that matters is whether the variable is `mut`",
                "A String it is willing to give up — a non-mut binding is fine",
                "A `&str`, because String cannot be passed",
            ],
            2,
            "A `String` parameter takes ownership, so the caller hands it over and no longer has it. That is a valid function; it is just a different contract from `&String`. What matters is that the signature and the call site agree.",
        )
        .code("fn shout(message: String) {\n    println!(\"{}\", message.to_uppercase());\n}\n\nfn main() {\n    let m = String::from(\"hey\");\n    shout(m);\n    // can we use m here?\n}")
        .xp(15)
        .hints(&[
            "Look at the parameter type: String or &String?",
            "If it is String, what happens to the caller's binding afterwards?",
        ]),

        // ── L10 Ownership across calls ─────────────────────────────────
        SeedExercise::new(
            Some(9),
            ExerciseKind::PredictOwnership,
            "PREDICT OWNERSHIP — does `name` survive the call to `consume_name`?",
            &[
                "Yes — the function only read it",
                "No — the String was moved into the function, so the final println! does not compile",
                "Yes, but name is now empty",
                "The program compiles but prints nothing",
            ],
            1,
            "A `String` parameter takes ownership. `consume_name` is now responsible for `name` and drops it at the end of its scope. The caller's `name` is gone, so the final `println!` is a borrow of a moved value. Taking `&String` instead fixes it.",
        )
        .code("fn consume_name(name: String) {\n    println!(\"{}\", name);\n}\n\nfn main() {\n    let name = String::from(\"Gus\");\n    consume_name(name);\n    println!(\"{}\", name);\n}")
        .xp(15)
        .hints(&[
            "Look at the parameter type. Is it `String` or `&String`?",
            "`&` means borrow. No `&` means move. Which is it here?",
        ]),
        SeedExercise::new(
            Some(9),
            ExerciseKind::PredictOwnership,
            "PREDICT OWNERSHIP — which change makes this program compile?",
            &[
                "Change nothing — it already compiles",
                "Change the parameter to `&String`",
                "Add `mut` to the parameter",
                "Add a `-> String` to the signature",
            ],
            1,
            "The caller passes `&name`, so the function must accept `&String`. When the call site and the signature disagree about borrowing, the compiler complains — which is exactly the protection you want.",
        )
        .code("fn consume(name: String) {\n    println!(\"{}\", name);\n}\n\nfn main() {\n    let name = String::from(\"Gus\");\n    consume(&name);\n    println!(\"{}\", name);\n}")
        .xp(15)
        .hints(&["The call already says `&name`. Does the signature agree?"]),
        SeedExercise::new(
            Some(9),
            ExerciseKind::PredictOwnership,
            "PREDICT OWNERSHIP — what happens to `s` after `println!(\"{}\", s);`?",
            &[
                "It is moved and gone",
                "It is still usable — println! borrows rather than consumes",
                "It becomes empty",
                "It is copied",
            ],
            1,
            "`println!` takes its arguments by reference, so reading a value does not take ownership of it. That is why printing a String does not stop you using it, while passing it to a `String` parameter does.",
        )
        .code("fn main() {\n    let s = String::from(\"hello\");\n    println!(\"{}\", s);\n    println!(\"done\");\n}")
        .xp(15)
        .hints(&["Compare a macro argument with a function parameter."]),
        SeedExercise::new(
            Some(9),
            ExerciseKind::PredictOwnership,
            "PREDICT OWNERSHIP — does this compile?",
            &[
                "Yes — the value is returned so it comes back",
                "No — the returned value is discarded and the caller has lost its String",
                "Yes — s is automatically copied",
                "Yes, but s is empty afterwards",
            ],
            1,
            "The value comes back from the function, but the call site throws that return value away. `s` itself was moved on the way in and is gone. Bind the result if you want it back.",
        )
        .code("fn take(s: String) -> String {\n    s\n}\n\nfn main() {\n    let s = String::from(\"a\");\n    take(s);\n    println!(\"{}\", s);\n}")
        .xp(15)
        .hints(&["Moving in and getting something back are separate things. Does the call bind it?"]),
        SeedExercise::mcq(
            9,
            "When should a function take `String` by value rather than by reference?",
            &[
                "Always — it is faster",
                "When it needs to keep, store, or return the value",
                "Only for small strings",
                "Never",
            ],
            1,
            "Take by value when the function genuinely owns the data afterwards — storing it in a struct, returning it, or consuming it. Take `&String` when the function only reads it, which is most of the time.",
        ),
        SeedExercise::mcq(
            9,
            "Which signature should a function use if it only reads a String?",
            &["`name: String`", "`name: &String`", "`name: &mut String`", "`name: -> String`"],
            1,
            "Take by reference when you only read. `String` would take ownership and leave the caller unable to use theirs; `&mut` would promise a change you are not making.",
        )
        .kind(ExerciseKind::FixCode)
        .with_code("fn shout(name: String) {\n    println!(\"{}\", name.to_uppercase());\n}\n\nfn main() {\n    let name = String::from(\"gus\");\n    shout(name);\n    println!(\"{}\", name);\n}")
        .with_hints(&["The function only prints — it does not keep the string", "One type on the parameter, one ampersand at the call"]),

        // ── L11 Mutable parameters ─────────────────────────────────────
        SeedExercise::mcq(
            10,
            "What does `fn double(value: &mut i32)` declare?",
            &[
                "That the function will return a new i32",
                "That the function receives a mutable reference and may write through it",
                "That the function copies the value",
                "That the parameter is optional",
            ],
            1,
            "`&mut` is a mutable reference — permission to write. It is a request, and the caller must agree by holding a `mut` variable and passing `&mut`.",
        )
        .kind(ExerciseKind::FixCode)
        .with_code("fn double(value: &mut i32) {\n    *value *= 2;\n}\n\nfn main() {\n    let number = 10;\n    double(&mut number);\n    println!(\"{}\", number);\n}")
        .with_hints(&["The error suggests adding one word to the `let`", "There is no value to lend mutably if it is not declared mutable"]),
        SeedExercise::mcq(
            10,
            "What does the `*` in `*value *= 2;` do?",
            &[
                "Multiplies by pointer address",
                "Dereferences — value is a &mut i32, and the star reaches the i32 behind it",
                "Is multiplication",
                "Marks the line as important",
            ],
            1,
            "`value` inside the function is a reference. `*value` is the integer it points at, and that is what the arithmetic needs.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn double(v: &mut i32) {\n    *v *= 2;\n}\n\nfn main() {\n    let mut n = 10;\n    double(&mut n);\n    double(&mut n);\n    println!(\"{}\", n);\n}")
        .with_hints(&["10 doubled twice"]),
        SeedExercise::mcq(
            10,
            "What is required at the call site for a `&mut` parameter?",
            &[
                "The variable must be declared `mut` and passed as `&mut`",
                "Only the `mut` on the variable",
                "Only the `&mut` in the call",
                "Nothing — the function handles it",
            ],
            0,
            "Both. The variable must be mutable, and the call must pass `&mut`. A function that mutates something always looks like it, which is a genuinely useful property.",
        ),

        // ── L12 Functions calling functions ────────────────────────────
        SeedExercise::mcq(
            11,
            "What does `main` do in a program where main calls calculate_total and then print_total?",
            &[
                "It performs the arithmetic and the formatting itself",
                "It orchestrates — calling helpers and stitching the result between them",
                "Nothing; it is required but ignored",
                "It converts the arguments to the right types",
            ],
            1,
            "That is the shape to aim for. `main` reads like a summary of the program, and every detail lives in a function you can find by name.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn apply_discount(p: f64) -> f64 { p * 0.9 }\nfn add_tax(p: f64) -> f64 { p * 1.2 }\n\nfn checkout(sub: f64) -> f64 {\n    add_tax(apply_discount(sub))\n}\n\nfn main() {\n    println!(\"{}\", checkout(100.0));\n}")
        .with_hints(&["Discount first, then tax — 100 becomes 90 becomes 108"]),
        SeedExercise::mcq(
            11,
            "Why is it good for a function to be readable without reading the ones above it?",
            &[
                "It compiles faster",
                "You can reason about it in isolation, and reuse it without dragging its context along",
                "Rust requires it",
                "It uses less memory",
            ],
            1,
            "If a function only makes sense after reading three others, the boundaries are in the wrong place. Independence is what makes reuse and testing possible.",
        )
        .kind(ExerciseKind::Refactor)
        .with_code("// Which helper is doing too much, and what would you split it into?\nfn report(items: &[f64]) -> String {\n    let mut total = 0.0;\n    for i in items { total += i; }\n    let taxed = total * 1.2;\n    format!(\"Total: {:.2}\", taxed)\n}")
        .with_hints(&["Try describing it in one sentence without using 'and'"]),
        SeedExercise::mcq(
            11,
            "What is the main benefit of small functions calling each other?",
            &[
                "The program uses less memory",
                "Each piece can be read, tested, and changed on its own",
                "It compiles faster",
                "It removes the need for comments",
            ],
            1,
            "That is the whole argument for decomposition. Independence is what makes a change to one piece provably safe for the others.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn add_tax(p: f64) -> f64 { p * 1.2 }\nfn apply_discount(p: f64) -> f64 { p * 0.9 }\n\nfn checkout(sub: f64) -> f64 {\n    add_tax(apply_discount(sub))\n}\n\nfn main() {\n    println!(\"{}\", checkout(100.0));\n}")
        .with_hints(&["Which call happens first?"]),
        SeedExercise::mcq(
            11,
            "Which of these should hold for a function you want to reuse?",
            &[
                "It must take exactly three arguments",
                "It should be readable on its own and take everything it needs as arguments",
                "It must be under ten lines",
                "It must be private",
            ],
            1,
            "Readability in isolation and explicit inputs are what make reuse possible. Hidden dependencies on a surrounding scope are what prevent it.",
        ),

        // ── L13 Designing good functions ───────────────────────────────
        SeedExercise::mcq(
            12,
            "What is the 'and' test?",
            &[
                "A function should never use the word and",
                "If you cannot describe a function in one sentence without 'and', it is doing more than one thing",
                "Functions should have fewer than five lines",
                "Every function should have an argument",
            ],
            1,
            "It is a far better signal than line count. A four-line function doing two small things is often clearer as one; a four-line function doing two large things is not.",
        ),
        SeedExercise::mcq(
            12,
            "Is splitting a function up to become smaller always the right move?",
            &[
                "Yes, always",
                "No — split when the name is unclear or the logic is tangled, not merely to reduce line count",
                "Yes, if it goes under ten lines",
                "Only if the function takes arguments",
            ],
            1,
            "Single responsibility is a heuristic, not a law. Over-splitting produces a maze of one-line functions that are harder to follow than the thing you started with.",
        ),
        SeedExercise::mcq(
            12,
            "What does a good function name buy you?",
            &[
                "Faster compilation",
                "The call site becomes readable as a summary, without opening the body",
                "Fewer arguments",
                "Automatic documentation",
            ],
            1,
            "Names are the cheapest documentation in programming. If reading `checkout(subtotal)` tells you the shape of the calculation, the name has done its job.",
        ),
        SeedExercise::mcq(
            12,
            "You have a 15-line function that reads a file, parses it, and prints a report. What first?",
            &[
                "Split it into three functions immediately",
                "Try to describe it in one sentence. If that needs 'and', split; if not, it may be fine",
                "Make it a method",
                "Move it to another module",
            ],
            1,
            "The test is the name, not the length. A 15-line function that reads a file and prints it can be perfectly coherent. Reach for the knife when the description is tangled.",
        )
        .kind(ExerciseKind::Refactor)
        .with_code("// Which of these is doing more than one job?\nfn handle(data: &str) -> String {\n    let trimmed = data.trim();\n    let upper = trimmed.to_uppercase();\n    format!(\"[{}]\", upper)\n}")
        .with_hints(&["clean, shout, wrap — three verbs hiding in one function"]),
        SeedExercise::mcq(
            12,
            "Why is over-splitting a real risk?",
            &[
                "It makes the binary larger",
                "A maze of one-line functions is harder to follow than one clear function that does two things",
                "Rust forbids it",
                "It runs slower",
            ],
            1,
            "Single responsibility is a heuristic. Applied without judgement it produces `fn part_one()` and `fn part_two()` and a reader who has to stitch it back together anyway.",
        ),

        // ── L14 Recursion ──────────────────────────────────────────────
        SeedExercise::mcq(
            13,
            "What happens if a recursive function has no reachable base case?",
            &[
                "It returns None",
                "It recurses until the stack overflows, then panics",
                "The compiler rejects it",
                "It runs forever without using memory",
            ],
            1,
            "Each recursive call takes a stack frame. With no base case, frames accumulate until the stack runs out. The compiler cannot prove your base case is reachable, so it cannot help here.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn countdown(n: i32) {\n    if n == 0 { return; }\n    println!(\"{}\", n);\n    countdown(n - 1);\n}\n\nfn main() {\n    countdown(3);\n}")
        .with_hints(&["3, 2, 1 — then 0 stops it"]),
        SeedExercise::mcq(
            13,
            "In Rust, when is recursion usually NOT the best answer?",
            &[
                "When the data is tree-shaped",
                "When the problem is a flat list — a loop or iterator is usually clearer and avoids stack growth",
                "When the function has more than one parameter",
                "Never — recursion is always preferred",
            ],
            1,
            "The borrow checker pushes you away from recursion on owned data, and rightly so. Recursion fits naturally on trees and graphs. A flat list is a loop.",
        )
        .kind(ExerciseKind::FixCode)
        .with_code("fn countdown(n: i32) {\n    println!(\"{}\", n);\n    countdown(n - 1);   // no base case\n}")
        .with_hints(&["What stops it from reaching zero?", "Add a condition that returns"]),
        SeedExercise::mcq(
            13,
            "What does `factorial(0)` return in a correct recursive implementation?",
            &[
                "It recurses forever",
                "0",
                "1 — because 0! is defined as 1, and the base case is where that comes from",
                "It panics",
            ],
            2,
            "The base case exists precisely to stop the recursion, and mathematicians defined 0! as 1 precisely so it would. Without the `if n == 0` arm, there is no value to return and the function has no exit.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn sum_to(n: u32) -> u32 {\n    if n == 0 {\n        return 0;\n    }\n    n + sum_to(n - 1)\n}\n\nfn main() {\n    println!(\"{}\", sum_to(4));\n}")
        .with_hints(&[
            "Work outwards: sum_to(1) = 1 + sum_to(0) = 1 + 0",
            "sum_to(4) = 4 + 3 + 2 + 1 + 0",
        ]),
        SeedExercise::mcq(
            13,
            "A recursive function over a `Vec<i32>` that never clones. What happens to the Vec at each level?",
            &[
                "Nothing — it is shared",
                "Each recursive call takes ownership, so the Vec is moved and you cannot reuse it in the loop that called it",
                "The Vec is copied automatically",
                "It is freed after the first call",
            ],
            1,
            "This is the concrete reason Rust programmers reach for a loop on owned flat data. Recursion wants to consume the value, and the loop that drives the recursion still needs it. A `&[i32]` parameter and a slice avoid the fight.",
        )
        .kind(ExerciseKind::Refactor)
        .with_code("fn total(values: Vec<i32>) -> i32 {\n    if values.is_empty() {\n        return 0;\n    }\n    values[0] + total(values[1..].to_vec())\n}")
        .with_hints(&[
            "This works and it allocates a new Vec on every call",
            "Compare with a loop that borrows the slice — one pass, no allocation, no stack growth",
        ]),

        // ── L15 Common mistakes ────────────────────────────────────────
        SeedExercise::mcq(
            14,
            "error[E0061]: this function takes 2 arguments but 1 was supplied. What is wrong?",
            &[
                "add is not defined",
                "The call gave one argument where the signature requires two",
                "The arguments are the wrong type",
                "add is private",
            ],
            1,
            "Rust does not fill in defaults. A function that sometimes receives one argument and sometimes two is carrying a hidden precondition.",
        )
        .kind(ExerciseKind::CompilerChallenge)
        .with_code("error[E0061]: this function takes 2 arguments but 1 argument was supplied\n --> src/main.rs:6:17\n   |\n6  |     let sum = add(10);\n   |                 ^^^ -- argument to this function\n   |                 |\n   |                 expected 2 arguments")
        .with_hints(&["Count what the call gives and what the signature wants"]),
        SeedExercise::mcq(
            14,
            "What does 'expected `i32`, found `()`' almost always mean?",
            &[
                "A type is misspelled",
                "A semicolon on a final expression discarded the value",
                "The function is missing",
                "An import is wrong",
            ],
            1,
            "It is the accidental-unit-return error. Look at where the caret points — there will be a semicolon on an expression you expected to produce a value.",
        )
        .kind(ExerciseKind::CompilerChallenge)
        .with_code("error[E0308]: mismatched types\n --> src/main.rs:2:5\n  |\n2 |     10;\n  |     ^^ expected `i32`, found `()`\n  |     |\n  |     this expression has type `()`\nhelp: if this is a `return` expression, remove the semicolon")
        .with_hints(&["The help line tells you exactly what to do", "One character"]),
        SeedExercise::mcq(
            14,
            "In 'borrow of moved value', the message points at two places. What do they mean?",
            &[
                "The first is the function, the second is the error",
                "The first marks where the value moved; the second marks where you used it afterwards",
                "The first is the old value, the second is the new one",
                "They are the same line",
            ],
            1,
            "Rust is drawing you a line between the move and the use. That is almost always enough to find the bug without reading the whole function.",
        )
        .kind(ExerciseKind::SpotDifference)
        .with_code("fn take(s: String) { println!(\"{}\", s); }\nfn main() {\n    let s = String::from(\"x\");\n    take(s);\n    println!(\"{}\", s);\n}")
        .with_code_b("fn take(s: &String) { println!(\"{}\", s); }\nfn main() {\n    let s = String::from(\"x\");\n    take(&s);\n    println!(\"{}\", s);\n}")
        .with_hints(&["One parameter type and one call changed"]),
        SeedExercise::mcq(
            14,
            "Which change fixes 'cannot borrow as mutable, as it is not declared as mutable'?",
            &[
                "Add `mut` to the variable declaration",
                "Remove `&mut` from the call",
                "Add `-> i32` to the function",
                "Change the parameter to `&i32`",
            ],
            0,
            "You cannot lend out write permission for something that was never declared writable. The `let` needs `mut`; the parameter and the call already agree on `&mut`.",
        ),

        // ── L16 Putting it together ────────────────────────────────────
        SeedExercise::mcq(
            15,
            "Why is a function's return type a contract the compiler can trust?",
            &[
                "Because it is checked at runtime",
                "Because the compiler verifies the body actually produces that type on every path",
                "Because Rust tests it automatically",
                "Because of the documentation",
            ],
            1,
            "If the signature says `-> i32` and any path through the body produces something else, the program does not compile. That is why you can read a signature and trust it without reading the body.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn discount(member: bool) -> f64 {\n    if member { 0.10 } else { 0.0 }\n}\n\nfn apply(total: f64, rate: f64) -> f64 {\n    total * (1.0 - rate)\n}\n\nfn main() {\n    println!(\"{}\", apply(100.0, discount(true)));\n}")
        .with_hints(&["discount(true) is 0.10, so what is 100 * (1 - 0.10)?"]),
        SeedExercise::mcq(
            15,
            "What comes immediately after Functions in the dependency chain?",
            &["Traits", "Ownership", "Generics", "Async"],
            1,
            "Functions → Ownership → Borrowing → Structs → Enums → (methods) → Traits → Generics. Each module builds on the vocabulary the previous one gave you.",
        ),
        SeedExercise::new(
            Some(15),
            ExerciseKind::SignatureReading,
            "READ THE SIGNATURE — `fn process(items: &mut Vec<i32>, factor: f64) -> usize`. What is the return type, and what does the `&mut` on the first parameter allow?",
            &[
                "usize; the function may modify the caller's vector in place",
                "&mut Vec<i32>; the function returns a reference",
                "Vec<i32>; the function creates and returns a vector",
                "f64; the factor is the return value",
            ],
            0,
            "The arrow gives `usize`, a plain count. The `&mut` is on the parameter, not the return, and it means the function may write through it — change the caller's vector. Reading the two halves separately prevents exactly this confusion.",
        )
        .code("fn process(items: &mut Vec<i32>, factor: f64) -> usize")
        .xp(15)
        .hints(&[
            "What comes after the arrow is the return type.",
            "The &mut is inside the parentheses, so it belongs to the parameter.",
        ]),
        SeedExercise::new(
            Some(15),
            ExerciseKind::PredictOwnership,
            "PREDICT OWNERSHIP — `process` takes `&mut Vec<i32>`. What must the caller have?",
            &[
                "A `mut` binding and an `&mut` at the call",
                "Nothing special — &mut on the parameter is enough",
                "Only a `mut` binding",
                "Only `&mut` at the call",
            ],
            0,
            "Both sides have to agree. The variable must be declared `mut` and the call must pass `&mut`. A function that mutates always looks like it, which is what makes side effects findable by reading signatures alone.",
        )
        .code("fn process(items: &mut Vec<i32>) -> usize {\n    items.push(0);\n    items.len()\n}\n\nfn main() {\n    let v = vec![1, 2, 3];\n    println!(\"{}\", process(&mut v));\n}")
        .xp(15)
        .hints(&[
            "The error you would get mentions the variable's declaration, not the call.",
            "There is no write permission to lend if the variable is not mutable.",
        ]),

        // ── Open-ended builds ──────────────────────────────────────────
        SeedExercise::new(
            None,
            ExerciseKind::BuildIt,
            "BUILD IT — Write a signature for every job in an order-processing function, no bodies yet.",
            &[
                "Write a real signature for each job — parameters and return types, no bodies",
                "Ask of each one: can I name it without the word 'and'?",
                "Two of them probably cannot. That is worth noticing before you write the bodies.",
            ],
            0,
            "This is the first half of decomposition and it is the harder half. Most people write the bodies first and end up with a function nobody can name.",
        )
        .hints(&[
            "Write a signature for each job, no bodies",
            "Ask of each: can I name it without 'and'?",
        ]),
        SeedExercise::new(
            None,
            ExerciseKind::BuildIt,
            "BUILD IT — Build a four-function pipeline so that main becomes a single call.",
            &[
                "Write `parse_price(text: &str) -> f64`",
                "Write `apply_tax(price: f64) -> f64`",
                "Write `format_currency(amount: f64) -> String`",
                "Write `receipt(price: f64) -> String` that composes the other three",
                "Add a fifth function `process(text: &str) -> String` so main is one call",
            ],
            0,
            "Count the total lines. Compare that to writing it all inline — the arithmetic is the same, but now every piece has a name you can test on its own.",
        )
        .hints(&[
            "parse → transform → format, one function each",
            "Then one function that takes raw input and produces the final line",
        ]),
    ]
}
