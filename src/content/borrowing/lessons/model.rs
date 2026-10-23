//! Module 06 — Borrowing, lessons 25–30.
//!
//! Consolidation. The experiments train cause-and-effect, the detective trains
//! tracing, and the last four parts turn the rules into design decisions and a
//! single model the learner can carry out of the module.

use crate::content::types::{
    broken, callout, checklist, code, diagram, labelled, paragraphs, steps, table, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        experiments(),
        detective(),
        design_decisions(),
        ownership_together(),
        what_it_does_not_mean(),
        final_model(),
    ]
}

fn experiments() -> SeedLesson {
    SeedLesson::new(
        "Borrowing Experiments",
        vec![
            labelled(
                "THE IDEA",
                "These are change-one-line experiments. Read each program, predict what happens, then \
                 move the named line and predict again. The point is cause and effect: which change \
                 turns a rejection into an acceptance.",
            ),
            labelled("Experiment 1 — move the use", "This program is rejected. Predict what happens if you move `println!(\"{r}\")` above the mutation."),
            broken(
                "fn main() {\n    let mut value = String::from(\"hello\");\n\n    let r = &value;\n\n    value.push('!');\n\n    println!(\"{r}\");\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Moving the print above the push ends the immutable borrow before the write, so the \
                 write no longer overlaps a live reader. The program is accepted. Nothing about the \
                 value changed — only the ordering of a use.",
            ),
            labelled("Experiment 2 — upgrade a reader to a writer", "Two readers are fine here. Predict what happens if `r2` becomes `&mut value`."),
            code(
                "rust",
                "fn main() {\n    let mut value = String::from(\"hello\");\n\n    let r1 = &value;\n    let r2 = &value;\n\n    println!(\"{r1}\");\n    println!(\"{r2}\");\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Turning `r2` into `&mut value` creates a reader and a writer whose regions overlap, \
                 because both are used at the end. The visit to Experiment 1 shows the escape: put a \
                 gap between them, or reorder so the reader finishes first. A mutable borrow is not \
                 forbidden forever — only while another view is live.",
            ),
            labelled("Experiment 3 — two writers that do not overlap", "Rust forbids two simultaneous mutable references, yet this compiles. Predict why."),
            code(
                "rust",
                "fn main() {\n    let mut value = String::from(\"hello\");\n\n    let r = &mut value;\n    r.push('!');\n\n    let r2 = &mut value;\n    r2.push('?');\n\n    println!(\"{value}\");\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "There are two mutable references, but they are not simultaneous. `r` is created, used \
                 once, and finished before `r2` exists. The rule is about overlap, not count. This is \
                 the same liveness idea from Part 08, now doing real work.",
            ),
            callout(
                "Predict, then move one line",
                "The habit worth building is to change exactly one thing and predict the outcome \
                 before compiling. Each experiment above is decided by a single use moving across a \
                 borrow boundary. That is the entire mechanism the borrow checker reasons with.",
            ),
        ],
    )
}

fn detective() -> SeedLesson {
    SeedLesson::new(
        "Borrowing Detective",
        vec![
            labelled(
                "THE IDEA",
                "For every snippet, determine the owner, the references, whether each is immutable or \
                 mutable, when each borrow becomes active, when it ends, whether there is a conflict, \
                 and the minimal conceptual fix. 'It does not compile' is not an answer — say why.",
            ),
            labelled("Case 1 — sequential borrows", "Predict, then explain."),
            code(
                "rust",
                "fn main() {\n    let mut name = String::from(\"Gus\");\n\n    let first = &name;\n    let second = &name;\n\n    println!(\"{first}\");\n    println!(\"{second}\");\n\n    let edit = &mut name;\n    edit.push('!');\n\n    println!(\"{name}\");\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Compiles. The two immutable borrows finish at their prints, before `edit` is created. \
                 The mutable borrow then has the value to itself. Three borrows, no overlap.",
            ),
            labelled("Case 2 — a reader that outlives a writer", "Predict, then explain."),
            broken(
                "fn main() {\n    let mut name = String::from(\"Gus\");\n\n    let first = &name;\n\n    let edit = &mut name;\n    edit.push('!');\n\n    println!(\"{first}\");\n}",
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Rejected. `first` is used after `edit` takes exclusive access, so the reader's region \
                 spans the write. The fix is to print `first` before the edit, or drop it first. The \
                 owner is `name` throughout; the conflict is between two borrows, not with the owner.",
            ),
            labelled(
                "Climb the difficulty ladder",
                "The module challenge below extends these two cases with nested scopes, functions, \
                 mutable functions, slices, collections, and returned references. The reasoning is the \
                 same at every rung — only the number of moving parts grows.",
            ),
            callout(
                "Trace, do not run",
                "The point of the detective is to reach the answer without executing the code. If you \
                 can only tell whether a program compiles by compiling it, the model is not yet \
                 load-bearing. Slow down and trace the borrows by hand.",
            ),
        ],
    )
}

fn design_decisions() -> SeedLesson {
    SeedLesson::new(
        "Borrowing Design Decisions",
        vec![
            labelled(
                "THE IDEA",
                "Borrowing is not only compiler law. It is an API-design vocabulary. The parameter \
                 type you choose tells every caller what your function does with their value.",
            ),
            diagram(
                "The three questions",
                "Does the function need ownership?\n\n        YES → take T\n\n        NO  → borrow &T\n\n        NO, but must modify → borrow &mut T",
            ),
            table(
                &["Signature", "Use when", "Example"],
                &[
                    &["`fn process(data: String)`", "the value is consumed or stored", "a builder taking a field"],
                    &["`fn process(data: &String)`", "you only read, and need `String` methods", "rare — prefer `&str`"],
                    &["`fn process(data: &str)`", "you only read text", "almost every text function"],
                    &["`fn process(data: &mut String)`", "you modify in place", "an append or normalise step"],
                ],
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The starting point is always 'does this function need to keep the value?'. If not, \
                 borrow. Then ask whether it must change the value; if so, `&mut`. Taking ownership \
                 when a borrow would do forces every caller to hand over something they wanted to keep, \
                 and the cost lands on them at the call site.",
            ),
            callout(
                "The most practical outcome",
                "This procedure is the module's most reusable result. Applied to every function you \
                 write, it keeps signatures honest: a reader reads, a writer writes, and an owner \
                 owns. When a signature is surprising, it is usually because one of these three jobs \
                 was mislabelled.",
            ),
        ],
    )
}

fn ownership_together() -> SeedLesson {
    SeedLesson::new(
        "Borrowing and Ownership Together",
        vec![
            labelled(
                "THE IDEA",
                "Bring the whole model together on one small program. Every line is either ownership, \
                 immutable borrowing, or mutable borrowing, and you should be able to name which.",
            ),
            code(
                "rust",
                "fn add_exclamation(text: &mut String) {\n    text.push('!');\n}\n\nfn print_text(text: &str) {\n    println!(\"{text}\");\n}\n\nfn main() {\n    let mut message = String::from(\"hello\");\n\n    print_text(&message);\n    add_exclamation(&mut message);\n    print_text(&message);\n}",
            ),
            steps(&[
                "`message` owns the String. It is declared `mut` because it will be modified.",
                "`print_text(&message)` borrows immutably for the duration of the call, then releases it.",
                "`add_exclamation(&mut message)` takes exclusive mutable access, appends, and returns it.",
                "`print_text(&message)` borrows immutably again — safe, because the write is over.",
                "At the end of `main`, `message` is still the sole owner and frees the String once.",
            ]),
            labelled(
                "LET'S BREAK IT DOWN",
                "Notice that ownership never left `main`. The two functions borrowed what they needed: \
                 one to read, one to write. Both borrows were temporary, and each ended before the \
                 next began. Nothing was cloned and nothing was moved.",
            ),
            callout(
                "Describe the program, do not recite it",
                "If you can narrate this program line by line in terms of owner, borrow, and region — \
                 without invoking any memorised rule — the module has done its job. That narration is \
                 what transfers to code you have never seen.",
            ),
        ],
    )
}

fn what_it_does_not_mean() -> SeedLesson {
    SeedLesson::new(
        "What Borrowing Does NOT Mean",
        vec![
            labelled(
                "THE IDEA",
                "These five misconceptions survive long after the syntax is learned. Correct them \
                 explicitly, because each one leads to a wrong prediction under pressure.",
            ),
            table(
                &["Claim", "Verdict", "Why"],
                &[
                    &["\"Borrowing means copying.\"", "wrong", "borrowing provides access without transferring ownership; no bytes are duplicated"],
                    &["\"A reference owns the value.\"", "wrong", "the original owner remains responsible for the value"],
                    &["\"`&mut` lets many things mutate.\"", "wrong", "a mutable reference is exclusive while active"],
                    &["\"References live to the end of the function.\"", "wrong", "the borrow checker tracks when references are actually used"],
                    &["\"Clone is the solution to borrowing.\"", "wrong", "clone creates a second owned value and changes the ownership design"],
                ],
            ),
            paragraphs(&[
                "Each of these is a shortcut that feels right and predicts wrong. 'Borrowing is \
                 copying' misses that a borrow allocates nothing. 'A reference owns' reintroduces the \
                 double-owner problem. 'Many can mutate' discards exclusivity. 'Lives to the end' \
                 fails on every re-borrow. 'Clone is the solution' trades understanding for an \
                 allocation.",
            ]),
            callout(
                "The correct one-liners",
                "Borrowing is access without ownership. The owner stays responsible. A mutable borrow \
                 is exclusive. A borrow lasts until its last use. Clone creates another owned value, \
                 which is a design change, not a fix.",
            ),
        ],
    )
}

fn final_model() -> SeedLesson {
    SeedLesson::new(
        "Final Borrowing Mental Model",
        vec![
            labelled(
                "THE IDEA",
                "One compact model to leave the module with. Each line is a question you can ask of \
                 unfamiliar code, in order.",
            ),
            table(
                &["Idea", "The question it answers"],
                &[
                    &["Ownership", "who is responsible?"],
                    &["Immutable borrowing", "who may read?"],
                    &["Mutable borrowing", "who may exclusively modify?"],
                    &["Reference validity", "does the borrowed value still exist?"],
                    &["Borrow checker", "do these accesses satisfy the rules?"],
                ],
            ),
            diagram(
                "The reasoning procedure",
                "TRACE THE VALUE\n     ↓\nWHO OWNS IT?\n     ↓\nWHO IS BORROWING IT?\n     ↓\nIMMUTABLE OR MUTABLE?\n     ↓\nWHEN IS THE BORROW USED?\n     ↓\nDO ANY BORROWS CONFLICT?\n     ↓\nDOES THE VALUE OUTLIVE THE REFERENCE?\n     ↓\nCOMPILE OR ERROR?",
            ),
            checklist(
                "You can now do this",
                &[
                    "Name the owner and every borrower in any snippet",
                    "Say whether each borrow is immutable or mutable",
                    "Find the region in which each borrow is used",
                    "Predict whether two borrows conflict, and why",
                    "Explain a borrow-checker error in terms of overlap",
                    "Decide whether a function should take `T`, `&T`, or `&mut T`",
                    "Recognise a dangling reference before compiling",
                    "Choose `&str` over `&String` where it is more flexible",
                ],
            ),
            callout(
                "The next question",
                "You can now describe who owns a value, who may borrow it, and for how long. The one \
                 thing left to describe exactly is *how long a reference is allowed to live* when it \
                 cannot be inferred from a single input. That question is lifetimes, and it is the next \
                 module. You are ready for it precisely because you can reason about borrows first.",
            ),
        ],
    )
}
