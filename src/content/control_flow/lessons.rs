//! Lessons for Module 03 — Control Flow.

use crate::content::types::{
    broken, callout, code, diagram, labelled, paragraphs, table, text_block, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        thinking(),
        deciding(),
        multiple_conditions(),
        if_as_expression(),
        match_intro(),
        match_values(),
        match_enums(),
        catch_all(),
        match_guards(),
        nested_match(),
        loop_basics(),
        break_continue(),
        loop_values(),
        while_basics(),
        while_let(),
        for_loop(),
        ranges(),
        collections(),
        ownership_looping(),
        nested_labels(),
        blocks_expressions(),
        statements_expressions(),
        option(),
        result(),
        putting_it_together(),
    ]
}

fn thinking() -> SeedLesson {
    SeedLesson::new(
        "Thinking in Control Flow",
        vec![
            labelled(
                "THE IDEA",
                "So far your programs have been a straight line. Top to bottom, every statement, no \
                 detours. Which is fine right up until the moment it isn't, because real software is full of \
                 questions. Should I do this? Should I skip that? Which case am I dealing with? How many \
                 times should I repeat this? What happens next?",
            ),
            paragraphs(&[
                "A program that cannot decide anything is a very expensive way to print one line of text. \
                 Everything interesting comes from being able to branch, repeat, and choose.",
                "Control flow is how a program answers its own questions at runtime. That is the entire idea. \
                 The syntax that follows is just how Rust spells it.",
            ]),
            callout(
                "The questions you will learn to ask",
                "Should I do this? Should I keep going? Should I stop? Which case am I in? What should \
                 happen next? Every tool in this module maps to one of those questions.",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Before any Rust syntax, here is the idea in plain language. If it is raining, take an \
                 umbrella. Otherwise, leave it at home. If it is raining harder, take the good one. If you \
                 are already outside, forget it. That is control flow. You have been doing it in your head \
                 your whole life without calling it that.",
            ),
            diagram(
                "the shape of every decision",
                "        ┌──────────────┐\n        │  a question  │\n        └──────┬───────┘\n               │\n        ┌──────┴──────┐\n     yes │             │ no\n        ▼             ▼\n    do this      do that",
            ),
            text_block(
                "Rust spells that shape with `if`. It spells 'keep asking until something changes' with \
                 `loop` and `while`. It spells 'handle every possible case' with `match`. That is the whole \
                 toolkit. Everything else in this module is detail on those three ideas.",
            ),
            callout(
                "Okay, this is where Rust starts making decisions.",
                "Fair warning: this is the first module where the compiler gets noticeably more opinionated. \
                 That is a good thing. Every rule you hit is a rule that stops a bug you would otherwise \
                 debug at 2am.",
            ),
            callout(
                "Practice — Try this now",
                "Without writing any code, list five decisions a vending machine makes before it dispenses \
                 your drink. Then list three decisions a chess engine makes on a single turn. Notice how \
                 much of each is 'check a condition, then act'.",
            ),
            callout(
                "Mini challenge",
                "Pick something from your own life that involves branching. A traffic light. A login \
                 screen. Choosing what to eat. Write it out as 'if A then B else C' in plain text, then \
                 translate it into Rust. You now know enough syntax to do that. Try it.",
            ),
        ],
    )
}

fn deciding() -> SeedLesson {
    SeedLesson::new(
        "if: Teaching Your Program to Decide",
        vec![
            labelled(
                "THE IDEA",
                "`if` runs a block of code when a condition is true. That is the entire tool. The syntax is \
                 four tokens, and the only real rule is that the condition must be a `bool`.",
            ),
            code(
                "rust",
                "fn main() {\n    let logged_in = true;\n\n    if logged_in {\n        println!(\"Welcome back\");\n    }\n\n    println!(\"End of program\");\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Read it top to bottom. `logged_in` is true, so the block runs. Then the last line runs \
                 regardless, because it is not inside the `if`. The braces matter — they are the boundary \
                 of the branch. Anything between them is conditional, anything outside is unconditional.",
            ),
            diagram(
                "what runs",
                "logged_in = true\n     │\n     ▼\n  ┌───────────┐\n  │ if block  │  ← runs, because true\n  └───────────┘\n     │\n     ▼\n  println!()      ← always runs\n",
            ),
            callout(
                "The condition must be a bool",
                "Rust does not quietly turn numbers into truthiness. There is no `if 1` here. This is not \
                 pedantry — a condition that reads as a number is almost always a bug waiting to happen, \
                 and `0` being false in some languages is a trap that has caught thousands of engineers.",
            ),
            broken("fn main() {\n    if 1 {\n        println!(\"why am I here\");\n    }\n}"),
            code(
                "text",
                "error[E0308]: mismatched types\n --> src/main.rs:2:8\n  |\n2 |     if 1 {\n  |        ^ expected `bool`, found integer `{integer}`",
            ),
            text_block(
                "Notice what the compiler does not say. It does not helpfully offer to treat 1 as true. It \
                 states the expectation, states the reality, and stops. You decide which one to change. If \
                 you genuinely meant 'is this non-zero', you have to say so:",
            ),
            code(
                "rust",
                "fn main() {\n    let count = 1;\n\n    if count != 0 {\n        println!(\"count was not zero\");\n    }\n}",
            ),
            callout(
                "Watch out",
                "The most common `if` bug is not a type error at all. It is forgetting the braces and \
                 getting surprising indentation, or writing a condition where a human reads a question but \
                 the code evaluates a value. Read your conditions as sentences: `if health <= 0 { }` reads \
                 correctly. `if health { }` does not.",
            ),
            callout(
                "Practice — Try this now",
                "Write a `has_ammunition` boolean and an `if` that prints a different message depending on \
                 it. Then try `if \"yes\" { }` and read the error. Then try `if 3 > 2 { }` and notice that \
                 one works perfectly, because `3 > 2` really is a bool.",
            ),
            code(
                "rust",
                "fn main() {\n    let health = 0;\n    let has_armor = false;\n\n    if health == 0 {\n        println!(\"You have fallen!\");\n    }\n\n    if has_armor {\n        println!(\"Your armor absorbed some of it\");\n    }\n}",
            ),
            callout(
                "Mini challenge",
                "Write four `if` statements, no `else` at all, each printing one line: a player is on the \
                 field, out of health, out of ammo, and dead. Pick a different combination of booleans to \
                 show in each. Then change one condition to a number and read the error.",
            ),
        ],
    )
}

fn multiple_conditions() -> SeedLesson {
    SeedLesson::new(
        "Multiple Conditions",
        vec![
            labelled(
                "THE IDEA",
                "Real decisions rarely have two outcomes. They have several. `else if` lets you test a \
                 second condition when the first fails, and `else` catches everything left over.",
            ),
            code(
                "rust",
                "fn main() {\n    let temperature: i32 = 32;\n\n    if temperature > 30 {\n        println!(\"Hot — drink water\");\n    } else if temperature > 20 {\n        println!(\"Warm — pleasant\");\n    } else if temperature > 10 {\n        println!(\"Cool — maybe a jacket\");\n    } else {\n        println!(\"Cold — stay inside\");\n    }\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Rust tests these in order and stops at the first match. At 32 degrees, the first branch \
                 wins and the other three never run. That ordering is not a detail — it is the whole design. \
                 It means you should write your conditions from most specific to most general.",
            ),
            callout(
                "The ordering trap",
                "If you wrote `if temperature > 10` first, then everything above 10 would print 'maybe a \
                 jacket' — including 40 degrees. The compiler will not warn you. The tests will pass. Your \
                 users will complain. Order your branches most-specific-first, always.",
            ),
            code(
                "rust",
                "fn main() {\n    let score = 85;\n    let passing = 70;\n\n    if score == 100 {\n        println!(\"Perfect!\");\n    } else if score >= passing {\n        println!(\"Passed with {}\", score);\n    } else if score >= 50 {\n        println!(\"Borderline fail\");\n    } else {\n        println!(\"Failed badly\");\n    }\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "You can combine conditions with `&&` (and) and `||` (or). They short-circuit, which means \
                 if the left side settles it, the right side is never evaluated. That is occasionally a \
                 performance trick and always a safety tool when the right side would panic on some inputs.",
            ),
            code(
                "rust",
                "fn main() {\n    let health: i32 = 0;\n    let has_shield: bool = true;\n    let score = 45;\n    let streak = 0;\n\n    // && means both must be true\n    if health > 0 && has_shield {\n        println!(\"Protected\");\n    }\n\n    // || means at least one must be true\n    if score >= 100 || streak >= 10 {\n        println!(\"Bonus awarded\");\n    }\n}",
            ),
            callout(
                "You could write this as seventeen nested if statements",
                "Rust will not stop you. Your future self, however, may file a complaint. Every extra level \
                 of nesting is a level where you can forget a brace. Flat `else if` chains stay readable \
                 at twenty branches. Nesting does not.",
            ),
            callout(
                "Practice — Try this now",
                "Write a four-branch chain for a game difficulty setting based on a score from 0 to 100. \
                 Then reorder two branches so one becomes unreachable, and run it to prove it. That is what \
                 a dead branch feels like.",
            ),
            callout(
                "Mini challenge",
                "Write a login check with this exact shape: if the username is empty, say so. Else if the \
                 username is 'admin', demand a password. Else if the username is 'guest', log them in \
                 directly. Else, say unknown user. Three conditions and an `else`. Then add a fourth \
                 condition at the top for locked accounts.",
            ),
        ],
    )
}

fn if_as_expression() -> SeedLesson {
    SeedLesson::new(
        "if as an Expression",
        vec![
            labelled(
                "THE IDEA",
                "Here is the part that trips up nearly everyone, and the part that will pay you back for \
                 the next five years. In Rust, `if` is not a statement. It is an expression, and expressions \
                 produce values.",
            ),
            code(
                "rust",
                "fn main() {\n    let logged_in = true;\n\n    let message = if logged_in {\n        \"Welcome back\"\n    } else {\n        \"Please log in\"\n    };\n\n    println!(\"{}\", message);\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Compare that with the `if` you already know, where the branches printed things and \
                 produced nothing. Here, each branch evaluates to a value, and the whole `if` evaluates to \
                 whichever value the matching branch produced. The `if` has become a thing that can be \
                 assigned, passed, or returned.",
            ),
            table(
                &["", "Statement style", "Expression style"],
                &[
                    &["Shape", "if c { do(); }", "let x = if c { a } else { b };"],
                    &["Produces", "nothing (unit)", "a value"],
                    &["Written to", "the screen", "a variable"],
                    &["Needs else?", "no", "yes, for a value"],
                ],
            ),
            callout(
                "The rule that makes this work",
                "If every branch is an expression and the `if` is being used for its value, you must give \
                 it an `else`. Otherwise the compiler has no way to know what the `if` produced when the \
                 condition was false, and it will say so.",
            ),
            broken("fn main() {\n    let admin = false;\n\n    let access = if admin {\n        \"full\"\n    };\n\n    println!(\"{}\", access);\n}"),
            code(
                "text",
                "error[E0004]: non-exhaustive patterns: type `&str` is not covered\n --> src/main.rs:4:22\n  |\n4 |     let access = if admin {\n  |                   ^^^^ this if has no else branch",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "That error says exactly what is wrong: an `if` used for a value has no else branch, so \
                 it might not produce anything. Add the `else` and the problem disappears.",
            ),
            callout(
                "Both branches must agree on a type",
                "This is where it gets strict, and where it starts saving you. If one branch produces a \
                 number and another produces a string, the `if` has no single type to be.",
            ),
            broken("fn main() {\n    let admin = true;\n\n    let access = if admin {\n        10\n    } else {\n        \"full access\"\n    };\n}"),
            code(
                "text",
                "error[E0308]: mismatched types\n --> src/main.rs:5:9\n  |\n5 |     let access = if admin {\n  |         ^^^^^^^^^^^^^ expected `&str`, found integer `{integer}`",
            ),
            text_block(
                "The compiler inferred the type from the `else` branch and then complained that the `if` \
                 branch disagreed. Fix it by making both branches the same kind of thing — usually by \
                 converting the number to a string, or the string to a number.",
            ),
            code(
                "rust",
                "fn main() {\n    let admin = true;\n\n    // both branches are &str\n    let access = if admin { \"full\" } else { \"none\" };\n\n    // both branches are i32\n    let level = if admin { 10 } else { 0 };\n\n    println!(\"{} {}\", access, level);\n}",
            ),
            callout(
                "Oh, if gives me a value",
                "That is the realisation. And once you have it, `match` becomes much easier to understand, \
                 because `match` is the same idea scaled up to more cases.",
            ),
            callout(
                "Practice — Try this now",
                "Write an `if` expression that produces a discount percentage: 20 if the player is a member, \
                 otherwise 0. Print it. Then try producing a number in one branch and a string in the \
                 other, and read the error carefully. The `expected` type is decided by your `else` branch — \
                 that is worth internalising.",
            ),
            callout(
                "Mini challenge",
                "Write a function `grade(score: i32) -> &str` that returns a letter grade using an `if` \
                 expression chain. Then call it with 95, 83, 71, 59, and 0 and print the results. Making a \
                 function return a value computed by an `if` is the pattern you will use constantly.",
            ),
        ],
    )
}

fn match_intro() -> SeedLesson {
    SeedLesson::new(
        "match: Rust's Pattern-Matching Powerhouse",
        vec![
            labelled(
                "THE IDEA",
                "Remember the `else if` chain from two lessons ago? It works, but it gets unwieldy fast, and \
                 nothing stops you forgetting a case. `match` is the tool Rust gives you when there are \
                 several distinct possibilities and you want to be sure you handled all of them.",
            ),
            code(
                "rust",
                "fn main() {\n    let number = 2;\n\n    match number {\n        1 => println!(\"one\"),\n        2 => println!(\"two\"),\n        3 => println!(\"three\"),\n        _ => println!(\"something else\"),\n    }\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The vocabulary, piece by piece. `number` is the expression being matched. Each line inside \
                 is an *arm*. The thing before `=>` is a *pattern* — a description of what you are looking \
                 for. The thing after is the code to run if it matches. The comma at the end separates \
                 arms. `_` is the catch-all: it matches anything and binds nothing.",
            ),
            diagram(
                "anatomy of a match",
                "  match number {\n      │\n      ├── arm 1 ── pattern: 1\n      │              code: println!(\"one\")\n      │\n      ├── arm 2 ── pattern: 2\n      │              code: println!(\"two\")\n      │\n      └── arm _ ── pattern: anything\n                     code: println!(\"other\")\n  }",
            ),
            callout(
                "Why Rust demands exhaustiveness",
                "If you ask Rust to handle every possible case, Rust wants you to actually handle every \
                 possible case. Omit the catch-all and the compiler stops you, because somewhere out there \
                 is a value you did not think about. This is not bureaucracy. It is the compiler refusing \
                 to let a bug ship.",
            ),
            code(
                "rust",
                "fn main() {\n    let number = 7;\n\n    match number {\n        1 => println!(\"one\"),\n        2 => println!(\"two\"),\n        // no _ and no 7 — this does not compile\n    }\n}",
            ),
            code(
                "text",
                "error[E0004]: non-exhaustive patterns: `i32` not covered\n --> src/main.rs:4:19\n  |\n4 |     match number {\n  |           ^^^^^^ not covered\n  |           _ => println!(\"other\"),\n  |           ^^^^^^^^^^^^^^^^^^^ this pattern is unhandled\n  |\nnote: `_` must be last",
            ),
            text_block(
                "That error is genuinely helpful. It tells you the type, tells you which patterns exist, \
                 and even tells you that `_` has to be last. Compare that to a chain of `else if` \
                 statements, where a missing case just silently does nothing.",
            ),
            table(
                &["", "else-if chain", "match"],
                &[
                    &["Missing a case", "silently falls through", "compile error"],
                    &["Scales to many cases", "poorly", "well"],
                    &["Can produce a value", "no", "yes"],
                    &["Can destructure", "no", "yes"],
                ],
            ),
            callout(
                "Practice — Try this now",
                "Write a `match` on a day number 1 to 7 that prints the day name. Add `_` printing \
                 'Unknown day'. Then remove the `_` arm and read the exhaustiveness error. Then try a `match` \
                 with no arms at all, which is legal, and think about what it means.",
            ),
            callout(
                "Mini challenge",
                "Take a five-branch `else if` chain from the previous lesson and rewrite it as a `match`. \
                 Notice which one is easier to scan. Notice also that the `match` version now knows it has \
                 handled everything.",
            ),
        ],
    )
}

fn match_values() -> SeedLesson {
    SeedLesson::new(
        "Matching Values",
        vec![
            labelled(
                "THE IDEA",
                "Patterns are not limited to single literals. Once you can describe a value, you can describe \
                 a *set* of values — a range, or a list of alternatives — with one arm.",
            ),
            code(
                "rust",
                "fn main() {\n    let dice = 4;\n\n    match dice {\n        1 | 2 => println!(\"low roll\"),\n        3 | 4 => println!(\"mid roll\"),\n        5 | 6 => println!(\"high roll\"),\n        _ => println!(\"not a die\"),\n    }\n}",
            ),
            text_block(
                "The `|` is an or. That arm matches 1, 2, 3, or 4. It is not four arms wearing a trench \
                 coat — it is one arm with four accepted patterns, and the `=>` applies to all of them.",
            ),
            code(
                "rust",
                "fn main() {\n    let score = 7;\n\n    match score {\n        0 => println!(\"zero — nothing done\"),\n        1..=5 => println!(\"small progress\"),\n        6..=10 => println!(\"solid progress\"),\n        11..=20 => println!(\"excellent\"),\n        _ => println!(\"off the charts\"),\n    }\n}",
            ),
            callout(
                "1..=5 includes 5, 1..5 does not",
                "The `=` in `..=` is load-bearing. `1..5` matches 1, 2, 3, 4. `1..=5` matches 1, 2, 3, 4, 5. \
                 Ranges in patterns are almost always inclusive on both ends, because a roll of exactly 5 \
                 should not fall through to your catch-all.",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Ranges are only valid in patterns, not in ordinary expressions. You cannot write \
                 `let x = 1..=5;` and use it with `<`. The `RangeInclusive` type exists and is real, but \
                 the pattern syntax is a separate, nicer thing. Worth knowing, easy to forget.",
            ),
            code(
                "rust",
                "fn main() {\n    let c = 'q';\n\n    match c {\n        'a'..='z' => println!(\"lowercase letter\"),\n        'A'..='Z' => println!(\"uppercase letter\"),\n        '0'..='9' => println!(\"digit\"),\n        _ => println!(\"something else\"),\n    }\n}",
            ),
            callout(
                "Combining them",
                "You can mix alternatives and ranges in one arm: `'a' | 'e' | 'i' | 'o' | 'u' =>`. Order \
                 the alternatives from most specific to least, just like `else if`. An arm that shadows an \
                 earlier one is dead code, and Rust will not warn you.",
            ),
            callout(
                "Practice — Try this now",
                "Write a `match` on an HTTP-like status code: 200 and 201 are success, 400 and 404 are \
                 client errors, 500 is a server error, anything else is unknown. Use ranges where they \
                 make sense. Then try `200 | 201..=299` and confirm it still works.",
            ),
            callout(
                "Mini challenge",
                "Write a `match` on a number from 0 to 100 that classifies it: 0 is nothing, 1..=59 is \
                 failing, 60..=69 is passing, 70..=89 is good, 90..=100 is excellent, and anything else is \
                 invalid. Print a different message for each. Then test with 0, 45, 65, 80, 95, and 200.",
            ),
        ],
    )
}

fn match_enums() -> SeedLesson {
    SeedLesson::new(
        "Matching Enums",
        vec![
            labelled(
                "THE IDEA",
                "This is where `match` stops being useful and starts being the reason people choose Rust. \
                 When you have a value that can be one of a fixed set of things, `match` handles every one \
                 of them, in one place, with the compiler checking you got them all.",
            ),
            code(
                "rust",
                "enum Status {\n    Online,\n    Offline,\n    Away,\n}\n\nfn main() {\n    let status = Status::Away;\n\n    match status {\n        Status::Online => println!(\"Active now\"),\n        Status::Offline => println!(\"Not reachable\"),\n        Status::Away => println!(\"Idle, will reply later\"),\n    }\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "No catch-all arm this time, and it compiles. Why? Because `Status` can only be one of \
                 three things, and you handled all three. The compiler knows the full set of possibilities \
                 because the type tells it. This is the exhaustiveness check doing real work for you \
                 instead of nagging you about a catch-all you do not need.",
            ),
            callout(
                "A sneak preview",
                "We give enums their own module later. For now, one thing to know: `enum` defines a type \
                 that is exactly one of the listed variants, and each variant can carry data. The next \
                 example shows why that matters.",
            ),
            code(
                "rust",
                "enum Message {\n    Text(String),\n    Number(i32),\n    Quit,\n}\n\nfn main() {\n    let message = Message::Text(String::from(\"hello\"));\n\n    match message {\n        Message::Text(text) => println!(\"Got text: {}\", text),\n        Message::Number(number) => println!(\"Got number: {}\", number),\n        Message::Quit => println!(\"Shutting down\"),\n    }\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Look at `Message::Text(text)`. That is not a comparison. It is a *pattern* that says: \
                 'if this is a Text variant, and it carries a String, pull that String out and call it \
                 `text`.' The parentheses are the binding. The compiler guarantees the binding is \
                 correct because it knows exactly what `Text` carries.",
            ),
            diagram(
                "how the pattern destructures",
                "  match message {\n      Message::Text(text) =>\n      │      │  │\n      │      │  └── the String, pulled out of the variant\n      │      └───── only matches the Text variant\n      │\n      Message::Quit =>\n             └── a variant with no data, so no binding\n",
            ),
            callout(
                "This is destructuring, and it is not a trick",
                "In most languages you would call a getter. In Rust, matching is how you get at the data, \
                 and you have to say which shape you are expecting. The upside: you can never forget to \
                 check the type, and you can never read a field that does not exist on this variant.",
            ),
            code(
                "rust",
                "fn main() {\n    let messages = [\n        Message::Quit,\n        Message::Text(String::from(\"first\")),\n        Message::Number(42),\n    ];\n\n    for message in messages {\n        match message {\n            Message::Text(text) => println!(\"text: {}\", text),\n            Message::Number(n) => println!(\"number: {}\", n),\n            Message::Quit => {\n                println!(\"quit\");\n                break;\n            }\n        }\n    }\n}",
            ),
            callout(
                "Practice — Try this now",
                "Define a `Light` enum with `Red`, `Yellow`, and `Green`. Write a `match` that prints a \
                 message for each. Now add a `Blinking(u8)` variant carrying a speed, handle it, and see \
                 the exhaustiveness error appear until you do.",
            ),
            callout(
                "Mini challenge",
                "Define an `Option`-like enum yourself: `enum MaybeNumber { Nothing, Number(i32) }`. Write a \
                 `match` that adds 1 to the number or prints 'nothing'. This is, in miniature, the exact \
                 pattern you will use for `Option` in a later module. You are already learning it.",
            ),
        ],
    )
}

fn catch_all() -> SeedLesson {
    SeedLesson::new(
        "Patterns, _, and Catch-All Cases",
        vec![
            labelled(
                "THE IDEA",
                "You have now met two kinds of pattern: literal patterns that match a specific value, and \
                 variant patterns that match a shape and pull data out. There is a third kind, and it is the \
                 most useful one.",
            ),
            text_block(
                "A pattern is simply a description of what you are looking for. `1` is a description of one \
                 specific value. `Message::Text(text)` is a description of a shape. `_` is a description of \
                 *anything at all*.",
            ),
            code(
                "rust",
                "fn main() {\n    let value = 7;\n\n    match value {\n        0 => println!(\"zero\"),\n        1 => println!(\"one\"),\n        _ => println!(\"many\"),\n    }\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`_` is the wildcard. It matches every value that the arms above it did not, and it binds \
                 nothing, so you cannot use the matched value inside the arm. Sometimes you want a named \
                 binding instead, and you can have one:",
            ),
            code(
                "rust",
                "fn main() {\n    let value = 7;\n\n    match value {\n        0 => println!(\"zero\"),\n        other => println!(\"something else: {}\", other),\n    }\n}",
            ),
            callout(
                "The difference that matters",
                "`_` throws the value away. `other` keeps it. If you write `n => ...` you can use `n` inside \
                 the arm. If you write `_ => ...` you cannot, because there is no name. Choose the named \
                 form when you want the value, the wildcard when you genuinely do not care.",
            ),
            callout(
                "A wildcard that discards a field",
                "In a destructuring pattern, you can ignore the data a variant carries. This is a real \
                 situation: a chat app with a `Message::Image(String)` variant where you only care that it \
                 was an image, not what the image is.",
            ),
            code(
                "rust",
                "enum Event {\n    KeyPress(char),\n    Paste(String),\n    Close,\n}\n\nfn main() {\n    let events = [\n        Event::KeyPress('q'),\n        Event::Paste(String::from(\"hello\")),\n        Event::Close,\n    ];\n\n    for event in events {\n        match event {\n            Event::KeyPress(c) => println!(\"key: {}\", c),\n            Event::Paste(_) => println!(\"pasted something\"),\n            Event::Close => println!(\"closed\"),\n        }\n    }\n}",
            ),
            callout(
                "This is also a place to stop caring",
                "The best `_` arms are the ones where you deliberately do not want to handle a case yet. \
                 That is a legitimate choice, and writing `Event::Paste(_) => {}` with a comment is much \
                 better than pretending you handled it.",
            ),
            callout(
                "Practice — Try this now",
                "Match on an `Option`-like value: `Some(number)` should print the number, `None` should \
                 print 'nothing'. Then try using the bound name from a `_` arm and read the error. The \
                 message explains the problem better than I can.",
            ),
            callout(
                "Mini challenge",
                "Match on a two-field tuple. Use one pattern that matches exactly `(0, 0)`, one that matches \
                 `(_, 0)` and uses the first value, and a catch-all. That is three arms, three different \
                 levels of specificity, one `match`.",
            ),
        ],
    )
}

fn match_guards() -> SeedLesson {
    SeedLesson::new(
        "Match Guards",
        vec![
            labelled(
                "THE IDEA",
                "Patterns match values exactly. Guards let you add a condition on top of a pattern. When \
                 the pattern matches, the guard is checked, and the arm only runs if the guard is also true.",
            ),
            code(
                "rust",
                "fn main() {\n    let number = 15;\n\n    match number {\n        n if n > 10 => println!(\"big: {}\", n),\n        n if n < 0 => println!(\"negative: {}\", n),\n        n => println!(\"small: {}\", n),\n    }\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Read `n if n > 10` as 'if the value, bound to the name n, is greater than 10'. The \
                 binding happens first, then the guard runs, then the arm's body runs. If the guard is \
                 false, Rust moves on to the next arm — and eventually to whatever catch-all you have.",
            ),
            code(
                "rust",
                "fn main() {\n    let age = 17;\n\n    match age {\n        n if n < 13 => println!(\"child\"),\n        n if n < 18 => println!(\"teenager\"),\n        n if n < 65 => println!(\"adult\"),\n        n => println!(\"senior ({} years)\", n),\n    }\n}",
            ),
            callout(
                "You could do this with else-if",
                "Yes, you could. Guards are not more powerful than `else if` — they are a different shape \
                 for the same idea. The advantage is that guard arms live in one flat list, so when you add \
                 a new age bracket you add one line instead of restructuring a chain.",
            ),
            callout(
                "Guards are not patterns",
                "This trips people up. A range pattern like `13..=17` is checked against the value \
                 directly. A guard like `n if n < 18` runs arbitrary code. If the guard can fail for a \
                 reason other than the value — an I/O error, say — Rust cannot reason about it, and a \
                 match that is exhaustive on patterns may still need a catch-all.",
            ),
            code(
                "rust",
                "fn main() {\n    let score = 85;\n\n    let grade = match score {\n        n if n >= 90 => 'A',\n        n if n >= 80 => 'B',\n        n if n >= 70 => 'C',\n        n if n >= 60 => 'D',\n        _ => 'F',\n    };\n\n    println!(\"grade: {}\", grade);\n}",
            ),
            callout(
                "Note what that just did",
                "A `match` produced a `char` and assigned it to a variable. No `println!` in the arms, no \
                 returning early, no mutable flag. This is the same expression trick from the `if` lesson, \
                 scaled to more cases. You will use this shape constantly.",
            ),
            callout(
                "Practice — Try this now",
                "Write a `match` with guards that classifies a number as `Negative`, `Zero`, `Small` (1-9), \
                 `Medium` (10-99), or `Large`. Then rewrite it using plain patterns only, and compare which \
                 reads better.",
            ),
            callout(
                "Mini challenge",
                "Write a `match` that takes a username and returns a message as a `&str`, using guards for \
                 'too short', 'all digits', and 'contains an @'. You will need string methods from module \
                 11 for the last two, so approximate them with length and a manual check for now.",
            ),
        ],
    )
}

fn nested_match() -> SeedLesson {
    SeedLesson::new(
        "Nested match and Combining Patterns",
        vec![
            labelled(
                "THE IDEA",
                "Sometimes the shape of the data has a shape. A move in a chess game has a piece and a \
                 square. Handling that means a `match` inside a `match`.",
            ),
            code(
                "rust",
                "enum Color {\n    Red,\n    Green,\n    Blue,\n}\n\nfn main() {\n    let pair = (Color::Red, 5);\n\n    match pair {\n        (Color::Red, 0) => println!(\"dark red\"),\n        (Color::Red, n) => println!(\"red, brightness {}\", n),\n        (_, 0) => println!(\"pure color, no brightness\"),\n        _ => println!(\"something else\"),\n    }\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The outer `match` destructures a tuple. Each arm's pattern is itself a tuple pattern, \
                 which means each position can have its own pattern. `(Color::Red, 0)` matches only red at \
                 exactly 0. `(Color::Red, n)` matches any red. `(_, 0)` matches any colour at 0 — the \
                 underscore is a wildcard inside a tuple pattern.",
            ),
            code(
                "rust",
                "enum Action {\n    Move { from: u8, to: u8 },\n    Capture(u8),\n    Pass,\n}\n\nfn describe(action: Action) {\n    match action {\n        Action::Move { from, to } => println!(\"move {} to {}\", from, to),\n        Action::Capture(piece) => println!(\"capture {}\", piece),\n        Action::Pass => println!(\"pass\"),\n    }\n}\n\nfn main() {\n    describe(Action::Move { from: 1, to: 2 });\n    describe(Action::Capture(3));\n    describe(Action::Pass);\n}",
            ),
            callout(
                "Named fields in patterns",
                "`Move { from, to }` matches a struct-like variant and pulls out two named fields. If the \
                 variant has fields you do not care about, use `..` — `Move { from, .. }` matches and \
                 ignores the rest.",
            ),
            code(
                "rust",
                "enum Action {\n    Move { from: u8, to: u8 },\n    Capture(u8),\n    Pass,\n}\n\nlet action = Action::Move { from: 1, to: 2 };\n\nmatch action {\n    Action::Move { from, .. } => println!(\"moving from {}\", from),\n    other => println!(\"not a move\"),\n}",
            ),
            callout(
                "When nesting gets deep, flatten instead",
                "Three levels of nested `match` is a sign you probably want a guard, a tuple pattern, or a \
                 helper function. Deep nesting is legal and readable up to a point, then it stops being \
                 either. When in doubt, pull one level out into its own function with a name.",
            ),
            callout(
                "Practice — Try this now",
                "Define a `Config` struct-like variant carrying `host: String` and `port: u16`. Match on it, \
                 printing the host, and handling three cases: a normal config, a config with port 0, and a \
                 config with an empty host. Three arms, one `match`.",
            ),
            callout(
                "Mini challenge",
                "Write a `match` on a tuple of two `Option`-like values where each has a data-carrying \
                 variant and an empty variant. You need four arms to be exhaustive. Try to write only three \
                 and read the error — it will name the case you forgot.",
            ),
        ],
    )
}

fn loop_basics() -> SeedLesson {
    SeedLesson::new(
        "loop: Repeat Until You Say Stop",
        vec![
            labelled(
                "THE IDEA",
                "Three loop tools, three different shapes. `loop` repeats forever until you break out. \
                 `while` repeats as long as a condition holds. `for` walks over a sequence. Start with the \
                 first one, because it is the one that teaches the others.",
            ),
            code(
                "rust",
                "fn main() {\n    loop {\n        println!(\"again!\");\n    }\n}",
            ),
            text_block(
                "That program never stops. It is not a bug — `loop` is defined as infinite, and the only \
                 way out is a `break`. Try running it and you will need Ctrl-C. Welcome to the most honest \
                 loop in any language.",
            ),
            code(
                "rust",
                "fn main() {\n    let mut attempts = 0;\n\n    loop {\n        attempts += 1;\n        println!(\"attempt {}\", attempts);\n\n        if attempts == 3 {\n            break;\n        }\n    }\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "This is the pattern for 'keep going until something happens'. The stopping condition is \
                 discovered inside the loop body, usually by checking state that changes. That is what \
                 makes `loop` different from `while`: with `while`, the exit condition is known before you \
                 start. With `loop`, it emerges.",
            ),
            callout(
                "Where loop genuinely beats while",
                "Waiting for a valid input where you only learn it is invalid after trying. A retry loop \
                 where the number of attempts is not known up front. A game loop where you break out when \
                 the player quits. In each case the exit is a *decision made inside*, not a condition \
                 checked before.",
            ),
            code(
                "rust",
                "fn main() {\n    let mut input = String::new();\n    let mut tries = 0;\n\n    // Pretend this is user input validation\n    loop {\n        tries += 1;\n        input = String::from(\"abc\");\n\n        if let Ok(number) = input.parse::<i32>() {\n            println!(\"got {}\", number);\n            break;\n        }\n\n        println!(\"not a number, try {}\", tries);\n    }\n}",
            ),
            callout(
                "Watch out — the infinite loop you did not mean",
                "A `loop` with a `break` inside an `if` that never becomes true runs forever. The compiler \
                 cannot help you here, because it cannot know when your condition becomes false. The only \
                 protection is testing the exit path. If you find yourself editing a `break` and nothing \
                 changes, the condition is probably not what you think.",
            ),
            callout(
                "Practice — Try this now",
                "Write a `loop` that increments a counter until it reaches 5, printing each value. Then add \
                 a `break` that exits at 3 instead and confirm the loop stops there. Both are one keyword \
                 different — that is the whole tool.",
            ),
            code(
                "rust",
                "fn main() {\n    let mut n = 0;\n\n    loop {\n        n += 1;\n        if n > 100 {\n            break;\n        }\n    }\n\n    println!(\"stopped at {}\", n);\n}",
            ),
            callout(
                "Mini challenge",
                "Write a loop that counts down from 5, printing each number, then prints 'liftoff'. Do it \
                 with `loop` and `break`, and do not use `while` yet. Then rewrite it with `while` and \
                 compare which one you found easier to write.",
            ),
        ],
    )
}

fn break_continue() -> SeedLesson {
    SeedLesson::new(
        "break and continue",
        vec![
            labelled(
                "THE IDEA",
                "Two keywords, two different jobs. `break` leaves the loop entirely. `continue` skips the \
                 rest of this iteration and starts the next one. Neither one is complicated. Both are \n                 genuinely useful.",
            ),
            code(
                "rust",
                "fn main() {\n    let numbers = [10, -5, 3, 0, 8, -1, 12];\n    let mut negatives = 0;\n\n    for n in numbers {\n        if n < 0 {\n            negatives += 1;\n            continue;\n        }\n\n        println!(\"processing {}\", n);\n    }\n\n    println!(\"skipped {} negatives\", negatives);\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "That pattern — check a condition, `continue` if it fails, otherwise do the work — is one \
                 of the most common shapes in all of programming. Notice it lets you keep the happy path \
                 at the top level of the loop body, unindented, instead of burying it under an `else`.",
            ),
            callout(
                "continue vs break in one line",
                "`continue` means 'skip the rest of this round'. `break` means 'stop the whole thing'. If \
                 you reach for `break` when you meant `continue`, you will exit early and probably wonder \
                 why the loop stopped.",
            ),
            code(
                "rust",
                "fn main() {\n    // continue in a while loop — the risk is an infinite loop\n    let mut n = 0;\n\n    while n < 5 {\n        n += 1;\n\n        if n == 3 {\n            continue;\n        }\n\n        println!(\"n is {}\", n);\n    }\n}",
            ),
            callout(
                "The infinite loop trap",
                "With `loop`, `continue` is safe. With `while`, `continue` can skip the line that updates \
                 the condition, and the loop never ends. In that loop above, `n += 1` comes first, so it is \
                 fine. Move it below the `continue` and watch the program hang. This has caught out every \
                 Rust developer at least once.",
            ),
            code(
                "rust",
                "fn main() {\n    let queue = [1, 0, 2, 0, 3];\n    let mut processed = 0;\n\n    for item in queue {\n        if item == 0 {\n            continue;   // skip empty slots, keep going\n        }\n        processed += 1;\n        println!(\"processed item {}\", item);\n    }\n\n    println!(\"{} items processed\", processed);\n}",
            ),
            callout(
                "You do not need break and continue in the same loop",
                "If you find yourself using both, the loop is probably doing two jobs. Split it. A loop \
                 that filters with `continue` and one that accumulates with a normal body is easier to \
                 change later.",
            ),
            callout(
                "Practice — Try this now",
                "Write a loop over `[3, 0, 7, 0, 0, 9]` that skips zeros with `continue` and sums the rest. \
                 Then try a version with `break` on the first zero and compare the results. Both are valid; \
                 only one is what you wanted.",
            ),
            callout(
                "Mini challenge",
                "Write a loop that prints the first 10 prime numbers, using `continue` to skip composites. \
                 You will need a helper function to test primality — write it as a separate `fn`. That is \
                 a good habit: small functions with names.",
            ),
        ],
    )
}

fn loop_values() -> SeedLesson {
    SeedLesson::new(
        "Returning Values From loop",
        vec![
            labelled(
                "THE IDEA",
                "Everything so far has been a statement. Now we get the payoff: `loop` can produce a value, \
                 and so can every other block-shaped construct in Rust. Remember `loop { break 42; }`?",
            ),
            code(
                "rust",
                "fn main() {\n    let mut counter = 0;\n\n    let result = loop {\n        counter += 1;\n\n        if counter == 10 {\n            break counter;   // break WITH a value — the loop produces it\n        }\n    };\n\n    println!(\"counter reached {}\", result);\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`break counter;` does not just exit. It exits *with that value attached*, and the whole \
                 `loop` expression evaluates to it. That is why `let result = loop { ... }` works. The loop \
                 has to be able to escape on every path, or the compiler will complain that the value is \
                 never produced — a check that saves you from a very confusing runtime bug.",
            ),
            code(
                "rust",
                "fn main() {\n    let mut attempts = 0;\n\n    let mut input = String::from(\"nope\");\n\n    loop {\n        attempts += 1;\n\n        match input.parse::<i32>() {\n            Ok(n) => break n * 2,\n            Err(_) => {\n                println!(\"try {} failed\", attempts);\n                if attempts == 3 {\n                    break 0;   // every exit must produce a value\n                }\n                input = String::from(\"42\");\n            }\n        }\n    }\n}",
            ),
            callout(
                "The same trick works everywhere",
                "A `loop` that produces a value. An `if` that produces a value. A `match` that produces a \
                 value. A plain block that produces a value. They are all the same mechanism, and they are \
                 all why Rust code looks different from what you learned in other languages. You are not \
                 writing commands. You are evaluating things.",
            ),
            code(
                "rust",
                "fn main() {\n    // A loop that finds the first even number and returns it\n    let numbers = [1, 3, 8, 4, 10];\n\n    let first_even = loop {\n        for n in numbers {\n            if n % 2 == 0 {\n                break n;\n            }\n        }\n        break -1;  // unreachable here, but the compiler needs a path\n    };\n\n    println!(\"first even: {}\", first_even);\n}",
            ),
            callout(
                "You will use this in every function that has a natural early return",
                "The idiomatic Rust shape for 'search a collection and return what you found' is exactly \
                 this: loop, break with the value. Function module next, but you now have the tool.",
            ),
            callout(
                "Practice — Try this now",
                "Write a loop that repeatedly doubles a number until it exceeds 100, then breaks with the \
                 final value. Print it. Then do the same with a `while` loop and print the value from \
                 outside the loop instead. Both work; the first is more direct.",
            ),
            code(
                "rust",
                "fn main() {\n    let mut n = 1;\n\n    let power_of_two = loop {\n        if n > 512 {\n            break n / 2;\n        }\n        n *= 2;\n    };\n\n    println!(\"largest power of two under 1000: {}\", power_of_two);\n}",
            ),
            callout(
                "Mini challenge",
                "Write a loop that generates a password by picking from a list of characters, and breaks \
                 with the password once it is long enough. Use `continue` to skip characters that fail your \
                 rules. The loop produces the final password as its value.",
            ),
        ],
    )
}

fn while_basics() -> SeedLesson {
    SeedLesson::new(
        "while: Repeat While Something Is True",
        vec![
            labelled(
                "THE IDEA",
                "When you know up front what stops the repetition, `while` says so directly. No `break` \
                 required, no manual flag. This is the loop you will reach for most often when the \
                 condition is known before you start.",
            ),
            code(
                "rust",
                "fn main() {\n    let mut countdown = 5;\n\n    while countdown > 0 {\n        println!(\"{}...\", countdown);\n        countdown -= 1;\n    }\n\n    println!(\"liftoff\");\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Read the condition first, then the body. That ordering is different from `loop`, where the \
                 condition lives inside. Here the loop can never run zero times if the condition starts \
                 false, and it can never run forever as long as you update the thing the condition reads.",
            ),
            diagram(
                "loop vs while",
                "  loop {              while condition {\n      body                  body\n      if !done {            update condition   ← the easy-to-forget part\n        break;\n      }\n  }                   }",
            ),
            callout(
                "They are the same thing",
                "`while c { body }` is exactly `loop { if !c { break; } body }`. Rust has a `while` \
                 because writing the second version when you mean the first is noise. Know that they are \
                 equivalent, and then use whichever states your intent more clearly.",
            ),
            code(
                "rust",
                "fn main() {\n    // The condition depends on something you update inside the loop\n    let mut health = 100;\n\n    while health > 0 {\n        health -= 25;\n        println!(\"health now {}\", health.max(0));\n    }\n\n    println!(\"you died\");\n}",
            ),
            callout(
                "The forgetting-the-update bug",
                "A `while` loop whose body never changes the condition runs forever, and the compiler \
                 cannot help. If your loop hangs, look for the line that was supposed to change the \
                 variable in the condition. It is almost always that.",
            ),
            code(
                "rust",
                "fn main() {\n    // Typical: bounded retry\n    let mut attempts = 0;\n    let max_attempts = 3;\n\n    while attempts < max_attempts {\n        attempts += 1;\n        println!(\"attempt {} of {}\", attempts, max_attempts);\n    }\n\n    println!(\"gave up after {} attempts\", attempts);\n}",
            ),
            callout(
                "Which one should I use?",
                "If the exit condition is a simple value you can check before each iteration, `while`. If \
                 the exit depends on something you only learn partway through — an input that failed to \
                 parse, a state that has to be reached — `loop` with a `break`. The compiler does not care. \
                 The next reader does.",
            ),
            callout(
                "Practice — Try this now",
                "Write three `while` loops: a countdown, a multiplication table up to 5, and a loop that \
                 halves a number until it reaches 1. Then deliberately remove the update line from one \
                 and confirm it hangs. Then add it back. Never make that mistake twice.",
            ),
            callout(
                "Mini challenge",
                "Write a `while` loop that simulates a day: start with energy 100, drop by 15 each hour, \
                 and stop when energy would go below 20. Print the hour each time. Then rewrite it with \
                 `loop` and `break` and decide which one you would rather read in a month.",
            ),
        ],
    )
}

fn while_let() -> SeedLesson {
    SeedLesson::new(
        "while let",
        vec![
            labelled(
                "THE IDEA",
                "Sometimes the loop condition is not a yes-or-no question but a pattern match. \
                 `while let` runs the loop as long as the pattern keeps matching. It is `while` and a \
                 `match` fused into one construct.",
            ),
            code(
                "rust",
                "fn main() {\n    let mut stack = vec![1, 2, 3];\n\n    while let Some(top) = stack.pop() {\n        println!(\"popped {}\", top);\n    }\n\n    println!(\"stack is empty\");\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`stack.pop()` gives you an `Option` — `Some(value)` if there was something, `None` if the \
                 stack is empty. The pattern `Some(top)` binds the value. While that pattern keeps \
                 matching, the loop runs. The moment it returns `None`, the pattern stops matching and the \
                 loop ends. No manual `is_empty()` check, no index bookkeeping.",
            ),
            diagram(
                "what while let is doing",
                "  loop {\n      match stack.pop() {\n          Some(top) => { /* loop body */ }\n          None => break,      ← implicit in while let\n      }\n  }",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "That is exactly what `while let Some(top) = stack.pop()` expands to. When you see \
                 `while let`, you are allowed to think of it as a loop whose exit condition is 'the pattern \
                 stopped matching'.",
            ),
            callout(
                "This is a preview, not a lecture",
                "`Option` gets its own module later. For now all you need is the shape: it is a type that \
                 is either `Some(value)` or `None`, it is how Rust represents 'maybe there is a value here', \
                 and `while let` is the idiomatic way to drain something that eventually runs dry.",
            ),
            code(
                "rust",
                "fn main() {\n    let mut queue = vec![\"job1\", \"job2\"];\n    let mut processed = 0;\n\n    while let Some(job) = queue.first() {\n        queue.remove(0);\n        processed += 1;\n        println!(\"processing {}\", job);\n    }\n\n    println!(\"processed {} jobs\", processed);\n}",
            ),
            callout(
                "Only reach for while let when the pattern is doing real work",
                "If your loop condition is a plain bool, use `while`. If it is a pattern match on an \
                 `Option` or a `Result`, use `while let`. If it is a pattern match on an enum with many \
                 variants, use `loop` with a `match` inside, because `while let` can only express one \
                 pattern plus the implicit failure case.",
            ),
            callout(
                "Practice — Try this now",
                "Build a `Vec` of ten numbers. Drain it with `while let Some(n) = v.pop()` and print each. \
                 Confirm the loop ends cleanly on an empty vector rather than panicking. Then try \
                 `while let Some(n) = v.first()` and see why that one would hang.",
            ),
            callout(
                "Mini challenge",
                "Write a `while let` that repeatedly pops pairs off a `Vec<(String, i32)>` and prints them, \
                 stopping when fewer than two items remain. You will need a length check — combine it with \
                 `while let` in a way that still reads clearly.",
            ),
        ],
    )
}

fn for_loop() -> SeedLesson {
    SeedLesson::new(
        "for: The Rust Way to Iterate",
        vec![
            labelled(
                "THE IDEA",
                "If you take one thing from this module, take this. `for` is the loop you will use most, \
                 because it cannot have the three classic bugs: it cannot forget to update a condition, it \
                 cannot run forever, and it cannot index out of bounds.",
            ),
            code(
                "rust",
                "fn main() {\n    for number in 1..=5 {\n        println!(\"{}\", number);\n    }\n\n    let names = [\"Gus\", \"Alex\", \"Sam\"];\n\n    for name in names {\n        println!(\"Hello, {}\", name);\n    }\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "There is no counter. No index variable. No condition. `for` asks a *sequence* for its next \
                 item, over and over, until there are none left. The sequence could be a range, an array, a \
                 `Vec`, a `HashMap`, or something you wrote yourself. The loop body is the same regardless.",
            ),
            callout(
                "Why this is safer",
                "Indexing loops ask you to remember the length, the start, the end, and whether the index \
                 is exclusive. Get any of that wrong and you get an off-by-one or a panic. `for` asks the \
                 sequence to tell you when it is done. That is one fewer thing to get wrong.",
            ),
            code(
                "rust",
                "fn main() {\n    let items = [\"sword\", \"shield\", \"potion\"];\n\n    for item in items {\n        println!(\"you carry: {}\", item);\n    }\n\n    // No index needed, no bounds to check, no length to remember\n}",
            ),
            callout(
                "for vs while — the honest comparison",
                "You can write a while loop over a collection if you maintain an index. It is more code, \
                 more room for error, and expresses the same intent more slowly. Reach for `while` when the \
                 condition is genuinely about a value's state. Reach for `for` when you are walking through \
                 a sequence. Those are the two cases.",
            ),
            callout(
                "The one thing to know later",
                "When you write `for item in items` where `items` is a `Vec` of `String`, that loop takes \
                 ownership and empties the vector. You will hit this in the next lesson. It is not a bug, \
                 and there is a one-character fix you will learn shortly.",
            ),
            callout(
                "Practice — Try this now",
                "Write three `for` loops: over a range, over an array of strings, and over an array of \
                 numbers where you print only the even ones using `if` and `continue`. Get comfortable with \
                 the absence of an index variable.",
            ),
            callout(
                "Mini challenge",
                "Write a `for` loop over the array `[\"a\", \"bb\", \"ccc\"]` that builds a visual bar, like \
                 `a   `, `bb  `, `ccc `. Use `String::from` and `push_str` to accumulate. This is your \
                 first real 'build something in a loop' exercise.",
            ),
        ],
    )
}

fn ranges() -> SeedLesson {
    SeedLesson::new(
        "Ranges",
        vec![
            labelled(
                "THE IDEA",
                "Ranges are the most common sequence you will iterate. They come in two forms and the \
                 difference between them is a single `=` character that catches everybody once.",
            ),
            diagram(
                "exclusive vs inclusive",
                "0..5     →  0  1  2  3  4\n                5 is NOT included\n\n0..=5    →  0  1  2  3  4  5\n                5 IS included\n\n..5      →  0  1  2  3  4     (starts at 0)\n5..      →  5  6  7  8  9     (runs forever)",
            ),
            code(
                "rust",
                "fn main() {\n    for n in 0..5 {\n        print!(\"{} \", n);\n    }\n    println!();   // 0 1 2 3 4\n\n    for n in 0..=5 {\n        print!(\"{} \", n);\n    }\n    println!();   // 0 1 2 3 4 5\n\n    for n in 5.. {\n        print!(\"{} \", n);\n        if n > 7 {\n            break;\n        }\n    }\n    println!();   // 5 6 7\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`..` means 'up to but not including'. `..=` means 'up to and including'. The `=` is the \
                 only difference, which is why it is easy to miss in a long line. Read `..=` as \
                 'through' — the range goes *through* the end value.",
            ),
            callout(
                "The mental model",
                "A range describes a *half-open* interval by default: start included, end excluded. That \
                 matches how slicing works, how lengths work, and how indexing works. Rust is consistent \
                 about this everywhere. `..=` is the deliberate exception you opt into.",
            ),
            code(
                "rust",
                "fn main() {\n    // Reverse: start must be less than end\n    for n in (1..=5).rev() {\n        print!(\"{} \", n);\n    }\n    println!();   // 5 4 3 2 1\n\n    // Skip every other number\n    for n in (0..=10).step_by(2) {\n        print!(\"{} \", n);\n    }\n    println!();   // 0 2 4 6 8 10\n}",
            ),
            callout(
                "Watch out",
                "`for n in 5..0` produces nothing at all, and does not panic. The range is empty because \
                 it cannot go backwards. That surprises people who expect an error or a descending \
                 sequence. If you want descending, use `.rev()` on a forward range.",
            ),
            callout(
                "Practice — Try this now",
                "Print every third number from 0 to 30 inclusive. Then print the same range descending. \
                 Then try `for n in 3..0` and confirm it prints nothing, and explain why to yourself in \
                 one sentence.",
            ),
            callout(
                "Mini challenge",
                "Print a multiplication table: the outer loop goes 1 to 5, the inner loop goes 1 to that \
                 number. Use `{:<3}` formatting so the columns line up. This is your first nested loop, \
                 and ranges make it readable.",
            ),
        ],
    )
}

fn collections() -> SeedLesson {
    SeedLesson::new(
        "Iterating Over Collections",
        vec![
            labelled(
                "THE IDEA",
                "`for` does not care what it is iterating. A range, an array, a `Vec` — the syntax is \
                 identical. That is not a coincidence; it is the whole design. There is a trait called \
                 `IntoIterator` that all of them implement, and `for` just uses it.",
            ),
            code(
                "rust",
                "fn main() {\n    let names = vec![\"Gus\", \"Alex\", \"Sam\"];\n\n    for name in &names {\n        println!(\"{}\", name);\n    }\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "That `&` is a borrow, and it is not cosmetic. Without it, `for name in names` would take \
                 ownership of the vector and empty it. With it, you walk through the values while leaving \
                 the collection intact. The next lesson is entirely about why, so for now just notice that \
                 the `&` was there and that the loop still worked.",
            ),
            callout(
                "Three ways to iterate, one difference",
                "`for x in collection` takes ownership — the collection is consumed. `for x in &collection` \
                 borrows it immutably — usable afterwards, cannot modify. `for x in &mut collection` \
                 borrows it mutably — you can modify each element in place. That is the entire table.",
            ),
            table(
                &["Syntax", "Ownership effect", "Can modify?"],
                &[
                    &["for x in coll", "consumes it", "no"],
                    &["for x in &coll", "borrows it", "no"],
                    &["for x in &mut coll", "borrows mutably", "yes"],
                    &["for x in coll.iter()", "borrows it", "no"],
                    &["for x in coll.iter_mut()", "borrows mutably", "yes"],
                    &["for x in coll.into_iter()", "consumes it", "yes"],
                ],
            ),
            code(
                "rust",
                "fn main() {\n    let mut scores = vec![10, 20, 30];\n\n    for score in &mut scores {\n        *score += 5;\n    }\n\n    println!(\"{:?}\", scores);   // [15, 25, 35]\n}",
            ),
            callout(
                "The star in *score += 5",
                "When you iterate `&mut`, each item is a mutable *reference*. To change the value behind \
                 it you dereference first with `*`. Miss the star and you get a confusing error about \
                 binary operations. It comes up in every Rust course exactly once.",
            ),
            code(
                "rust",
                "fn main() {\n    let data = vec![(\"alice\", 30), (\"bob\", 25)];\n\n    for (name, age) in &data {\n        println!(\"{} is {}\", name, age);\n    }\n}",
            ),
            callout(
                "Destructuring in a for pattern",
                "`for (name, age) in &data` pulls a tuple apart per iteration. Any pattern that works in a \
                 `match` works in a `for`. That is not a special case — it is the same pattern matching \
                 machinery doing the same thing.",
            ),
            callout(
                "Practice — Try this now",
                "Create a `Vec<i32>`, iterate with `&`, then iterate with `&mut` and double each value, \
                 then iterate with `&` again to print the result. Three loops, three different ownership \
                 outcomes. Watch which one leaves the vector usable.",
            ),
            callout(
                "Mini challenge",
                "Write a `Vec` of tuples for a small inventory: name and quantity. Iterate immutably to \
                 print the report, then mutably to restock anything below 5. Using the wrong one of `&` / \
                 `&mut` will not compile, and the error will tell you which you need.",
            ),
        ],
    )
}

fn ownership_looping() -> SeedLesson {
    SeedLesson::new(
        "Ownership While Looping",
        vec![
            labelled(
                "THE IDEA",
                "This is a short lesson about something that will get a full module of its own. We are not \
                 teaching ownership here. We are teaching you to recognise the moment you see it.",
            ),
            code(
                "rust",
                "fn main() {\n    let names = vec![String::from(\"Gus\"), String::from(\"Alex\")];\n\n    for name in names {\n        println!(\"{}\", name);\n    }\n\n    // ERROR: names has been moved\n    println!(\"{:?}\", names);\n}",
            ),
            code(
                "text",
                "error[E0382]: borrow of moved value: `names`\n --> src/main.rs:7:24\n  |\n5 |     for name in names {\n  |                ----- value moved here\n6 |         println!(\"{}\", name);\n7 |     }\n  |     ^ value moved due to this\n8 |\n9 |     println!(\"{:?}\", names);\n  |                       ^^^^^ value borrowed here after move",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The loop took the `Vec` and consumed it. Iteration hands out each element, and for a \
                 `Vec<String>` each element is itself something that has to be given away rather than \
                 copied. So the whole collection is gone by the time the loop ends. The fix is one \
                 character: borrow instead of consume.",
            ),
            code(
                "rust",
                "fn main() {\n    let names = vec![String::from(\"Gus\"), String::from(\"Alex\")];\n\n    for name in &names {\n        println!(\"{}\", name);\n    }\n\n    println!(\"{:?}\", names);   // fine — it was only borrowed\n}",
            ),
            callout(
                "Oh. A loop can move my collection",
                "Yes. And that is the realisation worth having right now. When you see `for x in things` \
                 and then use `things` afterwards, that is a move, and Rust will tell you. The `&` is the \
                 answer nearly every time.",
            ),
            code(
                "rust",
                "fn main() {\n    // Numbers are Copy, so this works without any & \n    let numbers = vec![1, 2, 3];\n\n    for n in numbers {\n        println!(\"{}\", n);\n    }\n\n    println!(\"{:?}\", numbers);   // compiles! numbers is Copy\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Here is why some loops consume and some do not bother you. Small numbers like `i32` are \
                 cheap to copy, and Rust copies them silently. `String` owns heap memory, so giving one \
                 away is a real event. That distinction — types that are cheap to copy versus types that \
                 own something — is the seed of the whole ownership module. Planted.",
            ),
            callout(
                "Which should I use?",
                "Borrow with `&` by default. You keep the collection, and there is no reason not to. \
                 Consume with no `&` when you genuinely want the collection gone afterwards, or when you \
                 are moving the contents into something else. Consuming on accident is a bug; borrowing \
                 by accident is a missed optimisation that costs nothing.",
            ),
            callout(
                "Practice — Try this now",
                "Write a `Vec<String>`, loop over it without `&`, and try to use the vector afterwards. Read \
                 the move error. Then add the `&`. Then repeat with a `Vec<i32>` and notice the error never \
                 appeared. That difference is the whole lesson.",
            ),
            callout(
                "Mini challenge",
                "Write a function that takes a `Vec<String>` and returns the longest string in it, using a \
                 `loop` with `break value` so you return early. Then call it twice with the same vector \
                 argument and see what the compiler says about the second call. That error is the ownership \
                 system telling you it already gave the data away.",
            ),
        ],
    )
}

fn nested_labels() -> SeedLesson {
    SeedLesson::new(
        "Nested Loops and Loop Labels",
        vec![
            labelled(
                "THE IDEA",
                "Loops inside loops work exactly as you would expect: the inner loop finishes all of its \
                 iterations before the outer one advances. Occasionally you need to exit *both* at once, \
                 and that is what labels are for.",
            ),
            code(
                "rust",
                "fn main() {\n    for row in 1..=3 {\n        for column in 1..=3 {\n            print!(\"{} \", row * column);\n        }\n        println!();\n    }\n}",
            ),
            code(
                "rust",
                "fn main() {\n    let mut grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];\n\n    for row in &mut grid {\n        for cell in row {\n            *cell *= 10;\n        }\n    }\n\n    println!(\"{:?}\", grid);\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The second example is the pattern you will write most: nested loops where the inner one \
                 modifies what the outer one handed it. Both loops use `&mut`, and that is why mutation \
                 works all the way down.",
            ),
            broken("fn main() {\n    'outer: loop {\n        for n in 0..3 {\n            if n == 2 {\n                break 'outer;   // ERROR: cannot break out of the for loop\n            }\n        }\n    }\n}"),
            code(
                "text",
                "error[E0426]: cannot break out of a `for` loop\n --> src/main.rs:4:26\n  |\n4 |             break 'outer;\n  |                   ^^^^^^^ cannot break out of a `for` loop",
            ),
            text_block(
                "A label on a `for` cannot be broken from, because `for` has no exit value to break with. \
                 Labels work on `loop` and `while`. If you find yourself wanting this, restructure — \
                 usually the inner loop belongs in its own function with an early return.",
            ),
            code(
                "rust",
                "fn main() {\n    'outer: loop {\n        for row in 1..=3 {\n            for column in 1..=3 {\n                if row * column > 6 {\n                    println!(\"first product over 6: {} * {} = {}\", row, column, row * column);\n                    break 'outer;   // leaves BOTH loops\n                }\n            }\n        }\n    }\n}",
            ),
            callout(
                "How a label works",
                "The label goes on the loop you want to leave, and `break 'label;` names it. The `'outer` \
                 is a lifetime-style syntax, so it reads as a name rather than a value. Only one label is \
                 needed here; you can chain as many as you have nested loops, but if you need three \n                 labels, the nesting is probably too deep.",
            ),
            code(
                "rust",
                "fn main() {\n    'search: loop {\n        let mut row = 0;\n\n        while row < 5 {\n            row += 1;\n            let mut col = 0;\n\n            while col < 5 {\n                col += 1;\n                if row * col == 12 {\n                    println!(\"found at {}, {}\", row, col);\n                    break 'search;\n                }\n            }\n        }\n\n    println!(\"done searching\");\n}",
            ),
            callout(
                "Do not overuse labels",
                "They are genuinely useful and genuinely easy to overuse. If you are labelling three \
                 nested loops, extract the inner ones into functions with early returns. The label is a \
                 signal that the nesting has gotten deep enough to want a function boundary.",
            ),
            callout(
                "Practice — Try this now",
                "Write nested loops that search a 5x5 grid for the value 23, printing the coordinates and \
                 using `break 'search;` to stop at the first hit. Then remove the label and use a plain \
                 `break` — you will only leave the inner loop, and the search keeps going. That difference \
                 is the point.",
            ),
            callout(
                "Mini challenge",
                "Print a multiplication table for 1 through 9, right-aligned, using nested loops and \
                 `{:>3}` formatting. Then find the first product greater than 200 in the same table and \
                 exit both loops with a label.",
            ),
        ],
    )
}

fn blocks_expressions() -> SeedLesson {
    SeedLesson::new(
        "Blocks Are Expressions Too",
        vec![
            labelled(
                "THE IDEA",
                "You have seen blocks produce values twice already: `if` and `loop`. Now let us look at a \
                 plain block on its own, because it is the simplest of the three and the one you will use \
                 most while writing real functions.",
            ),
            code(
                "rust",
                "fn main() {\n    let result = {\n        let x = 10;\n        let y = 20;\n\n        x + y\n    };\n\n    println!(\"{}\", result);   // 30\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The block has three statements and then `x + y` with no semicolon. That final line is the \
                 block's value. The whole `let result = { ... };` is one expression, and it evaluated to 30. \
                 Nothing was returned, nothing was assigned inside — the block simply *was* a value.",
            ),
            diagram(
                "the shape of a block expression",
                "  let result = {\n      let x = 10;\n      let y = 20;\n\n      x + y          ← last line, NO semicolon\n  };\n  ↑ the block is the value",
            ),
            code(
                "rust",
                "fn calculate() -> i32 {\n    let base = 100;\n    let bonus = 50;\n\n    base + bonus\n}\n\nfn main() {\n    println!(\"{}\", calculate());\n}",
            ),
            callout(
                "This is every function body, secretly",
                "A function body is a block. If it ends with an expression and no semicolon, that \
                 expression is the return value — no `return` keyword needed. This is why idiomatic Rust \
                 rarely uses `return`, and why the last line of most Rust functions looks like a bare \
                 value.",
            ),
            callout(
                "Why would you use a block?",
                "Because it gives you a scope. Variables declared inside a block do not leak out. That is \
                 the point: you can compute something with a temporary name and hand back only the value, \
                 with no clutter in the surrounding scope.",
            ),
            code(
                "rust",
                "fn main() {\n    let a = 5;\n\n    let doubled = {\n        let b = a * 2;\n        b + 1        // 11\n    };\n\n    println!(\"a is still {}\", a);   // 5 — b is out of scope\n    println!(\"doubled is {}\", doubled);\n}",
            ),
            callout(
                "Watch out — the semicolon",
                "A block ending in `b + 1;` produces nothing, and assigning it to `doubled` gives you a \
                 type error about `()`. This single character is the most common expression mistake in \n                 Rust, and it is why the compiler's error message is unusually helpful here.",
            ),
            callout(
                "Practice — Try this now",
                "Write a block that computes `(x + y) * 2` where x and y are declared *inside* the block \
                 and never appear outside. Assign it. Then add a semicolon to the last line and read the \
                 error carefully — the word `()` in the message is the clue.",
            ),
            callout(
                "Mini challenge",
                "Write a function `clamp(value: i32, low: i32, high: i32) -> i32` whose body is a single \
                 block expression that returns value, or low if too small, or high if too large. No `if` \
                 statements — use nested blocks. It is legal, it is idiomatic enough, and it teaches you \
                 what a block really is.",
            ),
        ],
    )
}

fn statements_expressions() -> SeedLesson {
    SeedLesson::new(
        "Expressions vs Statements",
        vec![
            labelled(
                "THE IDEA",
                "Rust draws a line between two things, and the line is drawn with one character: the \
                 semicolon. Understanding this one distinction removes an entire category of confusion.",
            ),
            table(
                &["", "Expression", "Statement"],
                &[
                    &["Produces", "a value", "nothing (unit)"],
                    &["Written as", "code, no trailing ;", "code with a trailing ;"],
                    &["Can be assigned", "yes", "no"],
                    &["Examples", "5, x + 1, if c { 1 } else { 2 }", "let x = 5; println!();"],
                ],
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "An expression is anything that evaluates to a value. A statement is an instruction that \
                 does the thing and, if it ends with a semicolon, evaluates to nothing. Almost every Rust \
                 construct is an expression, which is unusual — most languages have a much thinner set.",
            ),
            code(
                "rust",
                "fn main() {\n    // Statements — the semicolon discards the value\n    let x = 10;\n    println!(\"{}\", x);\n    x + 5;                 // evaluated, then thrown away\n\n    // Expressions — no semicolon, the value is kept\n    let y = {\n        let a = 10;\n        a + 5               // 15, this is the block's value\n    };\n\n    println!(\"y is {}\", y);\n}",
            ),
            code(
                "rust",
                "fn main() {\n    let a = 10;\n\n    // Adding a semicolon changes the type from i32 to ()\n    let b = { a + 5; };\n}",
            ),
            code(
                "text",
                "error[E0308]: mismatched types\n --> src/main.rs:4:21\n  |\n4 |     let b = { a + 5; };\n  |                 ^^^^^ expected `i32`, found `()`\n  |                 |\n  |                 expected due to this\n  |                 this expression has type `()`",
            ),
            text_block(
                "That error is as good as compiler errors get. It tells you the expected type, the found \
                 type, that `()` is the 'nothing' type, and exactly which characters are responsible. Every \
                 time you get it, look at where the caret points — that is your semicolon.",
            ),
            callout(
                "A useful rule of thumb",
                "If you are assigning the result, or returning it, or using it as a match arm body, the \
                 last line must not have a semicolon. If you are just doing something for its effect, the \
                 semicolon goes on. When in doubt, delete the semicolon and let the compiler tell you.",
            ),
            callout(
                "if and match are expressions too",
                "Which is why `let x = if c { 1 } else { 2 };` has no semicolon on the branches, and why \
                 `match` arms either end with a comma or an expression. Now you know why the rules differ: \
                 `if` and `match` can be values, statements cannot.",
            ),
            callout(
                "Practice — Try this now",
                "Write `let v = { 10 + 5; };` and read the error. Fix it. Then write `let w = 10 + 5;` and \
                 notice that this is fine — a bare expression statement is legal, it just evaluates to \
                 nothing and Rust warns you about it. Read that warning; it is telling you the value is \
                 being wasted.",
            ),
            code(
                "rust",
                "fn main() {\n    // Warning: unused_must_use or path statement with no effect\n    10 + 5;\n\n    // No warning — assigned to something\n    let useful = 10 + 5;\n    println!(\"{}\", useful);\n}",
            ),
            callout(
                "Mini challenge",
                "Predict the type of each of these without running them: `5;`, `5`, `{ 5 }`, \
                 `{ 5; }`, `if true { 1 } else { 2 };`, and `if true { 1 } else { 2 }`. Then write a \
                 program that forces each one to have a known type and check your predictions. The two \n                 that surprise people are the last pair.",
            ),
        ],
    )
}

fn option() -> SeedLesson {
    SeedLesson::new(
        "Control Flow + Option",
        vec![
            labelled(
                "THE IDEA",
                "Remember how `match` refused to compile unless you handled every case? Here is where that \
                 pays off completely. `Option` is a type with exactly two cases, and matching it is the \
                 single most common pattern in real Rust.",
            ),
            code(
                "rust",
                "fn main() {\n    let username: Option<String> = Some(String::from(\"Gus\"));\n\n    match username {\n        Some(name) => println!(\"Hello, {}\", name),\n        None => println!(\"No username found\"),\n    }\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`Option<T>` is either `Some(value)` or `None`. There is no third case — no null, no \
                 undefined, no 'maybe' that you can accidentally forget. If a function returns an \
                 `Option<String>`, the value is either a String or nothing, and the type says so. The \
                 compiler then forces you to deal with both.",
            ),
            callout(
                "Why this matters more than anything else in this module",
                "In most languages, a function can return a value or it can return nothing, and the type \
                 system does not care. You find out at runtime. In Rust, the *type* says whether absence is \
                 possible, and the compiler makes you handle it. That is the difference between a crash in \
                 production and a compile error in your editor.",
            ),
            code(
                "rust",
                "fn main() {\n    let username: Option<String> = Some(String::from(\"Gus\"));\n\n    // If you only care about one case, if let is shorter\n    if let Some(name) = username {\n        println!(\"Hello, {}\", name);\n    }\n\n    let missing: Option<String> = None;\n    if let Some(name) = missing {\n        println!(\"Hello, {}\", name);   // does not run\n    } else {\n        println!(\"nothing to greet\");\n    }\n}",
            ),
            table(
                &["", "match", "if let"],
                &[
                    &["Handles both cases", "always", "only if you write else"],
                    &["Required when", "you care about all cases", "one case is trivial"],
                    &["Exhaustiveness checked", "yes", "no"],
                ],
            ),
            code(
                "rust",
                "fn main() {\n    let scores = [Some(90), None, Some(75)];\n\n    for score in scores {\n        match score {\n            Some(s) if s >= 80 => println!(\"{} — great\", s),\n            Some(s) => println!(\"{} — ok\", s),\n            None => println!(\"no score recorded\"),\n        }\n    }\n}",
            ),
            callout(
                "The ubiquitous Option idiom",
                "You will write `match` on an `Option` in nearly every Rust program you ever write. It is \
                 the shape you saw a hundred times in the Rust Book. Now you know why it is written that \
                 way: the compiler is making sure you handled the empty case, and the pattern is pulling \
                 the value out so you can use it.",
            ),
            callout(
                "Practice — Try this now",
                "Write a function `first_char(text: &str) -> Option<char>` that returns the first character \
                 or `None` for an empty string. Then write a caller that uses `match` and a second caller \
                 that uses `if let`. Compile both. You have just written the most common shape in Rust.",
            ),
            callout(
                "Mini challenge",
                "Write a function `find_index(haystack: &[i32], needle: i32) -> Option<usize>` using a \
                 `loop` with `break Some(i)` and `break None`. This is exactly the search-loop pattern \
                 you learned two lessons ago, now returning an `Option`. Note that the `None` return type \
                 is what forces you to handle 'not found' at the call site.",
            ),
        ],
    )
}

fn result() -> SeedLesson {
    SeedLesson::new(
        "Control Flow + Result",
        vec![
            labelled(
                "THE IDEA",
                "Same idea, bigger stakes. `Result<T, E>` is either `Ok(value)` or `Err(error)`. Every \
                 operation that can fail returns one, and `match` on it is how you handle the failure. This \
                 is the pattern behind all of Rust's error handling.",
            ),
            code(
                "rust",
                "fn main() {\n    let input = \"42\";\n    let result = input.parse::<i32>();\n\n    match result {\n        Ok(number) => println!(\"parsed: {}\", number),\n        Err(error) => println!(\"failed: {}\", error),\n    }\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`parse` returns a `Result<i32, ParseIntError>`. It is either a number, or an explanation \
                 of why there is not one. Two cases, no third, and the compiler will not let you ignore \
                 either. Notice how the same match shape from the `Option` lesson now carries an error \
                 value you can actually inspect.",
            ),
            callout(
                "This is the shape you will write forever",
                "Fallible operation → `match` on the `Result` → handle `Ok` → handle `Err`. Every. Time. \
                 It looks different in every codebase, and it is always this. We are not teaching the full \
                 error handling system yet — that is its own module — but you now have the shape.",
            ),
            code(
                "rust",
                "fn parse_age(input: &str) -> Result<i32, String> {\n    let age = input.parse::<i32>()\n        .map_err(|_| format!(\"not a number: {}\", input))?;\n\n    if age < 0 {\n        return Err(\"age cannot be negative\".to_string());\n    }\n\n    Ok(age)\n}\n\nfn main() {\n    match parse_age(\"25\") {\n        Ok(age) => println!(\"age is {}\", age),\n        Err(e) => println!(\"error: {}\", e),\n    }\n}",
            ),
            callout(
                "The `?` operator, briefly",
                "That `?` says: if this failed, return the error immediately, otherwise unwrap the Ok \
                 value and carry on. It is the `match` you would have written, collapsed to one character. \
                 We will use it properly in the error handling module — for now, just notice it means \
                 'bail out on failure'.",
            ),
            code(
                "rust",
                "fn main() {\n    let inputs = [\"25\", \"abc\", \"-3\", \"40\"];\n\n    for input in inputs {\n        let outcome = match input.parse::<i32>() {\n            Ok(n) if n >= 0 => format!(\"valid: {}\", n),\n            Ok(n) => format!(\"negative: {}\", n),\n            Err(e) => format!(\"not a number ({})\", e),\n        };\n\n        println!(\"{:<8} -> {}\", input, outcome);\n    }\n}",
            ),
            callout(
                "Note the guard again",
                "`Ok(n) if n >= 0` uses a match guard to add a condition to a pattern. Same tool as two \
                 modules ago, now doing real work. The arms are checked in order, so the `Ok(n)` after it \
                 is only reached when the value is actually negative.",
            ),
            callout(
                "Practice — Try this now",
                "Parse a list of strings as `i32` and print a different message for each outcome: success, \
                 not-a-number, and negative. Use one `match` with a guard, not three separate ifs. Then \
                 handle a `Result` where the error type is `String` instead of a real error struct, and \
                 notice the arms look identical.",
            ),
            callout(
                "Mini challenge",
                "Write `fn double_first(text: &str) -> Result<i32, String>`. It should split the text on \
                 commas, parse the first piece as an integer, and double it. Return a `String` error for \
                 each failure mode: no input, not a number, or a value that would overflow. Call it three \
                 times — once with valid input, once with garbage, once with a huge number — and match on \
                 the result each time.",
            ),
        ],
    )
}

fn putting_it_together() -> SeedLesson {
    SeedLesson::new(
        "Putting It All Together",
        vec![
            labelled(
                "THE IDEA",
                "You have the whole toolkit now. `if` and `match` for decisions, `loop` / `while` / `for` \
                 for repetition, expressions everywhere, patterns everywhere, and the beginning of how \
                 `Option` and `Result` fit in. Time to see it working as one program.",
            ),
            code(
                "rust",
                "#[derive(Debug)]\nenum Item {\n    Weapon { name: String, damage: u32 },\n    Potion(u32),\n    Key,\n}\n\nfn describe(item: &Item) -> String {\n    match item {\n        Item::Weapon { name, damage } => format!(\"{} ({} dmg)\", name, damage),\n        Item::Potion(heal) => format!(\"potion (+{} hp)\", heal),\n        Item::Key => String::from(\"a key\"),\n    }\n}\n\nfn total_damage(items: &[Item]) -> u32 {\n    let mut total = 0;\n\n    for item in items {\n        match item {\n            Item::Weapon { damage, .. } => total += damage,\n            _ => continue,          // skip anything that is not a weapon\n        }\n    }\n\n    total\n}\n\nfn main() {\n    let inventory = [\n        Item::Weapon { name: String::from(\"Rusty Sword\"), damage: 12 },\n        Item::Potion(50),\n        Item::Weapon { name: String::from(\"Axe\"), damage: 20 },\n        Item::Key,\n    ];\n\n    for item in &inventory {\n        println!(\"inventory: {}\", describe(item));\n    }\n\n    let damage = total_damage(&inventory);\n    println!(\"total weapon damage: {}\", damage);\n\n    let grade = match damage {\n        0 => \"unarmed\",\n        d if d < 15 => \"lightly armed\",\n        d if d < 30 => \"dangerous\",\n        _ => \"lethal\",\n    };\n    println!(\"rating: {}\", grade);\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "That program is small and uses almost everything in this module. A `match` on an enum with \
                 named fields. A second `match` that only cares about one variant and skips the rest with \
                 `continue`. A `for` that borrows. A `match` with guards producing a `&str` value. \
                 Every one of those came from this module.",
            ),
            callout(
                "What you should be able to do now",
                "Reach for `if` when there are two cases. Reach for `match` when there are several, or \
                 when the type says 'one of these'. Reach for `for` when walking a sequence. Reach for \
                 `loop` when the exit is discovered inside. Reach for `while` when the condition is \
                 known. Reach for `if let` when one `Option` case is the interesting one.",
            ),
            paragraphs(&[
                "The next module is functions. You already have every tool you need — the reason to learn \
                 functions next is not new syntax but new structure. You will learn to take the logic you \
                 just wrote and put it behind a name, with parameters, so you can reuse it and test it.",
                "And once functions take arguments, ownership stops being an abstract idea and becomes \
                 something you feel every time you pass a value in or out. That is coming up fast.",
            ]),
            callout(
                "Practice — Try this now",
                "Extend the inventory program with a `loop` that lets the player pick items up until they \
                 type 'done', using `break` to exit and producing the final count as the loop's value. \
                 Then use a `while let` to pull items off a `Vec` one at a time.",
            ),
            callout(
                "Mini challenge — the gate",
                "Write a function `classify(score: u32) -> &'static str` that returns \"A\", \"B\", \"C\", \
                 or \"F\" using a `match` with guards, and rejects anything above 100 with a separate arm. \
                 Then write a `loop` that reads a hardcoded list of scores, skips any that are invalid with \
                 `continue`, and breaks with the count of valid ones. Two tools, one file.",
            ),
        ],
    )
}
