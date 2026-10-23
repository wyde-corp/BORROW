//! Module 02 — Variables & Types
//!
//! Teaches how Rust represents information: bindings, mutability, inference,
//! annotations, text, scalars, compounds, and conversions. Written against
//! `CONTENT_STYLE.md`.

use super::types::{
    breakdown, broken, broken_text, callout, checklist, code, diagram, fixed, gap_fill, guess_run,
    labelled, paragraphs, step_through, steps, table, text_block, Difficulty, ExerciseKind,
    SeedExercise, SeedLesson, SeedModule, SeedProject,
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
        lesson_text_str(),
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
                "THE IDEA",
                "`100 > 50` tells you almost nothing. `score > 50` tells you a story. A variable is a name \
                 attached to a value so later code can refer to that value by something meaningful — not to \
                 save typing, but so the code says what it means.",
            ),
            text_block(
                "Two things you will see in almost every example. `fn main()` is where a Rust program \
                 starts — read it as \"run this\"; functions get their own module later. `println!` prints a \
                 line, and `{}` inside the quotes is a placeholder for a value.",
            ),
            code(
                "rust",
                r#"fn main() {
    let score = 100;
    let username = "Gus";

    println!("{} scored {}", username, score);
}"#,
            ),
            gap_fill(
                "Give this value a name so the print works. Fill in the missing name.",
                "fn main() {\n    let ____ = 100;\n    println!(\"score: {score}\");\n}",
                &[("n1", "score", "The name used inside the braces of the print.")],
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`let` creates a **binding**: a name tied to a value. In `let score = 100;`, `score` is the \
                 name and `100` is the value. Every value also has a **type** — what kind of data it is. \
                 `100` is a whole number, `\"Gus\"` is text (a **string slice**, written `&str`), and `true` \
                 is a boolean. The next few lessons make types precise; for now, notice that `let` records \
                 the kind of thing you handed it, and Rust never forgets it.",
            ),
            guess_run(
                "The name is used before it is declared. Predict what the compiler does.",
                "fn main() {\n    println!(\"{score}\");\n    let score = 100;\n}",
                &[
                    "It prints 100 — Rust looks ahead",
                    "It does not compile — `score` is used before it is declared",
                    "It prints an empty line",
                ],
                1,
                "A name must be bound before it is used. Rust reads top to bottom here, so the `println!` \
                 has no `score` to find. The error names the undeclared value and suggests the \
                 declaration you forgot — which is the whole reason a variable has to be introduced with \
                 `let` first.",
            ),
            callout(
                "Why names matter",
                "To the compiler, `100` and `score` are nearly the same thing. The difference is entirely \
                 for the reader. Six months from now, `score > 50` is a gift; `100 > 50` is a riddle. \
                 That is the trade variables make.",
            ),
        ],
    )
}

fn lesson_immutable_by_default() -> SeedLesson {
    SeedLesson::new(
        "Immutable by Default",
        vec![
            labelled(
                "THE IDEA",
                "In Rust a plain `let` binding is immutable. Once it holds a value, that value is frozen: \
                 you can read it as often as you like, but you cannot change it. This surprises almost \
                 everyone on day one — and it is deliberate.",
            ),
            broken("fn main() {\n    let score = 100;\n\n    score = 200;\n}"),
            labelled(
                "LET'S BREAK IT DOWN",
                "The compiler stops on the second assignment. Read what it says: `cannot assign twice to \
                 immutable variable`. It is not scolding you. It is telling you that a plain `let` \
                 promised this value would not change, and this line breaks that promise.",
            ),
            broken_text(
                "error[E0384]: cannot assign twice to immutable variable `score`\n --> src/main.rs:4:5\n  |\n4 |     score = 200;\n  |     ^^^^^^^ cannot assign twice to immutable variable",
            ),
            text_block(
                "That promise is useful. If a value cannot change, then reading it later tells you \
                 something reliable: it is exactly what it said it was. The compiler gets to trust it, \
                 and so do you. Immutability is information, not red tape.",
            ),
            guess_run(
                "This binding is never reassigned. Predict whether it compiles.",
                "fn main() {\n    let score = 100;\n    println!(\"{score}\");\n}",
                &[
                    "It compiles — `let` without `mut` is fine as long as you never reassign",
                    "It does not compile — `let` always needs `mut`",
                ],
                0,
                "`let` is the normal case; `mut` is the exception you add only when you intend to change \
                 the value. Reading an immutable binding is perfectly fine — the restriction is on \
                 changing it.",
            ),
            callout(
                "Yes, Rust wants to know whether this changes",
                "No, it is not being nosy. Knowing a value cannot change lets the compiler trust it and \
                 lets a reader rely on it. That is the whole trade, and it pays off in every later \
                 module.",
            ),
        ],
    )
}

fn lesson_mut() -> SeedLesson {
    SeedLesson::new(
        "mut — When Values Need to Change",
        vec![
            labelled(
                "THE IDEA",
                "Some values genuinely change: a player's health drops, a timer counts down, a score \
                 climbs. For those, Rust asks you to say so out loud with `mut`. It is a small word \
                 carrying real information — this binding is intentionally allowed to change.",
            ),
            code(
                "rust",
                r#"fn main() {
    let mut health = 100;

    health -= 10;
    health -= 20;

    println!("Health: {health}");
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`let mut` does what `let` does and adds one permission: this binding may be reassigned. \
                 That lets you write `health = health - 10`, or the shorthand `health -= 10` — the same \
                 line with the operation moved in front of the `=`. Mutability attaches to the \
                 **binding**, not to the value. That distinction will matter a lot once you reach \
                 ownership, so it is worth planting now.",
            ),
            table(
                &["Binding", "Can it be reassigned?"],
                &[
                    &["`let x = 5;`", "No — frozen at 5."],
                    &["`let mut x = 5;`", "Yes — x can become 6, 7, or anything of the same type."],
                ],
            ),
            gap_fill(
                "The value changes, so the binding needs permission. Fill in the missing keyword.",
                "let ____ seconds = 3;\nseconds -= 1;\nprintln!(\"{seconds}\");",
                &[("m1", "mut", "The word that allows reassignment.")],
            ),
            callout(
                "`mut` is not a switch for everything",
                "It marks one binding as changeable. It does not make the value a different kind of \
                 thing, and it does not let you change the binding's type — you meet that boundary \
                 shortly, under shadowing.",
            ),
        ],
    )
}

fn lesson_why_immutable() -> SeedLesson {
    SeedLesson::new(
        "Why Rust Defaults to Immutability",
        vec![
            labelled(
                "THE IDEA",
                "Why make you opt in to change? It looks bureaucratic until you notice what it buys. If a \
                 value cannot silently change, then reading it later tells you something reliable: it is \
                 exactly what it said it was.",
            ),
            step_through(
                "An immutable value, read far from where it is set",
                r#"let user_count = 100;

let bonus = {
    let doubled = user_count * 2;
    doubled + 50
};

println!("users: {user_count}");"#,
                &[
                    (&[1][..], "`user_count` owns 100 and is immutable."),
                    (&[3][..], "A block reads `user_count` to build `bonus`."),
                    (&[4][..], "The block can read the value but cannot change it."),
                    (&[7][..], "So this line is guaranteed to print 100. Nothing could have changed it."),
                ],
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The value is read in two places, and because it is immutable, both reads agree. That is \
                 the whole argument for the default. A plain `let` is a promise the compiler enforces: \
                 this value does not move. You can therefore reason about it without hunting through the \
                 program for whoever might have changed it.",
            ),
            code(
                "rust",
                r#"fn main() {
    let tax_rate = 0.2;        // a promise: this never changes
    let mut cart_total = 0.0;  // in motion: this will change

    cart_total += 10.0;
    cart_total += 5.0;

    println!("tax: {}", cart_total * tax_rate);
}"#,
            ),
            text_block(
                "Same keyword, two different contracts with the reader. The first is a promise; the \
                 second is an admission that things are in motion. Both are useful — and saying which \
                 you mean is the point.",
            ),
            guess_run(
                "A block tries to change a value declared without `mut`. Predict what happens.",
                "fn main() {\n    let total = 0;\n\n    let result = {\n        total = total + 5;\n        total\n    };\n\n    println!(\"{result}\");\n}",
                &[
                    "It compiles and prints 5",
                    "It does not compile — `total` is immutable, even inside the block",
                ],
                1,
                "A block does not grant new powers. `total` was declared without `mut`, so nothing — not \
                 even a nested block — may reassign it. The fix is `let mut total = 0;`. This is exactly \
                 the reliability the immutable default buys.",
            ),
            callout(
                "Try it",
                "Declare one immutable binding and one mutable binding for the same kind of data, and \
                 write a comment on each saying which contract you meant.",
            ),
        ],
    )
}

fn lesson_type_inference() -> SeedLesson {
    SeedLesson::new(
        "Type Inference",
        vec![
            labelled(
                "THE IDEA",
                "You never told Rust the type of `age`. You never told it the type of `price`. It knows \
                 anyway. This is **type inference**: Rust works out the type from the value and the \
                 surrounding context, before your program runs.",
            ),
            code(
                "rust",
                r#"fn main() {
    let age = 25;        // i32
    let name = "Gus";     // &str
    let active = true;     // bool
    let price = 19.99;     // f64

    println!("{age} {name} {active} {price}");
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Rust reads the literal and decides. A whole number is `i32`. A number with a decimal \
                 point is `f64`. Text in double quotes is `&str` — a string slice, the type for text; \
                 the lesson after next gives it the attention it deserves. `true` is a `bool`. Having \
                 inferred the type once, Rust locks it in for the life of the binding.",
            ),
            gap_fill(
                "Write the type Rust infers for each value.",
                "let a = 42;       // ____\nlet b = 42.0;     // ____\nlet c = 'a';      // ____\nlet d = \"a\";      // ____\nlet e = true;     // ____",
                &[
                    ("t1", "i32", "Whole numbers default to 32-bit signed."),
                    ("t2", "f64", "A decimal point makes it a 64-bit float."),
                    ("t3", "char", "Single quotes hold one character."),
                    ("t4", "&str", "Double quotes hold a string slice."),
                    ("t5", "bool", "`true` and `false` are booleans."),
                ],
            ),
            guess_run(
                "Rust inferred a type for `count`, then a different type is assigned. Predict what happens.",
                "fn main() {\n    let count = 5;\n    count = \"now a string\";\n}",
                &[
                    "It compiles — Rust re-infers the type",
                    "It does not compile — the type was already decided",
                ],
                1,
                "`count` was inferred as `i32` from `5`, and a binding's type does not change. Assigning \
                 a `&str` is a mismatched-types error. To hold a different type you would need a new \
                 binding — that is shadowing, later in this module.",
            ),
            callout(
                "Inference saves typing, not thinking",
                "Rust still knows the exact type of every value and still holds you to it. Inference \
                 spares you the annotation, not the decision.",
            ),
        ],
    )
}

fn lesson_type_annotations() -> SeedLesson {
    SeedLesson::new(
        "Type Annotations",
        vec![
            labelled(
                "THE IDEA",
                "Sometimes you want to be explicit. `let age: i32 = 25;` puts the type right there in the \
                 source. This is a **type annotation**. It is optional, and Rust does not want it \
                 everywhere — but there are real places it earns its keep.",
            ),
            code(
                "rust",
                r#"fn main() {
    let age: i32 = 25;
    let score: u16 = 100;
    let price: f64 = 19.99;
    let username: &str = "Gus";
    let active: bool = true;

    println!("{age} {score} {price} {username} {active}");
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The `: i32` between the name and the `=` is the annotation. It states what type the \
                 binding should have, and Rust checks the value against it. That check is the value: \
                 annotate `: u8` and write `300`, and Rust stops you rather than silently wrapping.",
            ),
            steps(&[
                "Readability — the reader sees the type without inferring it.",
                "Intent — `let price: f64` says \"money, decimals matter\".",
                "Ambiguity — a bare value could be several types.",
                "Empty collections — a list created with nothing in it has no element type to infer.",
                "Function boundaries — clear signatures for callers (later module).",
            ]),
            gap_fill(
                "Add the missing annotation so this empty list knows what it holds. Fill in the type.",
                "let scores: ____ = Vec::new();\nprintln!(\"{}\", scores.len());",
                &[("v1", "Vec<i32>", "A growable list whose elements are i32.")],
            ),
            text_block(
                "`Vec` is a growable list — you meet it fully in Module 08. For now read `Vec<i32>` as \
                 \"a list of whole numbers\". The empty-list case is the classic reason an annotation is \
                 required.",
            ),
            callout(
                "When not to annotate",
                "If Rust already knows the type, an annotation is noise. `let count: i32 = 5;` tells a \
                 reader nothing they could not infer. Annotate for clarity or necessity, not by habit.",
            ),
        ],
    )
}

fn lesson_text_str() -> SeedLesson {
    SeedLesson::new(
        "Text: `&str`",
        vec![
            labelled(
                "THE IDEA",
                "You have been writing text since the first example — `\"Gus\"`, `\"hello\"` — without \
                 being told what it is. This lesson fixes that. Text in double quotes is a **string \
                 slice**, written `&str`. It is the type for a run of characters: one letter, a word, a \
                 whole paragraph.",
            ),
            code(
                "rust",
                r#"fn main() {
    let greeting = "hello";
    let name = "Gus";
    let empty = "";

    println!("{greeting}, {name}!");
    println!("empty is {} characters", empty.len());
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "A `&str` can hold any number of characters, including none. The `&` means it is a view \
                 into text rather than something that owns its own copy; for now, the practical rule is \
                 simply that **double quotes make text**. You can ask for its length with `.len()` — a \
                 dotted call is a **method**, a function attached to a value, and methods appear \
                 throughout the language.",
            ),
            table(
                &["You write", "Type", "Holds"],
                &[
                    &["`'A'`", "`char`", "exactly one character"],
                    &["`\"A\"`", "`&str`", "one character — or many"],
                    &["`\"\"`", "`&str`", "no characters at all"],
                ],
            ),
            guess_run(
                "A character type is given text in double quotes. Predict what happens.",
                "fn main() {\n    let letter: char = \"R\";\n    println!(\"{letter}\");\n}",
                &[
                    "It compiles and prints R",
                    "It does not compile — a `char` is one character in single quotes",
                ],
                1,
                "`\"R\"` is a `&str`; `'R'` is a `char`. They look alike and are different types, and the \
                 compiler will say `expected char, found &str`. This is the single most common quotes \
                 mistake for beginners, and now you know which quote means what.",
            ),
            gap_fill(
                "Write the type of each value. Choose `char` or `&str`.",
                "let a = 'x';     // ____\nlet b = \"x\";     // ____\nlet c = \"hello\"; // ____",
                &[
                    ("s1", "char", "Single quotes: one character."),
                    ("s2", "&str", "Double quotes: a string slice."),
                    ("s3", "&str", "Still a string slice, regardless of length."),
                ],
            ),
            callout(
                "There is also an owned string",
                "You will meet `String` soon — text that owns its own copy and can grow. For now, `&str` \
                 is enough: every literal you write is one, and every lesson before strings-proper uses \
                 them.",
            ),
        ],
    )
}

fn lesson_integer_types() -> SeedLesson {
    SeedLesson::new(
        "Integer Types",
        vec![
            labelled(
                "THE IDEA",
                "Rust has a whole family of integer types, and they are not interchangeable. When you \
                 write `25`, Rust picks `i32` for you. When `25` appears somewhere that demands a `u8`, it \
                 checks whether 25 actually fits.",
            ),
            table(
                &["Type", "Meaning", "Range (roughly)"],
                &[
                    &["i8", "signed 8-bit", "-128 to 127"],
                    &["i16", "signed 16-bit", "-32,768 to 32,767"],
                    &["i32", "signed 32-bit", "-2.1 billion to 2.1 billion"],
                    &["i64", "signed 64-bit", "-9.2 quintillion to 9.2 quintillion"],
                    &["i128", "signed 128-bit", "far larger than you will need"],
                    &["isize", "signed, pointer-sized", "depends on your machine"],
                    &["u8", "unsigned 8-bit", "0 to 255"],
                    &["u16", "unsigned 16-bit", "0 to 65,535"],
                    &["u32", "unsigned 32-bit", "0 to 4.2 billion"],
                    &["u64", "unsigned 64-bit", "0 to 18.4 quintillion"],
                    &["u128", "unsigned 128-bit", "far larger than you will need"],
                    &["usize", "unsigned, pointer-sized", "depends on your machine"],
                ],
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The letter and the number each say something. The letter is the signedness: `i` can be \
                 negative, `u` cannot. The number is the **bit width** — how much storage the type uses. \
                 One bit is a single on/off switch, so 8 bits give 2⁸ = 256 distinct values.",
            ),
            diagram(
                "reading the name",
                "i32\n│ │\n│ └── 32 bits of storage\n└──── i = signed (can be negative)\n\nu8\n│ │\n│ └── 8 bits of storage\n└──── u = unsigned (zero and up only)",
            ),
            text_block(
                "For ordinary whole numbers with no other context, Rust infers `i32`. That covers about \
                 two billion in each direction, so it is a safe default. You reach for the smaller and \
                 larger types when you have a reason: matching a file format, staying inside a hardware \
                 range, or holding values that genuinely might exceed `i32`.",
            ),
            gap_fill(
                "Name the type. Choose from `i8`, `u8`, `i32`, or `u64`.",
                "let level: ____ = 100;        // 1 to 100, never negative\nlet big: ____ = 9_000_000_000_000; // far beyond i32",
                &[
                    ("l1", "u8", "A small non-negative number."),
                    ("l2", "u64", "A number larger than i32 can hold."),
                ],
            ),
            callout(
                "Types have limits",
                "This matters more than it sounds. A `u8` holds at most 255, and try to store 256 in one \
                 and Rust will stop you. Later, working with ranges and conversions, you will meet the \
                 word *overflow*. For now carry the habit: when you pick a type, pick it on purpose.",
            ),
        ],
    )
}

fn lesson_integer_literals() -> SeedLesson {
    SeedLesson::new(
        "Integer Literals",
        vec![
            labelled(
                "THE IDEA",
                "Rust lets you write whole numbers in four notations. You are not required to use them, \
                 but you will meet all four eventually — and once you know what they are, they stop \
                 looking strange.",
            ),
            code(
                "rust",
                r#"fn main() {
    let decimal = 1_000_000;
    let hex = 0xff;          // 255
    let binary = 0b1010;     // 10
    let octal = 0o755;       // 493

    println!("{decimal} {hex} {binary} {octal}");
}"#,
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
                "LET'S BREAK IT DOWN",
                "The underscores are the part worth caring about. `1000000` is valid Rust, but counting \
                 the zeros wastes your attention. `1_000_000` is the same number and reads at a glance. \
                 Rust ignores the underscores entirely — they exist purely for humans.",
            ),
            gap_fill(
                "Write `255` in binary and in hexadecimal. Fill in the two literals.",
                "let binary = ____;      // 255\nlet hex = ____;         // 255",
                &[
                    ("b1", "0b1111_1111", "Binary starts with `0b`."),
                    ("h1", "0xff", "Hexadecimal starts with `0x`."),
                ],
            ),
            callout(
                "Where you will actually see these",
                "Hex in colours (`0xFF_6B_35` is BORROW_'s burnt orange) and memory addresses. Binary when \
                 you are reasoning about individual bits. Octal for file permissions (`0o755`) on Linux \
                 and macOS. Decimal the other ninety-five percent of the time.",
            ),
        ],
    )
}

fn lesson_signed_unsigned() -> SeedLesson {
    SeedLesson::new(
        "Signed vs Unsigned",
        vec![
            labelled(
                "THE IDEA",
                "A signed integer can be negative. An unsigned integer cannot. That is the whole \
                 difference — but its consequences ripple outward in ways that surprise people \
                 regularly.",
            ),
            code(
                "rust",
                r#"fn main() {
    let temperature: i32 = -10;  // signed — fine
    let age: u32 = 25;           // unsigned — fine
    let delta: i8 = -5;          // fine
    let count: u8 = 0;           // also fine

    println!("{temperature} {age} {delta} {count}");
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Signed types spend half their range on negatives. `i8` runs from -128 to 127 — 256 values, \
                 half of them negative. `u8` runs from 0 to 255 — 256 values, none negative. Same storage, \
                 different territory.",
            ),
            broken("fn main() {\n    let age: u32 = -1;\n}"),
            broken_text(
                "error[E0308]: mismatched types\n  --> src/main.rs:3:20\n   |\n3  |     let age: u32 = -1;\n   |                    ^^   expected `u32`, found integer `{integer}`",
            ),
            text_block(
                "Think about whether that is a *type* problem or a *sense* problem. An age of -1 is not a \
                 thing, and Rust is not merely enforcing a rule — it is agreeing with you that the value \
                 is nonsense. This is the type system doing exactly what you would want.",
            ),
            callout(
                "Reach for unsigned when the answer is never negative",
                "Array lengths, byte counts, ages, port numbers. Reach for signed when the value might dip \
                 below zero. The mistake is picking `u32` for everything because \"numbers are usually \
                 positive, right?\"",
            ),
        ],
    )
}

fn lesson_floats() -> SeedLesson {
    SeedLesson::new(
        "Floating-Point Numbers",
        vec![
            labelled(
                "THE IDEA",
                "Not every number is whole. Rust has two float types: `f32` (32 bits) and `f64` (64 bits). \
                 Write a decimal with no other context and Rust gives you `f64`.",
            ),
            code(
                "rust",
                r#"fn main() {
    let price = 19.99;              // f64 by inference
    let precise: f64 = 19.99;       // explicitly 64-bit
    let smaller: f32 = 19.99;       // explicitly 32-bit

    println!("{price} {precise} {smaller}");
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Floats store an *approximation* of your number, not the exact value. Most of the time \
                 that is invisible; it becomes visible with arithmetic on values that have no exact \
                 binary form. Run the example below and read the first line: `0.1 + 0.2` does not come \
                 out to `0.3`.",
            ),
            code(
                "rust",
                r#"fn main() {
    let a = 0.1;
    let b = 0.2;

    println!("a + b = {}", a + b);          // 0.30000000000000004
    println!("equal to 0.3: {}", a + b == 0.3);  // false
}"#,
            ),
            text_block(
                "That is not a Rust bug. `0.1` has no exact binary representation, in any language with a \
                 binary float type. The value you asked for, the value stored, and the value printed are \
                 three related but distinct things. `f32` is worse, because it has fewer bits to work \
                 with.",
            ),
            callout(
                "Formatting a float",
                "`{:.2}` prints a float with exactly two decimal places — `{:.2}` on `3.5` gives `3.50`. \
                 Since floats are approximations, this is how you keep money-looking output tidy.",
            ),
            callout(
                "Which should I use?",
                "Default to `f64`. Reach for `f32` only in a large array where halving the memory actually \
                 matters. For money specifically, look up a decimal crate later — floats are the wrong \
                 tool for accounting.",
            ),
        ],
    )
}

fn lesson_booleans() -> SeedLesson {
    SeedLesson::new(
        "Booleans",
        vec![
            labelled(
                "THE IDEA",
                "A boolean is two values: `true` and `false`. That is the whole type. In Rust it is a \
                 first-class value you can store in a variable — not merely something that appears inside \
                 an `if`.",
            ),
            code(
                "rust",
                r#"fn main() {
    let logged_in: bool = true;
    let has_permission: bool = false;
    let is_admin = true;          // inferred as bool

    println!("{logged_in} {has_permission} {is_admin}");
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "A `bool` takes one byte and has exactly two possible values. That constraint is the \
                 point: there is no third option, no `maybe`, no `null`. When a variable holds a bool, you \
                 know exactly what it is — and when you combine booleans with `&&` (and) or `||` (or), the \
                 result is another bool.",
            ),
            gap_fill(
                "Combine the two flags with a boolean operator. Fill in the operator and the result line.",
                "let logged_in = true;\nlet has_key = false;\n\nprintln!(\"can play: {}\", logged_in ____ has_key);  // false",
                &[("o1", "&&", "Both must be true.")],
            ),
            code(
                "rust",
                r#"fn main() {
    let logged_in = true;
    let has_key = false;

    println!("can play: {}", logged_in && has_key);  // false
    println!("either:  {}", logged_in || has_key);   // true
}"#,
            ),
            callout(
                "Booleans become decisions",
                "Right now a bool is a value sitting in a variable. The moment it meets an `if` or a \
                 `while`, it becomes a decision — and `if` is exactly where the next module starts. The \
                 groundwork is here.",
            ),
        ],
    )
}

fn lesson_characters() -> SeedLesson {
    SeedLesson::new(
        "Characters",
        vec![
            labelled(
                "THE IDEA",
                "A `char` holds exactly one character, written in single quotes: `'A'`, `'7'`, `'♥'`. It is \
                 not a string; it is a single value, and it is always four bytes wide.",
            ),
            code(
                "rust",
                r#"fn main() {
    let letter: char = 'A';
    let digit: char = '7';
    let heart: char = '♥';
    let ninja: char = '🥷';

    println!("{letter} {digit} {heart} {ninja}");
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Rust's `char` is a Unicode scalar value, which covers essentially every writing system on \
                 earth plus a large set of symbols and emoji. The type is always four bytes wide, no \
                 matter how many bytes the glyph itself occupies. Single quotes make a `char`; double \
                 quotes make a `&str`, which you already met.",
            ),
            broken("fn main() {\n    let c: char = \"A\";\n}"),
            broken_text(
                "error[E0308]: mismatched types\n  --> src/main.rs:2:18\n   |\n2  |     let c: char = \"A\";\n   |                  ^^^ expected `char`, found `&str`",
            ),
            gap_fill(
                "Each character answers a yes/no question using a method. Fill in the three printed values.",
                "let c = 'R';\n\nprintln!(\"is alphabetic: {}\", c.is_alphabetic());  // ____\nprintln!(\"is numeric:    {}\", c.is_numeric());     // ____\nprintln!(\"is uppercase:  {}\", c.is_uppercase());   // ____",
                &[
                    ("a1", "true", "`R` is a letter."),
                    ("a2", "false", "`R` is not a digit."),
                    ("a3", "true", "`R` is uppercase."),
                ],
            ),
            callout(
                "A `char` has useful methods",
                "`is_alphabetic()`, `is_numeric()`, `is_uppercase()`, `to_ascii_lowercase()` — all return \
                 values, and several return a `bool`, tying this lesson back to the last one. A dotted \
                 call like `c.is_alphabetic()` is a method call.",
            ),
        ],
    )
}

fn lesson_tuples() -> SeedLesson {
    SeedLesson::new(
        "Tuples",
        vec![
            labelled(
                "THE IDEA",
                "Sometimes a few values belong together and are not all the same type. That is a tuple: a \
                 fixed-size group of values that can each have their own type.",
            ),
            code(
                "rust",
                r#"fn main() {
    let user = ("Gus", 25, true);

    println!("Name: {}", user.0);
    println!("Age:  {}", user.1);
    println!("Active: {}", user.2);
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Each element has its own type, inferred independently: here `&str`, `i32`, and `bool`. \
                 Access them by position with `.0`, `.1`, `.2` — positions start at zero. The size is fixed \
                 at creation; a tuple cannot grow or shrink.",
            ),
            breakdown(
                "Destructuring a tuple",
                "let user = (\"Gus\", 25, true);\nlet (name, age, active) = user;\nprintln!(\"{name} is {age}\");",
                &[
                    "A tuple of three values is created.",
                    "The pattern `(name, age, active)` pulls each element out into its own named binding.",
                    "The named values print — no `.0` or `.1` in sight.",
                ],
            ),
            gap_fill(
                "Read the second element of the tuple. Fill in the access.",
                "let pair = (10, 20);\nprintln!(\"{}\", pair____);",
                &[("p1", ".1", "Tuples use a dot and a number, starting at zero.")],
            ),
            callout(
                "When to use a tuple",
                "When a handful of values genuinely travel together and you know the count at compile \
                 time: coordinates, colour channels, a version triple. If you find yourself writing \
                 `.0`, `.1`, `.2`, `.3` and losing track of what each one means, that is a sign you want \
                 something clearer. That something is a `struct`, and it arrives in Module 07.",
            ),
        ],
    )
}

fn lesson_arrays() -> SeedLesson {
    SeedLesson::new(
        "Arrays",
        vec![
            labelled(
                "THE IDEA",
                "An array holds a fixed number of values, all of the same type, in order. Fixed means \
                 fixed: `[i32; 5]` is not a suggestion, it is the type.",
            ),
            code(
                "rust",
                r#"fn main() {
    let numbers = [1, 2, 3, 4, 5];

    println!("{}", numbers[0]);  // 1
    println!("{}", numbers[4]);  // 5
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Two rules: every element is the same type, and the count is part of the type itself. \
                 Indexing starts at **zero**, so `numbers[0]` is the first element, not the second — this \
                 trips up almost everyone, usually exactly once.",
            ),
            callout(
                "Reading `[i32; 5]`",
                "Square brackets mean *array*. Inside is the element type, `i32`. The semicolon separates \
                 it from the length, `5`. Read it out loud: *array of `i32` with five elements*. Rust's \
                 type syntax is dense like this everywhere; getting fluent with it pays off across the \
                 whole language.",
            ),
            gap_fill(
                "Write the array type and the repeated-value shorthand. Fill in the two blanks.",
                "let zeros: ____ = [0; 4];\nprintln!(\"{} zeros\", zeros.len());",
                &[("z1", "[i32; 4]", "Element type, then length, in square brackets.")],
            ),
            code(
                "rust",
                r#"fn main() {
    let words = ["a", "b", "c"];
    let zeros: [u8; 4] = [0; 4];

    println!("{words:?}");
    println!("{zeros:?}");
}"#,
            ),
            callout(
                "`{:?}` prints a whole array",
                "`{zeros:?}` uses the debug formatter, which prints all the elements: `[0, 0, 0, 0]`. The \
                 plain `{}` formatter works on a single value; a collection needs `{:?}`.",
            ),
        ],
    )
}

fn lesson_constants() -> SeedLesson {
    SeedLesson::new(
        "Constants",
        vec![
            labelled(
                "THE IDEA",
                "Some values are fixed for the entire life of a program: the maximum score, the tax rate, \
                 the name of the app. A `const` says: this value will never change, and it is known before \
                 the program runs.",
            ),
            code(
                "rust",
                r#"const MAX_SCORE: u32 = 100;
const APP_NAME: &str = "BORROW_";
const TAX_RATE: f64 = 0.20;

fn main() {
    println!("{APP_NAME} — max score is {MAX_SCORE} (tax {TAX_RATE})");
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Three things make a constant different from a variable. It uses `const`, not `let`. It \
                 **requires** a type annotation — no inference here. And its name is written in \
                 `SCREAMING_SNAKE_CASE` by convention, because it reads as a named fact rather than a \
                 working value.",
            ),
            code(
                "rust",
                r#"const MAX_PLAYERS: usize = 4;

fn main() {
    // A constant can be used where a value is required at compile time.
    let table: [u8; MAX_PLAYERS] = [0; MAX_PLAYERS];

    println!("table size: {}", table.len());
}"#,
            ),
            callout(
                "Constants are not \"variables that never change\"",
                "They are a different kind of thing: a named fact about your program, available \
                 everywhere and known to the compiler before it runs. That is why a constant can set an \
                 array's length — a runtime value cannot.",
            ),
        ],
    )
}

fn lesson_shadowing() -> SeedLesson {
    SeedLesson::new(
        "Shadowing",
        vec![
            labelled(
                "THE IDEA",
                "You can declare a new `let` with a name that already exists. The new binding does not \
                 overwrite the old one — it *shadows* it. The old value still exists; you simply cannot \
                 reach it through that name any more.",
            ),
            code(
                "rust",
                r#"fn main() {
    let x = 5;

    let x = x + 1;   // new binding, same name
    let x = x * 2;   // another new binding

    println!("{x}");  // 12
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Read `let x = x + 1;` carefully. The right-hand `x` is the old binding, value 5. The \
                 result, 6, is bound to a brand-new `x`. The old `x` still holds 5, and will until it goes \
                 out of scope; it is just no longer reachable by that name.",
            ),
            step_through(
                "Three bindings, one name",
                r#"let value = 10;
let value = value + 5;
let value = value * 3;
println!("{value}");"#,
                &[
                    (&[1][..], "Binding 1 is created, holding 10."),
                    (&[2][..], "Binding 2 reads the first (10), adds 5, and takes the name."),
                    (&[3][..], "Binding 3 reads the second (15), multiplies by 3, and takes the name."),
                    (&[4][..], "The name `value` now reaches 45; the earlier bindings are unreachable."),
                ],
            ),
            callout(
                "This is not mutation",
                "Mutation would be `x = x + 1;` — no `let`. Shadowing is `let x = x + 1;` — a new `let`. \
                 The presence of `let` is the tell, and the next lesson compares them properly.",
            ),
        ],
    )
}

fn lesson_shadowing_vs_mutation() -> SeedLesson {
    SeedLesson::new(
        "Shadowing vs Mutation",
        vec![
            labelled(
                "THE IDEA",
                "These two look almost identical and behave differently in ways that matter. Understanding \
                 why is the difference between writing Rust and guessing at it.",
            ),
            table(
                &["", "Mutation", "Shadowing"],
                &[
                    &["Syntax", "`x = 6;`", "`let x = 6;`"],
                    &["Bindings", "One, reused", "Two — the second hides the first"],
                    &["Old value", "Gone", "Still there, unreachable"],
                    &["Can change type?", "No", "Yes"],
                    &["Reads as", "\"update the value\"", "\"a new thing in the same role\""],
                ],
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The deciding question is what the code is *saying*. If you are tracking one quantity as it \
                 changes — health dropping, a counter climbing — that is mutation. If you are transforming \
                 one representation into another — text parsed into a number, a value converted between \
                 units — that is shadowing.",
            ),
            guess_run(
                "The second line has no `let`. Predict what it does.",
                "fn main() {\n    let x = 5;\n    x = 6;\n\n    println!(\"{x}\");\n}",
                &[
                    "It compiles — assigning a new value is fine",
                    "It does not compile — `x` is immutable, and there is no `let` to make a new binding",
                ],
                1,
                "Without `let`, `x = 6` is an assignment to the existing immutable binding, so it fails. \
                 With `let x = 6;` it would be a new binding, which always works. The keyword is doing the \
                 work.",
            ),
            code(
                "rust",
                r#"fn main() {
    // Mutation: tracking one quantity as it changes
    let mut health = 100;
    health -= 30;
    health -= 20;
    println!("health: {health}");  // 50

    // Shadowing: transforming representations
    let raw = "42";
    let raw = raw.len();
    println!("length: {raw}");     // 2
}"#,
            ),
            callout(
                "You will reach for both",
                "Mutation for state that moves; shadowing for a value changing form. Most real code uses \
                 both, and now you can tell which is which at a glance.",
            ),
        ],
    )
}

fn lesson_shadowing_changes_types() -> SeedLesson {
    SeedLesson::new(
        "Shadowing Can Change Types",
        vec![
            labelled(
                "THE IDEA",
                "One of Rust's genuinely pleasant surprises: shadowing lets the same name hold a different \
                 type. The value has moved to a new representation of the same idea, and you never needed \
                 to invent a second name.",
            ),
            code(
                "rust",
                r#"fn main() {
    let value = "42";
    let value: i32 = value.parse().unwrap();

    println!("{}", value + 1);  // 43 — it is an i32 now
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The first `value` is a `&str` holding the characters `4` and `2`. `.parse()` asks Rust to \
                 read those characters as a number, and `.unwrap()` says \"assume it works\" — proper \
                 error handling is Module 09, so for now we take the success for granted. The result is \
                 an `i32`, and the annotation on the new `let` says so.",
            ),
            broken(
                "fn main() {\n    let mut value = \"42\";\n    value = 42;\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The mutation version cannot do this. A mutable binding's type is fixed the moment you \
                 declare it, so `value = 42` fails with `expected &str, found integer`. Shadowing creates \
                 a new binding, which is free to have a new type. This is the single most useful thing to \
                 remember from the whole module.",
            ),
            code(
                "rust",
                r#"fn main() {
    // A realistic chain of transformations
    let input = "255";
    let input: u8 = input.parse().unwrap();   // now a small number
    let input = input as u32 * 2;             // widened and doubled
    println!("value={input}");                // value=510
}"#,
            ),
            callout(
                "The compiler suggests shadowing",
                "Try the mutation version and read the error. Rust's own message includes a `help:` line \
                 pointing you at a shadowing fix. The compiler is not just a gate — it is a teacher. Start \
                 reading those help lines.",
            ),
        ],
    )
}

fn lesson_conversions() -> SeedLesson {
    SeedLesson::new(
        "Numeric Type Conversions",
        vec![
            labelled(
                "THE IDEA",
                "Rust will not quietly convert between numeric types for you. To turn an `i32` into an \
                 `f64` you have to say so with `as`, every time.",
            ),
            code(
                "rust",
                r#"fn main() {
    let x: i32 = 10;
    let y: f64 = x as f64;

    println!("{y}");        // 10
    println!("{}", y / 4.0); // 2.5
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`as` makes a conversion explicit and visible. Widening — a small integer into a bigger \
                 one, or an integer into a float — is safe and exact. Narrowing — a big number into a \
                 small type, or a float into an integer — can silently lose information, so Rust makes \
                 you write the `as` and take responsibility.",
            ),
            step_through(
                "What narrowing actually does",
                r#"let d: f64 = 3.99;
let e: i32 = d as i32;

let f: i32 = 300;
let g: u8 = f as u8;"#,
                &[
                    (&[1][..], "A float holding 3.99."),
                    (&[2][..], "`as i32` truncates toward zero — `e` is 3, not 4. It does not round."),
                    (&[3][..], "300 as a signed 32-bit number."),
                    (&[4][..], "`as u8` keeps only the low byte, so `g` is 44 — information is lost, silently."),
                ],
            ),
            callout(
                "Different types are different types",
                "Casting is always visible, always intentional, and sometimes lossy. When you see `as` in \
                 real code, someone made a decision. That is a good thing.",
            ),
        ],
    )
}

fn lesson_type_compatibility() -> SeedLesson {
    SeedLesson::new(
        "Type Compatibility",
        vec![
            labelled(
                "THE IDEA",
                "Two integer types do not mix just because both hold integers. `i32` and `i64` are \
                 different types, and Rust will not add them for you.",
            ),
            broken(
                "fn main() {\n    let x: i32 = 10;\n    let y: i64 = 20;\n\n    let total = x + y;\n}",
            ),
            broken_text(
                "error[E0308]: mismatched types\n  --> src/main.rs:4:18\n   |\n4  |     let total = x + y;\n   |                  ^   expected `i32`, found `i64`",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Read the error from the inside out. The caret points at `y`. Rust expected `i32`, the \
                 type of the left side of `+`, and found `i64`, the type of `y`. It cannot decide which \
                 one you meant, so it stops and asks rather than guessing.",
            ),
            fixed(
                "fn main() {\n    let a: i32 = 10;\n    let b: i64 = 20;\n\n    // Make them the same type, or cast the narrower one up.\n    let sum = a as i64 + b;\n\n    println!(\"{sum}\");\n}",
            ),
            text_block(
                "When you never write an annotation, Rust picks `i32` for both and everything just works. \
                 Type annotations are often what create the mismatch in the first place — not a problem, \
                 just something to be aware of.",
            ),
            callout(
                "Pick a direction",
                "Either make both sides the same type up front, or cast the narrower one up with `as`. The \
                 cast is more honest when the two types are genuinely different in the rest of your \
                 program.",
            ),
        ],
    )
}

fn lesson_expressions() -> SeedLesson {
    SeedLesson::new(
        "Expressions Produce Values",
        vec![
            labelled(
                "THE IDEA",
                "In many languages a line of code is either a value or an action. In Rust almost \
                 everything produces a value, including a block of code.",
            ),
            code(
                "rust",
                r#"fn main() {
    let x = 10 + 20;              // an expression producing 30

    let y = {
        let a = 10;
        let b = 20;
        a + b                    // no semicolon — this is the block's value
    };

    println!("{x} {y}");  // 30 30
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The rule that makes the block work: the final expression in a block, with no semicolon, is \
                 the block's value. Put a semicolon after `a + b` and the block instead produces `()`, the \
                 **unit** type — a value that means \"nothing\". One character is the entire difference.",
            ),
            breakdown(
                "Where a block's value comes from",
                "let y = {\n    let a = 10;\n    let b = 20;\n    a + b\n};",
                &[
                    "The block opens; `a` and `b` are ordinary statements.",
                    "`a + b` has no semicolon, so it is the block's value.",
                    "`y` receives that value (30).",
                ],
            ),
            callout(
                "This is the seed of control flow",
                "`if` produces a value. `match` produces a value. Once blocks evaluate to things, control \
                 flow stops feeling like a special case and starts feeling like more expression syntax — \
                 which is exactly where Module 03 begins.",
            ),
        ],
    )
}

fn lesson_reading_errors() -> SeedLesson {
    SeedLesson::new(
        "Reading Type Errors",
        vec![
            labelled(
                "THE IDEA",
                "Compiler errors are not rejection notices. They are the closest thing Rust has to a \
                 patient engineer sitting next to you. Learn their shape and they stop being walls.",
            ),
            broken("fn main() {\n    let age: i32 = \"25\";\n}"),
            broken_text(
                "error[E0308]: mismatched types\n --> src/main.rs:2:20\n  |\n2 |     let age: i32 = \"25\";\n  |                    ^^^^   expected `i32`, found `&str`\n  |                    |\n  |                    expected because of this annotation\n  |                    found because of this expression",
            ),
            steps(&[
                "`expected i32` — what Rust believed you wanted, from your annotation.",
                "`found &str` — what your value actually is.",
                "`expected because of this annotation` — the line that created the expectation.",
                "`found because of this expression` — the line that produced the real value.",
            ]),
            labelled(
                "LET'S BREAK IT DOWN",
                "Every `mismatched types` error has the same two-part shape: what Rust expected, and what \
                 it found. The extra lines point at where each belief came from. Scan for those two words \
                 and the error takes about three seconds instead of three minutes.",
            ),
            table(
                &["Error", "What it means", "The fix"],
                &[
                    &["`expected i32, found &str`", "text where a number belongs", "parse it, or change the annotation"],
                    &["`expected u8, found u32`", "close but not identical types", "cast with `as`, or match the types"],
                    &["`cannot assign twice to immutable variable`", "you reassigned a `let`", "add `mut`"],
                ],
            ),
            callout(
                "Read `expected` and `found` first",
                "The `help:` line at the bottom is useful, but only once you know what went wrong. The two \
                 types tell you that.",
            ),
        ],
    )
}

fn lesson_common_mistakes() -> SeedLesson {
    SeedLesson::new(
        "Things You Will Probably Break",
        vec![
            labelled(
                "THE IDEA",
                "Six mistakes account for most of what beginners hit in their first week. None of them are \
                 stupid — all of them have a shape, and once you see the shape you stop making them.",
            ),
            table(
                &["#", "Mistake", "Cause", "Fix"],
                &[
                    &["1", "Reassign an immutable variable", "`let`, not `let mut`", "add `mut`"],
                    &["2", "Add two different number types", "`i32` + `i64` are distinct", "cast with `as`, or match types"],
                    &["3", "Use `\"A\"` where a `char` is needed", "double quotes make `&str`", "single quotes: `'A'`"],
                    &["4", "Index an array out of range", "indexing is checked at runtime", "stay inside `0..len`"],
                    &["5", "Change a value's type by mutation", "a binding's type is fixed", "shadow with a new `let`"],
                    &["6", "Forget an annotation on an empty list", "nothing to infer from", "annotate the element type"],
                ],
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Two of these are worth a closer look. Out-of-range indexing (4) is the one mistake that is \
                 *not* caught at compile time — it compiles and then panics when it runs. And changing type \
                 by mutation (5) is impossible by design; shadowing is the tool, which is why it got its \
                 own lesson.",
            ),
            broken("fn main() {\n    let score = 10;\n    score = 20;\n}"),
            fixed("fn main() {\n    let mut score = 10;\n    score = 20;\n    println!(\"{score}\");\n}"),
            callout(
                "Use this as a checklist",
                "When something will not compile, find which of these six it is. Most of the time the \
                 answer is here, and the fix is the last column.",
            ),
        ],
    )
}

fn lesson_bridge() -> SeedLesson {
    SeedLesson::new(
        "Putting It All Together",
        vec![
            labelled(
                "THE IDEA",
                "You now have the vocabulary. Variables store information, types describe it, `mut` controls \
                 change, shadowing transforms, and conversions move between representations. Time to use \
                 all of it at once — and then notice what is still missing.",
            ),
            code(
                "rust",
                r#"const MAX_PLAYERS: usize = 4;

fn main() {
    // Immutable facts about the game
    let game_title: &str = "Dungeon Run";
    let starting_health: u16 = 100;

    // Mutable state that changes
    let mut players: usize = 0;
    let mut high_score: u32 = 0;

    // Shadowing to transform representations
    let score_input = "1500";
    let score_input: u32 = score_input.parse().unwrap();
    let score_input = score_input as i64 * 2;

    // A tuple for data that travels together
    let player = ("Gus", starting_health, true);

    // An array for a fixed collection
    let party: [&str; 4] = ["Gus", "Ada", "Lin", "Sam"];

    players = party.len();
    high_score = score_input as u32;

    println!("=== {game_title} ===");
    println!("Players: {players} / {MAX_PLAYERS}");
    println!("Player 1: {} ({} HP, alive: {})", player.0, player.1, player.2);
    println!("Party: {party:?}");
    println!("High score: {high_score}");
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "That program uses a constant, immutable and mutable bindings, a tuple, an array, shadowing \
                 across a type change, an `as` conversion, and a `.parse()`. None of it is advanced; all of \
                 it is this module's vocabulary. Read it and notice that every choice had a reason.",
            ),
            checklist(
                "You can now do this",
                &[
                    "Create a binding with `let`, and say what a name, a value, and a type are",
                    "Explain why bindings are immutable by default, and add `mut` when a value changes",
                    "Let Rust infer a type, and add an annotation when it cannot",
                    "Read and write text (`&str`) and single characters (`char`)",
                    "Choose an integer or float type on purpose and state its limits",
                    "Use booleans, tuples, arrays, and constants",
                    "Shadow a binding, and tell shadowing from mutation",
                    "Convert between numeric types with `as`",
                    "Read a `mismatched types` error and name the expected and found types",
                ],
            ),
            paragraphs(&[
                "Right now your game stores information and does exactly the same thing every run. What if \
                 health reaches zero? What if the score passes 100? What if you need to repeat something \
                 fifty times?",
                "That is where control flow comes in — `if`, `match`, `loop`, `while`, `for`. Every one of \
                 those tools also produces a value, which is exactly why the expressions lesson was worth \
                 sitting through.",
            ]),
            callout(
                "On to control flow",
                "The next module is about making decisions and repeating work. You have all the data; now \
                 you make it move.",
            ),
        ],
    )
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
                "So the code communicates what the values mean",
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
            "`+= 3` is shorthand for `count = count + 3`. 5 + 3 = 8.",
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
            "A block reads a value declared without `mut`, then tries to change it. What happens?",
            &["The change is allowed", "It does not compile — the binding is immutable", "It panics", "It works once"],
            1,
            "A nested block does not grant new powers. If the binding was declared without `mut`, nothing may reassign it.",
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
                "let c = Vec::new();",
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

        // L7 — text &str
        SeedExercise::mcq(
            6,
            "What is `\"hello\"`?",
            &["A char", "A &str (string slice)", "A tuple", "An integer"],
            1,
            "Double quotes make a string slice, `&str`, which can hold any number of characters. A char uses single quotes: `'h'`.",
        ),
        SeedExercise::mcq(
            6,
            "Which of these is a `char`?",
            &["\"R\"", "'R'", "\"Rust\"", "'Rust'"],
            1,
            "`'R'` is a char — one character in single quotes. `\"R\"` is a &str; `\"Rust\"` is a longer &str; `'Rust'` is not valid syntax at all.",
        )
        .kind(ExerciseKind::PredictOutput)
        .with_code("fn main() {\n    let letter: char = 'A';\n    let word: &str = \"A\";\n    println!(\"{letter} {word}\");\n}")
        .detached(),

        // L8 — integer types
        SeedExercise::mcq(
            7,
            "What is the maximum value a `u8` can hold?",
            &["127", "255", "256", "65535"],
            1,
            "u8 is unsigned, so it starts at 0. 8 bits give 256 distinct values: 0 through 255. i8 would be -128 through 127.",
        ),
        SeedExercise::mcq(
            7,
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
            7,
            "What is the largest value `i8` can hold?",
            &["0", "127", "128", "255"],
            1,
            "i8 is signed 8-bit: -128 to 127. Half the range goes to negatives, so the positive side stops at 127.",
        ),

        // L9 — literals
        SeedExercise::mcq(
            8,
            "What is `0b1010` equal to in decimal?",
            &["10", "8", "12", "20"],
            0,
            "Binary reads right-to-left: 8 + 2 = 10. The `0b` prefix marks it as binary.",
        ),
        SeedExercise::mcq(
            8,
            "What is `0o755` equal to in decimal?",
            &["755", "493", "435", "1000"],
            1,
            "Octal: 7×64 + 5×8 + 5 = 448 + 40 + 5 = 493. You will see this constantly in file permissions.",
        ),
        SeedExercise::mcq(
            8,
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

        // L10 — signed vs unsigned
        SeedExercise::mcq(
            9,
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
            9,
            "Which of these should most likely be `usize`?",
            &["A player's level", "The number of items in a list", "A temperature", "A score change"],
            1,
            "usize is pointer-sized and unsigned — ideal for counts and lengths, which is exactly what indexing and a list's `.len()` use.",
        ),

        // L11 — floats
        SeedExercise::mcq(
            10,
            "What type does Rust infer for `let price = 19.99;`?",
            &["f32", "f64", "i32", "f128"],
            1,
            "Floating-point literals with no other context default to f64. It is the safer default — more precision.",
        ),
        SeedExercise::mcq(
            10,
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
            10,
            "What does `3.99 as i32` produce?",
            &["4", "3", "0", "A compile error"],
            1,
            "`as` truncates toward zero — it does not round. 3.99 becomes 3. This surprises people regularly.",
        ),

        // L12 — booleans
        SeedExercise::mcq(
            11,
            "How much memory does a `bool` occupy?",
            &["0 bytes", "1 byte", "4 bytes", "8 bytes"],
            1,
            "A bool has exactly two values, so one byte is enough. It is the smallest type in Rust.",
        ),
        SeedExercise::mcq(
            11,
            "Which of these is a valid `bool` literal?",
            &["True", "true", "TRUE", "1"],
            1,
            "Rust is lowercase: `true` and `false`. `True` is an undefined name and will not compile.",
        ),

        // L13 — char
        SeedExercise::mcq(
            12,
            "How many bytes does a Rust `char` always occupy?",
            &["1", "2", "4", "It varies"],
            2,
            "Always 4. A char is a Unicode scalar value, so it needs a fixed width regardless of how many UTF-8 bytes the glyph itself takes.",
        ),
        SeedExercise::mcq(
            12,
            "What is the difference between 'A' and \"A\"?",
            &[
                "They are identical",
                "'A' is a char; \"A\" is a &str",
                "'A' is ASCII; \"A\" is Unicode",
                "Double quotes are deprecated",
            ],
            1,
            "Single quotes hold exactly one character. Double quotes hold a string slice — a whole run of text. Different types entirely.",
        )
        .kind(ExerciseKind::TypeDetective)
        .with_code("fn main() {\n    let a = 'x';\n    let b = \"x\";\n    let c = '🥷';\n}")
        .with_hints(&["Single quotes → char", "Double quotes → &str"]),

        // L14 — tuples
        SeedExercise::mcq(
            13,
            "How do you access the second element of `let pair = (10, 20);`?",
            &["pair[1]", "pair.1", "pair.2", "pair.get(1)"],
            1,
            "Tuples use dot notation with a numeric field: `.0` is first, `.1` is second. Indexing with square brackets is for arrays.",
        ),
        SeedExercise::mcq(
            13,
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

        // L15 — arrays
        SeedExercise::mcq(
            14,
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
            14,
            "What is the first element of `[10, 20, 30]`?",
            &["10", "20", "30", "There is no first element"],
            0,
            "Indexing is zero-based. `arr[0]` is the first, `arr[1]` is the second. This trips up everyone exactly once.",
        ),
        SeedExercise::mcq(
            14,
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

        // L16 — constants
        SeedExercise::mcq(
            15,
            "What does a `const` require that a `let` does not?",
            &["A value", "A type annotation", "A name in caps", "mut"],
            1,
            "Constants must state their type explicitly. `const MAX: u32 = 100;` — no inference. The compiler needs it before the program runs.",
        ),
        SeedExercise::mcq(
            15,
            "What is the conventional naming style for constants?",
            &["camelCase", "snake_case", "SCREAMING_SNAKE_CASE", "PascalCase"],
            2,
            "SCREAMING_SNAKE_CASE signals at a glance that this is a fixed named fact rather than a working value.",
        ),
        SeedExercise::mcq(
            15,
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

        // L17/L18/L19 — shadowing cluster
        SeedExercise::mcq(
            16,
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
            16,
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
            17,
            "In `let x = 5; let x = 6;` is the original value of x still stored anywhere?",
            &["No, it is overwritten", "Yes, it exists but is unreachable by that name", "No, it is deleted", "Yes, you can access it as x.0"],
            1,
            "Shadowing hides a binding; it does not destroy it. The old x still holds 5 until it goes out of scope.",
        ),
        SeedExercise::mcq(
            17,
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
            18,
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

        // L20 — conversions
        SeedExercise::mcq(
            19,
            "How do you convert an `i32` variable into an `f64`?",
            &["let y: f64 = x;", "let y: f64 = x as f64;", "convert(x, f64)", "let y = f64(x);"],
            1,
            "Rust never converts implicitly. `as` makes the conversion explicit and visible in the source.",
        ),
        SeedExercise::mcq(
            19,
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

        // L21 — compatibility
        SeedExercise::mcq(
            20,
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

        // L22 — expressions
        SeedExercise::mcq(
            21,
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
            21,
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

        // L23 — reading errors
        SeedExercise::mcq(
            22,
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

        // L24 — common mistakes
        SeedExercise::mcq(
            23,
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
        .with_code("let empty = Vec::new();")
        .with_code_b("let empty: Vec<i32> = Vec::new();")
        .with_hints(&["Which version tells Rust what the list holds?"]),
        SeedExercise::mcq(
            23,
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

        // L25 — bridge
        SeedExercise::mcq(
            24,
            "You have `let name = \"Gus\";` and want a numeric id. What is the idiomatic approach?",
            &[
                "Make name mutable and assign an integer",
                "Shadow name with a new binding of the new type",
                "Use a cast: name as i32",
                "You cannot change a variable's type in Rust",
            ],
            1,
            "Shadowing. `let name: i32 = name.parse().unwrap();` creates a new binding with a new type. Mutation cannot do this.",
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
