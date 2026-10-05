//! Module 08 — Collections, lessons 21–26.
//!
//! HashSets. Membership and uniqueness: inserting, checking `contains`, set
//! operations, de-duplicating a list, and who owns a set's items.

use crate::content::types::{
    callout, code, diagram, gap_fill, guess_run, labelled, step_through, table, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        uniqueness(),
        set_membership(),
        iterating_a_set(),
        set_operations(),
        deduplicating(),
        set_ownership(),
    ]
}

fn uniqueness() -> SeedLesson {
    SeedLesson::new(
        "Uniqueness",
        vec![
            labelled(
                "CONCEPT",
                "A playlist spans several genres, and you want the *distinct* ones. A `Vec` would \
                 happily hold \"Folk\" five times. A `HashSet` is a collection that ignores \
                 duplicates: add a value it already holds and nothing changes.",
            ),
            diagram(
                "A set keeps one of each",
                "pushing to a Vec:        inserting into a HashSet:\n  [Folk]                   { Folk }\n  [Folk, Folk]             { Folk }\n  [Folk, Folk, Folk]       { Folk }",
            ),
            code(
                "rust",
                r#"use std::collections::HashSet;

fn main() {
    let mut genres: HashSet<String> = HashSet::new();

    genres.insert(String::from("Folk"));
    genres.insert(String::from("Folk"));
    genres.insert(String::from("Alternative"));

    println!("{} distinct genres", genres.len());
}"#,
            ),
            labelled(
                "EXPLANATION",
                "Three inserts, two genres. The second `\"Folk\"` was already present, so the set \
                 ignored it and said nothing. The len is `2`. A set is the natural home for a \
                 question like 'which genres appear at all?'",
            ),
            callout(
                "Set semantics are the whole point",
                "The container is not just a `Vec` that filters duplicates on the way out — it never \
                 stores the duplicate in the first place. That is what makes membership checks fast \
                 and the guarantee absolute.",
            ),
        ],
    )
}

fn set_membership() -> SeedLesson {
    SeedLesson::new(
        "`insert` and `contains`",
        vec![
            labelled(
                "CONCEPT",
                "`insert` adds a value and tells you whether it was new. `contains` asks whether a \
                 value is present. Those two methods cover almost every set question a beginner has.",
            ),
            gap_fill(
                "Add a genre, then ask whether another is present. Fill in the two methods.",
                r#"let mut genres: HashSet<String> = HashSet::new();

let is_new = genres.____(String::from("Folk"));
let again = genres.____(String::from("Folk"));

println!("{is_new} {again}");
println!("has Alternative: {}", genres.____("Alternative"));"#,
                &[
                    ("i1", "insert", "Adds a value; returns `true` if it was new."),
                    ("i2", "insert", "Adding it a second time."),
                    ("c1", "contains", "Asks whether a value is present."),
                ],
            ),
            code(
                "rust",
                r#"use std::collections::HashSet;

fn main() {
    let mut genres: HashSet<&str> = HashSet::new();

    let first = genres.insert("Folk");
    let second = genres.insert("Folk");

    println!("first was new: {first}, second was new: {second}");
    println!("has Alternative: {}", genres.contains("Alternative"));
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`insert` returns a `bool`: `true` if the value was not already there, `false` if it \
                 was. That is how you can tell without a separate check. `contains` takes a value \
                 (or a borrowed form of it) and returns whether it is present, without changing \
                 anything.",
            ),
            table(
                &["Call", "Returns", "Changes the set?"],
                &[
                    &["`set.insert(v)`", "`bool` — was it new?", "yes"],
                    &["`set.contains(&v)`", "`bool` — is it present?", "no"],
                    &["`set.remove(&v)`", "`bool` — was it there?", "yes"],
                ],
            ),
            callout(
                "Membership, not position",
                "Notice there is no `set[0]`. A set has no order, so it has no positions. It answers \
                 'is this in here?' and nothing else — that is the trade for fast membership.",
            ),
        ],
    )
}

fn iterating_a_set() -> SeedLesson {
    SeedLesson::new(
        "Iterating a Set",
        vec![
            labelled(
                "CONCEPT",
                "You can loop over a set, but like a `HashMap`, the order is not guaranteed. Each \
                 item is a reference, and the set is borrowed for the loop.",
            ),
            guess_run(
                "Predict the iteration order of this set.",
                r#"use std::collections::HashSet;

fn main() {
    let mut genres: HashSet<&str> = HashSet::new();
    genres.insert("Folk");
    genres.insert("Alternative");
    genres.insert("Rock");

    for genre in &genres {
        println!("{genre}");
    }
}"#,
                &[
                    "Folk, Alternative, Rock — insertion order",
                    "Any order; a `HashSet` does not promise one",
                    "It does not compile",
                ],
                1,
                "A `HashSet` is unordered, exactly like a `HashMap` is. The items come out in a \
                 hash-dependent order that can differ between runs. If you need a stable order, \
                 collect into a `Vec` and sort, or use a `BTreeSet`.",
            ),
            code(
                "rust",
                r#"use std::collections::HashSet;

fn main() {
    let genres: HashSet<&str> = ["Folk", "Alternative"].into_iter().collect();

    for genre in &genres {
        println!("{genre}");
    }
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`&genres` borrows; each `genre` is a `&&str` reference to an item the set owns. The \
                 loop reads and the set is usable afterwards. The `.into_iter().collect()` on the \
                 first line is a compact way to build a set from a list — the subject of lesson 25.",
            ),
            callout(
                "Sorted when you need it",
                "`BTreeSet` is the ordered twin of `HashSet`, just as `BTreeMap` is for `HashMap`. \
                 Reach for it the moment iteration order matters.",
            ),
        ],
    )
}

fn set_operations() -> SeedLesson {
    SeedLesson::new(
        "Set Operations",
        vec![
            labelled(
                "CONCEPT",
                "Sets support the operations you remember from school: union (everything in either), \
                 intersection (only what is in both), and difference (what is in one but not the \
                 other). They answer real questions about two playlists.",
            ),
            step_through(
                "Genres shared between two playlists",
                r#"use std::collections::HashSet;

let a: HashSet<&str> = ["Folk", "Rock"].into_iter().collect();
let b: HashSet<&str> = ["Rock", "Jazz"].into_iter().collect();

let shared: HashSet<&&str> = a.intersection(&b).collect();
println!("{}", shared.len());"#,
                &[
                    (&[3][..], "Playlist A's genres: `{Folk, Rock}`."),
                    (&[4][..], "Playlist B's genres: `{Rock, Jazz}`."),
                    (&[6][..], "`intersection` yields the items in both: `Rock`."),
                    (&[6][..], "`.collect()` gathers the borrows into a third set — `&&str`, because the sets are borrowed."),
                ],
            ),
            code(
                "rust",
                r#"use std::collections::HashSet;

fn main() {
    let a: HashSet<&str> = ["Folk", "Rock"].into_iter().collect();
    let b: HashSet<&str> = ["Rock", "Jazz"].into_iter().collect();

    let shared: Vec<&&str> = a.intersection(&b).collect();
    let all: Vec<&&str> = a.union(&b).collect();

    println!("shared: {shared:?}");
    println!("combined: {} genres", all.len());
}"#,
            ),
            table(
                &["Operation", "Method", "Meaning"],
                &[
                    &["union", "`a.union(&b)`", "in either set"],
                    &["intersection", "`a.intersection(&b)`", "in both sets"],
                    &["difference", "`a.difference(&b)`", "in `a` but not `b`"],
                ],
            ),
            callout(
                "The result is a lazy view",
                "`union` and friends return an iterator, not a set. Collect it into a `Vec` or a \
                 `HashSet` to store it. This is why the examples finish with `.collect()`.",
            ),
        ],
    )
}

fn deduplicating() -> SeedLesson {
    SeedLesson::new(
        "De-duplicating a `Vec`",
        vec![
            labelled(
                "CONCEPT",
                "A common task: a list has duplicates and you want one of each. Collect the list into \
                 a `HashSet`, then collect the set back into a `Vec`. The duplicates collapse in the \
                 middle.",
            ),
            gap_fill(
                "Turn a list with repeats into one of each. Fill in the two `collect` targets.",
                r#"let raw = vec!["Folk", "Rock", "Folk", "Rock", "Jazz"];

let set: HashSet<&str> = raw.into_iter().____();
let unique: Vec<&str> = set.into_iter().____();

println!("{} unique", unique.len());"#,
                &[
                    ("c1", "collect", "Gather the items into the target type."),
                    ("c2", "collect", "Gather them back out."),
                ],
            ),
            code(
                "rust",
                r#"use std::collections::HashSet;

fn main() {
    let raw = vec!["Folk", "Rock", "Folk", "Rock", "Jazz"];
    let total = raw.len();

    let set: HashSet<&str> = raw.into_iter().collect();
    let unique_count = set.len();
    let unique: Vec<&str> = set.into_iter().collect();

    println!("{total} items, {unique_count} unique");
    println!("{unique:?}");
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`raw.into_iter().collect::<HashSet<_>>()` drains the list into a set, dropping the \
                 repeats. `set.into_iter().collect::<Vec<_>>()` drains the set into a list. If the \
                 order of the result matters, sort it afterwards — the set did not preserve it.",
            ),
            callout(
                "The type annotation does the work",
                "`collect` can build many containers, so it needs to know which one. Writing the \
                 target type (`let set: HashSet<&str> = ...`) is how you tell it. This is the same \
                 `collect` you will meet properly with iterators in Module 11.",
            ),
        ],
    )
}

fn set_ownership() -> SeedLesson {
    SeedLesson::new(
        "Sets and Ownership",
        vec![
            labelled(
                "CONCEPT",
                "A set owns its items, just as a `Vec` and a `HashMap` do. Inserting a `String` moves \
                 it in; membership checks borrow. The rules do not change because the container is a \
                 set.",
            ),
            step_through(
                "Moving a genre in, borrowing it back",
                r#"use std::collections::HashSet;

let mut genres: HashSet<String> = HashSet::new();
let genre = String::from("Folk");
genres.insert(genre);
println!("{}", genres.contains("Folk"));"#,
                &[
                    (&[3][..], "`genre` owns a fresh string."),
                    (&[4][..], "`insert` moves it into the set; `genre` is disarmed."),
                    (&[5][..], "`contains(\"Folk\")` borrows — a `&str` is enough even though the set holds `String`s."),
                ],
            ),
            code(
                "rust",
                r#"use std::collections::HashSet;

fn main() {
    let mut genres: HashSet<String> = HashSet::new();
    let genre = String::from("Folk");

    genres.insert(genre);

    // `genre` moved into the set; look it up through the set instead.
    if genres.contains("Folk") {
        println!("Folk is in the set");
    }
}"#,
            ),
            labelled(
                "EXPLANATION",
                "The ownership behaviour is identical to `Vec::push` and `HashMap::insert`: the \
                 container takes ownership. `contains` takes a borrowed form, so you do not need to \
                 rebuild a `String` to ask a question. If the same genre value is needed in several \
                 sets, clone it deliberately and say why.",
            ),
            callout(
                "One rule, three containers",
                "By now the pattern should feel familiar: containers own what you put in them, and \
                 reading borrows. `Vec`, `HashMap`, and `HashSet` share that, which is why the \
                 ownership material from Modules 05–06 keeps paying off.",
            ),
        ],
    )
}
