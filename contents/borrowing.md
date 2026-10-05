# BORROW — Module: Borrowing

## Module Purpose

Build a deep, practical understanding of Rust borrowing.

The learner has already completed Ownership. Do NOT re-teach the entire ownership module. Assume the learner understands:

* ownership
* moves
* `Copy`
* `Clone`
* scope
* `drop`
* ownership transfer through functions
* why Rust needs ownership

Borrowing should answer the next natural question:

> If I don't want to take ownership of a value, how can I temporarily use it?

The learner should finish this module able to look at Rust code containing `&T` and `&mut T` and reason about:

* who owns the value
* who is borrowing it
* whether the borrow is immutable or mutable
* how long the borrow is needed
* whether multiple borrows are allowed
* why the borrow checker accepts or rejects the code
* whether a function should take ownership or borrow
* when a reference becomes invalid
* why mutable and immutable references have different rules
* how slices are built from borrowing
* how to diagnose common borrow-checker errors

Do not turn this into a generic syntax chapter.

The central idea is:

> **Ownership answers “who is responsible for this value?” Borrowing answers “who may temporarily use it without taking responsibility for it?”**

---

# PART 01 — The Problem Borrowing Solves

Begin with a problem rather than syntax.

Show:

```rust
fn calculate_length(s: String) -> usize {
    s.len()
}

fn main() {
    let name = String::from("Gus");

    let length = calculate_length(name);

    println!("{length}");
    println!("{name}");
}
```

Ask the learner:

> What is wrong with this design if the function only needs to inspect the string?

Trace the ownership:

```text
main
  │
  │ owns String
  ↓
calculate_length
  │
  │ takes ownership
  ↓
function ends
  │
  ↓
String dropped
```

The function did not need ownership.

It only needed access.

Introduce the motivation for borrowing:

```text
OWNERSHIP

"I take responsibility for this value."


BORROWING

"I need temporary access to this value,
but I do not become its owner."
```

Do not introduce `&` immediately.

Let the learner understand the problem first.

---

# PART 02 — References

Introduce references as the mechanism Rust uses for borrowing.

Start with:

```rust
let s = String::from("hello");
let r = &s;
```

Explain precisely:

* `s` owns the `String`
* `r` does not own the `String`
* `r` is a reference to the value owned by `s`
* using `r` does not transfer ownership
* when `r` disappears, it does not drop the `String`
* `s` remains responsible for the value

Mental model:

```text
STACK

s ────────────────┐
                  │
r ────────────────┘
                  ↓
                HEAP
             "hello"
```

Be careful with the diagram.

Do not imply that a reference necessarily has the same representation as a `String`.

The important conceptual relationship is:

```text
s owns the value

r refers to the value
```

---

# PART 03 — Borrowing Without Ownership Transfer

Compare:

```rust
let s1 = String::from("hello");
let s2 = s1;
```

with:

```rust
let s1 = String::from("hello");
let s2 = &s1;
```

Ask:

> After each statement, who owns the String?

Expected reasoning:

```text
s2 = s1

ownership moves
s1 is no longer usable


s2 = &s1

ownership does not move
s1 remains the owner
s2 merely borrows
```

This distinction should be drilled repeatedly.

Do not describe a reference as “another owner.”

A reference is not an owner.

---

# PART 04 — Dereferencing

Introduce:

```rust
let s = String::from("hello");
let r = &s;

println!("{r}");
println!("{}", *r);
```

Explain:

```text
&   → create a reference / borrow

*   → dereference a reference
```

But do not overcomplicate dereferencing yet.

The learner should understand the conceptual distinction:

```text
s
↓
actual value


r
↓
reference to that value


*r
↓
access the value through the reference
```

Also explain why:

```rust
println!("{r}");
```

works without explicitly writing:

```rust
println!("{}", *r);
```

Rust's formatting and expression behavior should be explained where useful, without turning this into a deep autoderef/autoref lesson.

---

# PART 05 — Immutable Borrowing

Introduce immutable references:

```rust
let s = String::from("hello");

let r1 = &s;
let r2 = &s;

println!("{r1}");
println!("{r2}");
```

Establish the fundamental rule:

> Multiple immutable references can coexist.

Mental model:

```text
             ┌── r1
             │
s ───────────┼── r2
             │
             ↓
          "hello"
```

The learner should understand why this is safe:

* neither reference can modify the value
* multiple readers do not conflict

Do not reduce this to “Rust allows two references.”

Explain the underlying aliasing idea.

---

# PART 06 — Mutable Borrowing

Introduce:

```rust
let mut s = String::from("hello");

let r = &mut s;

r.push_str(" world");

println!("{r}");
```

Explain the requirements:

```text
The owner must permit mutation:

let mut s = ...

The borrow must be mutable:

let r = &mut s;
```

Contrast:

```rust
let s = String::from("hello");
let r = &s;

r.push_str(" world"); // ERROR
```

with:

```rust
let mut s = String::from("hello");
let r = &mut s;

r.push_str(" world"); // OK
```

The learner should understand that `mut` on the owner and `&mut` on the reference represent different things.

---

# PART 07 — The Core Borrowing Rule

Introduce the fundamental rule:

> At any point in time, you can have either:
>
> * any number of immutable references
>
> OR
>
> * exactly one mutable reference
>
> but not both simultaneously.

Use examples.

Valid:

```rust
let s = String::from("hello");

let r1 = &s;
let r2 = &s;

println!("{r1}");
println!("{r2}");
```

Valid:

```rust
let mut s = String::from("hello");

let r = &mut s;

r.push_str("!");
```

Invalid:

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &mut s;
```

Explain WHY.

The issue is not arbitrary Rust bureaucracy.

The rule prevents situations where code has:

```text
reader ──┐
         ├── same value
writer ──┘
```

and the reader cannot know whether the value may change underneath it.

Introduce the conceptual rule:

```text
Many readers
OR
One writer
```

This is one of the most important mental models in the entire module.

---

# PART 08 — Borrowing and Scope

Do not teach borrowing rules without teaching that borrows have a duration.

Start:

```rust
let mut s = String::from("hello");

{
    let r = &mut s;
    r.push_str(" world");
}

println!("{s}");
```

Explain why this works.

Then:

```rust
let mut s = String::from("hello");

let r = &mut s;

println!("{r}");

let r2 = &mut s;
```

Discuss when the first borrow is actually finished.

Introduce the idea of the borrow's **use/liveness region** without immediately diving into formal lifetime annotations.

Explain that modern Rust uses the borrow checker to reason about when references are actually needed.

Important:

Do not teach:

> “A reference always lives until the end of its lexical scope.”

That is an outdated oversimplification.

Use examples where a borrow ends after its final use.

---

# PART 09 — Multiple Immutable Borrows

Give progressively harder examples:

```rust
let s = String::from("hello");

let a = &s;
let b = &s;
let c = &s;

println!("{a}");
println!("{b}");
println!("{c}");
```

Then ask:

> Why is this safe?

Then introduce a mutation attempt after immutable borrows:

```rust
let mut s = String::from("hello");

let a = &s;
let b = &s;

s.push_str("!");

println!("{a}");
println!("{b}");
```

Ask the learner to predict the compiler.

Explain that the problem is not simply that references exist somewhere.

The problem is that the immutable borrows are still being used.

Then show a version where the borrows finish before mutation:

```rust
let mut s = String::from("hello");

let a = &s;
let b = &s;

println!("{a}");
println!("{b}");

s.push_str("!");
```

Use this to teach the learner to reason about **when a borrow is active**.

---

# PART 10 — Mutable Borrowing and Exclusivity

Explore:

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;
```

Explain why this is rejected.

Then show:

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
    r1.push('!');
}

let r2 = &mut s;
r2.push('?');
```

Explain that the first mutable borrow ends before the second begins.

The learner should understand:

```text
ONE MUTABLE BORROW
      ↓
use it
      ↓
borrow ends
      ↓
ANOTHER MUTABLE BORROW
```

---

# PART 11 — Borrowing in Functions

Compare:

```rust
fn print_name(name: String) {
    println!("{name}");
}
```

with:

```rust
fn print_name(name: &String) {
    println!("{name}");
}
```

Then explain why the second design is useful.

Use:

```rust
let name = String::from("Gus");

print_name(&name);

println!("{name}");
```

The function gets temporary access without taking ownership.

Then introduce the more idiomatic form where appropriate:

```rust
fn print_name(name: &str) {
    println!("{name}");
}
```

If `str` is introduced here, explain only what is necessary and use it as a bridge into slices.

Do not turn this section into a complete string-slice lesson.

---

# PART 12 — `&T` vs `&mut T`

Create a direct comparison.

```text
&T

borrow without modifying


&mut T

borrow with permission to modify
```

Examples:

```rust
fn length(s: &String) -> usize {
    s.len()
}
```

and:

```rust
fn add_suffix(s: &mut String) {
    s.push_str("!");
}
```

Explain that `&mut T` does not mean:

> “The reference itself is mutable.”

It means:

> “This reference grants mutable access to the borrowed value.”

This distinction matters.

---

# PART 13 — Mutable Owner vs Mutable Reference

Explicitly address this beginner confusion.

Compare:

```rust
let mut s = String::from("hello");
```

with:

```rust
let r = &mut s;
```

Explain:

```text
mut s

The owner is allowed to modify the value.


&mut s

A mutable reference is created,
allowing mutation through that reference.
```

Then show:

```rust
let mut s = String::from("hello");

let r = &s;

// r.push_str("!"); // invalid
```

and:

```rust
let mut s = String::from("hello");

let r = &mut s;

r.push_str("!");
```

---

# PART 14 — Borrowing vs Moving

This should be a major comparison exercise.

Given:

```rust
fn use_string(s: String) {
    println!("{s}");
}
```

and:

```rust
fn inspect_string(s: &String) {
    println!("{s}");
}
```

Compare:

```rust
use_string(s);
```

with:

```rust
inspect_string(&s);
```

Ask:

* Who owns the value before the call?
* Who owns it inside the function?
* Who owns it after the call?
* Can the caller use the value afterward?
* Did ownership move?
* Did a borrow occur?

The learner should become comfortable translating between:

```text
take ownership
borrow immutably
borrow mutably
```

---

# PART 15 — Returning References

Introduce returning references carefully.

Example:

```rust
fn first_character(s: &str) -> &str {
    &s[..1]
}
```

Explain the important question:

> What does the returned reference refer to?

Then discuss why this works when the referenced data comes from the input.

Contrast with the invalid idea:

```rust
fn bad() -> &String {
    let s = String::from("hello");
    &s
}
```

Explain:

* `s` owns the value
* `s` disappears when the function ends
* returning a reference to it would create a reference to something that no longer exists

This is an important conceptual bridge toward lifetimes.

Do NOT teach lifetime annotations deeply here.

The learner only needs to understand:

> A reference cannot outlive the value it refers to.

---

# PART 16 — Dangling References

Use a deliberately invalid example:

```rust
fn dangling() -> &String {
    let s = String::from("hello");
    &s
}
```

Explain what would happen in a language that allowed this:

```text
function creates value
       ↓
value exists
       ↓
function ends
       ↓
value dropped
       ↓
returned reference points nowhere
```

Then:

> Rust rejects this before the program can create that dangling reference.

Connect this back to Rust's safety guarantees.

Do not claim that Rust simply “knows everything about memory.”

Explain that the borrow checker enforces rules that prevent references from outliving the data they reference.

---

# PART 17 — Borrowing and Collections

Use collections to show realistic borrowing.

Start:

```rust
let names = vec![
    String::from("Gus"),
    String::from("Alice"),
    String::from("Bob"),
];
```

Show reading through references.

Then introduce:

```rust
let first = &names[0];
```

Explain:

* `names` owns the `String`s
* `first` borrows one element
* `first` does not own the element

Then explore mutation conflicts carefully:

```rust
let first = &names[0];

// mutate names here
```

Use compiler experiments to demonstrate why Rust cares about simultaneous access.

Do not dump advanced collection borrowing rules on the learner.

The goal is to reinforce the same ownership/borrowing model.

---

# PART 18 — Slices: Borrowing Part of a Value

Introduce slices as a natural extension of borrowing.

Start with strings:

```rust
let s = String::from("hello world");

let hello = &s[0..5];
```

Explain:

> A slice is a reference to a contiguous portion of a collection.

Use:

```text
String

h e l l o   w o r l d
0 1 2 3 4 5 6 7 8 9 10


        ┌───────────┐
        ↓           ↓
       [0 ........ 5]
        ↓
      "hello"
```

Explain that the slice does not own the text.

Then introduce `&str`.

---

# PART 19 — `String` vs `&str`

Make this distinction concrete.

```text
String

owned, growable UTF-8 string


&str

borrowed string slice
```

Show:

```rust
let owned = String::from("hello");
let borrowed: &str = &owned;
```

Then function design:

```rust
fn first_word(s: &str) -> &str {
    // ...
}
```

Explain why accepting `&str` is often more flexible than accepting `&String`.

The learner should understand:

```text
String
   ↓ borrow
&str
```

and that a `&str` can refer to part of a `String`.

Do not turn this into a comprehensive UTF-8 lesson.

---

# PART 20 — Slice Ranges and Borrowing

Teach:

```rust
let s = String::from("hello world");

let a = &s[0..5];
let b = &s[6..11];
```

Then explore:

```rust
let whole = &s[..];
let beginning = &s[..5];
let ending = &s[6..];
```

Explain what the ranges mean.

Also include an example that fails because the range is not valid UTF-8.

Keep the explanation focused on slices and borrowing, but mention that Rust strings are UTF-8 and slicing operates on byte indices.

---

# PART 21 — Borrowing Rules as a System

At this point, stop introducing new syntax.

Have the learner reconstruct the rules themselves.

The system:

```text
OWNER
  │
  ├── owns value
  │
  ├── can create immutable borrows
  │       └── many can coexist
  │
  └── can create mutable borrow
          └── exclusive access
```

Core rule:

```text
        IMMUTABLE BORROWS
        ┌────┬────┬────┐
        ↓    ↓    ↓
        READ READ READ


OR


        MUTABLE BORROW
              ↓
            WRITE
```

Never simultaneously:

```text
READ + WRITE
```

when those accesses overlap in a way that violates Rust's borrowing rules.

Explain that the rule is fundamentally about preventing unsafe aliasing and mutation.

---

# PART 22 — The Borrow Checker

Teach learners how to reason like the borrow checker.

Use a process:

```text
1. Who owns the value?

2. Where is the reference created?

3. Is the reference immutable or mutable?

4. How long is the reference used?

5. Are there other references active?

6. Is the owner being accessed directly?

7. Is anything being mutated?

8. Does the reference outlive the value?
```

Then apply this process to progressively harder examples.

The goal is NOT:

> “Memorize the borrow checker.”

The goal is:

> “Learn to predict what the borrow checker will reject.”

---

# PART 23 — Reading Borrow Checker Errors

Use actual compiler-error-style examples.

Example:

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &mut s;

println!("{r1}");
println!("{r2}");
```

Teach the learner to identify:

* first borrow
* second borrow
* conflict
* location of first borrow
* location of second borrow
* where the first borrow is later used

Then another example:

```rust
fn bad() -> &String {
    let s = String::from("hello");
    &s
}
```

Teach them to interpret the compiler's lifetime-related complaint without requiring them to understand explicit lifetime syntax yet.

---

# PART 24 — Common Borrowing Mistakes

Include a dedicated troubleshooting section.

Mistakes:

### Mistake 1

Thinking borrowing transfers ownership.

### Mistake 2

Thinking `&mut` means the reference itself is mutable.

### Mistake 3

Thinking `mut` automatically allows every kind of mutation.

### Mistake 4

Thinking multiple mutable references are harmless.

### Mistake 5

Thinking one immutable reference means nobody can ever mutate the value again.

### Mistake 6

Thinking a reference owns the value it points to.

### Mistake 7

Trying to return a reference to a local value.

### Mistake 8

Using `.clone()` to avoid understanding a borrow error.

### Mistake 9

Using `&String` everywhere without understanding `&str`.

### Mistake 10

Thinking borrow scope is always identical to the surrounding `{}` block.

Each mistake should have:

* broken example
* learner prediction
* compiler behavior
* explanation
* corrected version
* reasoning rule

---

# PART 25 — Borrowing Experiments

Create interactive “change this line” experiments.

Examples:

```rust
let mut value = String::from("hello");

let r = &value;

value.push('!');

println!("{r}");
```

Ask:

> What happens if you move `println!("{r}")` above the mutation?

Another:

```rust
let mut value = String::from("hello");

let r1 = &value;
let r2 = &value;

println!("{r1}");
println!("{r2}");
```

Ask:

> What happens if `r2` becomes `&mut value`?

Another:

```rust
let mut value = String::from("hello");

let r = &mut value;
r.push('!');

let r2 = &mut value;
r2.push('?');
```

Ask:

> Why can two mutable references exist here even though Rust forbids two simultaneous mutable references?

These experiments should train cause-and-effect reasoning.

---

# PART 26 — Borrowing Detective

Create a major reasoning challenge.

For every snippet, the learner must determine:

* owner
* references
* reference type
* active borrows
* whether the code compiles
* where the conflict occurs
* why the conflict occurs
* how to fix it without blindly cloning

Example:

```rust
let mut name = String::from("Gus");

let first = &name;
let second = &name;

println!("{first}");
println!("{second}");

let edit = &mut name;
edit.push('!');
```

Ask:

> Does this compile?

Then:

```rust
let mut name = String::from("Gus");

let first = &name;

let edit = &mut name;
edit.push('!');

println!("{first}");
```

Ask the same questions.

Then progressively introduce:

* nested scopes
* functions
* mutable functions
* slices
* collections
* returned references

The final challenge should require the learner to trace borrowing without running the code.

---

# PART 27 — Borrowing Design Decisions

Do not teach borrowing merely as compiler rules.

Teach API design.

Given:

```rust
fn process(data: String)
```

ask:

> Does this function actually need ownership?

Then compare:

```rust
fn process(data: &String)
```

and:

```rust
fn process(data: &str)
```

For mutation:

```rust
fn process(data: &mut String)
```

Teach the learner to ask:

```text
Does the function need ownership?

        YES → take T

        NO → borrow &T

        NO, but must modify → borrow &mut T
```

This is one of the most practical outcomes of the module.

---

# PART 28 — Borrowing and Ownership Together

Bring the whole model together.

Given:

```rust
fn add_exclamation(text: &mut String) {
    text.push('!');
}

fn print_text(text: &str) {
    println!("{text}");
}

fn main() {
    let mut message = String::from("hello");

    print_text(&message);
    add_exclamation(&mut message);
    print_text(&message);
}
```

Ask the learner to explain every line in terms of:

* ownership
* immutable borrowing
* mutable borrowing
* ownership remaining with `message`
* when each borrow ends

The learner should now be able to describe the program without relying on memorized syntax.

---

# PART 29 — What Borrowing Does NOT Mean

Explicitly correct these misconceptions:

```text
"Borrowing means copying."

WRONG.

Borrowing provides access without transferring ownership.
```

```text
"A reference owns the value."

WRONG.

The original owner remains responsible.
```

```text
"&mut means multiple things can mutate the value."

WRONG.

A mutable reference is exclusive while active.
```

```text
"References are always valid until the end of the function."

WRONG.

The borrow checker tracks when references are actually needed.
```

```text
"Clone is the solution to borrowing."

WRONG.

Clone creates another owned value. It changes the ownership design.
```

---

# PART 30 — Final Borrowing Mental Model

End the instructional content with a compact model.

## Ownership

```text
WHO IS RESPONSIBLE?
```

## Immutable borrowing

```text
WHO MAY READ?
```

## Mutable borrowing

```text
WHO MAY EXCLUSIVELY MODIFY?
```

## Reference validity

```text
DOES THE BORROWED VALUE STILL EXIST?
```

## Borrow checker

```text
DO THESE ACCESS PATTERNS
SATISFY RUST'S SAFETY RULES?
```

Then give the learner this final reasoning procedure:

```text
TRACE THE VALUE

    ↓

WHO OWNS IT?

    ↓

WHO IS BORROWING IT?

    ↓

IS THE BORROW IMMUTABLE OR MUTABLE?

    ↓

WHEN IS THE BORROW USED?

    ↓

DO ANY OTHER BORROWS CONFLICT?

    ↓

DOES THE REFERENCED VALUE OUTLIVE THE REFERENCE?

    ↓

COMPILE OR ERROR?
```

The learner should be able to use this procedure on unfamiliar Rust code.

---

# FINAL MODULE CHALLENGE — BORROWING DETECTIVE

Build one large challenge consisting of multiple Rust snippets.

Difficulty should increase.

For every snippet the learner must predict:

1. Does it compile?
2. Who owns the value?
3. What references exist?
4. Which references are immutable?
5. Which references are mutable?
6. When does each borrow become active?
7. When does each borrow end?
8. Is there an aliasing conflict?
9. Is there a lifetime/validity problem?
10. What would be the minimal conceptual fix?

Do not accept only "it doesn't compile" as an answer.

The learner must explain WHY.

Include snippets involving:

* simple immutable borrowing
* multiple immutable references
* mutable references
* mutable + immutable conflicts
* sequential mutable borrows
* nested scopes
* functions
* returning references
* dangling references
* `String`
* `&str`
* slices
* vectors
* collections
* API design

The final challenge should feel like debugging real Rust rather than completing a syntax quiz.

---

# EXERCISE TYPES

Do not rely exclusively on code-writing exercises.

Use a mixture of:

## 1. Predict the Compiler

```text
COMPILES
or
DOES NOT COMPILE
```

Then require an explanation.

## 2. Trace the Borrow

Give code and ask:

```text
Owner:
Borrower:
Borrow type:
Borrow starts:
Borrow ends:
```

## 3. Fix the Borrow

Give intentionally invalid code.

The learner must fix it without automatically using `.clone()`.

## 4. Explain the Compiler

Show an error and ask:

> What rule did Rust enforce here?

## 5. Change One Line

Ask the learner to predict what changes if one statement moves.

## 6. API Design

Give a function and ask:

> Should this function take ownership, `&T`, or `&mut T`?

## 7. Memory / Reference Diagrams

Have learners interpret diagrams showing:

```text
OWNER
  ↓
VALUE

REFERENCE
  ↓
VALUE
```

## 8. Ownership vs Borrowing Classification

Give expressions and classify them:

```text
moves
copies
clones
borrows immutably
borrows mutably
```

---

# PROJECTS

The projects must be genuinely achievable after completing this module.

Do not artificially add advanced Rust just to make the projects sound impressive.

Each project must contain:

* Name
* Difficulty
* Estimated time
* Goal
* Requirements
* Concepts practiced
* Constraints
* Example interaction/output where useful
* Stretch goals

Projects should progressively increase in complexity.

## Project 1 — Borrowing Playground

Difficulty: ★

Build a small CLI program containing functions that:

* inspect strings through `&str`
* modify strings through `&mut String`
* demonstrate ownership versus borrowing

Focus:

* `&T`
* `&mut T`
* function parameters
* ownership remaining with caller

---

## Project 2 — String Inspector

Difficulty: ★

Build a program that receives a `String` and uses borrowed functions to calculate:

* length
* first character
* last character
* whether a substring exists

Do not consume the original string.

Focus:

* immutable borrowing
* `&str`
* slices
* returning borrowed slices where appropriate

---

## Project 3 — Mutable Text Editor

Difficulty: ★★

Build a simple text transformation tool.

Functions should borrow a mutable `String` and perform operations such as:

* append text
* remove trailing whitespace
* replace a character
* add prefixes/suffixes

Focus:

* `&mut String`
* mutable borrowing
* sequential mutable borrows
* ownership remaining with caller

---

## Project 4 — Shopping List Editor

Difficulty: ★★

Build a CLI shopping-list application.

The program should use borrowed functions to:

* display items
* search for an item
* modify an item
* inspect the list without taking ownership unnecessarily

Focus:

* borrowing collections
* immutable references
* mutable references
* API design

---

## Project 5 — Word Finder

Difficulty: ★★

Build a function that receives text and returns a borrowed slice representing a selected word.

Example:

```text
Input:
"Rust makes systems programming fun"

Output:
"systems"
```

Focus:

* `&str`
* slices
* borrowed return values
* reference validity

Do not introduce lifetime annotations unless genuinely required by the chosen implementation.

---

## Project 6 — Inventory Inspector

Difficulty: ★★★

Build an inventory represented by a collection.

Functions should:

* search for items
* inspect items
* modify quantities
* find an item without taking ownership
* mutate an item through a controlled mutable borrow

Focus:

* borrowing collection elements
* immutable vs mutable borrows
* avoiding conflicting borrows

---

## Project 7 — Text Processing Pipeline

Difficulty: ★★★

Build a small text-processing pipeline.

Example:

```text
input
 ↓
trim
 ↓
find words
 ↓
inspect
 ↓
transform
 ↓
output
```

Design the functions so that they borrow where ownership is unnecessary.

Focus:

* `&str`
* `&mut String`
* function API design
* sequential borrowing
* ownership/borrowing decisions

---

## Project 8 — Mini Document Analyzer

Difficulty: ★★★

Build a document analyzer that can:

* count words
* find the longest word
* find a requested word
* return borrowed slices
* perform optional transformations

Focus:

* slices
* borrowed return values
* collection borrowing
* API design

---

## Project 9 — Borrow Checker Challenge Lab

Difficulty: ★★★★

Build an interactive CLI containing intentionally broken Rust snippets.

The user sees a snippet and must identify:

* owner
* borrow
* borrow type
* conflict
* fix

The application can present predefined scenarios rather than dynamically compiling Rust.

Focus:

* borrowing reasoning
* compiler diagnostics
* ownership/borrowing relationships

---

## Project 10 — Borrowing Lab

Difficulty: ★★★★☆

Build a small Rust program that combines:

* owned `String`
* `&str`
* immutable borrowing
* mutable borrowing
* slices
* collection borrowing
* functions taking references
* functions returning references where appropriate

The project should include multiple operations over the same data without unnecessarily cloning or transferring ownership.

The learner should be required to justify the ownership/borrowing design of their functions.

Include a final written task:

> For every major function in your program, explain why it takes ownership, `&T`, or `&mut T`.

This is important.

The learner should demonstrate understanding rather than merely producing working code.

---

# TEACHING REQUIREMENTS

The entire module must follow these rules.

## Rule 1 — Never teach syntax without motivation

Do not begin with:

> "`&` creates a reference."

Begin with:

> "The function needs to inspect this value, but it shouldn't become responsible for it."

Then introduce the syntax.

---

## Rule 2 — Make the learner predict

Before revealing an answer, ask questions such as:

> Does this compile?

> Who owns the value here?

> Is this borrow still active?

> Can this mutation happen?

> What will happen if this line moves?

---

## Rule 3 — Explain compiler errors

Never simply say:

> "Rust doesn't allow this."

Explain:

* what access was requested
* what existing borrow conflicts with it
* why that conflict could be dangerous
* what Rust is protecting
* what design alternatives exist

---

## Rule 4 — Don't overuse `.clone()`

If `.clone()` fixes a compiler error, explicitly ask:

> Is creating another owned value actually what this program needs?

If not, teach the borrowing solution.

---

## Rule 5 — Don't teach incorrect simplifications

Never say:

* "references are pointers, full stop"
* "references always last until the end of the scope"
* "mutable references can only exist once in the entire program"
* "borrowing means copying"
* "the reference owns the data"
* "Rust doesn't allow multiple references"
* "just use clone when borrowing gets difficult"

References may be compared to pointers as a mental model, but make clear that Rust references carry language-level validity and borrowing guarantees.

---

# DIAGRAM REQUIREMENTS

Use diagrams when they make ownership relationships easier to understand.

Useful diagrams include:

```text
OWNER
  │
  ↓
VALUE
```

```text
OWNER
  │
  ├────→ IMMUTABLE REFERENCE
  │
  ├────→ IMMUTABLE REFERENCE
  │
  └────→ IMMUTABLE REFERENCE
```

versus:

```text
OWNER
  │
  └────→ MUTABLE REFERENCE
```

and:

```text
String
┌────────────────────────┐
│ ptr │ len │ capacity   │
└───────────┬────────────┘
            │
            ↓
        HEAP DATA
```

Do not create diagrams merely for decoration.

Every diagram must answer a specific conceptual question.

---

# CONTENT STYLE

The writing should feel like an experienced Rust developer teaching another programmer.

Use:

* concise explanations
* concrete code
* compiler experiments
* mental models
* tables where comparisons help
* diagrams where relationships are spatial
* increasingly difficult exercises
* deliberate mistakes
* practical API-design decisions

Avoid:

* generic motivational paragraphs
* repetitive definitions
* excessive emojis
* fake enthusiasm
* filler introductions
* giant walls of prose
* repeating the same explanation five different ways
* "Rust is an amazing language" style writing
* vague statements like "this is important"
* unexplained jargon
* blindly telling learners to memorize rules

Every lesson should answer at least one concrete question.

The module should be **dense, technical, and practical**, but not artificially long.

---

# PREREQUISITES

Assume the learner already understands:

* variables
* mutability
* functions
* parameters
* return values
* scope
* `String`
* `Vec`
* ownership
* move
* `Copy`
* `Clone`
* `drop`

Do not reteach those modules.

Use them as prerequisites and build directly on top of them.

---

# MODULE OUTCOME

At the end of Module 06, the learner should be able to look at unfamiliar Rust code and ask:

```text
Who owns this value?

Who is borrowing it?

Is this an immutable or mutable borrow?

Can these borrows coexist?

When is the borrow actually used?

Does the reference remain valid?

Does this function need ownership?

Would &T be enough?

Would &mut T be appropriate?

Should this function return owned data or a reference?

Why is the borrow checker rejecting this?
```

The learner should not merely know what `&` and `&mut` mean.

They should be able to **reason about borrowing before compiling the code**.

The final conceptual bridge should be:

```text
OWNERSHIP

Who is responsible for the value?


BORROWING

Who can temporarily access the value?


NEXT QUESTION

What happens when we need to describe
exactly how long a reference is allowed to live?

                ↓

             LIFETIMES
```

Do not teach lifetime annotations deeply in this module.

End with the learner understanding why the next concept exists.
