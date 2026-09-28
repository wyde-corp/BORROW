# Module 01 — Getting Started

## Module Goal

Take a complete beginner from "I have never written Rust" to "I can create a Rust project, run it with Cargo, understand `main()`, create variables, understand mutability, and recognize Rust's basic data types."

## Prerequisites

- A computer with Rust installed (via `rustup`)
- A text editor
- No prior programming experience required

## Learning Outcomes

After this module, the learner will be able to:

1. Create a new Rust project with Cargo
2. Build and run a Rust program
3. Explain what `main()` does and why it is special
4. Create variables with `let`
5. Understand the difference between immutable and mutable variables
6. Use shadowing correctly
7. Recognize Rust's basic types: integers, floats, booleans, chars, tuples, arrays
8. Read and write simple type annotations
9. Understand type inference
10. Write and run a small Rust program from scratch

---

# 01 — Cargo

## Concepts

- What Cargo is
- Why Rust uses Cargo
- Creating projects with `cargo new` and `cargo init`
- Building with `cargo build`
- Running with `cargo run`
- Fast checking with `cargo check`
- Release builds with `cargo build --release`
- `Cargo.toml` — the project manifest
- `src/main.rs` — the default entry point
- Packages and crates
- Basic project structure
- Dependencies at an introductory level

## Explanation

Cargo is Rust's build system and package manager. It is the tool you will use for almost everything in Rust: creating projects, building code, running tests, and managing dependencies.

When you install Rust via `rustup`, Cargo is installed automatically. You can verify it is available by running:

```bash
cargo --version
```

### Why Cargo Exists

Many languages have separate tools for building, dependency management, and running. Rust combines these into one tool. This means you rarely need to configure a build system manually. You describe your project in a `Cargo.toml` file, and Cargo handles the rest.

### Creating a New Project

To create a new project:

```bash
cargo new my_project
```

This creates a directory called `my_project` with the following structure:

```
my_project/
├── Cargo.toml
└── src/
    └── main.rs
```

`Cargo.toml` is the project manifest. It contains metadata about your project: its name, version, edition, and dependencies.

`src/main.rs` is the default entry point. When you run your program, Rust starts executing code in this file.

### Initializing in an Existing Directory

If you already have a directory and want to turn it into a Rust project:

```bash
cd my_existing_directory
cargo init
```

This creates `Cargo.toml` and `src/main.rs` in the current directory without creating a subdirectory.

### Building and Running

To compile your project:

```bash
cargo build
```

This produces a executable binary in the `target/debug/` directory.

To compile and run in one step:

```bash
cargo run
```

This is the command you will use most often during development.

### Fast Syntax Checking

If you want to check your code without producing a binary:

```bash
cargo check
```

This is faster than `cargo build` because it skips the final compilation step. It is useful for quickly verifying that your code compiles.

### Release Builds

By default, `cargo build` creates a debug build. Debug builds compile faster but run slower. When you want an optimized build:

```bash
cargo build --release
```

This produces a binary in `target/release/` that runs faster but takes longer to compile.

### Cargo.toml

A minimal `Cargo.toml` looks like this:

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"
```

- `name` — the name of your package
- `version` — the current version
- `edition` — the Rust edition to use (2021 is current)

### Packages and Crates

A **crate** is a compilation unit in Rust. A binary crate produces an executable. A library crate produces code that other crates can use.

A **package** is one or more crates that are distributed together. When you run `cargo new`, you create a package that contains one binary crate.

The terms are related: a package contains crates. For now, you can think of "your project" as both a package and a crate.

### Dependencies

Dependencies are external crates your project uses. You add them to `Cargo.toml`:

```toml
[dependencies]
rand = "0.8"
```

Then run `cargo build` and Cargo will download and compile the dependency.

You do not need to worry about dependencies in this module. They are mentioned here so you understand what the `[dependencies]` section is for when you see it.

## Examples

### Example 1: Creating and Running a Project

```bash
cargo new hello_cargo
cd hello_cargo
cargo run
```

Output:

```
Hello, world!
```

### Example 2: Cargo.toml Contents

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
```

### Example 3: src/main.rs Default Contents

```rust
fn main() {
    println!("Hello, world!");
}
```

## Common Mistakes

**Forgetting to `cd` into the project directory.**
Cargo commands must be run from the project root (where `Cargo.toml` is located).

**Editing the wrong file.**
Your code goes in `src/main.rs`, not in `Cargo.toml` or a random file.

**Expecting `cargo run` to work without building.**
`cargo run` builds automatically, but if you have a syntax error, the build will fail and you will see a compiler error instead of program output.

**Typing `cargo build` in the wrong directory.**
If you see "could not find `Cargo.toml`", you are not in the project root.

## Practice Ideas

1. Create a new project called `my_first_rust_project`. Run it. Confirm you see "Hello, world!"
2. Modify the `println!` message to say your name. Run again.
3. Run `cargo check` after making a change. Notice it is faster than `cargo run`.
4. Run `cargo build --release` and find the binary in `target/release/`.
5. Open `Cargo.toml` and identify the name, version, and edition fields.

---

# 02 — main()

## Concepts

- The `fn` keyword
- What `main()` means
- Why `main()` is special
- Function bodies and `{}` blocks
- Statements vs expressions (basic level)
- `println!` and the meaning of `!`
- Semicolons
- Running a Rust program

## Explanation

Every Rust executable starts with a function called `main`. A function is a named block of code that can be executed. The `main` function is special because it is the entry point — it is the first code that runs when you execute your program.

### The `fn` Keyword

`fn` is how you declare a function in Rust. It is followed by the function name, parentheses for parameters, and curly braces for the body:

```rust
fn function_name() {
    // code goes here
}
```

### The `main` Function

```rust
fn main() {
    // your program starts here
}
```

The `main` function takes no parameters and returns nothing (implicitly). You do not need to write a return type for `main`.

### Function Body

The function body is the code inside the curly braces `{}`. This is where you write the instructions your program should execute.

### Statements and Expressions

A **statement** is an instruction that performs an action and does not return a value. Most lines in a function body are statements.

An **expression** evaluates to a value. For example, `5 + 3` is an expression that evaluates to `8`.

At a basic level: statements do things, expressions produce values. You will learn more about this distinction in later modules.

### The Semicolon

In Rust, statements end with a semicolon `;`. This tells the compiler "this statement is complete."

```rust
let x = 5;
println!("Hello");
```

Both lines above are statements and end with semicolons.

### `println!`

`println!` is a macro that prints text to the console. The `!` indicates that it is a macro, not a regular function. Macros are a more advanced topic, but for now you just need to know that `println!` prints text followed by a newline.

```rust
println!("Hello, world!");
```

The text inside the quotes is called a string literal. It is printed exactly as written.

You can also print values:

```rust
let name = "Gus";
println!("Hello, {name}!");
```

This prints: `Hello, Gus!`

The `{name}` syntax is called format string interpolation. It inserts the value of the variable into the string.

### Running a Program

When you run `cargo run`, the following happens:

1. Cargo compiles your code
2. Cargo finds the `main` function
3. Cargo executes the code inside `main` from top to bottom
4. The program exits when `main` finishes

## Examples

### Example 1: A Simple main Function

```rust
fn main() {
    println!("Hello, world!");
}
```

### Example 2: Multiple Statements

```rust
fn main() {
    println!("Line one");
    println!("Line two");
    println!("Line three");
}
```

Output:

```
Line one
Line two
Line three
```

### Example 3: Printing Values

```rust
fn main() {
    let name = "Gus";
    let age = 25;
    println!("My name is {name} and I am {age} years old.");
}
```

Output:

```
My name is Gus and I am 25 years old.
```

### Example 4: Code Runs Top to Bottom

```rust
fn main() {
    println!("First");
    println!("Second");
    println!("Third");
}
```

Output:

```
First
Second
Third
```

## Common Mistakes

**Forgetting the semicolon.**
```rust
let x = 5  // Error: expected `;`
```

**Forgetting the `!` on `println!`.**
```rust
println("Hello"); // Error: println is a macro, not a function
```

**Putting code outside `main`.**
```rust
println!("Hello"); // Error: expected function body
fn main() {}
```

**Mismatched curly braces.**
```rust
fn main() {
    println!("Hello");
// Error: expected `}`
```

## Practice Ideas

1. Write a program that prints your name on one line and your favorite color on the next line.
2. Write a program that prints the numbers 1 through 5, each on its own line.
3. Create a variable called `city` with your city name. Print "I live in {city}."
4. Predict what this program prints before running it:
   ```rust
   fn main() {
       println!("A");
       println!("B");
       println!("C");
   }
   ```
5. Write a program that prints a simple ASCII art box using `println!`.

---

# 03 — Variables

## Concepts

- `let` keyword
- Variable bindings
- Variable names
- Initialization
- Type inference
- Explicit type annotations
- Scope
- Shadowing
- Rebinding with `let`
- Difference between shadowing and reassignment

## Explanation

Variables in Rust are created with the `let` keyword. A variable binding associates a name with a value.

### Creating a Variable

```rust
let age = 25;
```

This creates a variable called `age` with the value `25`.

### Variable Names

Variable names in Rust:

- Must start with a letter or underscore
- Can contain letters, numbers, and underscores
- Are case-sensitive (`age` and `Age` are different)
- Use `snake_case` by convention (lowercase with underscores)

Good names: `user_name`, `total_count`, `is_ready`
Bad names: `1st_place` (starts with number), `user-name` (contains hyphen)

### Initialization

When you create a variable with `let`, you give it an initial value:

```rust
let name = "Gus";
```

This is called initialization. The variable `name` is bound to the value `"Gus"`.

### Type Inference

Rust can often determine the type of a variable automatically. This is called type inference.

```rust
let age = 25;       // Rust infers i32
let pi = 3.14;      // Rust infers f64
let name = "Gus";   // Rust infers &str
let is_ready = true; // Rust infers bool
```

You do not need to write the type explicitly in most cases.

### Explicit Type Annotations

You can also write the type explicitly using a colon:

```rust
let age: u32 = 25;
let pi: f64 = 3.14;
let name: &str = "Gus";
```

This is called a type annotation. It is useful when:

- You want a specific type (e.g., `u32` instead of the default `i32`)
- The type cannot be inferred
- You want to make the code clearer

### Scope

A variable is valid from the point where it is declared until the end of the block it is declared in.

```rust
fn main() {
    let x = 5;
    println!("{x}"); // Works: x is in scope
}
// x is no longer valid here
```

### Shadowing

Shadowing means creating a new variable with the same name as an existing variable:

```rust
let x = 5;
let x = x + 1;
println!("{x}"); // Prints 6
```

The second `let x` creates a new variable that shadows the first. The first `x` is no longer accessible.

Shadowing is different from mutation. With shadowing, you create a new binding. With mutation, you change the value of an existing binding.

### Shadowing vs Reassignment

Shadowing:
```rust
let x = 5;
let x = 10; // New variable, shadows the first
```

Reassignment (requires `mut`):
```rust
let mut x = 5;
x = 10; // Modifies the existing variable
```

The key difference: shadowing creates a new variable, reassignment modifies an existing one. Shadowing does not require `mut`.

### Why Shadowing Exists

Shadowing is useful when you want to transform a value but keep the same name:

```rust
let input = "  hello  ";
let input = input.trim();
let input = input.to_uppercase();
```

Each line creates a new `input` that shadows the previous one. This is cleaner than coming up with new names at each step.

## Examples

### Example 1: Basic Variable Creation

```rust
fn main() {
    let name = "Gus";
    let age = 25;
    let is_student = true;

    println!("Name: {name}");
    println!("Age: {age}");
    println!("Student: {is_student}");
}
```

### Example 2: Type Annotations

```rust
fn main() {
    let age: u32 = 25;
    let pi: f64 = 3.14159;
    let letter: char = 'A';

    println!("Age: {age}");
    println!("Pi: {pi}");
    println!("Letter: {letter}");
}
```

### Example 3: Shadowing

```rust
fn main() {
    let x = 5;
    println!("First x: {x}");

    let x = x + 1;
    println!("Second x: {x}");

    let x = x * 2;
    println!("Third x: {x}");
}
```

Output:

```
First x: 5
Second x: 6
Third x: 12
```

### Example 4: Shadowing with Different Types

```rust
fn main() {
    let value = "42";
    let value: i32 = value.parse().unwrap();
    println!("{value}");
}
```

The second `value` has a different type from the first. This is only possible with shadowing, not with mutation.

### Example 5: Scope

```rust
fn main() {
    let x = 5;

    {
        let y = 10;
        println!("x: {x}, y: {y}"); // Both in scope
    }

    println!("x: {x}"); // Works
    // println!("{y}"); // Error: y is out of scope
}
```

## Common Mistakes

**Using a variable before it is declared.**
```rust
println!("{x}"); // Error: x not yet declared
let x = 5;
```

**Using a variable outside its scope.**
```rust
{
    let x = 5;
}
println!("{x}"); // Error: x is out of scope
```

**Confusing shadowing with mutation.**
```rust
let x = 5;
x = 10; // Error: x is not mutable
```

**Expecting shadowing to modify the original.**
```rust
let x = 5;
let x = 10;
// The original x = 5 is gone. It was shadowed, not modified.
```

## Practice Ideas

1. Create variables for your name, age, and favorite number. Print them all.
2. Create a variable `x` with value 5. Shadow it with `x + 1`. Shadow it again with `x * 2`. Print the final value.
3. Predict what this prints:
   ```rust
   let a = 10;
   let a = a + 5;
   let a = a * 2;
   println!("{a}");
   ```
4. Write a program that creates a variable `score` with value 100, then shadows it with `score + 50`. Print both the original and new values (hint: you cannot access the original after shadowing — this is a teaching moment).
5. Create a variable `temperature` with value 72. Shadow it with `temperature - 10`. Print the result.

---

# 04 — Mutability

## Concepts

- Immutable by default
- `let mut`
- Reassignment
- Why immutability is the default
- Compiler errors when modifying immutable variables
- Mutable vs immutable bindings
- Difference between shadowing and mutation

## Explanation

Variables in Rust are **immutable by default**. This means once you assign a value to a variable, you cannot change it.

### Immutable by Default

```rust
let x = 5;
x = 10; // Error: cannot assign twice to immutable variable
```

This code will not compile. The compiler will tell you that `x` is immutable.

### Making a Variable Mutable

To allow a variable to be changed, use `mut`:

```rust
let mut x = 5;
x = 10; // Works: x is mutable
```

The `mut` keyword tells Rust that this variable's value can be changed after initialization.

### Why Immutability is the Default

Immutability is the default because it prevents accidental modifications. When a variable cannot be changed, you know its value will always be what you set it to. This makes code easier to reason about and prevents entire classes of bugs.

### Reassignment

With a mutable variable, you can reassign its value:

```rust
let mut score = 0;
score = 10;
score = 20;
score = 30;
```

Each line changes the value of `score`. This is called reassignment.

### Mutable vs Immutable Bindings

Immutable binding:
```rust
let x = 5;
// x cannot be changed
```

Mutable binding:
```rust
let mut x = 5;
x = 10; // x can be changed
```

The difference is the `mut` keyword. Without it, the binding is immutable. With it, the binding is mutable.

### Shadowing vs Mutation

Shadowing creates a new variable:
```rust
let x = 5;
let x = 10; // New variable, shadows the first
```

Mutation changes an existing variable:
```rust
let mut x = 5;
x = 10; // Modifies the existing variable
```

Key differences:

| Aspect | Shadowing | Mutation |
|--------|-----------|----------|
| Keyword | `let` | `mut` |
| Creates new variable? | Yes | No |
| Changes existing variable? | No | Yes |
| Can change type? | Yes | No |
| Requires `mut`? | No | Yes |

### Compiler Errors

If you try to modify an immutable variable, the compiler will give you a clear error:

```rust
let x = 5;
x = 10;
```

Error message:

```
error[E0384]: cannot assign twice to immutable variable `x`
```

This error tells you exactly what is wrong and where.

## Examples

### Example 1: Immutable Variable

```rust
fn main() {
    let x = 5;
    println!("{x}");
}
```

Output:

```
5
```

### Example 2: Attempting to Modify an Immutable Variable

```rust
fn main() {
    let x = 5;
    x = 10; // Error: cannot assign twice to immutable variable
    println!("{x}");
}
```

This code will not compile.

### Example 3: Mutable Variable

```rust
fn main() {
    let mut x = 5;
    x = 10;
    println!("{x}");
}
```

Output:

```
10
```

### Example 4: Shadowing vs Mutation

Shadowing:
```rust
fn main() {
    let x = 5;
    let x = 10;
    println!("{x}"); // Prints 10
}
```

Mutation:
```rust
fn main() {
    let mut x = 5;
    x = 10;
    println!("{x}"); // Prints 10
}
```

Both print `10`, but they work differently.

### Example 5: Multiple Reassignments

```rust
fn main() {
    let mut score = 0;
    println!("Starting score: {score}");

    score = score + 10;
    println!("After first round: {score}");

    score = score + 20;
    println!("After second round: {score}");
}
```

Output:

```
Starting score: 0
After first round: 10
After second round: 30
```

### Example 6: Shadowing with Type Change

```rust
fn main() {
    let value = "42";
    let value: i32 = value.parse().unwrap();
    println!("{value}");
}
```

Output:

```
42
```

The first `value` is a string. The second `value` is an integer. This is only possible with shadowing.

## Common Mistakes

**Forgetting `mut` when you need to reassign.**
```rust
let x = 5;
x = 10; // Error: x is not mutable
```

**Using `mut` when you only need shadowing.**
```rust
let mut x = 5;
let x = 10; // Warning: variable does not need to be mutable
```

**Confusing shadowing with mutation.**
```rust
let x = 5;
let x = 10; // This is shadowing, not mutation
// The original x = 5 is gone
```

**Expecting mutation to work on a shadowed variable.**
```rust
let mut x = 5;
let x = 10; // This shadows, so mut is no longer relevant
x = 20; // Error: x is not mutable (the shadowed binding is immutable)
```

## Practice Ideas

1. Write a program with an immutable variable `x = 5`. Try to change it to `x = 10`. Read the compiler error. Then fix it by adding `mut`.
2. Write a program that uses shadowing to transform a value through three steps. Print the value at each step.
3. Write a program with a mutable `score` variable. Simulate a game where the score increases by 10 three times. Print the score after each increase.
4. Predict what this code does:
   ```rust
   let mut x = 5;
   let x = x + 1;
   x = 10;
   ```
   Will it compile? Why or why not?
5. Write a program that demonstrates the difference between shadowing and mutation. Use comments to explain what is happening at each step.

---

# 05 — Basic Types

## Concepts

- Type inference review
- Integer types (signed and unsigned)
- Floating-point types
- Boolean type
- Character type
- Tuples
- Arrays
- Type annotations
- Numeric literals

## Explanation

Every value in Rust has a type. The type determines what kind of value it is and what operations can be performed on it.

Rust has two main categories of types: scalar and compound. Scalar types represent a single value. Compound types group multiple values together.

### Type Inference

As you learned earlier, Rust can often determine the type automatically:

```rust
let age = 25;    // i32 (default integer type)
let pi = 3.14;    // f64 (default float type)
let name = "Gus"; // &str (string slice)
let is_ready = true; // bool
```

You can also write the type explicitly:

```rust
let age: u32 = 25;
let pi: f64 = 3.14159;
```

### Integer Types

Integers are whole numbers. Rust has two kinds: signed (can be negative) and unsigned (only zero and positive).

Signed integers:

| Type | Minimum | Maximum |
|------|---------|---------|
| `i8` | -128 | 127 |
| `i16` | -32,768 | 32,767 |
| `i32` | -2,147,483,648 | 2,147,483,647 |
| `i64` | -9,223,372,036,854,775,808 | 9,223,372,036,854,775,807 |
| `i128` | Very large | Very large |
| `isize` | Platform-dependent | Platform-dependent |

Unsigned integers:

| Type | Minimum | Maximum |
|------|---------|---------|
| `u8` | 0 | 255 |
| `u16` | 0 | 65,535 |
| `u32` | 0 | 4,294,967,295 |
| `u64` | 0 | 18,446,744,073,709,551,615 |
| `u128` | 0 | Very large |
| `usize` | 0 | Platform-dependent |

The number after `i` or `u` is the number of bits used to store the value. More bits means a larger range.

**Why `i32` is the default:** When you write `let x = 25;`, Rust infers `i32` because it is a good balance of range and performance for most use cases.

**When `usize` appears:** `usize` is used for indexing into collections (like arrays). You will see it when you work with arrays and other collections.

### Floating-Point Types

Floating-point numbers are numbers with decimal points.

| Type | Precision | Use case |
|------|-----------|----------|
| `f32` | Single precision | When memory is tight |
| `f64` | Double precision | Default, more precise |

```rust
let pi: f64 = 3.14159;
let temperature: f32 = 98.6;
```

When you write `let x = 3.14;`, Rust infers `f64` because it is more precise.

### Boolean Type

The boolean type has two values: `true` and `false`.

```rust
let is_ready = true;
let is_finished = false;
```

Booleans are used for conditions and decisions. You will learn more about them in the control flow module.

### Character Type

The `char` type represents a single Unicode character. Characters use single quotes:

```rust
let letter = 'A';
let emoji = '🦀';
let digit = '7';
```

A `char` is not the same as a string. A `char` is a single character. A string literal uses double quotes and can contain multiple characters:

```rust
let letter: char = 'A';     // Single character
let name: &str = "Alice";   // Multiple characters
```

### Tuples

A tuple is a fixed-size grouping of values. Tuples can contain values of different types:

```rust
let person = ("Gus", 25, true);
```

This tuple contains a string, an integer, and a boolean.

You access tuple elements with a dot and a zero-based index:

```rust
let person = ("Gus", 25, true);
println!("Name: {}", person.0);     // "Gus"
println!("Age: {}", person.1);      // 25
println!("Active: {}", person.2);   // true
```

Tuples have a fixed size. You cannot add or remove elements after creation.

### Arrays

An array is a fixed-size collection of elements of the same type:

```rust
let numbers = [1, 2, 3, 4, 5];
```

All elements must be the same type. You cannot mix types in an array.

You access array elements with square brackets and a zero-based index:

```rust
let numbers = [1, 2, 3, 4, 5];
println!("First: {}", numbers[0]);  // 1
println!("Last: {}", numbers[4]);   // 5
```

Arrays have a fixed size. You cannot add or remove elements after creation.

**Arrays vs Tuples:**

| Aspect | Array | Tuple |
|--------|-------|-------|
| Element type | Same type | Can be different types |
| Access | `arr[index]` | `tuple.index` |
| Size | Fixed | Fixed |
| Use case | Collection of same values | Grouping different values |

### Numeric Literals

Rust provides several ways to write numeric literals:

```rust
let decimal = 98_222;      // Decimal (underscores for readability)
let hex = 0xff;            // Hexadecimal
let binary = 0b1111_0000;  // Binary
let octal = 0o77;          // Octal
```

Underscores can be used to improve readability. `98_222` is the same as `98222`.

## Examples

### Example 1: Integer Types

```rust
fn main() {
    let small: i8 = 100;
    let medium: i32 = 100_000;
    let large: i64 = 1_000_000_000;

    println!("Small: {small}");
    println!("Medium: {medium}");
    println!("Large: {large}");
}
```

### Example 2: Unsigned Integers

```rust
fn main() {
    let age: u8 = 25;
    let score: u32 = 10_000;
    let big: u64 = 1_000_000_000_000;

    println!("Age: {age}");
    println!("Score: {score}");
    println!("Big: {big}");
}
```

### Example 3: Floating-Point Types

```rust
fn main() {
    let pi: f64 = 3.14159;
    let temperature: f32 = 98.6;

    println!("Pi: {pi}");
    println!("Temperature: {temperature}");
}
```

### Example 4: Boolean and Character

```rust
fn main() {
    let is_ready = true;
    let letter = 'A';

    println!("Ready: {is_ready}");
    println!("Letter: {letter}");
}
```

### Example 5: Tuples

```rust
fn main() {
    let person = ("Gus", 25, true);

    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Active: {}", person.2);
}
```

### Example 6: Destructuring a Tuple

```rust
fn main() {
    let person = ("Gus", 25, true);
    let (name, age, active) = person;

    println!("Name: {name}");
    println!("Age: {age}");
    println!("Active: {active}");
}
```

### Example 7: Arrays

```rust
fn main() {
    let numbers = [10, 20, 30, 40, 50];

    println!("First: {}", numbers[0]);
    println!("Third: {}", numbers[2]);
    println!("Last: {}", numbers[4]);
}
```

### Example 8: Numeric Literals

```rust
fn main() {
    let decimal = 98_222;
    let hex = 0xff;
    let binary = 0b1111_0000;
    let octal = 0o77;

    println!("Decimal: {decimal}");
    println!("Hex: {hex}");
    println!("Binary: {binary}");
    println!("Octal: {octal}");
}
```

Output:

```
Decimal: 98222
Hex: 255
Binary: 240
Octal: 63
```

### Example 9: Type Annotations

```rust
fn main() {
    let age: u32 = 25;
    let pi: f64 = 3.14159;
    let name: &str = "Gus";
    let is_ready: bool = true;
    let letter: char = 'A';

    println!("Age: {age}");
    println!("Pi: {pi}");
    println!("Name: {name}");
    println!("Ready: {is_ready}");
    println!("Letter: {letter}");
}
```

### Example 10: Array with Type Annotation

```rust
fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);
}
```

The `[i32; 5]` syntax means "an array of 5 i32 values." The `{:?}` format specifier prints the entire array.

## Common Mistakes

**Mixing types in an array.**
```rust
let numbers = [1, 2, "three"]; // Error: expected integer, found &str
```

**Accessing an out-of-bounds index.**
```rust
let numbers = [1, 2, 3];
println!("{}", numbers[5]); // Error: index out of bounds
```

**Using double quotes for char.**
```rust
let letter = "A"; // Error: expected char, found &str
```

**Using single quotes for string.**
```rust
let name = 'Gus'; // Error: character literal may only contain one codepoint
```

**Forgetting that tuples use dot indexing.**
```rust
let person = ("Gus", 25);
println!("{}", person[0]); // Error: cannot index into a tuple with []
```

## Practice Ideas

1. Create variables of each integer type (`i8`, `i16`, `i32`, `i64`, `u8`, `u16`, `u32`, `u64`). Print them all.
2. Create a tuple representing a book: title (string), pages (integer), available (boolean). Print each field.
3. Create an array of 5 floating-point numbers representing temperatures. Print the first and last.
4. Write a program that uses numeric literals in different bases (decimal, hex, binary, octal). Print each one.
5. Create a variable with an explicit type annotation for each basic type. Print them all.
6. Write a program that creates an array of 5 integers and prints each element using its index.
7. Create a tuple with three different types. Destructure it into three variables. Print each variable.

---

# Module Review

## Concepts Learner Should Know

- Cargo is Rust's build system and package manager
- `cargo new` creates a new project
- `cargo run` builds and runs a project
- `cargo check` quickly checks for compilation errors
- `main()` is the entry point of every Rust executable
- `fn` declares a function
- `println!` prints text to the console
- Variables are created with `let`
- Variables are immutable by default
- `mut` makes a variable mutable
- Shadowing creates a new variable with the same name
- Rust has integer, float, boolean, char, tuple, and array types
- Type inference determines types automatically
- Type annotations specify types explicitly

## Common Confusions

**Shadowing vs Mutation:** Shadowing creates a new variable. Mutation changes an existing variable. Shadowing uses `let`, mutation uses `mut`.

**Immutable by default:** Variables cannot be changed unless you use `mut`. This is different from many other languages.

**char vs string:** `char` is a single character with single quotes. A string literal uses double quotes and can contain multiple characters.

**Tuple vs Array:** Tuples can contain different types and use dot indexing. Arrays contain the same type and use bracket indexing.

**Type inference vs type annotations:** Rust can usually infer the type. You write a type annotation when you need a specific type or when inference is not possible.

## Practice Progression

### Level 1: Recognition

1. What does `cargo new` do?
2. What is the entry point of a Rust program?
3. What keyword creates a variable?
4. What keyword makes a variable mutable?
5. What are the two boolean values?

### Level 2: Prediction

Predict the output of:

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("{x}");
}
```

Predict whether this compiles:

```rust
fn main() {
    let x = 5;
    x = 10;
}
```

### Level 3: Modification

Fix this code so it compiles:

```rust
fn main() {
    let score = 10;
    score = 20;
    println!("{score}");
}
```

Fix this code so it compiles:

```rust
fn main() {
    let numbers = [1, 2, 3];
    println!("{}", numbers[5]);
}
```

### Level 4: Write from Scratch

Write a program that:

1. Creates a variable `name` with your name
2. Creates a variable `age` with your age
3. Creates a mutable variable `score` starting at 0
4. Increases `score` by 10 three times
5. Prints the final score

### Level 5: Debug Broken Code

Find and fix all errors in this program:

```rust
fn main() {
    let x = 5
    let y = "hello";
    let y = 'world';
    let mut z = 10;
    z = 20;
    z = 30;
    println!("{x} {y} {z}");
}
```

### Level 6: Small Challenge

Create a program that:

1. Creates a tuple representing a person: name, age, and whether they are a student
2. Destructures the tuple into three variables
3. Prints each variable
4. Creates an array of 5 integers
5. Prints the sum of the first and last elements

## Final Challenge

Create a complete Rust program that demonstrates everything you learned in this module. Your program should:

1. Be created with `cargo new`
2. Have a `main()` function
3. Use variables with `let`
4. Use at least one mutable variable
5. Use shadowing at least once
6. Use at least three different basic types (integer, float, boolean, char, tuple, or array)
7. Print output using `println!`
8. Compile and run without errors

Example:

```rust
fn main() {
    // Variables
    let name = "Gus";
    let age: u32 = 25;
    let is_student = true;

    // Mutable variable
    let mut score = 0;
    score = score + 10;
    score = score + 20;

    // Shadowing
    let status = "active";
    let status = status.to_uppercase();

    // Tuple
    let person = (name, age, is_student);

    // Array
    let grades = [90, 85, 95, 88, 92];

    // Print everything
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Student: {}", person.2);
    println!("Score: {score}");
    println!("Status: {status}");
    println!("First grade: {}", grades[0]);
    println!("Last grade: {}", grades[4]);
}
```

---

# Source References

- The Rust Book: https://doc.rust-lang.org/book/
- The Cargo Book: https://doc.rust-lang.org/cargo/
- Rust By Example: https://doc.rust-lang.org/rust-by-example/
- The Rust Reference: https://doc.rust-lang.org/reference/
- Rustlings: https://github.com/rust-lang/rustlings
