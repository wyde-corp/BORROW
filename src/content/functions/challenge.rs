//! Challenges for Module 04 — Functions.

use crate::content::types::{ExerciseKind, SeedExercise};

/// The program to be decomposed. Valid Rust — it works, it is just doing too much.
pub const REFACTOR_CODE: &str = r#"// A working program. Every line is fine. The structure is the problem.
fn main() {
    let raw = "12, 7, 25, 3, 18, 9";
    let mut parts: Vec<i32> = Vec::new();

    // parse the comma-separated text
    for piece in raw.split(',') {
        let trimmed = piece.trim();
        match trimmed.parse::<i32>() {
            Ok(n) => parts.push(n),
            Err(_) => println!("skipping bad piece: {}", trimmed),
        }
    }

    // compute three separate figures
    let total: i32 = parts.iter().sum();
    let mut biggest = 0;
    for n in &parts {
        if *n > biggest {
            biggest = *n;
        }
    }
    let mut above_ten = 0;
    for n in &parts {
        if *n > 10 {
            above_ten += 1;
        }
    }
    let average = total as f64 / parts.len() as f64;

    // print everything
    println!("values: {:?}", parts);
    println!("count:  {}", parts.len());
    println!("total:  {}", total);
    println!("largest: {}", biggest);
    println!("over 10: {}", above_ten);
    println!("average: {:.2}", average);
}"#;

pub const REFACTOR_SOLUTION: &str = r#"A reasonable decomposition. There is no single right answer — the point
is that each function is nameable without the word "and".

  fn parse_list(raw: &str) -> Vec<i32>
      Takes the text, splits it, parses each piece, skips the bad ones.
      One job: text in, numbers out.

  fn total(values: &[i32]) -> i32
  fn largest(values: &[i32]) -> i32
  fn count_above(values: &[i32], threshold: i32) -> usize
      Three of these, each one verb. Note they all take &[i32]
      rather than Vec<i32> — they only need to read, so they
      borrow. That is what the ownership lesson was about.

  fn average(values: &[i32]) -> f64
      One job. Note the cast: total as f64, because integer
      division would truncate.

  fn print_report(values: &[i32])
      All the printing, in one place, so the calculations stay
      free of output.

Then main reads as a summary:

  fn main() {
      let values = parse_list("12, 7, 25, 3, 18, 9");
      print_report(&values);
  }

Two things worth noticing:

1. main no longer knows how anything is computed. It knows the
   order of operations and nothing else. That is the goal.

2. None of the helpers println!. The parsing function does print
   warnings about bad pieces, which is a reasonable exception —
   but if you wanted pure functions, collect the bad pieces and
   report them separately.

HARD MODE: make parse_list return a Result instead of silently
skipping bad input, so the caller decides what to do about a
failure. That is the error handling module, reached early."#;

pub const FORGE_SPEC: &str = r#"// Your program needs all of the following.
// How you divide it up is your decision.

// REQUIRED BEHAVIOUR
// 1. Accept a list of task names with a time estimate each
// 2. Reject an empty name, and reject a negative estimate
// 3. Total the time across all valid tasks
// 4. Report how many tasks are "large" (over 90 minutes)
// 5. Find the longest task by name
// 6. Print a summary line per task plus a final total

// HARD MODE
// Group tasks by size: small (under 60), medium (60-90), large (over 90)
// and print one count per group using a match."#;

pub const FORGE_HINTS: &str = r#"This challenge is about DESIGN, not syntax. Nothing can tell you
whether your decomposition was good — that is the part you are practising.

Things to actually decide:

1. How many functions, and what is each one named? If you cannot
   name a function without using the word "and", it is doing too
   much.

2. What is the data type? A tuple, a struct, or parallel vectors?
   You have met tuples and arrays; structs arrive in module 07. Pick
   whatever lets each function state clearly what it needs.

3. Which functions need to mutate something, and which only read?
   Read-only functions should take & and mutating ones &mut. Get
   this right and the borrow checker stops fighting you.

4. What does each function return? A total? A name? A count? If a
   function both computes and prints, consider splitting it — the
   calculation becomes testable without capturing output.

5. Where does the input validation live? Once, or per task? Once is
   usually cleaner, and it means only one place produces the error
   message.

The most common outcome here is one giant main that does everything
inline. It works. It is also exactly the problem this module set out
to fix."#;

pub fn challenges() -> Vec<SeedExercise> {
    vec![
        SeedExercise::new(
            None,
            ExerciseKind::Challenge,
            "FUNCTION REFACTORING CHALLENGE — This program works. That is the problem. Your job is to restructure it without changing what it does.",
            &[
                "Write a function that parses the comma-separated text into a Vec<i32>, skipping bad pieces",
                "Write a function that totals the values",
                "Write a function that finds the largest value",
                "Write a function that counts values above a threshold",
                "Write a function that computes the average as an f64",
                "Write a function that does all the printing",
                "Reduce main to: parse, then print — no arithmetic in main",
            ],
            0,
            REFACTOR_SOLUTION,
        )
        .code(REFACTOR_CODE)
        .xp(120)
        .hints(&[
            "Look at main and count the verbs. Parse, total, find, count, compute, print. That is six.",
            "Try describing a candidate function in one sentence. If you need 'and', split it further.",
            "The three numeric helpers all take &[i32] rather than Vec<i32>. Why? They only read.",
            "The average function needs a cast: total as f64 / len as f64, or you get integer division.",
            "Compare the length of main before and after. It should shrink to about two lines.",
        ]),
        SeedExercise::new(
            None,
            ExerciseKind::Challenge,
            "FUNCTION FORGE — Build a task estimator. You are not told which functions to write. Deciding that is the exercise.",
            &[
                "Decide how many functions you need, and give each one a name that does not need 'and'",
                "Decide what type represents a task — a tuple, parallel vectors, or something else",
                "Decide which functions read their input (&) and which change it (&mut)",
                "Decide what each function returns, and make sure no function both computes and prints",
                "Put input validation in exactly one place, not repeated per task",
                "Hard mode: group tasks by size and count each group with a match",
            ],
            0,
            FORGE_HINTS,
        )
        .code(FORGE_SPEC)
        .xp(200)
        .hints(&[
            "Start by writing down the function names only, no bodies. Six behaviours in the spec means roughly five or six functions.",
            "If two of your functions both need a task's name and estimate, a tuple might be worth it. Structs arrive in module 07 — a tuple is the tool you have now.",
            "Try the 'and' test on every name you have written. Any failure means split again.",
            "If the borrow checker complains, check whether a function that only reads has been given ownership.",
            "Then add the hard mode: three groups, one match, one count each.",
        ]),
    ]
}
