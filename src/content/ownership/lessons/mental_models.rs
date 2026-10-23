//! Module 05 — Ownership, lessons 19–24.
//!
//! The consolidation lessons: mental models, the myths, the compiler errors,
//! deliberate debugging practice, the recap, and the problem that borrowing
//! exists to solve.

use crate::content::types::{
    breakdown, broken, broken_text, callout, checklist, code, diagram, fixed, labelled, table,
    text_block, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        mental_models(),
        myths(),
        compiler_errors(),
        debugging_exercises(),
        what_you_can_do_now(),
    ]
}

fn mental_models() -> SeedLesson {
    SeedLesson::new(
        "Ownership Mental Models",
        vec![
            labelled(
                "THE IDEA",
                "Three pictures that answer most ownership questions faster than the rules do. Rules \
                 tell you what is allowed. Models tell you what is happening.",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "You now have the facts. What follows is the compression — three ways of holding the same \
                 idea in your head so you can reach for them when the compiler says something you did not \
                 expect. Pick whichever one clicks. Most people end up using all three without noticing.",
            ),

            labelled(
                "MODEL A — The name tag",
                "Every value has a tag attached to it with a name written on it. Ownership is holding the \
                 tag. When ownership moves, the tag is physically transferred to another hand, and the old \
                 holder is left empty-handed — the value never moved, only the tag did.",
            ),
            diagram(
                "model A — before and after",
                "  let s1 = String::from(\"hello\");\n  let s2 = s1;\n\n  BEFORE                          AFTER\n  ┌────────────────┐             ┌────────────────┐\n  │ OWNER: s1      │             │ OWNER: s2      │\n  ├────────────────┤             ├────────────────┤\n  │ String data    │             │ String data    │\n  └────────────────┘             └────────────────┘\n\n  s1 is not a destroyed variable.\n  s1 is a name that is no longer holding a tag.\n  the value is untouched, sitting where it always was.",
            ),

            labelled(
                "MODEL B — One responsible person",
                "Some things must be cleaned up exactly once. Two people both believing they are \
                 responsible is not a mild problem — it is a corrupted heap. Ownership is the assignment \
                 of that responsibility, and the rule that there is exactly one owner is what keeps the \
                 question from ever coming up.",
            ),
            diagram(
                "model B — the only two acceptable states",
                "  one person responsible      nobody responsible\n  ┌────────┐                   ┌────────┐\n  │   s    │──> resource       │ (gone) │──> resource   ← leak\n  └────────┘                   └────────┘\n  ✓ fine\n\n  two people responsible\n  ┌────────┐\n  │   s    │──> resource  ← freed twice. corrupts unrelated memory.\n  └────────┐\n  ┌────────┐\n  │   t    │──>\n  └────────┘\n  ✗ forbidden, and not by convention",
            ),

            labelled(
                "MODEL C — Ownership travels",
                "The value does not have a permanent address in the program. It has a *current* holder, \
                 and that holder changes. Variables to functions, functions to return values, return \
                 values to variables. At every point in the program exactly one name is responsible, and \
                 \"where does the value live forever\" is the wrong question to be asking.",
            ),
            diagram(
                "model C — a value's entire career",
                "  String::from(\"hello\")\n          │  created; the temporary owns it\n          ▼\n       let s                  s owns it\n          │\n          │  passed by value\n          ▼\n   fn shout(t: String)      t owns it\n          │\n          │  returns t\n          ▼\n       let s = shout(s)      s owns it again — same value, new owner\n          │\n          ▼\n       end of scope           dropped, once, by s\n\n  at no point were there two owners.\n  at no point was anything copied.\n  the value simply had a succession of holders.",
            ),
            callout(
                "Which model to reach for",
                "The name tag when you are confused about *what happened to the variable*. The responsible \
                 person when you are trying to understand *why the rule exists*. Ownership travels when \
                 you are tracing a value through a function and losing track of it. They are three views \
                 of one design, and you will find yourself using all three within a week.",
            ),
            callout(
                "Practice — Which model explains this?",
                "For each situation, name which model you would reach for and say why: (a) a `String` \
                 moves into a function and you want to know if you still have it (b) someone asks why \
                 Rust forbids two owners of a `Vec` (c) a value passes through three functions and you \
                 cannot work out which one is responsible now. Then invent a fourth situation and decide \
                 which model covers it.",
            ),
            callout(
                "Mini challenge — Explain it to a non-programmer",
                "Choose the simplest of the three models and explain ownership to someone who has never \
                 written code, in under a minute, without using the words 'heap', 'allocation', or \
                 'memory'. If you cannot, you have learned the vocabulary but not the idea. Then explain \
                 it to a programmer who knows C, using only the words they already use, without saying \
                 'borrow checker' once. Both audiences are a real test.",
            ),
        ],
    )
}

fn myths() -> SeedLesson {
    SeedLesson::new(
        "Rust Ownership Myths",
        vec![
            labelled(
                "THE IDEA",
                "Eight things people are confident about ownership that are not true. Most of them are \
                 half-true, and a half-truth is worse than a lie because it survives contact with code \
                 until the day it does not.",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "You will meet all eight of these within a month of writing Rust, usually from someone \
                 who learned it last week. Having them written down means you can check the next one you \
                 hear against a list instead of against a feeling.",
            ),

            labelled(
                "MYTH 1 — \"Rust copies everything\"",
                "It does the opposite. `String`, `Vec<T>`, `Box<T>` and every other owned heap-backed type \
                 move on assignment. The types that copy are the ones that implement `Copy`, and they copy \
                 because duplicating them is free and harmless.",
            ),
            labelled(
                "MYTH 2 — \"Rust never copies\"",
                "It copies constantly — `let y = x;` with an `i32` is a copy, and so is passing any `Copy` \
                 type to a function. The compiler is copying where copying is free and transferring where \
                 it is not.",
            ),
            labelled(
                "MYTH 3 — \"Clone is always bad\"",
                "`clone()` is a legitimate tool whenever you actually want two independent owners, and it \
                 is the right answer far more often than its reputation suggests. The bad habit is not \
                 using clone — it is reaching for it because you cannot be bothered to think about \
                 ownership, and then never coming back.",
            ),
            labelled(
                "MYTH 4 — \"A move means the data physically moved\"",
                "It does not. The heap allocation stays exactly where it was. What moved is a handful of \
                 words on the stack — a pointer, a length, maybe a capacity. Moves are cheap precisely \
                 because no data is relocated, and if moves *did* relocate data then borrowing would be \
                 pointless and `String` would be unusable.",
            ),
            labelled(
                "MYTH 5 — \"Only heap values have ownership\"",
                "Ownership is a language-level model, not a synonym for \"who frees the memory\". `let x \
                 = 5;` has an owner, participates in the rules, and goes out of scope like anything else. \
                 It just happens to require no cleanup, so the rules are invisible.",
            ),
            labelled(
                "MYTH 6 — \"The variable is the value\"",
                "The variable is a name that refers to a value. This distinction is invisible for `i32` \
                 and lethal for `String`. When a `String` moves, the value does not change and the \
                 variable is not deleted — the name stops referring to the value.",
            ),
            labelled(
                "MYTH 7 — \"Rust has garbage collection\"",
                "It does not. There is no collector, no tracing, no pause, and no heap-walking pass. \
                 Cleanup happens at a point the compiler determined before the program started, and it is \
                 handled by inserting instructions at scope ends, not by a background process. If you can \
                 watch a Rust program's memory and never see a pause, that is because there is nothing \
                 running to pause.",
            ),
            labelled(
                "MYTH 8 — \"Ownership means only one reference\"",
                "Ownership means one *owner*. Multiple references are exactly what Rust's borrowing system \
                 is for — that is the entire next module. You can have any number of people reading the \
                 same value at once, and exactly one of them responsible for it. Those are different \
                 counts, and Rust keeps them separate.",
            ),
            callout(
                "The pattern behind all eight",
                "Each myth comes from correctly noticing half of something. Copies exist, so everything \
                 copies. Ownership exists, so it must be about ownership of memory and nothing else. \
                 Moves exist, so the data must move. When a claim feels obviously true and you are about \
                 to repeat it to someone else, that is exactly when to check it.",
            ),
            callout(
                "Practice — Correct the record",
                "Pick any three of these eight. For each, write the myth as a beginner would say it, then \
                 write the correction in one sentence a beginner would believe. Then find a piece of code \
                 in your own history where you believed the myth, and say which correction would have \
                 saved you the time.",
            ),
            callout(
                "Mini challenge — Write myth nine",
                "Every myth on this list came from a reasonable person misreading a real mechanism. Write \
                 a ninth one that is not here yet — something you can currently imagine yourself \
                 believing about ownership. Leave it somewhere you will find it again. Odds are it will \
                 turn out to be true, and that is the interesting part.",
            ),
        ],
    )
}

fn compiler_errors() -> SeedLesson {
    SeedLesson::new(
        "Common Ownership Compiler Errors",
        vec![
            labelled(
                "THE IDEA",
                "Five errors cover almost every ownership mistake you will make. Each one is a \
                 statement about a *rule*, not a random complaint, and once you can read them that way you \
                 will not need to look them up.",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "A good error message tells you the rule, the evidence, and the fix. Rust's ownership \
                 errors are unusually good at this — they annotate the move and then annotate the use, and \
                 draw you a line between them. The habit worth building is reading it *bottom-up* from \
                 the annotations rather than top-down from the error text.",
            ),

            labelled(
                "ERROR 1 — Use after move",
                "The single most common ownership error you will see. The message points at two places: \
                 where the value moved, and where you used it afterwards.",
            ),
            broken(
                "fn main() {\n    let s = String::from(\"hello\");\n    let s2 = s;\n\n    println!(\"{s}\");\n}",
            ),
            broken_text(
                "error[E0382]: borrow of moved value: `s`\n --> src/main.rs:4:19\n  |\n2 |     let s = String::from(\"hello\");\n  |         -- move occurs because `s` has type `String`,\n  |            which does not implement the `Copy` trait\n3 |     let s2 = s;\n  |              -- value moved here\n4 |     println!(\"{s}\");\n  |                ^^ value borrowed here after move\n  |\nhelp: consider cloning the value if the performance cost is acceptable\n  |\n3 |     let s2 = s.clone();\n  |                ++++++++",
            ),
            text_block(
                "Read it in this order: the reason (`does not implement the Copy trait`), then the move \
                 site, then the use site. The reason tells you the rule that fired. Everything after that \
                 is just bookkeeping. And note the compiler's suggestion is *clone* — helpful, and not \
                 always what you want. The other two options are to reorder your code or to borrow \
                 instead, and the compiler cannot know which you meant.",
            ),

            labelled(
                "ERROR 2 — Moved into a function",
                "Identical error, different location. The move happened at the call, not at an assignment, \
                 so people tend to look for the problem in the function body. It is in the signature.",
            ),
            broken(
                "fn consume(s: String) {\n    println!(\"{s}\");\n}\n\nfn main() {\n    let s = String::from(\"hello\");\n    consume(s);\n    println!(\"{s}\");\n}",
            ),
            text_block(
                "Ask two questions when you see this: *does the function need to keep the value?* If not, \
                 it should take a reference and the error disappears. If it does, then the caller's copy \
                 is genuinely gone, and the answer is to have the function hand it back — which costs \
                 nothing.",
            ),

            labelled(
                "ERROR 3 — Moving the same value twice",
                "You moved it once, so the second move is a use of a value that is no longer yours. The \
                 compiler is enforcing rule 2 in its purest form.",
            ),
            broken(
                "fn main() {\n    let s = String::from(\"hello\");\n\n    let a = s;\n    let b = s;   // error[E0382] — s was already moved into a\n\n    println!(\"{a} {b}\");\n}",
            ),

            labelled(
                "ERROR 4 — Assigning to a moved variable",
                "A different error code, and worth recognising: this is **E0384**, *assigning to a value \
                 after it was moved*, not E0382. It fires when you try to give a disarmed variable a new \
                 value, which is the one thing you genuinely cannot do.",
            ),
            broken(
                "fn main() {\n    let s = String::from(\"hello\");\n    let moved = s;\n\n    s = String::from(\"goodbye\");   // error[E0384]\n\n    println!(\"{s} {moved}\");\n}",
            ),
            fixed(
                "fn main() {\n    let s = String::from(\"hello\");\n    let moved = s;\n\n    let s = String::from(\"goodbye\");   // shadowed: a new binding, not a reassignment\n\n    println!(\"{s} {moved}\");\n}",
            ),
            text_block(
                "The fix is shadowing rather than assignment. `let s = String::from(\"goodbye\");` creates \
                 a fresh binding that legally takes a value, where `s = ...` tries to resurrect a binding \
                 that has nothing to resurrect. If you have seen E0384 and assumed it was a typo for \
                 E0382, that is a very common and very understandable confusion.",
            ),

            labelled(
                "ERROR 5 — Confusion between Copy and Clone",
                "Not one error but a family. The pattern is always the same: a line that works fine with \
                 an `i32` fails with a `String`, and the question is which of the three behaviours you are \
                 actually looking at.",
            ),
            table(
                &["What you wrote", "What you meant", "What to do"],
                &[
                    &["`let b = a;` with a `String`", "Two independent values", "`a.clone()` — and consider borrowing instead"],
                    &["`let b = a;` with an `i32`", "Two values", "Nothing. It already works. This is `Copy`."],
                    &["`let b = a.clone();` with a `String`", "Two values, knowingly", "Nothing. That is correct code."],
                    &["`let b = a;` then using both", "To keep `a`", "Move it later, or clone it deliberately, or borrow it"],
                ],
            ),
            callout(
                "Read it bottom-up, and annotate the reason",
                "The habit that turns these from obstacles into information: find the line in the error \
                 that starts with `--` or `^^` and read that annotation, not the error text. The error \
                 text names the symptom. The annotation names the rule. Every ownership error in this \
                 module contains a sentence explaining why, and reading it is faster than guessing.",
            ),
            callout(
                "Practice — Diagnose without running",
                "Five snippets. For each: does it compile, which error code, which rule fired, and what \
                 are the two realistic fixes? (a) `let s = String::new(); let t = s; println!(\"{s}\");` \
                 (b) `let s = String::new(); let t = s; let u = s;` (c) `let v = vec![1]; let w = v; \
                 v = vec![2];` (d) `let s = String::new(); let t = s.clone(); println!(\"{t}\");` \
                 (e) `let n = 5; let m = n; n = 7;` Item (e) is the control: it should fail, and for a \
                 different reason than you might expect.",
            ),
            callout(
                "Mini challenge — Write the error yourself",
                "Deliberately trigger all five errors in a scratch program, one at a time, and save the \
                 full compiler output for each. Then delete every line of the error messages from the \
                 output and see whether you can reconstruct the annotations from the code alone. If you \
                 can, you are genuinely reading the compiler rather than pattern-matching its text — which \
                 is the difference between knowing the five common errors and knowing ownership.",
            ),
        ],
    )
}

fn debugging_exercises() -> SeedLesson {
    SeedLesson::new(
        "Ownership Debugging Exercises",
        vec![
            labelled(
                "THE IDEA",
                "Predicting the compiler is a trainable skill, and like any skill it needs reps in the \
                 form that matters: a small program, an honest guess, then a check. Here is the procedure.",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Everything so far has been recognition — someone showed you a moved value and you worked \
                 out what happened. This lesson is generation: you produce the outcome, and then check. \
                 The gap between your prediction and the compiler is the useful part. If you are right, \
                 move on quickly. If you are wrong, that gap is the exact edge of your model, and it is \
                 worth finding.",
            ),
            diagram(
                "the nine questions — the ownership checklist",
                "  1. what type is this?\n  2. does that type own heap data?\n  3. does it implement Copy?\n  4. is this line an assignment?\n  5. is there a .clone() anywhere in the expression?\n  6. is a value entering a function?\n  7. is a value returning?\n  8. has the owner gone out of scope yet?\n  9. has this value already been moved?\n\n  questions 1–5 decide the current line.\n  questions 6–7 decide the next line.\n  questions 8–9 decide whether the line is legal at all.",
            ),
            breakdown(
                "worked example — run the checklist",
                "fn main() {\n    let name = String::from(\"Gus\");\n    let shout = shout_it(name);\n    println!(\"{name} {shout}\");\n}\n\nfn shout_it(s: String) -> String {\n    s.to_uppercase()\n}",
                &[
                    "Q1/Q2: `String` — owns heap data. It is the interesting case by definition.",
                    "Q3: does `String` implement `Copy`? No. So passing it by value moves it.",
                    "Q6: `shout_it(name)` — the argument moves into the parameter. `name` is now disarmed.",
                    "Q7: the function returns a new `String` — `shout` owns it.",
                    "Q9: line 4 uses `name` — it was moved on line 3. **This does not compile.**",
                    "The three real fixes: print `name` before the call, clone it, or borrow in `shout_it`.",
                ],
            ),
            code(
                "rust",
                "fn main() {\n    let name = String::from(\"Gus\");\n    println!(\"{name}\");           // fine, before the move\n\n    let shout = shout_it(name);  // moves\n    println!(\"{shout}\");\n}\n\nfn shout_it(s: String) -> String {\n    s.to_uppercase()\n}",
            ),
            text_block(
                "Three options, and choosing between them is a design decision rather than a lookup. \
                 Reorder if the value was not needed afterwards. Clone if you genuinely want a second \
                 independent owner and the data is small. Borrow if the function only needs to look — \
                 which is the common answer, and the reason module 6 exists.",
            ),
            table(
                &["Situation", "Reorder", "Clone", "Borrow"],
                &[
                    &["Function only reads the value", "sometimes", "wasteful", "correct"],
                    &["Value is needed again immediately after", "awkward", "fine if small", "correct"],
                    &["Function must keep the value past return", "no", "correct", "will not work — it outlives the borrow"],
                    &["Large data, still needed", "maybe", "expensive", "correct"],
                ],
            ),
            callout(
                "Practice — Nine questions on a real snippet",
                "Take this and answer all nine checklist questions in writing, in order, before deciding \
                 anything: `fn add_tag(mut s: String, tag: &str) -> String { s.push_str(tag); s }` called \
                 as `add_tag(name, \"!\"); println!(\"{name}\");`. Then the same for a version taking \
                 `&mut String`. Notice that question 3 has a different answer depending on the parameter, \
                 and that this is visible entirely from the signature.",
            ),
            callout(
                "Mini challenge — Predict ten, check once",
                "Write ten one-line programs, each involving a move, a copy, or a clone. For every one, \
                 predict: does it compile, and if not, which error code. Then run all ten in one go. Score \
                 yourself. Below seven out of ten, redo the ones you missed and find the single question \
                 from the checklist that you skipped. That question is the one to memorise.",
            ),
        ],
    )
}

fn what_you_can_do_now() -> SeedLesson {
    SeedLesson::new(
        "What You Can Do Now",
        vec![
            labelled(
                "THE IDEA",
                "Ownership is not finished until you can use it to *predict* rather than to react. This \
                 is the whole module in one checklist, and the only honest way to use it is to try each \
                 item on code you have not seen before.",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "You arrived knowing that Rust has ownership. You are leaving able to look at a line of \
                 code and say what will happen to a value before running it — and, more usefully, say why \
                 the language is built this way. The difference is the difference between memorising a \
                 rule and having a model you can apply to code you have never seen.",
            ),
            checklist(
                "What you can do now",
                &[
                    "Explain why Rust needs ownership at all, without saying 'memory safety' as the whole answer",
                    "Explain the stack and heap distinction, and say why it is a simplification worth making",
                    "Distinguish a name, a binding, and a value",
                    "Say when a value is dropped, and in what order",
                    "State the three ownership rules and explain what each one prevents",
                    "Explain why `String` behaves differently from `i32`",
                    "Predict what a move does to the original variable",
                    "Explain `Copy` as a decision the type makes, not a property of primitives",
                    "Explain `Clone` as an explicit request, and say what it costs",
                    "Tell copy, move, and clone apart from the type alone",
                    "Track ownership through a function call and back out again",
                    "Read a use-after-move error and say which rule fired",
                    "Draw a stack and heap diagram for a move and for a clone",
                    "Decide deliberately whether a function should take ownership, copy, or clone",
                ],
            ),
            diagram(
                "the whole module in one picture",
                "  A VALUE\n     │\n     ▼\n   has an OWNER\n     │\n     ├── assignment ──► MOVE      (old binding disarmed)\n     ├── Copy type ──► COPY      (both names stay live)\n     └── .clone() ───► CLONE     (two real values)\n\n  OWNER GOES OUT OF SCOPE\n            │\n            ▼\n          DROP\n            │\n            ▼\n    RESOURCE CLEANUP\n\n  and if two owners were ever allowed, the double free\n  at the bottom of that diagram would be waiting for you.",
            ),
            text_block(
                "One last precision, because it will keep you honest for the rest of the course: ownership \
                 is not a heap-management scheme. It is Rust's model for **responsibility and validity of \
                 values**, and memory cleanup is one of the consequences rather than the definition. `let x \
                 = 5;` has an owner. It goes out of scope. Nothing needs freeing. The model was still \
                 running the whole time — there was just nothing for it to do.",
            ),
            callout(
                "Practice — Cold review",
                "Close the module. Write, from memory, a program that moves a `String` through two \
                 functions and drops it at the end of a nested scope — and annotate every line with the \
                 owner. Then find the one thing you got wrong. That one thing is the part of ownership you \
                 have not actually learned yet, and it is worth ten more exercises.",
            ),
            code(
                "rust",
                "fn main() {\n    let title = String::from(\"BORROW_\");   // title owns it\n\n    {\n        let shouted = shout(title);              // moved into shout, then back out\n        println!(\"{shouted}\");\n    }                                           // shouted dropped HERE\n\n    // the outer `title` no longer owns anything. it was moved on\n    // line 6 and never returned — which is fine, because you\n    // never used it again. that is the whole trade.\n}\n\nfn shout(s: String) -> String {\n    s.to_uppercase()\n}",
            ),
            callout(
                "Mini challenge — The problem borrowing solves",
                "You have hit this four times now: a function that only needs to *read* a `String`, and the \
                 caller loses it anyway. `total(values: Vec<i32>)` cannot be called twice. `println_it(name)` \
                 would consume the name. `shout_it(s: String)` would work right up until you wanted the \
                 original back. Every workaround so far has been a cost — a clone, a reorder, a slightly \
                 worse design — and you have probably noticed that the compiler's own suggestion is always \
                 a reference, which you have not been taught yet. Write down that problem in as much \
                 detail as you can, because it is the last thing in this module and the first thing in the \
                 next one.",
            ),
            diagram(
                "the bridge",
                "  I have a String.\n       │\n       ▼\n  I want a function to read it.\n       │\n       ▼\n  I do NOT want to give up ownership.\n       │\n       ▼\n  ...so how do I hand out access\n  without handing over the keys?\n\n       OWNERSHIP answered:\n         \"exactly one owner\"\n\n       that is correct, and it is also\n       the reason I cannot do what I want.\n\n       ┌────────────────────────────────┐\n       │  OWNERSHIP   →   BORROWING     │\n       │  one owner   →  temporary access│\n       └────────────────────────────────┘",
            ),
        ],
    )
}
