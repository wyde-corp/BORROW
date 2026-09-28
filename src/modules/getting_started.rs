use super::*;

pub fn module() -> Module {
    Module {
        number: "01",
        title: "Getting Started",
        lessons: vec![
            Lesson {
                title: "Cargo".into(),
                content: vec![
                    LessonBlock::Text("Cargo is Rust's build system and package manager. It handles downloading dependencies, compiling your code, and running tests.".into()),
                    LessonBlock::Heading("Creating a Project".into()),
                    LessonBlock::Code("cargo new my_project\ncd my_project".into()),
                    LessonBlock::Text("This creates a new directory with a Cargo.toml file (project manifest) and a src/main.rs file.".into()),
                    LessonBlock::Heading("Building and Running".into()),
                    LessonBlock::Code("cargo build    # Compile\ncargo run      # Compile and run\ncargo test     # Run tests".into()),
                    LessonBlock::Tip("Use `cargo check` for a fast syntax check without producing a binary.".into()),
                    LessonBlock::Exercise(Exercise::MultipleChoice {
                        question: "What command creates a new Rust project?".into(),
                        options: vec![
                            "cargo new my_project".into(),
                            "cargo create my_project".into(),
                            "cargo init my_project".into(),
                            "cargo build my_project".into(),
                        ],
                        correct: 0,
                    }),
                    LessonBlock::Exercise(Exercise::MultipleChoice {
                        question: "What command compiles and runs your project?".into(),
                        options: vec![
                            "cargo build".into(),
                            "cargo run".into(),
                            "cargo check".into(),
                            "cargo test".into(),
                        ],
                        correct: 1,
                    }),
                ],
                starter_code: "fn main() {\n    println!(\"Hello, cargo!\");\n}\n".into(),
                project: Some(Project {
                    title: "Hello Cargo".into(),
                    description: "Create a new cargo project and print your name.".into(),
                    requirements: vec![
                        "Run `cargo new hello_cargo`".into(),
                        "Modify main.rs to print your name".into(),
                        "Run `cargo run` and verify output".into(),
                    ],
                }),
            },
            Lesson {
                title: "main()".into(),
                content: vec![
                    LessonBlock::Text("Every Rust executable starts with a `main` function. This is the entry point of your program.".into()),
                    LessonBlock::Code("fn main() {\n    // Your code here\n}".into()),
                    LessonBlock::Text("The `fn` keyword declares a function. `main` takes no parameters and returns nothing (implicitly `()`).".into()),
                    LessonBlock::Heading("println!".into()),
                    LessonBlock::Code("println!(\"Hello, world!\");\nprintln!(\"I'm a Rustacean!\");".into()),
                    LessonBlock::Text("`println!` is a macro (note the `!`). It prints to stdout with a newline.".into()),
                    LessonBlock::Exercise(Exercise::PredictOutput {
                        code: "fn main() {\n    println!(\"Hello, world!\");\n}".into(),
                        options: vec![
                            "Hello, world!".into(),
                            "Hello world".into(),
                            "println!(\"Hello, world!\")".into(),
                            "Nothing".into(),
                        ],
                        correct: 0,
                    }),
                    LessonBlock::Exercise(Exercise::MultipleChoice {
                        question: "What does the `!` in `println!` indicate?".into(),
                        options: vec![
                            "It is a macro, not a regular function".into(),
                            "It is a reference operator".into(),
                            "It is a dereference operator".into(),
                            "It is a mutable borrow".into(),
                        ],
                        correct: 0,
                    }),
                ],
                starter_code: "fn main() {\n    println!(\"Hello from main!\");\n}\n".into(),
                project: Some(Project {
                    title: "Greeting Program".into(),
                    description: "Write a program that prints a multi-line greeting.".into(),
                    requirements: vec![
                        "Use at least 3 println! statements".into(),
                        "Include your name and today's date".into(),
                        "Add a border of asterisks around the greeting".into(),
                    ],
                }),
            },
            Lesson {
                title: "Variables".into(),
                content: vec![
                    LessonBlock::Text("Variables in Rust are immutable by default. Use `let` to declare one.".into()),
                    LessonBlock::Code("let x = 5;\nlet name = \"Rust\";\nlet pi = 3.14159;".into()),
                    LessonBlock::Text("Rust uses type inference — you rarely need to annotate types explicitly.".into()),
                    LessonBlock::Heading("Type Annotations".into()),
                    LessonBlock::Code("let x: i32 = 5;\nlet name: &str = \"Rust\";\nlet pi: f64 = 3.14159;".into()),
                    LessonBlock::Tip("Use `let` for constants too — Rust doesn't have a separate `const` keyword for local bindings (though `const` exists for compile-time constants).".into()),
                    LessonBlock::Exercise(Exercise::MultipleChoice {
                        question: "How do you create a variable in Rust?".into(),
                        options: vec![
                            "let x = 5;".into(),
                            "var x = 5;".into(),
                            "int x = 5;".into(),
                            "x := 5;".into(),
                        ],
                        correct: 0,
                    }),
                    LessonBlock::Exercise(Exercise::PredictOutput {
                        code: "let x = 5;\nlet x = x + 1;\nprintln!(\"{x}\");".into(),
                        options: vec![
                            "5".into(),
                            "6".into(),
                            "x + 1".into(),
                            "Error".into(),
                        ],
                        correct: 1,
                    }),
                ],
                starter_code: "fn main() {\n    let x = 5;\n    let name = \"Rust\";\n    println!(\"x = {}, name = {}\", x, name);\n}\n".into(),
                project: Some(Project {
                    title: "Variable Explorer".into(),
                    description: "Create variables of different types and print them.".into(),
                    requirements: vec![
                        "Declare at least 4 variables with explicit type annotations".into(),
                        "Use i32, f64, bool, and &str".into(),
                        "Print all variables using println!".into(),
                    ],
                }),
            },
            Lesson {
                title: "Mutability".into(),
                content: vec![
                    LessonBlock::Text("To make a variable mutable, add `mut` after `let`.".into()),
                    LessonBlock::Code("let mut count = 0;\ncount += 1;\nprintln!(\"Count: {}\", count);".into()),
                    LessonBlock::Text("Without `mut`, trying to modify a variable is a compile-time error.".into()),
                    LessonBlock::Heading("Constants".into()),
                    LessonBlock::Code("const MAX_POINTS: u32 = 100_000;".into()),
                    LessonBlock::Text("Constants are always immutable and must have a type annotation. They're inlined at compile time.".into()),
                    LessonBlock::Exercise(Exercise::FixCode {
                        broken_code: "let x = 5;\nx = 10;\nprintln!(\"{x}\");".into(),
                        fixed_code: "let mut x = 5;\nx = 10;\nprintln!(\"{x}\");".into(),
                        hint: "Add `mut` to make x mutable".into(),
                    }),
                    LessonBlock::Exercise(Exercise::MultipleChoice {
                        question: "What is the difference between shadowing and mutation?".into(),
                        options: vec![
                            "Shadowing creates a new variable; mutation changes an existing one".into(),
                            "They are the same thing".into(),
                            "Shadowing requires mut; mutation does not".into(),
                            "Mutation creates a new variable; shadowing changes an existing one".into(),
                        ],
                        correct: 0,
                    }),
                ],
                starter_code: "fn main() {\n    let mut count = 0;\n    count += 1;\n    count += 1;\n    println!(\"Count: {}\", count);\n}\n".into(),
                project: Some(Project {
                    title: "Counter".into(),
                    description: "Build a simple counter that increments and prints.".into(),
                    requirements: vec![
                        "Start with a mutable counter at 0".into(),
                        "Increment it 5 times in a loop".into(),
                        "Print the final value".into(),
                    ],
                }),
            },
            Lesson {
                title: "Basic Types".into(),
                content: vec![
                    LessonBlock::Text("Rust has two data type subsets: scalar and compound.".into()),
                    LessonBlock::Heading("Scalar Types".into()),
                    LessonBlock::Code("let a: i32 = -42;        // Signed integer\nlet b: u32 = 100;         // Unsigned integer\nlet c: f64 = 3.14;        // Floating point\nlet d: bool = true;       // Boolean\nlet e: char = 'z';        // Unicode character".into()),
                    LessonBlock::Heading("Integer Types".into()),
                    LessonBlock::Code("i8, i16, i32, i64, i128, isize\nu8, u16, u32, u64, u128, usize".into()),
                    LessonBlock::Text("The default integer type is `i32`. The default float type is `f64`.".into()),
                    LessonBlock::Heading("Tuples".into()),
                    LessonBlock::Code("let tup: (i32, f64, bool) = (500, 6.4, true);\nlet (x, y, z) = tup;".into()),
                    LessonBlock::Heading("Arrays".into()),
                    LessonBlock::Code("let arr = [1, 2, 3, 4, 5];\nlet first = arr[0];".into()),
                    LessonBlock::Exercise(Exercise::MultipleChoice {
                        question: "What is the default integer type in Rust?".into(),
                        options: vec![
                            "i32".into(),
                            "i64".into(),
                            "u32".into(),
                            "isize".into(),
                        ],
                        correct: 0,
                    }),
                    LessonBlock::Exercise(Exercise::MultipleChoice {
                        question: "How do you access the first element of an array?".into(),
                        options: vec![
                            "arr[0]".into(),
                            "arr[1]".into(),
                            "arr.0".into(),
                            "arr.first()".into(),
                        ],
                        correct: 0,
                    }),
                    LessonBlock::Exercise(Exercise::PredictOutput {
                        code: "let tup = (10, 20, 30);\nprintln!(\"{}\", tup.1);".into(),
                        options: vec![
                            "10".into(),
                            "20".into(),
                            "30".into(),
                            "Error".into(),
                        ],
                        correct: 1,
                    }),
                ],
                starter_code: "fn main() {\n    let a: i32 = -42;\n    let b: f64 = 3.14;\n    let c: bool = true;\n    let d: char = 'x';\n    println!(\"{} {} {} {}\", a, b, c, d);\n}\n".into(),
                project: Some(Project {
                    title: "Type Showcase".into(),
                    description: "Create variables of every basic type and print them.".into(),
                    requirements: vec![
                        "Declare one variable of each scalar type".into(),
                        "Create a tuple and destructure it".into(),
                        "Create an array and access elements by index".into(),
                    ],
                }),
            },
        ],
    }
}
