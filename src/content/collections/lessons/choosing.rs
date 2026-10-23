//! Module 08 — Collections, lessons 27–30.
//!
//! Choosing. The decision table, the two honourable mentions, a plain-language
//! sense of cost, and the ten mistakes that most often mean the model has not
//! landed.

use crate::content::types::{
    callout, code, diagram, gap_fill, guess_run, labelled, table, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        decision_table(),
        more_containers(),
        performance_intuition(),
        common_mistakes(),
    ]
}

fn decision_table() -> SeedLesson {
    SeedLesson::new(
        "The Decision Table",
        vec![
            labelled(
                "THE IDEA",
                "Three containers, three questions. Ask what shape your question is and the container \
                 follows. This lesson is the decision, stated once.",
            ),
            diagram(
                "Position, lookup, membership",
                "  POSITION            LOOKUP             MEMBERSHIP\n  \"what is at N?\"     \"what is for K?\"    \"is X present?\"\n   Vec<T>              HashMap<K,V>       HashSet<T>\n   ordered list        key -> value       unique items",
            ),
            table(
                &["Your question", "Container", "Why"],
                &[
                    &["what is the fifth song?", "`Vec<Song>`", "position matters"],
                    &["what is the duration of \"Nude\"?", "`HashMap<String, u32>`", "lookup by key"],
                    &["is \"Jazz\" in this playlist?", "`HashSet<String>`", "membership"],
                    &["what order were songs added?", "`Vec<Song>`", "insertion order is kept"],
                    &["how many songs per artist?", "`HashMap<String, u32>`", "a count per key"],
                    &["which genres appear at all?", "`HashSet<String>`", "unique values"],
                ],
            ),
            gap_fill(
                "Name the container for each. Write `Vec`, `HashMap`, or `HashSet`.",
                r#"// Track the order songs are added:      ____
// Find a song by its title:            ____
// Know whether a genre is present:     ____
// Count songs per genre:               ____"#,
                &[
                    ("a1", "Vec", "Order is the point."),
                    ("a2", "HashMap", "Lookup by title."),
                    ("a3", "HashSet", "Membership."),
                    ("a4", "HashMap", "A count per key."),
                ],
            ),
            callout(
                "Say the question out loud",
                "'What is at position three', 'what is under this key', 'is this in here' — the verb \
                 tells you the container. Getting this right is most of the skill in this module.",
            ),
        ],
    )
}

fn more_containers() -> SeedLesson {
    SeedLesson::new(
        "`BTreeMap` and `VecDeque`",
        vec![
            labelled(
                "THE IDEA",
                "Two more containers are worth knowing by name, even though you will not use them yet. \
                 `BTreeMap` is a map that keeps its keys sorted. `VecDeque` is a list that is fast at \
                 both ends.",
            ),
            table(
                &["Container", "Like", "Reach for it when"],
                &[
                    &["`BTreeMap<K, V>`", "a `HashMap`", "entries must iterate in sorted key order"],
                    &["`BTreeSet<T>`", "a `HashSet`", "membership, but iterating in order"],
                    &["`VecDeque<T>`", "a `Vec`", "you add and remove at the *front* as much as the back"],
                ],
            ),
            code(
                "rust",
                r#"use std::collections::VecDeque;

fn main() {
    let mut queue: VecDeque<&str> = VecDeque::new();

    queue.push_back("Blackbird");
    queue.push_front("Nude");

    println!("{:?}", queue);
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`VecDeque` supports `push_front`, `pop_front`, `push_back`, and `pop_back` all at \
                 the same cost. A `Vec` only does the back cheaply — pushing to the front shifts every \
                 item. A queue where you always take the oldest item is a `VecDeque`.",
            ),
            callout(
                "Name them, do not memorise them",
                "The point of this lesson is that the three you learned are not the only ones. When a \
                 problem needs sorted iteration or a fast front, you now know the container exists \
                 and can look up the rest.",
            ),
        ],
    )
}

fn performance_intuition() -> SeedLesson {
    SeedLesson::new(
        "A Little Performance Intuition",
        vec![
            labelled(
                "THE IDEA",
                "You do not need formal analysis to choose well. One idea is enough: some operations \
                 take the same time no matter how big the collection is, and others take longer as it \
                 grows.",
            ),
            table(
                &["Operation", "Rough cost", "Plain meaning"],
                &[
                    &["`vec[i]`", "instant", "jumping to a known position is one step"],
                    &["`vec.push(x)`", "instant (usually)", "adding at the end is cheap"],
                    &["scanning a `Vec` for a value", "grows with the list", "the longer the list, the more to check"],
                    &["`map.get(&k)`", "instant", "a lookup does not care how many entries there are"],
                    &["`set.contains(&x)`", "instant", "membership is a lookup"],
                    &["`vec.insert(0, x)`", "grows with the list", "everything after shifts along"],
                ],
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The word for 'instant' here is *constant time* — the cost does not grow with the \
                 collection. The word for the others is *linear* — the cost grows in step with the \
                 number of items. A `HashMap` lookup and a `Vec` scan can both find a song; on a \
                 playlist of three it does not matter, and on a playlist of a million it is the whole \
                 difference.",
            ),
            callout(
                "Choose for the question first, cost second",
                "Correctness and clarity come before speed. Once the container matches the question, \
                 the cost usually takes care of itself — and the places it does not are far beyond \
                 this module.",
            ),
        ],
    )
}

fn common_mistakes() -> SeedLesson {
    SeedLesson::new(
        "The Ten Common Mistakes",
        vec![
            labelled(
                "THE IDEA",
                "These are the mistakes that survive long after the syntax is learned. Read them as a \
                 checklist against your own understanding.",
            ),
            table(
                &["#", "Mistake", "The correction"],
                &[
                    &["1", "indexing a `Vec` past the end", "`[i]` panics; `get(i)` returns `Option`"],
                    &["2", "treating `HashMap` iteration as ordered", "order is not guaranteed; use a `Vec` or `BTreeMap` when it matters"],
                    &["3", "scanning a `Vec` for a key every time", "that question wants a `HashMap`"],
                    &["4", "thinking a `HashSet` removes duplicates on read", "it never stored the duplicate in the first place"],
                    &["5", "mutating a collection while iterating it", "the borrow conflict; collect changes and apply after"],
                    &["6", "forgetting `mut` on a `Vec`/map/set you change", "the binding must be `mut`"],
                    &["7", "expecting `map[k]` to behave like `get`", "indexing panics on a missing key"],
                    &["8", "inserting into a map and losing the old value silently", "`insert` replaces; use `entry` if you care"],
                    &["9", "assuming a `Vec` is fast to edit at the front", "front insert/remove shifts everything — `VecDeque` for that"],
                    &["10", "using `len` as capacity (or vice versa)", "`len` is how many; `capacity` is how many fit"],
                ],
            ),
            guess_run(
                "A collection is mutated while a loop borrows it. Predict what happens.",
                r#"use std::collections::HashMap;

fn main() {
    let mut durations: HashMap<&str, u32> = HashMap::new();
    durations.insert("Nude", 255);

    for (title, secs) in &durations {
        println!("{title} {secs}");
        durations.insert("Blackbird", 208);
    }
}"#,
                &[
                    "It compiles and prints one line",
                    "It does not compile — the map is borrowed by the loop while `insert` needs a mutable borrow",
                ],
                1,
                "`&durations` borrows the map for the loop, and `insert` needs a mutable borrow at the \
                 same time. The same wall you hit with `Vec::push` inside a loop over the vector. \
                 Gather the new entries and insert them after the loop.",
            ),
            callout(
                "Use the list in reverse",
                "When a collection program will not compile, find which of these ten it is. Nine times \
                 in ten the answer is here.",
            ),
        ],
    )
}
