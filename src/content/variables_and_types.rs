//! Module 02 — Variables & Types
//!
//! Teaches how Rust represents information: bindings, mutability, inference,
//! annotations, scalars, compounds, and conversions.

use super::types::{
    callout, code, diagram, paragraphs, steps, table, Difficulty, ExerciseKind, SeedExercise,
    SeedLesson, SeedModule, SeedProject,
};

pub fn content() -> SeedModule {
    SeedModule::new(
        "variables-and-types",
        "Variables & Types",
        "How Rust stores information, why variables are immutable by default, and how the type system earns its keep.",
        lessons(),
        exercises(),
        projects(),
    )
}

// ─── LESSONS ───────────────────────────────────────────────────────────────

fn lessons() -> Vec<SeedLesson> {
    vec![
        lesson_what_is_a_variable(),
        lesson_immutable_by_default(),
        lesson_mut(),
        lesson_why_immutable(),
        lesson_type_inference(),
        lesson_type_annotations(),
        lesson_integer_types(),
        lesson_integer_literals(),
        lesson_signed_unsigned(),
        lesson_floats(),
        lesson_booleans(),
        lesson_characters(),
        lesson_tuples(),
        lesson_arrays(),
        lesson_constants(),
        lesson_shadowing(),
        lesson_shadowing_vs_mutation(),
        lesson_shadowing_changes_types(),
        lesson_conversions(),
        lesson_type_compatibility(),
        lesson_expressions(),
        lesson_reading_errors(),
        lesson_common_mistakes(),
        lesson_bridge(),
    ]
}

fn lesson_what_is_a_variable() -> SeedLesson {
    SeedLesson::new(
        "What Is a Variable?",
        vec![
            labelled(
                "CONCEPT",
                "Before variables existed, programs were just numbers floating around with nothing to say \
                 what they meant. `100 > 50` tells you almost nothing. `score > 50` tells you a story. \
                 A variable is simply a name attached to a value, so that later code can refer to that \
                 value by something meaningful.",
            ),
            diagram(
                "value:\n25",
                "value:\n  25          ← the thing itself\n\nname:\n  age          ← the label you use to reach it",
            ),
            code(
                "rust",
                "fn main() {\n    let username = \"Gus\";\n    let score = 100;\n    let logged_in = true;\n\n    println!(\"{} has a score of {}\", username, score);\n    println!(\"Logged in: {}\", logged_in);\n}",
            ),
            labelled(
                "EXPLANATION",
                "Rust is about to ask you a question that JavaScript would probably just shrug at. \
                 Before you can name a value, you have to say how to store it. `let` creates a \
                 binding — a name tied to a value. The name is `username`. The value is `\"Gus\"`. \
                 Rust records the value's type at this moment, and it never forgets.",
            ),
            code(
                "rust",
                "let score = 100;\n\n// With a name, this reads as intent:\nif score > 50 {\n    println!(\"Nice run\");\n}\n\n// Without one, it reads as arithmetic noise:\nif 100 > 50 {\n    println!(\"Nice run\");\n}",
            ),
            text_block(
                "This is the whole reason variables exist. Not to save typing. To make the code say what it means. \
                 Six months from now, `score > 50` is a gift. `100 > 50` is a riddle.",
            ),
            callout(
                "Practice — Try this now",
                "Declare three variables: a `username` (a string), a `score` (a whole number), and \
                 `logged_in` (true or false). Print all three on separate lines. Then change the score \
                 to a different number and print it again. Run with `cargo run`.",
            ),
            code(
                "rust",
                "fn main() {\n    let username = \"Gus\";\n    let score = 100;\n    let logged_in = true;\n\n    println!(\"user: {}\", username);\n    println!(\"score: {}\", score);\n    println!(\"logged in: {}\", logged_in);\n}",
            ),
            callout(
                "Mini challenge",
                "Write four lines that store your own name, your favourite number, whether you have a \
                 cat, and your favourite programming language. Print them in a single formatted line \
                 using one `println!`. Format it however you like — this one is yours.",
            ),
        ],
    )
}

fn lesson_immutable_by_default() -> SeedLesson {
    SeedLesson::new(
        "Immutable by Default",
        vec![
            labelled(
                "CONCEPT",
                "In Rust, a plain `let` binding is immutable. Once it holds a value, that value is frozen. \
                 You can read it forever. You cannot change it. This surprises almost everyone on day one, \
                 and it is not an oversight.",
            ),
            code(
                "rust",
                "fn main() {\n    let score = 100;\n\n    score = 200;  // ← this does not compile\n}",
            ),
            labelled(
                "EXPLANATION",
                "Rust refuses with something close to:",
            ),
            code(
                "text",
                "error[E0384]: cannot assign twice to immutable variable `score`\n --> src/main.rs:4:5\n  |\n4 |     score = 200;\n  |     ^^^^^^^ cannot assign twice to immutable variable",
            ),
            text_block(
                "Read that error carefully. It is not scolding you. It is telling you exactly what it thinks \
                 you meant, and what it is willing to do instead. `E0384` is Rust being precise about a \
                 decision it needs you to make explicitly.",
            ),
            callout(
                "Yes, Rust wants to know whether this changes",
                "No, it isn't being nosy. This information actually matters — the compiler now knows whether \
                 to trust the value or to keep checking it. That is the whole trade.",
            ),
            callout(
                "Practice — Try this now",
                "Write `let score = 100;` then try `score = 200;`. Run it. Read the error all the way \
                 through, including the line number. Then fix it by adding `mut` and run it again. Keep \
                 both versions around so you can compare them.",
            ),
            callout(
                "Mini challenge",
                "Predict the error before you run it: what will Rust say if you try to change a variable \
                 that was declared with `let` and no `mut`? Write your prediction down, then check. \
                 Getting a compiler error right *before* you see it is a real skill.",
            ),
        ],
    )
}

fn lesson_mut() -> SeedLesson {
    SeedLesson::new(
        "mut — When Values Need to Change",
        vec![
            labelled(
                "CONCEPT",
                "Some values genuinely change. A player's health drops. A timer counts down. A score climbs. \
                 For these, Rust wants you to say so out loud with `mut`. It is a small word that carries \
                 real information: *this binding is intentionally allowed to change.*",
            ),
            code(
                "rust",
                "fn main() {\n    let mut health = 100;\n\n    health -= 10;\n    health -= 20;\n\n    println!(\"Health: {}\", health);  // 70\n}",
            ),
            labelled(
                "EXPLANATION",
                "`let mut` does two things. It creates the binding, same as `let`. It also marks that binding \
                 as reassignable. After that, `health = health - 10` is allowed, and so is the shorthand \
                 `health -= 10`. Mutability attaches to the *binding*, not to the value. That distinction \
                 will matter enormously in the ownership module, so plant the seed now.",
            ),
            table(
                &["Binding", "Can it be reassigned?"],
                &[
                    &["let x = 5;", "No. Frozen at 5."],
                    &["let mut x = 5;", "Yes. x becomes 6, 7, or anything else."],
                ],
            ),
            text_block(
                "You can change this variable, but Rust wants you to say so first. Honestly, fair.",
            ),
            code(
                "rust",
                "fn main() {\n    // A countdown timer\n    let mut seconds = 3;\n\n    seconds -= 1;\n    seconds -= 1;\n    println!(\"{} seconds left\", seconds);  // 1\n\n    // Bank balance\n    let mut balance = 250.50;\n    balance += 100.0;\n    println!(\"Balance: {}\", balance);  // 350.5\n\n    // Attempts remaining\n    let mut attempts = 3;\n    attempts -= 1;\n    println!(\"attempts: {}\", attempts);  // 2\n}",
            ),
            callout(
                "Practice — Try this now",
                "Write a `let mut` counter starting at 0. Increment it five times, printing the value after \
                 each increment. Then create a `let mut` for player health at 100, subtract 30 twice, and \
                 print what is left.",
            ),
            code(
                "rust",
                "fn main() {\n    let mut count = 0;\n\n    count += 1;\n    println!(\"{}\", count);\n    count += 1;\n    println!(\"{}\", count);\n    count += 1;\n    println!(\"{}\", count);\n    count += 1;\n    println!(\"{}\", count);\n    count += 1;\n    println!(\"{}\", count);\n\n    let mut health = 100;\n    health -= 30;\n    health -= 30;\n    println!(\"health: {}\", health);  // 40\n}",
            ),
            callout(
                "Mini challenge",
                "Write a fuel gauge. Start at a full tank of 50.0. Burn fuel in three steps: 12.5, then \
                 8.25, then 4.0. Print the remaining fuel after each burn, and print a warning when it \
                 drops below 10.0. (You will need an `if` for the warning — if you have not met one yet, \
                 just print a fixed warning on the last line and come back to it.)",
            ),
        ],
    )
}

fn lesson_why_immutable() -> SeedLesson {
    SeedLesson::new(
        "Why Rust Defaults to Immutability",
        vec![
            labelled(
                "CONCEPT",
                "Why would a language make you opt in to change? It seems bureaucratic until you notice \
                 what it buys you. If a value cannot silently change, then reading it later tells you \
                 something reliable: it is exactly what it said it was.",
            ),
            code(
                "rust",
                "fn calculate_bonus(user_count: u32) -> u32 {\n    let user_count = user_count * 2;\n    user_count\n}\n\nfn main() {\n    let user_count = 100;\n    let bonus = calculate_bonus(user_count);\n\n    println!(\"users: {}\", user_count);      // still 100\n    println!(\"bonus: {}\", bonus);\n}",
            ),
            labelled(
                "EXPLANATION",
                "`calculate_bonus` cannot touch your `user_count`. It is not that it chooses not to — it \
                 *cannot*. When you read `user_count` on line 13, you know it is 100, because nothing in \
                 the whole program was able to change it behind your back.",
            ),
            diagram(
                "reading the source",
                "let user_count = 100;\n  ↑\n  immutable — trust this value\n\nlet mut user_count = 100;\n  ↑\n  mutable — assume it can change\n     and check before you rely on it",
            ),
            text_block(
                "Now read this line of code:",
            ),
            code(
                "rust",
                "let tax_rate = 0.2;\n// tax_rate cannot change anywhere in this program\n\nlet mut cart_total = 0.0;\n// cart_total will change as items are added",
            ),
            text_block(
                "Same word `let`. Completely different contract with the reader. The first one is a promise. \
                 The second one is an admission that things are in motion.",
            ),
            callout(
                "Practice — Try this now",
                "Write a function `double(value: i32) -> i32` that takes a number and returns it doubled. \
                 Call it with a variable and print both the argument and the result. Then try to change the \
                 argument inside the function. Read what the compiler says and explain it in your own words.",
            ),
            callout(
                "Mini challenge",
                "Write three functions, each taking a parameter, and each using that parameter to compute \
                 a result. Between them, demonstrate that the parameters never change. Then add a fourth \
                 function that takes a `&mut` style intent — actually, just add a function that *does* change \
                 a local variable it owns, and compare the two experiences. Write one sentence about the \
                 difference.",
            ),
        ],
    )
}

fn lesson_type_inference() -> SeedLesson {
    SeedLesson::new(
        "Type Inference",
        vec![
            labelled(
                "CONCEPT",
                "You did not tell Rust what type `age` is. You did not tell it what type `price` is. It knows \
                 anyway. This is type inference — Rust works out the type from the value and the surrounding \
                 context, and it does this before your program runs.",
            ),
            code(
                "rust",
                "fn main() {\n    let age = 25;        // inferred: i32\n    let name = \"Gus\";     // inferred: &str\n    let active = true;     // inferred: bool\n    let price = 19.99;     // inferred: f64\n}",
            ),
            labelled(
                "EXPLANATION",
                "Rust looks at the literal and decides. `25` is a whole number, so `i32`. `19.99` has a \
                 decimal point, so `f64`. `\"Gus\"` is text, so `&str`. `true` is a boolean, so `bool`. The \
                 compiler then locks that type in for the rest of the binding's life.",
            ),
            callout(
                "You don't have to tell Rust everything. But Rust still needs to know.",
                "That sentence is the whole idea. Inference saves you the typing, not the thinking. Rust \
                 still knows exactly what type each value is, and it will still hold you to it.",
            ),
            code(
                "rust",
                "fn main() {\n    // Each of these has a definite type, even though you never wrote it down\n    let count = 5;\n    let ratio = 0.5;\n    let label = \"hp\";\n    let alive = false;\n\n    // count: i32, ratio: f64, label: &str, alive: bool\n}",
            ),
            callout(
                "Practice — Try this now",
                "Declare six variables without any type annotations: a whole number, a negative whole \
                 number, a decimal, a string, a character, and a boolean. Run `cargo run`. Then deliberately \
                 break one — try `count = \"now a string\"` — and read what the compiler says about the type \
                 it expected versus the type it found.",
            ),
            callout(
                "Mini challenge",
                "What type will Rust infer for each of these?\n\n1. `let x = 42;`\n2. `let x = 42.0;`\n3. `let x = 'a';`\n4. `let x = \"a\";`\n5. `let x = true;`\n\nWrite your answers down first, then verify each one with a deliberate type error.",
            ),
        ],
    )
}

fn lesson_type_annotations() -> SeedLesson {
    SeedLesson::new(
        "Type Annotations",
        vec![
            labelled(
                "CONCEPT",
                "Sometimes you want to be explicit. `let age: i32 = 25;` puts the type right there in the \
                 source. This is a *type annotation*. It is optional, and Rust does not want you adding it \
                 everywhere. But there are real situations where it earns its place.",
            ),
            code(
                "rust",
                "fn main() {\n    let age: i32 = 25;\n    let score: u16 = 100;\n    let price: f64 = 19.99;\n    let username: &str = \"Gus\";\n    let active: bool = true;\n\n    println!(\"{} {} {} {} {}\", age, score, price, username, active);\n}",
            ),
            labelled(
                "EXPLANATION",
                "The `: i32` between the name and the `=` is the annotation. It tells Rust: *when this value \
                 has no type of its own, make it an i32.* When the literal already implies a type, the \
                 annotation is a claim Rust will check. That check is the value — if you annotate `: u8` and \
                 write `300`, Rust stops you.",
            ),
            steps(&[
                "Readability — a reader sees the type without inferring it",
                "Intent — `let price: f64` signals 'money, decimals matter'",
                "Ambiguity — when a bare value could be several types",
                "Empty collections — `let items: Vec<i32> = Vec::new();` needs the type",
                "Function boundaries — clear signatures for callers",
            ]),
            callout(
                "When NOT to annotate",
                "If Rust already knows perfectly well what the type is, adding an annotation is just noise. \
                 `let count: i32 = 5;` tells a reader nothing they couldn't infer. Annotate for clarity, not out of habit.",
            ),
            code(
                "rust",
                "fn main() {\n    // Useful: a bare 0 could be many types\n    let mut total = 0;\n    total += 5;  // works, but total is i32 because of the literal\n\n    // Useful: a vector needs the element type\n    // let empty = Vec::new();  // ← error: type annotations needed\n    let empty: Vec<i32> = Vec::new();\n    println!(\"{} items, total {}\", empty.len(), total);\n}",
            ),
            callout(
                "Practice — Try this now",
                "Take the variables you wrote in the last practice step and add explicit annotations to all \
                 of them. Then change one annotation to a type that cannot hold the value — `let age: u8 = 25;` \
                 is fine, but `let age: u8 = 300;` is not. Read the error.",
            ),
            callout(
                "Mini challenge",
                "Rust cannot infer the type of an empty collection. Write `let scores = Vec::new();` and \
                 read the error. Then fix it with an annotation that works. Finally, try to write the \
                 annotation in a way that is too vague, like `: Vec` with no element type, and see what Rust says.",
            ),
        ],
    )
}

fn lesson_integer_types() -> SeedLesson {
    SeedLesson::new(
        "Integer Types",
        vec![
            labelled(
                "CONCEPT",
                "Rust has a whole family of integer types. They are not interchangeable. When you write \
                 `25`, Rust picks `i32` for you. When you write `25` somewhere that demands a `u8`, it checks \
                 whether 25 actually fits.",
            ),
            table(
                &["Type", "Meaning", "Range (roughly)"],
                &[
                    &["i8", "signed 8-bit", "-128 to 127"],
                    &["i16", "signed 16-bit", "-32,768 to 32,767"],
                    &["i32", "signed 32-bit", "-2.1B to 2.1B"],
                    &["i64", "signed 64-bit", "-9.2Q to 9.2Q"],
                    &["i128", "signed 128-bit", "very large"],
                    &["isize", "signed, pointer-sized", "depends on your machine"],
                    &["u8", "unsigned 8-bit", "0 to 255"],
                    &["u16", "unsigned 16-bit", "0 to 65,535"],
                    &["u32", "unsigned 32-bit", "0 to 4.2B"],
                    &["u64", "unsigned 64-bit", "0 to 18.4Q"],
                    &["u128", "unsigned 128-bit", "very large"],
                    &["usize", "unsigned, pointer-sized", "depends on your machine"],
                ],
            ),
            labelled(
                "EXPLANATION",
                "The letter before the number is the important part.",
            ),
            diagram(
                "reading the name",
                "i32\n│ │\n│ └── 32 bits of storage\n└──── i = signed (can be negative)\n\nu8\n│ │\n│ └── 8 bits of storage\n└──── u = unsigned (zero and up only)",
            ),
            text_block(
                "For ordinary whole numbers with no other context, Rust infers `i32`. That covers a genuinely \
                 enormous range — about two billion in each direction — so it is a safe default. You reach \
                 for the smaller and larger types when you have a reason: matching a file format, staying \
                 inside a hardware range, or working with values that genuinely might exceed i32.",
            ),
            code(
                "rust",
                "fn main() {\n    let small: u8 = 255;\n    let medium: u16 = 65_535;\n    let normal = 42;         // i32 by inference\n    let big: u64 = 18_446_744_073_709_551_615;\n\n    println!(\"{} {} {} {}\", small, medium, normal, big);\n}",
            ),
            callout(
                "Types have limits",
                "This matters more than it sounds. A `u8` can hold at most 255. Try to store 256 in one and \
                 Rust will stop you. Later, when you work with ranges and conversions, you will meet the word \
                 *overflow*. For now just carry the habit: when you pick a type, pick it on purpose.",
            ),
            callout(
                "Practice — Try this now",
                "Declare one variable of each type: `i8`, `u8`, `i32`, `u32`, `i64`, `u64`, `f64`, `bool`, \
                 and `char`. Print them all. Then deliberately overflow one: write `let x: u8 = 256;` and \
                 read what Rust tells you about the maximum value.",
            ),
            code(
                "rust",
                "fn main() {\n    let a: i8 = -100;\n    let b: u8 = 200;\n    let c: i32 = 2_000_000_000;\n    let d: u32 = 4_000_000_000;\n    let e: i64 = 9_000_000_000_000_000_000;\n    let f: u64 = 18_000_000_000_000_000_000;\n    let g: f64 = 3.14159;\n    let h: bool = true;\n    let i: char = 'R';\n\n    println!(\"{} {} {} {} {} {} {} {} {}\", a, b, c, d, e, f, g, h, i);\n}",
            ),
            callout(
                "Mini challenge",
                "You are building a game. Pick a type for each and justify it in a comment:\n\n1. A player's level (1 to 100)\n2. Their health points (0 to 100)\n3. A world seed (up to 2 billion)\n4. Total milliseconds of playtime (can exceed 2 billion)\n5. A percentage from 0 to 100",
            ),
        ],
    )
}

fn lesson_integer_literals() -> SeedLesson {
    SeedLesson::new(
        "Integer Literals",
        vec![
            labelled(
                "CONCEPT",
                "Rust lets you write whole numbers in four different notations. You are not required to use \
                 them, but you will meet all four eventually — and once you know what they are, they stop \
                 looking strange.",
            ),
            code(
                "rust",
                "fn main() {\n    let decimal = 1_000_000;\n    let hex = 0xff;          // 255\n    let binary = 0b1010;     // 10\n    let octal = 0o755;       // 493\n\n    println!(\"{} {} {} {}\", decimal, hex, binary, octal);\n}",
            ),
            table(
                &["Notation", "How it starts", "Example", "Value"],
                &[
                    &["Decimal", "just digits", "255", "255"],
                    &["Hexadecimal", "0x", "0xff", "255"],
                    &["Binary", "0b", "0b1111_1111", "255"],
                    &["Octal", "0o", "0o377", "255"],
                ],
            ),
            labelled(
                "EXPLANATION",
                "The underscores are the part worth caring about. `1000000` is perfectly valid Rust, but \
                 counting the zeros is a waste of your one remaining brain cell at 2am. `1_000_000` is the \
                 same number, and you can read it at a glance. Rust ignores the underscores entirely — \
                 they exist purely for humans.",
            ),
            code(
                "rust",
                "fn main() {\n    // Same number, written four ways\n    let a = 255;\n    let b = 0xff;\n    let c = 0b1111_1111;\n    let d = 0o377;\n\n    // Underscores work anywhere, even in the middle of the prefix\n    let e = 0xFF_FF;\n    let f = 1_2_3_4;\n\n    println!(\"{} {} {} {} {} {}\", a, b, c, d, e, f);\n}",
            ),
            callout(
                "Where you'll actually see these",
                "Hex in colours (`0xFF_6B_35` is BORROW_'s burnt orange) and memory addresses. Binary when \
                 you are reasoning about individual bits. Octal for file permissions (`0o755`) on Linux and macOS. \
                 Decimal the other 95% of the time.",
            ),
            callout(
                "Practice — Try this now",
                "Write `255` in all four notations, one per variable, and print all four. Then write \
                 `1_048_576` (one million in binary-friendly form) using underscores. Finally, convert \
                 `0o755` to decimal by hand, then verify with `println!`.",
            ),
            callout(
                "Mini challenge",
                "A file permission is written as `0o755`. Work out what each digit means: 7 for owner, 5 for \
                 group, 5 for others. If you know how permission numbers work, explain why 7 means 'rwx' and \
                 why 5 means 'r-x'. Then write `0o644` and explain it the same way.",
            ),
        ],
    )
}

fn lesson_signed_unsigned() -> SeedLesson {
    SeedLesson::new(
        "Signed vs Unsigned",
        vec![
            labelled(
                "CONCEPT",
                "A signed integer can be negative. An unsigned integer cannot. That is genuinely the whole \
                 difference — but the *consequences* of that difference ripple outward in ways that surprise \
                 people regularly.",
            ),
            code(
                "rust",
                "fn main() {\n    let temperature: i32 = -10;  // signed — fine\n    let age: u32 = 25;           // unsigned — fine\n\n    let below_zero: i8 = -5;     // fine\n    let almost_zero: u8 = 0;     // also fine\n}",
            ),
            labelled(
                "EXPLANATION",
                "Signed types spend half their range on negatives. `i8` goes from -128 to 127 — 256 total \
                 values, half of them negative. `u8` goes from 0 to 255 — 256 total values, none of them \
                 negative. Same storage. Different territory.",
            ),
            code(
                "rust",
                "fn main() {\n    let age: u32 = -1;  // ← this does not compile\n}",
            ),
            code(
                "text",
                "error[E0308]: mismatched types\n  --> src/main.rs:3:20\n   |\n3  |     let age: u32 = -1;\n   |                    ^^   expected `u32`, found integer `{integer}`",
            ),
            text_block(
                "Now think about whether that's a *type* problem or a *sense* problem. Age of -1 is not a \
                 thing. Rust is not just enforcing a rule here — it is agreeing with you that the value is \
                 nonsense. This is a case where the type system is doing exactly what you would want.",
            ),
            callout(
                "Reach for unsigned when the answer is genuinely 'never negative'",
                "Array lengths, byte counts, ages, counts of items, channel numbers, port numbers. Reaching \
                 for signed when the value might dip below zero is equally sensible. The mistake is picking \
                 `u32` for everything because 'numbers are usually positive, right?'",
            ),
            code(
                "rust",
                "fn main() {\n    // Unsigned makes sense: these are never negative\n    let items: usize = 5;        // count of things\n    let byte: u8 = 255;           // a raw byte value\n    let port: u16 = 8080;          // network port\n\n    // Signed makes sense: these can dip below zero\n    let temp: i16 = -40;           // temperature\n    let delta: i32 = -1;           // a change, up or down\n}",
            ),
            callout(
                "Practice — Try this now",
                "Try `let age: u32 = -1;` and read the error. Then try `let count: i32 = 5;` and notice it \
                 works fine. Finally, try subtracting past zero on an unsigned type: `let mut n: u8 = 0; n = n - 1;` \
                 and read what happens. That last one is a good preview of underflow.",
            ),
            callout(
                "Mini challenge",
                "Decide signed or unsigned for each, and write one sentence of justification:\n\n1. A countdown timer that goes below zero\n2. The number of items in a shopping cart\n3. A temperature in Celsius\n4. A file size in bytes\n5. A score that can go up or down by 5",
            ),
        ],
    )
}

fn lesson_floats() -> SeedLesson {
    SeedLesson::new(
        "Floating-Point Numbers",
        vec![
            labelled(
                "CONCEPT",
                "Not every number is a whole number. Rust has two float types: `f32` (32 bits) and `f64` \
                 (64 bits). When you write a decimal with no other context, Rust gives you `f64`.",
            ),
            code(
                "rust",
                "fn main() {\n    let price = 19.99;              // f64 by inference\n    let precise: f64 = 19.99;     // explicitly 64-bit\n    let smaller: f32 = 19.99;     // explicitly 32-bit\n\n    println!(\"{} {} {}\", price, precise, smaller);\n}",
            ),
            labelled(
                "EXPLANATION",
                "Floats store a *decimal approximation* of your number, not the exact value. Most of the time \
                 that is invisible. It becomes visible when you do a lot of arithmetic with values that \
                 cannot be represented exactly — a recurring theme in anything involving money.",
            ),
            code(
                "rust",
                "fn main() {\n    let a = 0.1;\n    let b = 0.2;\n    println!(\"a + b = {}\", a + b);      // 0.30000000000000004\n\n    let c: f32 = 0.1;\n    let d: f32 = 0.2;\n    println!(\"c + d = {}\", c + d);      // 0.30000004\n\n    println!(\"a + b == 0.3\");              // false\n}",
            ),
            text_block(
                "That is not a Rust bug. `0.1` has no exact binary representation, in any language with a \
                 binary float type. The value you asked for, the value stored, and the value printed are \
                 three closely related but distinct things. `f32` makes it worse because it has fewer bits to \
                 work with.",
            ),
            callout(
                "Which should I use?",
                "Default to `f64`. Reach for `f32` when you are storing a large array of numbers where halving \
                 the memory actually matters. For money specifically, look into a decimal crate later — floats \
                 are the wrong tool for accounting.",
            ),
            callout(
                "Practice — Try this now",
                "Compute `0.1 + 0.2` and print it. Compute it again with `f32` values and print that. Then \
                 try `(0.1 + 0.2) == 0.3` and see what it says. Finally, compute a 15% tip on a bill of \
                 42.50 and print it.",
            ),
            code(
                "rust",
                "fn main() {\n    let bill = 42.50;\n    let tip_rate = 0.15;\n    let tip = bill * tip_rate;\n    let total = bill + tip;\n\n    println!(\"Bill:  ${:.2}\", bill);\n    println!(\"Tip:   ${:.2}\", tip);\n    println!(\"Total: ${:.2}\", total);\n}",
            ),
            callout(
                "Mini challenge",
                "Convert a temperature in Celsius to Fahrenheit. Store the Celsius value in an `f32` and the \
                 result in an `f64`, and notice whether the printed precision differs. The formula is \
                 `fahrenheit = celsius * 9.0 / 5.0 + 32.0`. Try it with 100.0 and see why round numbers can \
                 look unexpected.",
            ),
        ],
    )
}

fn lesson_booleans() -> SeedLesson {
    SeedLesson::new(
        "Booleans",
        vec![
            labelled(
                "CONCEPT",
                "Booleans are two values: `true` and `false`. That is the whole type. In Rust they are a \
                 first-class value you can store in a variable — not just something that appears in an `if`.",
            ),
            code(
                "rust",
                "fn main() {\n    let logged_in: bool = true;\n    let has_permission: bool = false;\n    let is_admin = true;          // inferred as bool\n\n    println!(\"{} {} {}\", logged_in, has_permission, is_admin);\n}",
            ),
            labelled(
                "EXPLANATION",
                "A `bool` takes one byte of memory and has exactly two possible values. That constraint is \
                 the point: there is no third option, no `maybe`, no `null`. When a variable holds a bool, you \
                 know exactly what it is.",
            ),
            callout(
                "Booleans become interesting in control flow",
                "Right now a bool is just a value sitting in a variable. The moment you use it in an `if` or a \
                 `while`, it becomes a decision. That is exactly where the next module starts — but the \
                 groundwork is here now. `if logged_in { }` is a perfectly understandable line even before we \
                 dig into how `if` works.",
            ),
            code(
                "rust",
                "fn main() {\n    let logged_in = true;\n    let attempts_left = 3;\n    let has_key = false;\n\n    // Reads clearly even before you learn if/else\n    println!(\"Can play: logged in AND has key\");\n    println!(\"Attempts remaining: {}\", attempts_left);\n    println!(\"Logged in: {}\", logged_in);\n    println!(\"Has key: {}\", has_key);\n}",
            ),
            callout(
                "Practice — Try this now",
                "Write three boolean variables that describe a character: `alive`, `has_weapon`, and \
                 `is_poisoned`. Print them each on its own line with a label. Then try the mistake that catches \
                 everyone: `let flag = True;` (capital T) and read what the compiler says.",
            ),
            callout(
                "Mini challenge",
                "Write four booleans describing a coffee order: `has_milk`, `has_sugar`, `is_hot`, \
                 `is_large`. Print a receipt-style summary. Then write one boolean that captures something \
                 that is *not* a yes/no question, and think about whether a bool is really the right type for it.",
            ),
        ],
    )
}

fn lesson_characters() -> SeedLesson {
    SeedLesson::new(
        "Characters",
        vec![
            labelled(
                "CONCEPT",
                "A `char` holds exactly one Unicode scalar value. Single quotes. Four bytes. That is all it \
                 is — and that is a surprising amount for four bytes to hold.",
            ),
            code(
                "rust",
                "fn main() {\n    let letter: char = 'A';\n    let digit: char = '7';\n    let heart: char = '♥';    // 3 bytes of UTF-8, 1 char\n    let ninja: char = '🥷';    // 4 bytes of UTF-8, 1 char\n\n    println!(\"{} {} {} {}\", letter, digit, heart, ninja);\n}",
            ),
            labelled(
                "EXPLANATION",
                "Rust's `char` is **not** an ASCII character. It is a Unicode scalar value, which covers \
                 essentially every writing system on earth plus a large set of symbols and emoji. The type is \
                 always four bytes wide, regardless of how many bytes the actual glyph needs.",
            ),
            callout(
                "'A' versus \"A\" — the difference matters",
                "`'A'` is a `char` — exactly one Unicode scalar value, written with single quotes. \
                 `\"A\"` is a `&str` — a string slice, a whole run of text, written with double quotes. \
                 One letter versus a sequence. Different types, different sizes, different rules. \
                 We will not get deep into strings yet, but recognise the quotes and know they are not \
                 interchangeable.",
            ),
            code(
                "rust",
                "fn main() {\n    let a: char = 'A';    // char — one value\n    let b: &str = \"A\";    // &str — text, could be longer\n\n    // This is a compile error, and it is a good one:\n    // let c: char = \"A\";\n    //   error[E0308]: mismatched types\n    //   expected `char`, found `&str`\n}",
            ),
            text_block(
                "A `char` also carries useful methods. `is_alphabetic()`, `is_numeric()`, `is_uppercase()`, \
                 `to_ascii_lowercase()` — all of which return `bool`, tying this lesson back to the last one.",
            ),
            code(
                "rust",
                "fn main() {\n    let c = 'R';\n\n    println!(\"is alphabetic: {}\", c.is_alphabetic());\n    println!(\"is numeric:    {}\", c.is_numeric());\n    println!(\"is uppercase:  {}\", c.is_uppercase());\n    println!(\"lowercase:     {}\", c.to_ascii_lowercase());\n}",
            ),
            callout(
                "Practice — Try this now",
                "Declare five char variables: your initial, a digit, a symbol, an accented letter, and an \
                 emoji. Print them with labels. Then write `let x: char = \"R\";` (double quotes by mistake) \
                 and read the error — it is a great introduction to `expected` versus `found`.",
            ),
            callout(
                "Mini challenge",
                "Store your three initials as three separate `char` variables. Print them on one line. Then \
                 try to store them as a single `char` by writing something like `let initials = 'GAB';` and \
                 read the error. What would be the right type for a string of initials?",
            ),
        ],
    )
}

fn lesson_tuples() -> SeedLesson {
    SeedLesson::new(
        "Tuples",
        vec![
            labelled(
                "CONCEPT",
                "Sometimes a few values belong together and they are not all the same type. That is a tuple: \
                 a fixed-size group of values that can each be a different type.",
            ),
            code(
                "rust",
                "fn main() {\n    let user = (\"Gus\", 25, true);\n\n    println!(\"Name: {}\", user.0);\n    println!(\"Age:  {}\", user.1);\n    println!(\"Active: {}\", user.2);\n}",
            ),
            labelled(
                "EXPLANATION",
                "Each element has its own type, inferred independently. `\"Gus\"` is `&str`, `25` is `i32`, \
                 `true` is `bool`. Access them by position: `.0`, `.1`, `.2`. Positions start at zero, same \
                 as arrays. The size is fixed at creation — a tuple cannot grow or shrink.",
            ),
            callout(
                "Destructuring — the good way to read a tuple",
                "When you know what is inside, pulling the values out into named variables is far more \
                 readable than `user.0` and `user.1` scattered through your code. This is called \
                 destructuring, and it is worth using immediately.",
            ),
            code(
                "rust",
                "fn main() {\n    let user = (\"Gus\", 25, true);\n\n    // Positional access — fine for a quick lookup\n    println!(\"{}\", user.0);\n\n    // Destructuring — much better for real code\n    let (name, age, active) = user;\n    println!(\"{} is {} and active: {}\", name, age, active);\n}",
            ),
            code(
                "rust",
                "fn main() {\n    // Tuples work well for coordinates, RGB values, and small fixed groups\n    let coords = (10, 20);\n    let rgb = (255, 107, 53);   // BORROW_ burnt orange\n    let version = (1, 0, 0);\n\n    let (x, y) = coords;\n    let (r, g, b) = rgb;\n\n    println!(\"x={} y={}\", x, y);\n    println!(\"rgb=({}, {}, {})\", r, g, b);\n}",
            ),
            callout(
                "When to use a tuple",
                "When a handful of values genuinely travel together and you know the count at compile time. \
                 Coordinates, colour channels, a version triple, a (x, y) pair. If you find yourself writing \
                 `tuple.0`, `tuple.1`, `tuple.2`, `tuple.3` and losing track of what each one means, that is \
                 a sign you want something clearer. That something is `struct`, and it is coming in module 07.",
            ),
            callout(
                "Practice — Try this now",
                "Create a tuple for a product: (name, price, in_stock). Print each field positionally. Then \
                 destructure it into three named variables and print them again. Add a fourth field — a stock \
                 count as a `u32`.",
            ),
            callout(
                "Mini challenge",
                "Write a tuple for a point in 2D space as two `f64` values. Destructure it and print both \
                 coordinates. Then write a tuple for a 3D point with three `f64` values and do the same. \
                 Finally, try to make a tuple where two elements have deliberately different types you think \
                 should match, and see what happens.",
            ),
        ],
    )
}

fn lesson_arrays() -> SeedLesson {
    SeedLesson::new(
        "Arrays",
        vec![
            labelled(
                "CONCEPT",
                "An array holds a fixed number of values, all of the same type, in order. Fixed means fixed. \
                 `[i32; 5]` is not a suggestion — it is four values worth of information about the size.",
            ),
            code(
                "rust",
                "fn main() {\n    let numbers = [1, 2, 3, 4, 5];\n\n    println!(\"{}\", numbers[0]);  // 1\n    println!(\"{}\", numbers[1]);  // 2\n    println!(\"{}\", numbers[4]);  // 5\n}",
            ),
            labelled(
                "EXPLANATION",
                "Two rules. Every element is the same type. The count is part of the type. That is why Rust \
                 wants you to write the type when you do not have literal values to count.",
            ),
            callout(
                "Reading [i32; 5]",
                "Square brackets mean *array*. Inside is the element type, `i32`. The semicolon separates it \
                 from the length, `5`. Read it out loud: *array of i32 with 5 elements.* Rust's type \
                 syntax is dense like this everywhere — `Option<i32>`, `Result<T, E>`, `Vec<String>`. \
                 Getting fluent with this notation pays off across the whole language.",
            ),
            code(
                "rust",
                "fn main() {\n    // Inferred: array of i32, length 5\n    let numbers = [1, 2, 3, 4, 5];\n\n    // Explicit — same array, said out loud\n    let explicit: [i32; 5] = [1, 2, 3, 4, 5];\n\n    // Useful when every element is the same thing\n    let zeros: [u8; 4] = [0; 4];           // [0, 0, 0, 0]\n    let names: [&str; 3] = [\"a\", \"b\", \"c\"];\n\n    println!(\"{:?}\", zeros);\n    println!(\"{:?}\", names);\n}",
            ),
            text_block(
                "Indexing starts at **zero**. `numbers[0]` is the first element, not the second. This trips up \
                 almost everyone, usually exactly once, usually at midnight.",
            ),
            callout(
                "Practice — Try this now",
                "Create an array of five whole numbers. Print each one by index. Then declare the same array \
                 with an explicit type: `let nums: [i32; 5]`. Finally, try `let wrong: [i32; 4] = [1, 2, 3, 4, 5];` \
                 and read what Rust says about the length.",
            ),
            code(
                "rust",
                "fn main() {\n    let numbers = [10, 20, 30, 40, 50];\n\n    for index in 0..5 {\n        println!(\"numbers[{}] = {}\", index, numbers[index]);\n    }\n}",
            ),
            callout(
                "Mini challenge",
                "Create a 6-element array of whole numbers. Print the first and last. Then print the sum of all \
                 six by adding `numbers[0]` through `numbers[5]` explicitly. Finally, write `let a: [i32; 3] = [1, 2, 3];` \
                 and then `let b: [i32; 3] = a;` and explain what copying an array does.",
            ),
        ],
    )
}

fn lesson_constants() -> SeedLesson {
    SeedLesson::new(
        "Constants",
        vec![
            labelled(
                "CONCEPT",
                "Some values are fixed for the entire life of a program. The maximum score. The tax rate. \
                 The name of the app. A `const` says: this value will never change.",
            ),
            code(
                "rust",
                "const MAX_SCORE: u32 = 100;\nconst APP_NAME: &str = \"BORROW_\";\nconst TAX_RATE: f64 = 0.20;\n\nfn main() {\n    println!(\"{} — max score is {}\", APP_NAME, MAX_SCORE);\n}",
            ),
            labelled(
                "EXPLANATION",
                "Three things make a constant different from a variable. It uses `const`, not `let`. It \
                 **requires** a type annotation — no inference here. And it is written in `SCREAMING_SNAKE_CASE` \
                 by convention, because it reads as a named constant rather than a working value.",
            ),
            callout(
                "Constants are not 'variables that never change'",
                "They serve a different purpose. A constant is a named fact about your program, available \
                 everywhere, known to the compiler before the program runs. Constants can be used in places \
                 where a runtime value cannot — array lengths, for example. When you see `MAX_PLAYERS: usize = 4;` \
                 you are learning a rule of the system, not storing a number.",
            ),
            code(
                "rust",
                "const MAX_PLAYERS: usize = 4;\n\nfn main() {\n    // A constant can be used where a value is required at compile time\n    let table: [u8; MAX_PLAYERS] = [0; MAX_PLAYERS];\n\n    println!(\"Table size: {}\", table.len());\n    println!(\"Max players: {}\", MAX_PLAYERS);\n}",
            ),
            callout(
                "Practice — Try this now",
                "Define three constants: `MAX_LEVEL: u8 = 50`, `STARTING_HEALTH: u16 = 100`, and `GAME_TITLE: &str`. \
                 Print them. Then try to change one: `MAX_LEVEL = 60;` and read the error. Finally, try defining \
                 a constant without a type annotation and see what Rust insists on.",
            ),
            callout(
                "Mini challenge",
                "Define a constant for `ITEMS_PER_PAGE: usize = 20`. Then define an array whose length is \
                 that constant: `let items: [u8; ITEMS_PER_PAGE] = [0; ITEMS_PER_PAGE];`. Print its length. \
                 Now change the constant to 10, re-run, and watch the array change with it. Nothing else in the \
                 program needed editing — that is the point of a constant.",
            ),
        ],
    )
}

fn lesson_shadowing() -> SeedLesson {
    SeedLesson::new(
        "Shadowing",
        vec![
            labelled(
                "CONCEPT",
                "You can declare a new `let` with a name that already exists. The new binding does not \
                 overwrite the old one — it *shadows* it. The old value still exists. You just cannot reach it \
                 through that name anymore.",
            ),
            code(
                "rust",
                "fn main() {\n    let x = 5;\n\n    let x = x + 1;   // new binding, takes the name\n    let x = x * 2;   // another new binding\n\n    println!(\"{}\", x);  // 12\n}",
            ),
            diagram(
                "what actually happens",
                "x → 5            binding 1 created\n\nshadow\n\nx → 6            binding 2 created, binding 1 hidden\n\nshadow\n\nx → 12           binding 3 created, binding 2 hidden",
            ),
            labelled(
                "EXPLANATION",
                "Read `let x = x + 1;` carefully. The right-hand `x` is the old binding, value 5. The result, \
                 6, is bound to a brand-new `x`. The old `x` still exists, still holds 5, and will still be \
                 there until it goes out of scope. It is simply no longer reachable by that name.",
            ),
            callout(
                "This is not mutation",
                "Mutation would be `x = x + 1;` — no `let`. Shadowing is `let x = x + 1;` — a new `let`. \
                 The presence of `let` is the tell. We will compare them properly in the next lesson, \
                 because the difference matters more than it looks.",
            ),
            code(
                "rust",
                "fn main() {\n    // Shadowing to transform a value through steps\n    let measurement = 12;\n\n    let measurement = measurement * 2;    // 24\n    let measurement = measurement + 6;    // 30\n    let measurement = measurement / 3;    // 10\n\n    println!(\"{}\", measurement);  // 10\n}",
            ),
            callout(
                "Practice — Try this now",
                "Start with `let value = 10;`. Shadow it three times: `value + 5`, then `value * 3`, then \
                 `value - 5`. Print after each step. Then try the same three steps with `let mut` and `=` \
                 instead, and compare the two approaches line by line.",
            ),
            callout(
                "Mini challenge",
                "Write a unit conversion that uses shadowing to change the unit as it goes. Start with a \
                 distance in kilometres, shadow it into metres, then into centimetres. Print all three values \
                 with their units.",
            ),
        ],
    )
}

fn lesson_shadowing_vs_mutation() -> SeedLesson {
    SeedLesson::new(
        "Shadowing vs Mutation",
        vec![
            labelled(
                "CONCEPT",
                "These two look almost identical and behave differently in ways that matter. Understanding \
                 *why* is the difference between writing Rust and guessing at it.",
            ),
            code(
                "rust",
                "fn main() {\n    // MUTATION — one binding, new value\n    let mut x = 5;\n    x = 6;\n    println!(\"{}\", x);  // 6\n\n    // SHADOWING — a new binding, same name\n    let x = 5;\n    let x = 6;\n    println!(\"{}\", x);  // 6\n}",
            ),
            table(
                &["", "Mutation", "Shadowing"],
                &[
                    &["Syntax", "x = 6;", "let x = 6;"],
                    &["Bindings", "One, reused", "Two, second hides first"],
                    &["Old value", "Gone", "Still there, unreachable"],
                    &["Can change type?", "No", "Yes"],
                    &["Reads as", "'update the value'", "'a new thing in the same role'"],
                ],
            ),
            labelled(
                "EXPLANATION",
                "The deciding question is what the code is *saying*. If you are tracking the same quantity as it \
                 changes — health dropping, a counter climbing, a balance moving — that is mutation. If you \
                 are transforming one representation into another — a string parsed into a number, a value \
                 converted between units — that is shadowing.",
            ),
            callout(
                "Spot the difference",
                "Read both versions and notice the tell: mutation has no `let` on the reassignment line. \
                 Shadowing has `let` on every line. The keyword is doing the work.",
            ),
            code(
                "rust",
                "fn main() {\n    // This is mutation: tracking one quantity as it changes\n    let mut health = 100;\n    health -= 30;\n    health -= 20;\n    println!(\"health: {}\", health);  // 50\n\n    // This is shadowing: transforming representations\n    let raw = \"42\";\n    let raw = raw.len();\n    let raw = raw as i32 * 2;\n    println!(\"raw: {}\", raw);  // 4\n}",
            ),
            callout(
                "Practice — Try this now",
                "Write the same three-step calculation twice. First with `let mut` and `=`. Then with \
                 repeated `let x = x + 1;`. Both produce the same output. Now write one sentence for each \
                 version explaining which one you would prefer in real code, and why.",
            ),
            callout(
                "Mini challenge",
                "You have a value that starts as a whole number, is doubled, then converted to a float, then \
                 halved. Write it twice — once using mutation with an explicit type change, once using \
                 shadowing. If mutation fights you on the type change, that fight is the lesson. Read the \
                 error it gives you.",
            ),
        ],
    )
}

fn lesson_shadowing_changes_types() -> SeedLesson {
    SeedLesson::new(
        "Shadowing Can Change Types",
        vec![
            labelled(
                "CONCEPT",
                "This is one of Rust's genuinely pleasant surprises. Shadowing lets the same name hold a \
                 different type. The variable has moved to a new representation of the same idea, and you never \
                 needed to invent a second name.",
            ),
            code(
                "rust",
                "fn main() {\n    let value = \"42\";\n    let value: i32 = value.parse().unwrap();\n\n    println!(\"{}\", value + 1);  // 43 — it's an i32 now\n}",
            ),
            labelled(
                "EXPLANATION",
                "The first `value` is a `&str` holding the characters `4` and `2`. `.parse()` asks Rust to \
                 interpret those characters as a number. `.unwrap()` says: if that fails, panic — we are \
                 handling errors properly in module 09, so for now assume it works. The result is an `i32`, and \
                 the annotation on the new `let` says so.",
            ),
            callout(
                "Now try the mutation version",
                "`let mut value = \"42\"; value = 42;` does not compile. The type of a mutable variable is \
                 fixed the moment you declare it. Shadowing is how you change representation in Rust. This \
                 is the single most useful thing to remember from the whole module.",
            ),
            code(
                "rust",
                "fn main() {\n    // Mutation cannot change the type\n    let mut value = \"42\";\n    // value = 42;\n    //   error[E0308]: mismatched types\n    //   expected `&str`, found integer `{integer}`\n    //   help: if you meant to assign a different type, use shadowing:\n    //   let value = 42;\n\n    // Shadowing can\n    let value = \"42\";\n    let value: i32 = value.parse().unwrap();\n    println!(\"{}\", value);\n}",
            ),
            text_block(
                "Notice that Rust's own error message suggests shadowing. The compiler is not just a gate — \
                 it is a teacher. Most of its errors include a `help:` line telling you what to do instead. \
                 Start reading those.",
            ),
            code(
                "rust",
                "fn main() {\n    // A realistic chain of transformations\n    let input = \"255\";\n    let input: u8 = input.parse().unwrap();     // now a small number\n    let input = input as u32 * 2;                // widened and doubled\n    let input = format!(\"value={}\", input);     // now a String\n\n    println!(\"{}\", input);  // value=510\n}",
            ),
            callout(
                "Practice — Try this now",
                "Start with `let x = \"100\";` and shadow it three times: parse to i32, double it, then convert \
                 to a string with `format!`. Print after each step and note the type each time. Then write the \
                 `let mut` equivalent and watch it fail on the second step.",
            ),
            callout(
                "Mini challenge",
                "Write a program that takes a number as a string and reports what kind of number it is. \
                 Start with `let input = \"42\";`, shadow it into an `i32` with `.parse().unwrap()`, then check \
                 whether it is positive, zero, or negative using comparisons and print the result. Three \
                 variables would be the wrong way to write this.",
            ),
        ],
    )
}

fn lesson_conversions() -> SeedLesson {
    SeedLesson::new(
        "Numeric Type Conversions",
        vec![
            labelled(
                "CONCEPT",
                "Rust will not quietly convert between numeric types for you. If you want an `i32` to become \
                 an `f64`, you have to say so with `as`. Every time.",
            ),
            code(
                "rust",
                "fn main() {\n    let x: i32 = 10;\n    let y: f64 = x as f64;\n\n    println!(\"{}\", y);        // 10\n    println!(\"{}\", y / 4.0);   // 2.5 — floating point division\n}",
            ),
            labelled(
                "EXPLANATION",
                "`as` is a conversion, not a cast in the C sense. Rust is telling you: *I will do this, and \
                 here is what it costs.* Widening — turning a small integer into a bigger one, or an integer \
                 into a float — is safe and exact. Narrowing — a big number into a small type, or a float \
                 into an integer — can silently lose information.",
            ),
            code(
                "rust",
                "fn main() {\n    // Safe: i32 → i64 → f64 (nothing is lost)\n    let a: i32 = 100;\n    let b: i64 = a as i64;\n    let c: f64 = b as f64;\n\n    // Lossy: f64 → i32 truncates the decimal part\n    let d: f64 = 3.99;\n    let e: i32 = d as i32;   // 3, not 4 — it truncates, it does not round\n\n    // Lossy: i32 → u8 keeps only the low byte\n    let f: i32 = 300;\n    let g: u8 = f as u8;    // 44\n\n    println!(\"{} {} {}\", c, e, g);\n}",
            ),
            callout(
                "Different types are different types. Rust won't silently pretend otherwise.",
                "This is the mental model. Casting is always visible, always intentional, and never free. \
                 When you see `as` in real code, someone made a decision. That is a good thing.",
            ),
            callout(
                "Practice — Try this now",
                "Declare `let x: i32 = 10;`. Try `let y: i64 = x;` without `as` and read the error — it is the \
                 exact `expected i64, found i32` message from the spec. Fix it with `as`. Then try \
                 `let z: u8 = 300;` and read what happens. Finally, try casting `f64` to `i32` and confirm it \
                 truncates rather than rounds.",
            ),
            callout(
                "Mini challenge",
                "Write a conversion helper by hand. Take a `u16` value representing a temperature in tenths of a \
                 degree (`253` means 25.3°C). Cast it to f64, divide by 10.0, and print the result with one \
                 decimal place. Then do the reverse: take a `f64` temperature, multiply by 10.0, cast to `u16`, \
                 and print the raw integer.",
            ),
        ],
    )
}

fn lesson_type_compatibility() -> SeedLesson {
    SeedLesson::new(
        "Type Compatibility",
        vec![
            labelled(
                "CONCEPT",
                "Two integer types do not mix just because both hold integers. `i32` and `i64` are different \
                 types, and Rust will not add them for you.",
            ),
            code(
                "rust",
                "fn main() {\n    let x: i32 = 10;\n    let y: i64 = 20;\n\n    let total = x + y;  // ← this does not compile\n}",
            ),
            code(
                "text",
                "error[E0308]: mismatched types\n  --> src/main.rs:4:18\n   |\n4  |     let total = x + y;\n   |                  ^   expected `i32`, found `i64`",
            ),
            labelled(
                "EXPLANATION",
                "Read that error from the inside out. `^` points at `x`. Rust expected `i32` (the type of the \
                 left side of `+`) and found `i64` (the type of `y`). It cannot decide which one you meant, so \
                 it stops and asks.",
            ),
            callout(
                "How to fix it — pick a direction",
                "Either widen the narrow one: `let y: i32 = 20;`. Or cast the narrow one up: `let total = x as i64 + y;`. \
                 Both are correct. The second is more honest when the types are genuinely different in the \
                 rest of your program.",
            ),
            code(
                "rust",
                "fn main() {\n    let a: i32 = 10;\n    let b: i64 = 20;\n\n    // Option 1: make them the same type up front\n    let b: i32 = 20;\n    let sum1 = a + b;   // i32\n\n    // Option 2: cast the narrower one up\n    let sum2 = a as i64 + 20;\n\n    // Option 3: let inference do the work\n    let c = 10;      // i32\n    let d = 20;      // i32\n    let sum3 = c + d;\n\n    println!(\"{} {} {}\", sum1, sum2, sum3);\n}",
            ),
            text_block(
                "Notice option 3. When you never write an annotation, Rust picks `i32` for both and everything \
                 just works. Type annotations are what create the mismatch in the first place. Neither approach \
                 is wrong — you just need to know which problem you are solving.",
            ),
            callout(
                "Practice — Try this now",
                "Write `let x: i32 = 5;` and `let y: u64 = 10;` and try `x + y`. Read the error. Now try all \
                 three fixes: change y's annotation, cast x up with `as`, and remove both annotations. Print \
                 all three results.",
            ),
            callout(
                "Mini challenge",
                "You have a `u8` quantity, a `u32` quantity, and a `f64` quantity. Write a single expression \
                 that adds all three. It will not work on the first try — that is the point. Get it to work, \
                 then write one sentence explaining what type the final result is and why.",
            ),
        ],
    )
}

fn lesson_expressions() -> SeedLesson {
    SeedLesson::new(
        "Expressions Produce Values",
        vec![
            labelled(
                "CONCEPT",
                "In most languages, an expression is either a value or a thing you do. In Rust, that line is \
                 blurry. Almost everything produces a value, including blocks.",
            ),
            code(
                "rust",
                "fn main() {\n    let x = 10 + 20;              // an expression that produces 30\n\n    let y = {\n        let a = 10;\n        let b = 20;\n        a + b                    // no semicolon — this is the value\n    };\n\n    println!(\"{} {}\", x, y);  // 30 30\n}",
            ),
            labelled(
                "EXPLANATION",
                "That block assigned to `y` produced `30`. The rule that makes this work: the final expression \
                 in a block, with no semicolon, is the block's value. Put a semicolon on `a + b` and the block \
                 produces `()`, which is the absence of a value. That one character is the entire difference.",
            ),
            callout(
                "Rust keeps producing values from things",
                "Hold onto that thought. It is the seed of the entire next module. `if` produces a value. \
                 `match` produces a value. A loop body produces a value. Once you internalise that blocks \
                 evaluate to things, control flow stops feeling like a special case and starts feeling like \
                 just more expression syntax.",
            ),
            code(
                "rust",
                "fn main() {\n    // A block that produces a value\n    let result = {\n        let base = 100;\n        let bonus = 50;\n        base + bonus        // 150\n    };\n\n    // The same block, but the value is thrown away\n    let discarded = {\n        let base = 100;\n        let bonus = 50;\n        base + bonus;       // semicolon — produces ()\n    };\n\n    println!(\"result: {}\", result);\n    println!(\"discarded still works: {}\", discarded);\n}",
            ),
            callout(
                "Practice — Try this now",
                "Write a block that computes `let x = 5;` then `let y = x * 2;` then returns `x + y` as the \
                 block's value. Assign it to `total` and print. Now add a semicolon to the last line and print \
                 again — see what breaks.",
            ),
            callout(
                "Mini challenge",
                "Write three blocks. The first returns a whole number. The second returns a float. The third \
                 contains two nested blocks and returns the sum of both. Assign each to a variable and print. \
                 Then remove the final semicolon from one of them and read the error carefully — it is a \
                 preview of module 09.",
            ),
        ],
    )
}

fn lesson_reading_errors() -> SeedLesson {
    SeedLesson::new(
        "Reading Type Errors",
        vec![
            labelled(
                "CONCEPT",
                "Compiler errors are not rejection notices. They are the closest thing Rust has to a pair \
                 programmer sitting next to you. Learn to read them and you will stop treating them as walls.",
            ),
            code(
                "rust",
                "fn main() {\n    let age: i32 = \"25\";\n}",
            ),
            code(
                "text",
                "error[E0308]: mismatched types\n --> src/main.rs:2:20\n  |\n2 |     let age: i32 = \"25\";\n  |                    ^^^^   expected `i32`, found `&str`\n  |                    |\n  |                    expected because of this annotation\n  |                    found because of this expression",
            ),
            steps(&[
                "expected `i32` — what Rust believed you wanted (from your annotation)",
                "found `&str` — what your value actually is (a string slice)",
                "because of this annotation — which line of code created that expectation",
                "because of this expression — which line produced the real value",
            ]),
            labelled(
                "EXPLANATION",
                "Every type error has the same two-part shape. Rust states what it expected, states what it \
                 found, and points at the two places those beliefs came from. Once you scan for those four \
                 pieces, a `mismatched types` error takes about three seconds instead of three minutes.",
            ),
            callout(
                "Three errors you will meet today",
                "`expected i32, found &str` — you put a string where a number belongs. \
                 `expected u8, found u32` — the types are close but not identical. \
                 `cannot assign twice to immutable variable` — you forgot `mut`.",
            ),
            code(
                "rust",
                "fn main() {\n    let x: i32 = \"hello\";      // expected i32, found &str\n\n    let y: u8 = \"300\";        // expected u8, found &str\n\n    let z = 5;\n    z = 10;                     // cannot assign twice to immutable variable `z`\n\n    let a: i32 = 10;\n    let b: i64 = 20;\n    let c = a + b;               // expected i32, found i64\n}",
            ),
            callout(
                "Practice — Try this now",
                "Write all four of the broken lines above in one file. Run it. Read the errors in order. For \
                 each one, write down: what was expected, what was found, and the one-line fix. Do not fix them \
                 in the file yet — reading first is the skill.",
            ),
            callout(
                "Mini challenge",
                "Deliberately produce an error that mentions a type you have not used yet, such as `u128` or \
                 `i64`. Read the message and extract the same four pieces. Then introduce an error with a \
                 *named* type like `String` or `Vec` and see how the message shape stays identical even when \
                 the type is far more complicated.",
            ),
        ],
    )
}

fn lesson_common_mistakes() -> SeedLesson {
    SeedLesson::new(
        "Things You Will Probably Break",
        vec![
            labelled(
                "CONCEPT",
                "Six mistakes that account for most of what beginners hit in their first week. None of them \
                 are stupid. All of them have a shape, and once you see the shape you stop making them.",
            ),
            callout(
                "Mistake 1 — Trying to mutate an immutable variable",
                "`let score = 10; score = 20;` — the compiler says cannot assign twice to immutable variable. \
                 The concept is that mutability is opt-in. Fix: `let mut score = 10;`.",
            ),
            code(
                "rust",
                "fn main() {\n    let score = 10;\n    score = 20;   // error[E0384]\n}",
            ),
            callout(
                "Mistake 2 — Mixing numeric types",
                "`let a: i32 = 10; let b: i64 = 20; let total = a + b;` — expected i32, found i64. \
                 The concept is that integer types are distinct. Fix: cast with `as`, or annotate both the same.",
            ),
            code(
                "rust",
                "fn main() {\n    let a: i32 = 10;\n    let b: i64 = 20;\n    let total = a as i64 + b;   // fix\n}",
            ),
            callout(
                "Mistake 3 — Confusing char and strings",
                "`let letter = \"A\";` when you meant a char. The concept is that single quotes are a char, \
                 double quotes are a &str. Fix: `let letter = 'A';`.",
            ),
            code(
                "rust",
                "fn main() {\n    let letter: char = 'A';    // fix\n    let text: &str = \"A\";     // also valid, but a different type\n}",
            ),
            callout(
                "Mistake 4 — Wrong array index",
                "`values[10]` on a three-element array. The concept is that indexing is checked at runtime, not \
                 compile time. Fix: check the length, or stay inside the range.",
            ),
            code(
                "rust",
                "fn main() {\n    let values = [1, 2, 3];\n    println!(\"{}\", values[0]);   // fine\n    // println!(\"{}\", values[10]);  // panics at runtime\n}",
            ),
            callout(
                "Mistake 5 — Expecting mutation to change a variable's type",
                "`let mut value = \"42\"; value = 42;` — expected &str, found integer. The concept is that \
                 a mutable binding's type is fixed at declaration. Fix: shadow instead.",
            ),
            code(
                "rust",
                "fn main() {\n    let mut value = \"42\";\n    // value = 42;                 // error: type is fixed\n    let value: i32 = value.parse().unwrap();   // shadow instead — works\n}",
            ),
            callout(
                "Mistake 6 — Forgetting an annotation when Rust cannot infer",
                "`let empty = Vec::new();` — the compiler says type annotations needed. The concept is that \
                 an empty collection carries no element information. Fix: annotate the element type.",
            ),
            code(
                "rust",
                "fn main() {\n    // let empty = Vec::new();            // error: cannot infer type\n    let empty: Vec<i32> = Vec::new();  // fix\n    let scores: [i32; 0] = [];\n\n    println!(\"{} {}\", empty.len(), scores.len());\n}",
            ),
            callout(
                "Practice — Try this now",
                "Write a file with all six mistakes, uncommented and unfixed. Run it. Work through the errors \
                 one at a time, fixing as you go, and note which of the six felt most surprising.",
            ),
            callout(
                "Mini challenge",
                "Invent a seventh mistake. It should be something a beginner would plausibly write, that \
                 produces a real compiler error, and that teaches a concept from this module. Write the broken \
                 version, the error it produces, and the fix. Then verify all three by running them.",
            ),
        ],
    )
}

fn lesson_bridge() -> SeedLesson {
    SeedLesson::new(
        "Putting It All Together",
        vec![
            labelled(
                "CONCEPT",
                "You now have the vocabulary. Variables store information, types describe it, `mut` controls \
                 change, shadowing transforms, and conversions move between representations. Time to use all of \
                 it at once, and then to notice what is still missing.",
            ),
            code(
                "rust",
                "const MAX_PLAYERS: usize = 4;\n\nfn main() {\n    // Immutable facts about the game\n    let game_title: &str = \"Dungeon Run\";\n    let starting_health: u16 = 100;\n\n    // Mutable state that changes\n    let mut players: usize = 0;\n    let mut high_score: u32 = 0;\n\n    // Shadowing to transform representations\n    let score_input = \"1500\";\n    let score_input: u32 = score_input.parse().unwrap();\n    let score_input = score_input as i64 * 2;\n\n    // A tuple for data that travels together\n    let player = (\"Gus\", starting_health, true);\n\n    // An array for a fixed collection\n    let party: [&str; 4] = [\"Gus\", \"Ada\", \"Lin\", \"Sam\"];\n\n    players = party.len();\n    high_score = score_input as u32;\n\n    println!(\"=== {} ===\", game_title);\n    println!(\"Players: {} / {}\", players, MAX_PLAYERS);\n    println!(\"Player 1: {} ({} HP, alive: {})\", player.0, player.1, player.2);\n    println!(\"Party: {:?}\", party);\n    println!(\"High score: {}\", high_score);\n}",
            ),
            labelled(
                "EXPLANATION",
                "That program uses a constant, immutable bindings, mutable bindings, a tuple, an array, \
                 shadowing across two type changes, an explicit `as` conversion, and a `.parse()`. None of it \
                 is advanced. All of it is the vocabulary of this module. Read it and notice that every choice \
                 had a reason.",
            ),
            callout(
                "Cool. We can store information now.",
                "But storing information isn't very interesting if our program can't do anything with it.",
            ),
            paragraphs(&[
                "Right now your game has four players and a high score, and the program does exactly the \
                 same thing every time you run it. What if the player has no health left? What if the score \
                 went above 100? What if the user picked an option that does not exist? What if you need to \
                 do the same thing fifty times?",
                "What if there are seven possible states and you need to handle each one?",
            ]),
            callout(
                "That's where control flow comes in",
                "`if`, `match`, `loop`, `while`, `for`. The next module is about making decisions and \
                 repeating work. Every one of those tools produces a value, which is exactly why the \
                 expression lesson in this module was worth sitting through.",
            ),
            callout(
                "Mini challenge — the gate",
                "Extend the program above with three new pieces of data: a `difficulty` as a `&str` (immutable), \
                 a `lives_left` as a `u8` (mutable, starts at 3), and a `completed` as a `bool` (immutable, \
                 `false`). Print them all in a summary block. Do not add any `if` yet. Just prove to yourself \
                 that you can choose types and mutability deliberately.",
            ),
        ],
    )
}

// ─── HELPERS ───────────────────────────────────────────────────────────────

/// A paragraph prefixed with a section label, e.g. "CONCEPT — ...".
/// A single argument is used verbatim, which suits continuation paragraphs.
fn text_block(text: &str) -> serde_json::Value {
    super::types::text(text)
}

/// A paragraph explicitly tagged with a label from the 6-step strategy.
fn labelled(label: &str, body: &str) -> serde_json::Value {
    super::types::text(&format!("{label} — {body}"))
}

// ─── EXERCISES ──────────────────────────────────────────────────────────────

fn exercises() -> Vec<SeedExercise> {
    vec![
        // L1 — what is a variable
        SeedExercise::mcq(
            0,
            "What is the main reason we give values names?",
            &[
                "To make programs run faster",
                "So the code communicates what values mean",
                "Because Rust requires names",
                "To reduce memory usage",
            ],
            1,
            "The compiler does not care about names. `100 > 50` and `score > 50` compile to the same thing. The value is entirely in what the reader understands.",
        ),
        SeedExercise::mcq(
            0,
            "In `let score = 100;` what is `score`?",
            &["The value", "The type", "The binding (name)", "The keyword"],
            2,
            "`score` is the name. `100` is the value. `let` is the keyword that creates the binding between them.",
        )
        .kind(ExerciseKind::TypeDetective)
        .with_code("fn main() {\n    let username = \"Gus\";\n    let score = 100;\n    let logged_in = true;\n}")
        .with_hints(&["Whole number", "Text", "Two values"]),

        // L2 — immutable by default
        SeedExercise::mcq(
            1,
            "What does Rust say if you reassign a `let` binding with no `mut`?",
            &[
                "A runtime panic",
                "error[E0384]: cannot assign twice to immutable variable",
                "A warning, then it works",
                "Nothing — it just allows it",
            ],
            1,
            "It is a hard compile error, not a warning and not a runtime failure. Rust stops before your program ever runs.",
        ),
        SeedExercise::mcq(
            1,
            "Why is a compile-time rejection better than a runtime crash here?",
            &[
                "Because it is faster to write",
                "Because the program never runs with the bug present",
                "Because Rust is faster at runtime",
                "Because it uses less memory",
            ],
            1,
            "If the compiler catches it, no user ever runs the broken version. The class of bug simply cannot ship. That is the whole promise of a type system.",
        )
        .kind(ExerciseKind::FixCode)
        .with_code("fn main() {\n    let score = 10;\n    score = 20;\n    println!(\"{}\", score);\n}")
        .with_hints(&["What does the binding need?", "One keyword"]),

        // L3 — mut
        SeedExercise::mcq(
            2,
            "What does `let mut health = 100;` create?",
            &[
                "A mutable value",
                "A mutable binding that holds 100",
                "A constant named health",
                "A reference to 100",
            ],
            1,
            "Mutability attaches to the binding, not the value. This distinction matters enormously once you reach ownership.",
        ),
        SeedExercise::mcq(
            2,
            "After `let mut count = 5; count += 3;` what is `count`?",
            &["3", "5", "8", "It does not compile"],
            2,
            "`+= 3` adds 3 to the existing value. 5 + 3 = 8. The semicolon at the end of a line is a statement terminator, not part of the expression.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    let mut health = 100;\n    health -= 10;\n    health -= 20;\n    println!(\"{}\", health);\n}")
        .detached(),

        // L4 — why immutable
        SeedExercise::mcq(
            3,
            "What does `let user_count = 100;` promise to any later reader?",
            &[
                "That user_count is a number",
                "That user_count is still 100 wherever it appears",
                "That user_count was written last",
                "Nothing — `let` is just shorthand",
            ],
            1,
            "Because nothing in the program can change it, reading `user_count` later tells you something reliable. That reliability is the point.",
        ),
        SeedExercise::mcq(
            3,
            "A function takes `value: i32` and you pass `user_count`. Can the function change `user_count`?",
            &["Yes, it can reassign it", "No, it has no way to reach it", "Only if it is `mut`", "Only in release mode"],
            1,
            "The function received a copy of the value. It has no binding to your variable at all. (Reference semantics come later — in the borrowing module.)",
        ),

        // L5 — inference
        SeedExercise::mcq(
            4,
            "What type does Rust infer for `let x = 42;`?",
            &["u8", "i32", "i64", "f64"],
            1,
            "Whole-number literals with no other context default to i32. It covers roughly ±2.1 billion, which is a safe default.",
        )
        .kind(ExerciseKind::TypeDetective)
        .with_code("fn main() {\n    let a = 42;\n    let b = 42.0;\n    let c = 'a';\n    let d = \"a\";\n    let e = true;\n}")
        .with_hints(&[
            "Whole number → i32",
            "Has a decimal point → f64",
            "Single quotes → char, double quotes → &str",
        ]),
        SeedExercise::mcq(
            4,
            "Which variable did Rust NOT infer a type for in this code?",
            &[
                "let a = 1;",
                "let b = \"hello\";",
                "let c = vec![];",
                "let d = 3.14;",
            ],
            2,
            "An empty Vec has no elements to look at, so there is nothing to infer from. This is exactly when an annotation is required.",
        ),

        // L6 — annotations
        SeedExercise::mcq(
            5,
            "In `let age: i32 = 25;` what is `: i32`?",
            &["A cast", "A type annotation", "A label", "A comment"],
            1,
            "A type annotation. It tells Rust what type the binding should have, and Rust will check the value against it.",
        ),
        SeedExercise::mcq(
            5,
            "When is a type annotation genuinely necessary?",
            &[
                "Always, for every variable",
                "When Rust cannot infer the type, e.g. an empty collection",
                "Only in release builds",
                "Only for numbers",
            ],
            1,
            "Annotations are a tool, not a tax. `let scores: Vec<i32> = Vec::new();` needs one. `let x = 5;` does not.",
        )
        .kind(ExerciseKind::FixCode)
        .with_code("fn main() {\n    let scores = Vec::new();\n    println!(\"{}\", scores.len());\n}")
        .with_hints(&["An empty collection tells Rust nothing about its elements"]),

        // L7 — integer types
        SeedExercise::mcq(
            6,
            "What is the maximum value a `u8` can hold?",
            &["127", "255", "256", "65535"],
            1,
            "u8 is unsigned, so it starts at 0. 8 bits gives 256 distinct values: 0 through 255. i8 would be -128 through 127.",
        ),
        SeedExercise::mcq(
            6,
            "In `u16`, what do `u` and `16` mean?",
            &[
                "unsigned, 16 bits",
                "unbounded, 16 values",
                "unique, 16 digits",
                "unsigned integer, base 16",
            ],
            0,
            "The leading letter is the signedness, the number is the bit width. `i` = signed, `u` = unsigned.",
        ),
        SeedExercise::mcq(
            6,
            "What is the largest value `i8` can hold?",
            &["0", "127", "128", "255"],
            1,
            "i8 is signed 8-bit: -128 to 127. Half the range goes to negatives, so the positive side stops at 127.",
        )
        .kind(ExerciseKind::FixCode)
        .with_code("fn main() {\n    let value: u8 = 256;\n    println!(\"{}\", value);\n}")
        .with_hints(&["What is the maximum u8?", "Maybe u16 is a better fit"]),

        // L8 — literals
        SeedExercise::mcq(
            7,
            "What is `0b1010` equal to in decimal?",
            &["10", "8", "12", "20"],
            0,
            "Binary reads right-to-left: 8 + 2 = 10. The `0b` prefix marks it as binary.",
        ),
        SeedExercise::mcq(
            7,
            "What is `0o755` equal to in decimal?",
            &["755", "493", "435", "1000"],
            1,
            "Octal: 7×64 + 5×8 + 5 = 448 + 40 + 5 = 493. You will see this constantly in file permissions.",
        ),
        SeedExercise::mcq(
            7,
            "What do the underscores in `1_000_000` do?",
            &[
                "Multiply the number by 1000",
                "Nothing — Rust ignores them entirely",
                "Make it a float",
                "Mark it as a constant",
            ],
            1,
            "They are purely for human readability. `1_000_000` and `1000000` compile to exactly the same value.",
        ),

        // L9 — signed vs unsigned
        SeedExercise::mcq(
            8,
            "Why is `let age: u32 = -1;` rejected?",
            &[
                "Because -1 is not a number",
                "Because u32 cannot represent negative values",
                "Because u32 is too small for -1",
                "Because ages must be i32",
            ],
            1,
            "Unsigned types have no room for negatives — they start at 0. Rust agrees that an age of -1 is not a real thing.",
        ),
        SeedExercise::mcq(
            8,
            "Which of these should most likely be `usize`?",
            &["A player's level", "The number of items in a list", "A temperature", "A score change"],
            1,
            "usize is pointer-sized and unsigned — ideal for counts and lengths, which is exactly what indexing and Vec::len() use.",
        ),

        // L10 — floats
        SeedExercise::mcq(
            9,
            "What type does Rust infer for `let price = 19.99;`?",
            &["f32", "f64", "i32", "f128"],
            1,
            "Floating-point literals with no other context default to f64. It is the safer default — more precision.",
        ),
        SeedExercise::mcq(
            9,
            "Why does `0.1 + 0.2` not equal `0.3` exactly?",
            &[
                "Rust has a rounding bug",
                "0.1 has no exact binary representation",
                "f64 is too small",
                "It actually does equal 0.3",
            ],
            1,
            "0.1 is not exactly representable in binary, in any language with binary floats. The stored value is extremely close, but not identical.",
        ),
        SeedExercise::mcq(
            9,
            "What does `let x: f64 = 3.99 as i32;` produce?",
            &["4", "3", "0", "A compile error"],
            1,
            "`as` truncates toward zero — it does not round. 3.99 becomes 3. This surprises people regularly.",
        ),

        // L11 — bools
        SeedExercise::mcq(
            10,
            "How much memory does a `bool` occupy?",
            &["0 bytes", "1 byte", "4 bytes", "8 bytes"],
            1,
            "A bool has exactly two values, so one byte is enough. It is the smallest type in Rust.",
        ),
        SeedExercise::mcq(
            10,
            "Which of these is a valid `bool` literal?",
            &["True", "true", "TRUE", "1"],
            1,
            "Rust is lowercase: `true` and `false`. `True` is an undefined name and will not compile.",
        ),

        // L12 — char
        SeedExercise::mcq(
            11,
            "How many bytes does a Rust `char` always occupy?",
            &["1", "2", "4", "It varies"],
            2,
            "Always 4. A char is a Unicode scalar value, so it needs a fixed width regardless of how many UTF-8 bytes the glyph itself takes.",
        ),
        SeedExercise::mcq(
            11,
            "What is the difference between 'A' and \"A\"?",
            &[
                "They are identical",
                "'A' is a char; \"A\" is a &str",
                "'A' is ASCII; \"A\" is Unicode",
                "Double quotes are deprecated",
            ],
            1,
            "Single quotes hold exactly one Unicode scalar value. Double quotes hold a string slice — a whole run of text. Different types entirely.",
        )
        .kind(ExerciseKind::TypeDetective)
        .with_code("fn main() {\n    let a = 'x';\n    let b = \"x\";\n    let c = '🥷';\n}")
        .with_hints(&["Single quotes → char", "Double quotes → &str"]),

        // L13 — tuples
        SeedExercise::mcq(
            12,
            "How do you access the second element of `let pair = (10, 20);`?",
            &["pair[1]", "pair.1", "pair.2", "pair.get(1)"],
            1,
            "Tuples use dot notation with a numeric field: `.0` is first, `.1` is second. Indexing with square brackets is for arrays.",
        ),
        SeedExercise::mcq(
            12,
            "What does `let (a, b) = (1, 2);` do?",
            &[
                "Creates a tuple of two variables",
                "Destructures the tuple into a and b",
                "Swaps a and b",
                "Compares the tuples",
            ],
            1,
            "Destructuring pulls the values out of the tuple and binds them to names. Much more readable than .0 and .1.",
        ),

        // L14 — arrays
        SeedExercise::mcq(
            13,
            "What does `[i32; 5]` describe?",
            &[
                "An array of 5 i32 values",
                "A tuple with 5 elements",
                "A slice of unknown length",
                "An integer of value 5",
            ],
            0,
            "Square brackets = array, inner type = element type, the number after the semicolon = length. The length is part of the type.",
        ),
        SeedExercise::mcq(
            13,
            "What is the first element of `[10, 20, 30]`?",
            &["10", "20", "30", "There is no first element"],
            0,
            "Indexing is zero-based. `arr[0]` is the first, `arr[1]` is the second. This trips up everyone exactly once.",
        ),
        SeedExercise::mcq(
            13,
            "What is `let zeros: [u8; 4] = [0; 4];`?",
            &[
                "An array of four zeros",
                "An array of zero elements",
                "The number 0, four times as an integer",
                "A syntax error",
            ],
            0,
            "The `[value; count]` form repeats a value. `[0; 4]` is `[0, 0, 0, 0]` — extremely handy for fixed buffers.",
        ),

        // L15 — constants
        SeedExercise::mcq(
            14,
            "What does a `const` require that a `let` does not?",
            &["A value", "A type annotation", "A name in caps", "mut"],
            1,
            "Constants must state their type explicitly. `const MAX: u32 = 100;` — no inference. The compiler needs it before the program runs.",
        ),
        SeedExercise::mcq(
            14,
            "What is the conventional naming style for constants?",
            &["camelCase", "snake_case", "SCREAMING_SNAKE_CASE", "PascalCase"],
            2,
            "SCREAMING_SNAKE_CASE signals at a glance that this is a fixed named fact rather than a working value.",
        ),
        SeedExercise::mcq(
            14,
            "What is special about a constant compared to a variable?",
            &[
                "It is faster at runtime",
                "It is known at compile time",
                "It cannot be printed",
                "It must be a number",
            ],
            1,
            "Constants are resolved before the program runs, which is why they can be used where a runtime value cannot — like an array length.",
        ),

        // L16/L17/L18 — shadowing cluster
        SeedExercise::mcq(
            15,
            "What does `let x = 5; let x = 6;` do?",
            &[
                "Changes x from 5 to 6",
                "Creates a new binding that shadows the first",
                "Is a compile error",
                "Creates a mutable x",
            ],
            1,
            "The second `let` creates a brand-new binding. The first x still exists at 5, just unreachable by that name.",
        ),
        SeedExercise::mcq(
            15,
            "After `let x = 5; let x = x + 1; let x = x * 2;` what is x?",
            &["5", "6", "12", "10"],
            2,
            "5 → 6 → 12. Each `let` reads the current value and produces a new binding with the new one.",
        )
        .kind(ExerciseKind::SpotDifference)
        .with_code("let mut x = 5;\nx = 6;")
        .with_code_b("let x = 5;\nlet x = 6;")
        .with_hints(&["Look for the `let` on the second line"]),
        SeedExercise::mcq(
            16,
            "In `let x = 5; let x = 6;` is the original value of x still stored anywhere?",
            &["No, it is overwritten", "Yes, it exists but is unreachable by that name", "No, it is deleted", "Yes, you can access it as x.0"],
            1,
            "Shadowing hides a binding; it does not destroy it. The old x still holds 5 until it goes out of scope.",
        ),
        SeedExercise::mcq(
            16,
            "Which line mutates, and which shadows?",
            &[
                "`x = 6;` mutates, `let x = 6;` shadows",
                "Both mutate",
                "Both shadow",
                "Neither does anything",
            ],
            0,
            "The presence of `let` is the tell. `x = 6;` reassigns. `let x = 6;` creates a new binding.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    let x = 5;\n    let x = x + 1;\n    let x = x * 2;\n    println!(\"{}\", x);\n}")
        .detached(),
        SeedExercise::mcq(
            17,
            "Which of these can change a variable's type?",
            &[
                "`let mut x = \"42\"; x = 42;`",
                "`let x = \"42\"; let x: i32 = x.parse().unwrap();`",
                "Neither works",
                "Both work",
            ],
            1,
            "Mutation fixes the type at declaration. Shadowing creates a new binding, which is free to have a different type.",
        )
        .kind(ExerciseKind::FixCode)
        .with_code("fn main() {\n    let mut value = \"42\";\n    value = 42;\n    println!(\"{}\", value);\n}")
        .with_hints(&[
            "A mutable binding keeps its original type",
            "Try shadowing with a new `let` instead",
        ]),

        // L19 — conversions
        SeedExercise::mcq(
            18,
            "How do you convert an `i32` variable into an `f64`?",
            &["let y: f64 = x;", "let y: f64 = x as f64;", "convert(x, f64)", "let y = f64(x);"],
            1,
            "Rust never converts implicitly. `as` makes the conversion explicit and visible in the source.",
        ),
        SeedExercise::mcq(
            18,
            "What does `let x: i32 = 3.99 as i32;` produce?",
            &["4", "3", "0", "A compile error"],
            1,
            "Casting float to integer truncates toward zero. It does not round. 3.99 becomes 3.",
        ),
        SeedExercise::mcq(
            18,
            "Which conversion is guaranteed to lose no information?",
            &[
                "f64 → i32",
                "i32 → u8 (when the value is 300)",
                "i32 → i64",
                "u64 → u8",
            ],
            2,
            "Widening a signed integer to a larger signed integer is always exact. Narrowing in either direction can lose information.",
        ),

        // L20 — compatibility
        SeedExercise::mcq(
            19,
            "Why does `let x: i32 = 10; let y: i64 = 20; x + y` fail?",
            &[
                "i64 is not a real type",
                "i32 and i64 are different types, and + requires both sides to match",
                "The addition would overflow",
                "Rust does not support addition on i32",
            ],
            1,
            "Rust does not promote types automatically. `+` needs both operands to be the same type, and these are not.",
        )
        .kind(ExerciseKind::FixCode)
        .with_code("fn main() {\n    let x: i32 = 10;\n    let y: i64 = 20;\n    let total = x + y;\n    println!(\"{}\", total);\n}")
        .with_hints(&["Which direction do you want to convert?", "`as` makes it explicit"]),
        SeedExercise::mcq(
            19,
            "What does this error mean? `error[E0308]: mismatched types — expected i32, found &str`",
            &[
                "Your code is too long",
                "Rust expected an i32 but your value is a string",
                "The i32 type is missing from your Cargo.toml",
                "You need to install a crate",
            ],
            1,
            "Two-part error: what Rust expected, and what it actually got. Scan for those two words every time.",
        ),

        // L21 — expressions
        SeedExercise::mcq(
            20,
            "What does a block's value come from?",
            &[
                "The first statement",
                "The final expression, if it has no semicolon",
                "The last line, always",
                "Whatever you return",
            ],
            1,
            "The final expression without a trailing semicolon is the block's value. Add the semicolon and the block produces (), which means nothing.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    let y = {\n        let a = 10;\n        let b = 20;\n        a + b\n    };\n    println!(\"{}\", y);\n}")
        .detached(),
        SeedExercise::mcq(
            20,
            "What happens if you add a semicolon to `a + b` inside that block?",
            &[
                "The block produces no value, so `y` cannot be used as a number",
                "Nothing changes",
                "It rounds the result",
                "The block returns 0",
            ],
            0,
            "`a + b;` becomes a statement whose value is discarded. The block then yields `()`, and using it as a number is an error.",
        ),

        // L22 — reading errors
        SeedExercise::mcq(
            21,
            "In `error[E0308]: expected i32, found &str`, what do the two words tell you?",
            &[
                "How long the file is and how many lines it has",
                "What Rust wanted and what you gave it",
                "The line number and the column",
                "The type to import and the crate to install",
            ],
            1,
            "Expected = Rust's belief, from your annotation. Found = reality, from your value. Fix one or the other.",
        )
        .kind(ExerciseKind::CompilerChallenge)
        .with_code("error[E0308]: mismatched types\n --> src/main.rs:2:20\n  |\n2 |     let age: i32 = \"25\";\n  |                    ^^^^   expected `i32`, found `&str`")
        .with_hints(&["Read the words after 'expected' and 'found'", "What did the annotation say?"]),
        SeedExercise::mcq(
            21,
            "What should you read first in a `mismatched types` error?",
            &[
                "The line number",
                "The `expected` and `found` types",
                "The error code",
                "The `help:` suggestion at the bottom",
            ],
            1,
            "The two types tell you what the disagreement is. The help line is useful, but only once you know what went wrong.",
        ),

        // L23 — common mistakes
        SeedExercise::mcq(
            22,
            "You write `let empty = Vec::new();` and get 'type annotations needed'. Why?",
            &[
                "Vec is not imported",
                "An empty collection has no elements to infer a type from",
                "You need to call .collect() first",
                "Vec::new() is deprecated",
            ],
            1,
            "Inference works from values. With zero values, there is nothing to go on. Annotate: `let empty: Vec<i32> = Vec::new();`",
        )
        .kind(ExerciseKind::SpotDifference)
        .with_code("let letter = 'A';")
        .with_code_b("let letter = \"A\";")
        .with_hints(&["One is a char, one is a &str"]),
        SeedExercise::mcq(
            22,
            "Which mistake does NOT produce a compile error?",
            &[
                "Reassigning an immutable variable",
                "Mixing i32 and i64",
                "Indexing past the end of an array",
                "Using `True` instead of `true`",
            ],
            2,
            "Out-of-bounds indexing is caught at runtime, not compile time. That distinction matters and is worth remembering.",
        ),

        // L24 — bridge
        SeedExercise::mcq(
            23,
            "You have `let name = \"Gus\";` and want a numeric id. What is the idiomatic approach?",
            &[
                "Make name mutable and assign an integer",
                "Shadow name with a new binding of the new type",
                "Use a cast: name as i32",
                "You cannot change a variable's type in Rust",
            ],
            1,
            "Shadowing. `let name = name.parse().unwrap();` creates a new binding with a new type. Mutation cannot do this.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    let value = \"42\";\n    let value: i32 = value.parse().unwrap();\n    println!(\"{}\", value + 8);\n}")
        .detached(),

        // ── MODULE CHALLENGE: The Type Detective ────────────────────────
        SeedExercise {
            lesson_index: None,
            kind: ExerciseKind::Challenge,
            prompt: "THE TYPE DETECTIVE — A program that does not compile, and your job is to make it work."
                .to_string(),
            code: Some(CHALLENGE_CODE.to_string()),
            code_b: None,
            options: vec![
"Identify the type of every variable".to_string(),
"Find all four errors and explain each one".to_string(),
"Fix the program so it compiles and runs".to_string(),
"Predict the correct output once fixed".to_string(),
            ],
            correct_answer: 0,
            explanation: CHALLENGE_SOLUTION.to_string(),
            xp_reward: 100,
            hints: vec![
"Error 1 is a subtraction between two different types. Which one is the odd one out?".to_string(),
"Error 2 is a string being assigned where a number is required. Shadowing is your friend here.".to_string(),
"Error 3 is not a compile error at all — it compiles fine and panics when it runs. How many elements does the array actually have?".to_string(),
"Error 4 is a number too large for the type you chose. 300 needs at least 16 bits.".to_string(),
            ],
        },

        // ── BUILD IT ─────────────────────────────────────────────────────
        SeedExercise {
            lesson_index: None,
            kind: ExerciseKind::BuildIt,
            prompt: "BUILD IT — Write a program that reports on a game session.".to_string(),
            code: None,
            code_b: None,
            options: vec![
"Store the player's name as a &str and level as a u8".to_string(),
"Store the current health as a mutable i32, starting at 100".to_string(),
"Define a const MAX_HEALTH: i32 = 100 and compare health against it".to_string(),
"Store a tuple of (name, level, health) and destructure it to print each field".to_string(),
"Shadow a &str into a u8 using .parse().unwrap(), then double it".to_string(),
            ],
            correct_answer: 0,
            explanation: "There is no single right answer here. What matters is that you can state WHY each type was chosen, and that your mutable and immutable bindings are deliberate."
                .to_string(),
            xp_reward: 50,
            hints: vec![
"Start with the immutable facts, then the mutable state".to_string(),
"A const needs an explicit type — there is no inference there".to_string(),
            ],
        },
    ]
}

const CHALLENGE_CODE: &str = r#"fn main() {
    let player_name = "Gus";
    let mut health = 100;
    let max_health: u32 = 100;
    let damage = 25.5;

    // ERROR 1: health is i32, damage is f64
    health = health - damage;

    let score = "100";
    // ERROR 2: &str cannot be assigned to i32
    let score: i32 = score;

    let values = [1, 2, 3];
    // ERROR 3: compiles, but panics at runtime
    println!("{}", values[5]);

    // ERROR 4: 300 does not fit in a u8
    let temperature: u8 = 300;
}"#;

const CHALLENGE_SOLUTION: &str = r#"Here is a fully corrected version with the reasoning attached:

1. health = health - damage;
   health is i32, damage is f64. Mixing them is a type error.
   Fix: either make health an f64, or make damage an integer.
   `let damage: i32 = 25;` is the smaller change, but a fractional damage
   value is realistic, so f64 is arguably the better fix:
       let mut health: f64 = 100.0;
       let damage: f64 = 25.5;

2. let score: i32 = score;
   score is a &str. The annotation does not convert it — it just states a
   requirement the value does not meet.
   Fix: shadow with a parse:
       let score = "100";
       let score: i32 = score.parse().unwrap();

3. println!("{}", values[5]);
   values has 3 elements: indices 0, 1, 2. Index 5 is out of bounds.
   This one is NOT a compile error. Rust checks it at runtime and panics.
   Fix: use a valid index, or check the length first.

4. let temperature: u8 = 300;
   u8 tops out at 255.
   Fix: widen the type — let temperature: u16 = 300; — or clamp the value."#;

// ─── PROJECTS ──────────────────────────────────────────────────────────────

fn projects() -> Vec<SeedProject> {
    vec![
        SeedProject::new(
            "Personal Profile",
            "Store a small set of facts about yourself and print them as a profile card.",
            Difficulty::Beginner,
            "15 min",
            &[
                "Store a name, an age, a favourite number, and an online status",
                "Print a formatted profile card with at least four lines",
                "Use an immutable binding for anything that never changes",
            ],
            &["variables", "&str", "integers", "booleans"],
            &[
                "Every variable must be `let` — do not use `mut` at all",
                "Infer types where Rust can work it out",
            ],
            "Name: Gus\nAge: 25\nFavourite number: 42\nOnline: true",
            &["Add a `char` status icon", "Add the profile inside a tuple"],
        ),
        SeedProject::new(
            "Unit Converter",
            "Convert between three common unit pairs and print the results.",
            Difficulty::Beginner,
            "20 min",
            &[
                "Convert kilometres to miles",
                "Convert Celsius to Fahrenheit",
                "Convert kilograms to pounds",
                "Print each result with the original value",
            ],
            &["f64", "expressions", "type annotations", "constants"],
            &[
                "Use `f64` for every measurement",
                "Define the conversion factors as constants",
                "Do not use `mut` — these are one-off calculations",
            ],
            "10.0 km = 6.21 miles\n25.0 C = 77.0 F\n70.0 kg = 154.3 lbs",
            &["Add the reverse conversions", "Round the output to two decimals"],
        ),
        SeedProject::new(
            "Character Analyzer",
            "Given a character, print everything Rust can tell you about it.",
            Difficulty::Beginner,
            "15 min",
            &[
                "Store one `char` value",
                "Report whether it is alphabetic, numeric, uppercase, and alphanumeric",
                "Print a one-word summary",
            ],
            &["char", "bool", "method calls", "expressions"],
            &[
                "Use the char's built-in methods — do not compare against a list",
                "No `mut` required",
            ],
            "Analyzing 'R'\n  alphabetic:  true\n  numeric:     false\n  uppercase:   true\n  alphanumeric: true",
            &["Print the Unicode scalar value", "Analyze several characters in one array"],
        ),
        SeedProject::new(
            "Score Calculator",
            "Store several test scores and compute the statistics.",
            Difficulty::Beginner,
            "20 min",
            &[
                "Store five scores in a fixed-size array",
                "Calculate the total and the average",
                "Print both, with the average to one decimal place",
            ],
            &["arrays", "i32", "f64", "mutability", "arithmetic"],
            &[
                "Use a fixed-size array, not a Vec",
                "Use a `mut` running total while summing",
                "The array must be `[i32; 5]`",
            ],
            "Scores: 80, 90, 70, 85, 95\nTotal: 420\nAverage: 84.0",
            &["Find the highest and lowest score", "Report whether the average passed"],
        ),
        SeedProject::new(
            "Player Stats",
            "Build a minimal RPG character sheet with mutable state.",
            Difficulty::Intermediate,
            "30 min",
            &[
                "Store a name, health, level, experience, and an alive status",
                "Health starts at 100 and level at 1",
                "Simulate taking damage and updating health",
                "Print the full sheet before and after damage",
            ],
            &["variables", "mutability", "numeric types", "booleans", "shadowing"],
            &[
                "Health must be a `u16` and experience a `u32`",
                "Name and alive must be immutable",
                "Derive `alive` using an expression, not a stored duplicate",
            ],
            "=== Gus ===\nHP: 100/100  Lv: 1  XP: 0\nAlive: true\n\n--- took 35 damage ---\n\n=== Gus ===\nHP: 65/100  Lv: 1  XP: 0\nAlive: true",
            &["Add an experience reward for the kill", "Add a level-up that increases health"],
        ),
        SeedProject::new(
            "Shopping Receipt",
            "Calculate a subtotal, tax, and final total for a small basket.",
            Difficulty::Intermediate,
            "30 min",
            &[
                "Store three items with a name, unit price, and quantity",
                "Calculate the subtotal",
                "Apply a tax rate defined as a constant",
                "Print a receipt with the final total to two decimal places",
            ],
            &["f64", "i32", "expressions", "constants", "tuples"],
            &[
                "Prices must be `f64`, quantities must be `u32`",
                "The tax rate must be a `const`",
                "Format all money to exactly two decimal places",
            ],
            "ITEM              QTY   PRICE\nCoffee             2     9.00\nMug                1    12.50\nBeans              1    14.00\n\nSubtotal:  44.50\nTax (8%):   3.56\nTotal:     48.06",
            &["Add a discount percentage", "Support a coupon code as a tuple"],
        ),
        SeedProject::new(
            "Temperature Station",
            "Store a week of readings and report the statistics.",
            Difficulty::Intermediate,
            "30 min",
            &[
                "Store seven daily temperatures in a fixed-size array of `f64`",
                "Calculate the average, the highest, and the lowest",
                "Print all three with one decimal place",
            ],
            &["arrays", "indexing", "f64", "expressions", "mutability"],
            &[
                "The array must be exactly `[f64; 7]`",
                "Use `mut` for the running sum and max/min trackers",
                "No `if` is required — assume at least one reading",
            ],
            "Mon 72.1  Tue 75.4  Wed 69.8\nThu 81.2  Fri 78.0  Sat 65.5  Sun 70.3\n\nAverage: 74.6 F\nHighest: 81.2 F\nLowest:  65.5 F",
            &["Find the median", "Count how many days were above 75"],
        ),
        SeedProject::new(
            "User Profile Parser",
            "Start with everything as text and shadow your way into real types.",
            Difficulty::Intermediate,
            "45 min",
            &[
                "Start with four `&str` values: an id, an age, an active flag, and a balance",
                "Shadow each one into its proper type using `.parse().unwrap()`",
                "Print each value with its final type named",
                "Do the maths on the parsed values to prove they are real numbers",
            ],
            &["shadowing", "type changes", "parsing", "u32", "f64", "bool"],
            &[
                "Do not declare a second variable for any converted value — shadow the original",
                "Every converted value must be used in a calculation",
            ],
            "raw: \"42\"\nparsed: 42 (u32)\ndoubled: 84 (u32)\n\nraw: \"19.99\"\nparsed: 19.99 (f64)\nwith tax: 21.59 (f64)",
            &["Handle a parse failure without panicking", "Store all raw inputs in an array first"],
        ),
        SeedProject::new(
            "Mini Bank Account",
            "Simulate an account with deposits and withdrawals.",
            Difficulty::Advanced,
            "45 min",
            &[
                "Start with a balance of 1000.00 as `f64`",
                "Make three deposits and two withdrawals",
                "Print the balance after every transaction",
                "Show the starting and ending balance",
            ],
            &["mutability", "f64", "expressions", "type conversions", "arrays"],
            &[
                "The balance must be `f64` — all casts to and from integer cents must be explicit",
                "Use `mut` for the balance",
                "No `if` is required yet — assume every withdrawal is valid",
            ],
            "Opening balance: 1000.00\n  +500.00  -> 1500.00\n  -250.00  -> 1250.00\n  +100.00  -> 1350.00\n  -100.00  -> 1250.00\n  +750.00  -> 2000.00\n\nClosing balance: 2000.00",
            &["Store the transactions in an array and total them at the end", "Add a fee that scales with the balance"],
        ),
        SeedProject::new(
            "Character Creation System",
            "Design your own data model for an RPG character generator.",
            Difficulty::Advanced,
            "1 hour",
            &[
                "Let the player supply a name, age, and three ability scores",
                "Calculate at least three derived stats from those scores",
                "Decide for yourself which types each value needs, and say why in comments",
                "Print a full character sheet",
            ],
            &[
                "type selection",
                "mutability decisions",
                "constants",
                "tuples and arrays",
                "numeric conversions",
                "shadowing",
            ],
            &[
                "No `if` or loops required — focus purely on data modelling",
                "Every type choice must be justified in a comment",
                "At least one value must be immutable, at least one mutable",
            ],
            "=== CHARACTER SHEET ===\nName:     Gus\nAge:      25\nClass:    Warrior\n\nSTR 16   INT 12   AGI 14\nHP  120   MP   40   ATK 22\n\nDerived: HP = STR * 8 + 20",
            &[
                "Add a level-up system using shadowing to change the character's stats",
                "Store the abilities in a fixed array and iterate to total them",
                "Add a tuple for position and another for inventory",
            ],
        ),
    ]
}
