use super::types::{
    callout, code, text, Difficulty, ExerciseKind, SeedExercise, SeedLesson, SeedModule,
    SeedProject,
};

pub fn content() -> SeedModule {
    SeedModule::new(
        "getting-started",
        "Getting Started",
        "Install Rust, write your first program, and understand the basics of the language.",
        vec![
            SeedLesson {
                title: "Installing Rust with rustup".to_string(),
                content: serde_json::json!({
                    "sections": [
                        text("CONCEPT — Rust is installed via rustup, a cross-platform toolchain manager that handles the Rust compiler (rustc), Cargo (the package manager), and supporting tools. One installer works on Linux, macOS, and Windows."),
                        code("bash", "# Linux / macOS — one-liner installer\ncurl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh\n\n# Or with package managers:\n# Ubuntu / Debian:\nsudo apt install rustc cargo\n\n# Fedora / RHEL:\nsudo dnf install rust cargo\n\n# macOS with Homebrew:\nbrew install rustup\nrustup-init"),
                        code("powershell", "# Windows — download and run the installer:\n# https://rustup.rs\n# Or use winget:\nwinget install Rustlang.Rustup\n\n# Or use Chocolatey:\nchoco install rustup.install"),
                        text("EXPLANATION — After installation you get three essential tools in your terminal. rustc is the Rust compiler that turns your code into a binary. cargo is the build tool and package manager that handles compiling, running, testing, and dependencies. rustup manages multiple Rust versions and updates. You can update at any time with rustup update. To verify everything works, run cargo --version, rustc --version, and rustup --version."),
                        code("bash", "# Verify installation\ncargo --version    # cargo 1.xx.x\nrustc --version    # rustc 1.xx.x\nrustup --version   # rustup x.x.x\n\n# Update everything\nrustup update"),
                        callout("Practice — Try this now", "Open your terminal and run cargo --version. You should see the version number. If you get 'command not found', your terminal needs to source the environment: run source $HOME/.cargo/env (Linux/macOS) or restart your terminal (Windows)."),
                        code("bash", "# If cargo is not found on Linux/macOS:\nsource $HOME/.cargo/env\n\n# Verify it works now\ncargo --version"),
                        callout("Mini challenge", "Run cargo --help and explore the output. Find the command that creates a new project. Then create a project called 'hello_rust' and run it."),
                        callout("Project — Setup your environment", "Install Rust on your machine. Create a new project called 'my_journal'. Add a second project called 'hello_world'. Run both. You now have a working Rust environment — every module from here on will use it.")
                    ]
                }),
            },
            SeedLesson {
                title: "main() — your first program".to_string(),
                content: serde_json::json!({
                    "sections": [
                        text("CONCEPT — Every Rust program starts execution from a function called main(). It is the entry point — the first code that runs when you execute your program. Without main(), cargo build will produce an error because there is no starting point."),
                        code("rust", "fn main() {\n    println!(\"Hello, world!\");\n}"),
                        text("EXPLANATION — fn is the keyword for defining a function. main is the reserved name for the entry point. The parentheses () hold parameters (none here). The curly braces { } wrap the body. println! is a macro (note the !) that prints text to standard output. Each statement ends with a semicolon. The string \"Hello, world!\" is an argument passed to the macro."),
                        code("rust", "// Multiple statements in main\nfn main() {\n    let name = \"Rust\";\n    println!(\"Hello, {}!\", name);\n    println!(\"Welcome to your first program.\");\n}"),
                        callout("Practice — Try this now", "Replace the contents of src/main.rs with fn main() { println!(\"I am learning Rust!\"); }. Run cargo run. Then change the text and run again. Add a second println! below the first one."),
                        code("rust", "fn main() {\n    println!(\"I am learning Rust!\");\n    println!(\"This is my second line.\");\n}"),
                        callout("Mini challenge", "Write a program that prints your name on one line and your favorite programming language on the next. Use two separate println! statements. Run it with cargo run."),
                        callout("Project — Your first binary", "Create a program called 'greeter' that asks for the user's name with println! and then prints a personalized greeting like 'Hello, [name]! Welcome to Rust.' Use a variable to store the name.")
                    ]
                }),
            },
            SeedLesson {
                title: "Cargo and project structure".to_string(),
                content: serde_json::json!({
                    "sections": [
                        text("CONCEPT — Cargo is Rust's build system and package manager combined. It creates project structure, compiles code, manages dependencies, runs tests, and builds releases. Every Rust project has a Cargo.toml manifest file and a src/ directory with your source code."),
                        code("bash", "cargo new my_project\n# Creates:\n# my_project/\n# ├── Cargo.toml    <- project metadata + dependencies\n# └── src/\n#     └── main.rs    <- entry point"),
                        text("EXPLANATION — Cargo.toml uses TOML format. The [package] section defines your project name, version, and edition. The [dependencies] section lists external crates (libraries). src/main.rs is the default entry point. cargo build compiles in debug mode (fast compile, slower run). cargo build --release creates an optimized binary in target/release/. cargo check quickly checks for errors without producing a binary."),
                        code("toml", "# Cargo.toml\n[package]\nname = \"my_project\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n# Add dependencies here, e.g.:\n# rand = \"0.8\""),
                        callout("Practice — Try this now", "Run cargo new practice_project. Open the Cargo.toml and src/main.rs files. Read them. Then run cargo run inside the directory. Now run cargo check — notice how much faster it is than cargo run."),
                        code("bash", "# Essential cargo commands\ncargo new <name>      # create new project\ncargo build           # compile (debug)\ncargo build --release # compile (optimized)\ncargo run             # compile + execute\ncargo test            # run tests\ncargo check           # quick syntax/type check\ncargo add <crate>     # add dependency"),
                        callout("Mini challenge", "Create a project called 'calculator'. Add a [dependencies] section to Cargo.toml (you do not need to add any actual dependency — just create the section). Run cargo build. Then run cargo check and compare the output."),
                        callout("Project — Multi-file project", "Create a project called 'modular'. Move your main logic from src/main.rs into a new file src/utils.rs. Declare the module in main.rs with `mod utils;`. Use a function from utils in main. Run cargo run to see it working.")
                    ]
                }),
            },
            SeedLesson {
                title: "Variables and mutability".to_string(),
                content: serde_json::json!({
                    "sections": [
                        text("CONCEPT — In Rust, variables are immutable by default. Once you bind a value to a name, you cannot change it. This is a core safety feature — it prevents accidental modifications. To make a variable mutable, you explicitly opt in with the mut keyword."),
                        code("rust", "fn main() {\n    let x = 5;\n    // x = 6;  <- COMPILE ERROR: cannot assign twice\n\n    let mut y = 5;\n    y = 6;    <- allowed\n    println!(\"y = {}\", y);\n}"),
                        text("EXPLANATION — The let keyword creates a new binding. Without mut, the binding is immutable — you can read but not write. With mut, the binding is mutable — you can reassign. The compiler enforces this at compile time with error[E0384]: cannot assign twice to immutable variable. You can also shadow a variable by declaring a new variable with the same name, which is different from mutation."),
                        code("rust", "fn main() {\n    // Shadowing\n    let x = 5;\n    let x = x + 1;   // new binding, shadows the old one\n    let x = x * 2;\n    println!(\"x = {}\", x);  // prints 12\n\n    // Mutability\n    let mut count = 0;\n    count += 1;\n    count += 1;\n    println!(\"count = {}\", count);  // prints 2\n}"),
                        callout("Practice — Try this now", "Write a program with an immutable variable let age = 25. Try to change it — observe the compile error. Then add mut and try again. Use shadowing to transform a value through three steps."),
                        code("rust", "fn main() {\n    let mut score = 0;\n    println!(\"Start: {}\", score);\n    score += 10;\n    println!(\"After +10: {}\", score);\n    score *= 2;\n    println!(\"After *2: {}\", score);\n}"),
                        callout("Mini challenge", "Write a program that converts a temperature from Fahrenheit to Celsius. Use immutable variables for the input and formula steps. The formula is: celsius = (fahrenheit - 32) * 5/9. Try both i32 and f64 for the values."),
                        callout("Project — Interactive counter", "Write a program that simulates a simple counter. Start with a count of 0. Use mut to increment it five times, printing the value after each increment. Then use shadowing to reset it to 0 and print 'Counter reset.'")
                    ]
                }),
            },
            SeedLesson {
                title: "Basic types".to_string(),
                content: serde_json::json!({
                    "sections": [
                        text("CONCEPT — Rust is statically typed: every variable has a known type at compile time. However, Rust uses type inference — you often do not need to write types explicitly because the compiler figures them out. This lesson covers scalar types: integers, floats, booleans, and characters."),
                        code("rust", "fn main() {\n    let age: u32 = 25;           // unsigned 32-bit integer\n    let temperature: f64 = 98.6; // 64-bit float\n    let is_learning: bool = true; // boolean\n    let grade: char = 'A';       // single Unicode char\n\n    println!(\"age = {}, temp = {}, learning = {}, grade = {}\",\n             age, temperature, is_learning, grade);\n}"),
                        text("EXPLANATION — Integers come in signed (i8, i16, i32, i64, i128, isize) and unsigned (u8, u16, u32, u64, u128, usize) variants. The number indicates bit size. i32 is the default. Floats are f32 or f64 (default). Booleans are true or false (1 byte). Chars are 4 bytes and hold any Unicode value — use single quotes. Rust errors if a value does not fit the type: let x: u8 = 300; fails because u8 max is 255. Type annotations are optional when the type is obvious: let x = 5; infers i32."),
                        code("rust", "fn main() {\n    // Integer types\n    let small: u8 = 255;\n    let big: u64 = 1_000_000;\n\n    // Float types\n    let pi: f64 = 3.14159;\n\n    // Boolean\n    let is_ready: bool = true;\n\n    // Character\n    let heart: char = '\\u{2764}';\n\n    // Type inference\n    let inferred = 42;         // i32\n    let inferred_float = 3.14; // f64\n}"),
                        callout("Practice — Try this now", "Declare variables of each type: u8, i32, f64, bool, char. Print them all. Try assigning a negative number to a u8 — read the error message. Try assigning 300 to a u8 — read the error. These errors are helpful."),
                        code("rust", "fn main() {\n    let a: u8 = 10;\n    let b: i32 = -50;\n    let c: f64 = 3.14;\n    let d: bool = false;\n    let e: char = 'Z';\n\n    println!(\"u8: {}, i32: {}, f64: {}, bool: {}, char: {}\",\n             a, b, c, d, e);\n}"),
                        callout("Mini challenge", "Write a program that declares a variable for each of these: your age (u8), the year (u16), your height in meters (f32), whether you like Rust (bool), and the first letter of your name (char). Print them all in one formatted output."),
                        callout("Project — Personal card", "Write a program that prints a personal information card. Use variables for name (&str), age (u8), height (f64), is_student (bool), and grade (char). Format the output as a nice card with borders made of dashes and pipes.")
                    ]
                }),
            },
        ],
        vec![
            SeedExercise::mcq(
                0,
                "Which command installs Rust on Linux/macOS?",
                &[
                    "rustup install",
                    "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh",
                    "cargo install rust",
                    "apt install rust",
                ],
                1,
                "rustup is the official toolchain manager. The curl command downloads and runs the rustup-init script which installs rustc, cargo, and rustup.",
            ),
            SeedExercise::mcq(
                0,
                "After installing Rust on Linux/macOS, cargo is not found. What should you do?",
                &[
                    "Reinstall Rust",
                    "Run source $HOME/.cargo/env",
                    "Use sudo cargo",
                    "Nothing, it always works",
                ],
                1,
                "The cargo binary is installed in ~/.cargo/bin. Sourcing ~/.cargo/env adds it to your PATH for the current terminal session.",
            ),
            SeedExercise::mcq(
                1,
                "What is the entry point of every Rust program?",
                &["fn start()", "fn main()", "fn init()", "fn begin()"],
                1,
                "Rust requires a function called main as the execution entry point. Without it, the compiler produces an error.",
            )
            .with_code("fn main() {\n    // ...\n}"),
            SeedExercise::mcq(
                1,
                "What does the ! in println! indicate?",
                &["It is a macro", "It is unsafe", "It panics on error", "It is deprecated"],
                0,
                "The ! suffix indicates that println! is a macro, not a regular function. Macros generate code at compile time.",
            ),
            SeedExercise::mcq(
                2,
                "Which command creates a new Rust project?",
                &["cargo init", "cargo new", "cargo create", "cargo build --new"],
                1,
                "cargo new <name> creates a new project with Cargo.toml, src/main.rs, and a .git directory.",
            ),
            SeedExercise::mcq(
                2,
                "What does cargo check do?",
                &[
                    "Runs the program",
                    "Checks for errors without producing a binary",
                    "Installs dependencies",
                    "Formats the code",
                ],
                1,
                "cargo check quickly verifies your code compiles correctly without producing a final binary. Much faster than cargo build.",
            ),
            SeedExercise::mcq(
                3,
                "What happens if you try to change a variable declared with let (without mut)?",
                &[
                    "It works fine",
                    "Runtime error",
                    "Compile error: cannot assign twice to immutable variable",
                    "It creates a new variable",
                ],
                2,
                "Rust enforces immutability at compile time. Trying to reassign an immutable variable produces error[E0384].",
            )
            .with_code("let x = 5;\nx = 6;"),
            SeedExercise::mcq(
                3,
                "What is the difference between mut and shadowing?",
                &[
                    "They are the same",
                    "mut modifies the existing binding; shadowing creates a new binding",
                    "mut is faster",
                    "Shadowing only works with integers",
                ],
                1,
                "mut allows you to change the value of an existing binding. Shadowing creates a new binding with the same name, hiding the old one.",
            ),
            SeedExercise::mcq(
                4,
                "What is the default integer type in Rust?",
                &["u32", "i32", "u8", "i64"],
                1,
                "When Rust cannot infer a specific integer type, it defaults to i32 — a 32-bit signed integer.",
            ),
            SeedExercise::mcq(
                4,
                "What is the maximum value of u8?",
                &["127", "255", "32767", "65535"],
                1,
                "u8 is an unsigned 8-bit integer: 0 to 255. u16 goes to 65535. i8 goes from -128 to 127.",
            ),
            SeedExercise::mcq(
                4,
                "Which type would you use for precise decimal calculations?",
                &["u32", "i64", "f64", "bool"],
                2,
                "f64 is a 64-bit floating-point type. Use f64 when you need decimal precision.",
            ),
            SeedExercise::mcq(
                0,
                "What will this code output?\n\nlet mut x = 10;\nx += 5;\nx *= 2;\nprintln!(\"{}\", x);",
                &["15", "20", "25", "30"],
                3,
                "x starts at 10, += 5 makes it 15, *= 2 makes it 30.",
            )
            .kind(ExerciseKind::PredictOutput)
            .with_code("fn main() {\n    let mut x = 10;\n    x += 5;\n    x *= 2;\n    println!(\"{}\", x);\n}")
            .detached()
            .xp(15),
        ],
        vec![SeedProject::new(
            "Personal Card Generator",
            "Build a program that prints a formatted personal information card using variables and println!.",
            Difficulty::Beginner,
            "15 min",
            &[
                "Use at least one variable of each type: u8, f64, bool, char",
                "Format output with borders using dashes and pipes",
                "Print at least 5 lines of personal info",
                "Use type inference where possible",
            ],
            &["variables", "types", "println!", "formatting"],
            &["All variables must be immutable", "No `mut` keyword anywhere in the file"],
            "+-------------------+\n|      RUST CARD     |\n+-------------------+\n| Age: 25 (u8)       |\n| Height: 1.8 (f64) |\n| Student: true      |\n| Grade: A (char)    |\n+-------------------+",
            &["Add a `const` for the border width", "Use a tuple to group the name and grade"],
        )],
    )
}
