//! Module 08 — Collections, lessons 11–20.
//!
//! HashMaps. Looking things up by key: creating a map, reading with `get`,
//! ownership of keys and values, the `entry` API, counting, maps of vectors,
//! and the ordered alternative for when iteration order matters.

use crate::content::types::{
    broken, callout, code, diagram, gap_fill, guess_run, labelled, paragraphs, step_through, table,
    SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        the_scan_problem(),
        creating_a_map(),
        looking_up(),
        key_ownership(),
        entry_api(),
        iterating_a_map(),
        counting(),
        map_of_vectors(),
        updating_removing(),
        when_order_matters(),
    ]
}

fn the_scan_problem() -> SeedLesson {
    SeedLesson::new(
        "The Scan Problem",
        vec![
            labelled(
                "THE IDEA",
                "Finding a song by title in a `Vec` means walking the list and checking each one. It \
                 works, and it is the wrong tool once you do it often. The problem is not speed yet \
                 — it is that the code is doing a *lookup* and the data is not shaped for it.",
            ),
            code(
                "rust",
                r#"#[derive(Debug)]
struct Song {
    title: String,
    artist: String,
}

fn find<'a>(playlist: &'a [Song], title: &str) -> Option<&'a Song> {
    for song in playlist {
        if song.title == title {
            return Some(song);
        }
    }
    None
}

fn main() {
    let playlist = vec![
        Song { title: String::from("Blackbird"), artist: String::from("The Beatles") },
        Song { title: String::from("Nude"), artist: String::from("Radiohead") },
    ];

    if let Some(song) = find(&playlist, "Nude") {
        println!("{} by {}", song.title, song.artist);
    }
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The `for` loop scans from the front and stops when it finds a match. Ask for the \
                 last item and it has to look at all of them. Now imagine asking 'which songs are by \
                 Radiohead?' or 'how many songs do I have?' — each question means another scan. The \
                 data is keyed by title; the container should say so.",
            ),
            diagram(
                "Scanning vs looking up",
                "SCAN (Vec):                    LOOKUP (HashMap):\n  \"Nude?\" -> [1] no              \"Nude?\" -> key hashes\n             [2] no                        -> bucket\n             [3] yes  (walked 3)            -> value  (one step)",
            ),
            callout(
                "The container should match the question",
                "If your question is 'give me the item for this key', the container should be a map. \
                 A `Vec` answers 'what is at position N?' and 'what comes next?'. This lesson is the \
                 moment to notice which question you are really asking.",
            ),
        ],
    )
}

fn creating_a_map() -> SeedLesson {
    SeedLesson::new(
        "Creating a `HashMap` and Inserting",
        vec![
            labelled(
                "THE IDEA",
                "A `HashMap<K, V>` stores values under keys. You create it with `HashMap::new()` and \
                 add entries with `insert(key, value)`. Reading it back is `get`.",
            ),
            gap_fill(
                "Build a map from song title to duration. Fill in the container, the constructor, and \
                 the method that adds an entry.",
                r#"use std::collections::____;

let mut durations: ____<String, u32> = HashMap::____();
durations.____(String::from("Blackbird"), 208);
durations.____(String::from("Nude"), 255);

println!("Nude is {:?} seconds", durations.get("Nude"));"#,
                &[
                    ("i1", "HashMap", "The module that defines the map."),
                    ("t1", "HashMap", "The key-value type."),
                    ("c1", "new", "The empty constructor."),
                    ("m1", "insert", "Adds or replaces one entry."),
                    ("m2", "insert", "You already know this one."),
                ],
            ),
            code(
                "rust",
                r#"use std::collections::HashMap;

fn main() {
    let mut durations: HashMap<String, u32> = HashMap::new();
    durations.insert(String::from("Blackbird"), 208);
    durations.insert(String::from("Nude"), 255);

    println!("Nude is {:?} seconds", durations.get("Nude"));
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The type is `HashMap<String, u32>` — keys are owned strings, values are durations. \
                 `insert` takes the key and value by value, so the map owns both. `get(\"Nude\")` \
                 looks up by `&str` and returns `Option<&u32>`.",
            ),
            callout(
                "Keys must be hashable",
                "A key has to be a type Rust can hash — text, numbers, and structs you have asked to \
                 be hashable all work. You will not need the details yet; the practical rule is that \
                 the types you have used for keys so far all qualify.",
            ),
        ],
    )
}

fn looking_up() -> SeedLesson {
    SeedLesson::new(
        "Looking Up with `get`",
        vec![
            labelled(
                "THE IDEA",
                "`map.get(&key)` returns `Option<&V>`: `Some` if the key is present, `None` if it is \
                 not. `get_mut` does the same and hands back a mutable reference, so you can change \
                 the stored value in place.",
            ),
            guess_run(
                "The key is not in the map. Predict what this prints.",
                r#"use std::collections::HashMap;

fn main() {
    let map: HashMap<String, u32> = HashMap::new();

    match map.get("Nude") {
        Some(d) => println!("{d} seconds"),
        None => println!("not in the map"),
    }
}"#,
                &[
                    "0 seconds",
                    "not in the map",
                    "It panics",
                ],
                1,
                "`get` returns `None` when the key is missing — no panic, no default. You must \
                 decide what 'missing' means, which is why the return type is `Option` and not a \
                 bare number. Compare this to `v[i]`, which panics.",
            ),
            paragraphs(&["`get_mut` gives you write access to the stored value:"]),
            code(
                "rust",
                r#"use std::collections::HashMap;

fn main() {
    let mut durations: HashMap<String, u32> = HashMap::new();
    durations.insert(String::from("Nude"), 255);

    if let Some(d) = durations.get_mut("Nude") {
        *d += 10;
    }

    println!("{:?}", durations.get("Nude"));
}"#,
            ),
            table(
                &["Call", "Returns", "Missing key"],
                &[
                    &["`map.get(&k)`", "`Option<&V>`", "`None`"],
                    &["`map.get_mut(&k)`", "`Option<&mut V>`", "`None`"],
                    &["`map[&k]`", "`&V` (panics if missing)", "panic"],
                ],
            ),
            callout(
                "Indexing a map also panics",
                "`map[&key]` exists and panics on a missing key, just like `v[i]`. Prefer `get` when \
                 the key might be absent — and it usually might.",
            ),
        ],
    )
}

fn key_ownership() -> SeedLesson {
    SeedLesson::new(
        "Who Owns the Keys and Values",
        vec![
            labelled(
                "THE IDEA",
                "A `HashMap` owns its keys and values. When you `insert` a `String`, that string \
                 moves into the map. But you can look it up with a borrowed `&str`, so you do not \
                 need to build a `String` just to ask a question.",
            ),
            step_through(
                "Moves in, borrows out",
                r#"use std::collections::HashMap;

let key = String::from("Nude");
let mut durations: HashMap<String, u32> = HashMap::new();
durations.insert(key, 255);
println!("{}", durations.get("Nude").unwrap());"#,
                &[
                    (&[3][..], "`key` owns the string."),
                    (&[4][..], "`insert` moves the string into the map — `key` is now disarmed."),
                    (&[5][..], "Looking up with a `&str` literal borrows; the map still owns the key."),
                ],
            ),
            code(
                "rust",
                r#"use std::collections::HashMap;

fn main() {
    let key = String::from("Nude");
    let mut durations: HashMap<String, u32> = HashMap::new();

    durations.insert(key, 255);

    // `durations.get("Nude")` borrows; it does not need an owned String.
    println!("{:?}", durations.get("Nude"));
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`insert(key, value)` takes both by value, so the map owns them. `get` takes a \
                 reference, so a `&str` is enough even though the key is a `String` — that is why \
                 `durations.get(\"Nude\")` compiles without `String::from`. The map is the owner; \
                 lookups borrow.",
            ),
            broken(
                r#"use std::collections::HashMap;

fn main() {
    let key = String::from("Nude");
    let mut durations: HashMap<String, u32> = HashMap::new();

    durations.insert(key, 255);

    println!("{key}");
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Using `key` after inserting it is a use-after-move — Module 05 again. If you need \
                 the key afterwards, look up with a `&str`, or clone before inserting (and be ready \
                 to justify the clone).",
            ),
        ],
    )
}

fn entry_api() -> SeedLesson {
    SeedLesson::new(
        "The `entry` API",
        vec![
            labelled(
                "THE IDEA",
                "The common pattern 'insert this key if it is not there, then give me the value' has \
                 a dedicated method: `entry`. It looks the key up once and lets you decide what to do \
                 if it is missing.",
            ),
            gap_fill(
                "Count how many songs each artist has. Fill in the entry chain.",
                r#"for artist in artists {
    let count = counts.____(artist).____(0);
    *count += 1;
}"#,
                &[
                    ("e1", "entry", "Looks the key up and returns a handle."),
                    ("o1", "or_insert", "Inserts a default if the key was missing."),
                ],
            ),
            code(
                "rust",
                r#"use std::collections::HashMap;

fn main() {
    let artists = vec!["Radiohead", "Radiohead", "The Beatles"];

    let mut counts: HashMap<&str, u32> = HashMap::new();
    for artist in artists {
        let count = counts.entry(artist).or_insert(0);
        *count += 1;
    }

    println!("{:?}", counts.get("Radiohead"));
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`entry(artist)` returns a handle to that key's slot. `or_insert(0)` says 'if the slot \
                 is empty, put `0` there' and returns a `&mut u32` to the value — whether it was just \
                 inserted or already present. `*count += 1` then increments it. One lookup, no \
                 double `get`/`insert`, no panic on missing keys.",
            ),
            guess_run(
                "Only one entry was inserted. Predict what this prints.",
                r#"use std::collections::HashMap;

fn main() {
    let mut m: HashMap<&str, u32> = HashMap::new();
    m.insert("a", 1);

    let v = m.entry("b").or_insert(0);
    println!("{}", *v);
}"#,
                &[
                    "0 — `or_insert` put the default there and returned it",
                    "It panics because \"b\" was not present",
                    "1 — it reuses \"a\"'s value",
                ],
                0,
                "`or_insert(0)` sees that \"b\" is absent, inserts the default `0`, and returns a \
                 mutable reference to it. The result is `0`. This single method is why counting with \
                 a `HashMap` is so short.",
            ),
            callout(
                "`entry` looks up exactly once",
                "Doing `if !map.contains_key(k) { map.insert(k, 0); }` then `map.get_mut(k)` looks the \
                 key up twice and is easy to get wrong. `entry(...).or_insert(...)` is the idiomatic \
                 form, and the one to recognise on sight.",
            ),
        ],
    )
}

fn iterating_a_map() -> SeedLesson {
    SeedLesson::new(
        "Iterating a `HashMap`",
        vec![
            labelled(
                "THE IDEA",
                "Iterating a map yields `(key, value)` pairs, but the order is not guaranteed. The \
                 map stores entries by hash, not by when you added them, so do not depend on any \
                 particular sequence.",
            ),
            guess_run(
                "Two songs are inserted in one order. Predict what the iteration prints.",
                r#"use std::collections::HashMap;

fn main() {
    let mut m: HashMap<&str, u32> = HashMap::new();
    m.insert("Blackbird", 208);
    m.insert("Nude", 255);

    for (title, secs) in &m {
        println!("{title} {secs}");
    }
}"#,
                &[
                    "Always Blackbird then Nude, in insertion order",
                    "Either order — a `HashMap` does not promise one",
                    "It does not compile",
                ],
                1,
                "The map has no notion of insertion order. Iteration order depends on the hash of the \
                 keys and can change as the map grows or between runs. If order matters, that is a \
                 signal to use a `Vec` — or a `BTreeMap`, which sorts by key.",
            ),
            code(
                "rust",
                r#"use std::collections::HashMap;

fn main() {
    let mut durations: HashMap<String, u32> = HashMap::new();
    durations.insert(String::from("Blackbird"), 208);
    durations.insert(String::from("Nude"), 255);

    for (title, secs) in &durations {
        println!("{title}: {secs}s");
    }
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`&durations` borrows the map, and each item is a `(&K, &V)` pair — here `(&String, \
                 &u32)`. Destructuring in the loop header binds the two. The map is untouched, and \
                 you can still use it after the loop.",
            ),
            callout(
                "Unordered is a feature, not a bug",
                "A map is organised by key, not position. When you iterate, you are asking 'what is in \
                 here?', not 'what next?'. Keep the two questions separate and the right container \
                 becomes obvious.",
            ),
        ],
    )
}

fn counting() -> SeedLesson {
    SeedLesson::new(
        "Counting with a Map",
        vec![
            labelled(
                "THE IDEA",
                "Counting is the canonical map exercise: for each item, add one to its running total. \
                 The `entry` API makes it three lines and impossible to get wrong.",
            ),
            gap_fill(
                "Count how many of each genre appear in a playlist. Fill in the entry chain.",
                r#"let mut genre_counts: HashMap<String, u32> = HashMap::new();

for song in &playlist {
    let count = genre_counts.____(&song.genre).____(0);
    *count += 1;
}

println!("{:?}", genre_counts);"#,
                &[
                    ("c1", "entry", "Look up the genre's slot."),
                    ("c2", "or_insert", "Default it to zero if absent."),
                ],
            ),
            code(
                "rust",
                r#"use std::collections::HashMap;

fn main() {
    let genres = vec!["Folk", "Alternative", "Alternative", "Folk", "Folk"];

    let mut counts: HashMap<&str, u32> = HashMap::new();
    for genre in genres {
        *counts.entry(genre).or_insert(0) += 1;
    }

    for (genre, count) in &counts {
        println!("{genre}: {count}");
    }
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The loop takes one genre at a time. `entry(genre).or_insert(0)` gives a mutable \
                 reference to that genre's count, creating it as `0` on first sight. The `+= 1` \
                 increments through the reference. After the loop, the map holds one entry per \
                 distinct genre.",
            ),
            callout(
                "This is `uniq -c`, in Rust",
                "Counting occurrences of a key is one of the most common data tasks there is. The \
                 three-line shape — `entry(key).or_insert(0)`, increment — is worth memorising, \
                 because you will write it constantly.",
            ),
        ],
    )
}

fn map_of_vectors() -> SeedLesson {
    SeedLesson::new(
        "A Map of Vectors",
        vec![
            labelled(
                "THE IDEA",
                "A map's value can be any type, including a `Vec`. An index from artist to that \
                 artist's songs is `HashMap<String, Vec<String>>` — a lookup whose result is a list.",
            ),
            step_through(
                "Building an artist index",
                r#"use std::collections::HashMap;

let mut by_artist: HashMap<String, Vec<String>> = HashMap::new();
by_artist
    .entry(String::from("Radiohead"))
    .or_insert_with(Vec::new)
    .push(String::from("Nude"));"#,
                &[
                    (&[3][..], "The value type is `Vec<String>` — a list of titles per artist."),
                    (&[5][..], "`entry(artist)` looks up the artist's slot."),
                    (&[6][..], "`or_insert_with(Vec::new)` creates an empty list if this is a new artist."),
                    (&[7][..], "`.push(...)` then adds the title to that artist's list."),
                ],
            ),
            code(
                "rust",
                r#"use std::collections::HashMap;

fn main() {
    let songs = vec![
        ("Radiohead", "Nude"),
        ("Radiohead", "Karma Police"),
        ("The Beatles", "Blackbird"),
    ];

    let mut by_artist: HashMap<String, Vec<String>> = HashMap::new();

    for (artist, title) in songs {
        by_artist
            .entry(String::from(artist))
            .or_insert_with(Vec::new)
            .push(String::from(title));
    }

    for (artist, titles) in &by_artist {
        println!("{artist}: {}", titles.len());
    }
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`or_insert_with(Vec::new)` is like `or_insert(0)` for a count, but it builds a fresh \
                 empty vector only when the key is missing. Then `.push` appends. The result is a map \
                 from artist to all their titles — a lookup that returns a list.",
            ),
            table(
                &["Want", "Value type", "Default"],
                &[
                    &["a count per key", "`u32`", "`or_insert(0)`"],
                    &["a list per key", "`Vec<T>`", "`or_insert_with(Vec::new)`"],
                    &["another map per key", "`HashMap<K, V>`", "`or_insert_with(HashMap::new)`"],
                ],
            ),
            callout(
                "Nested containers are normal",
                "A map of vectors, a vector of maps, a map of maps — these are everyday shapes once \
                 you can read the type as a sentence: 'a map from artist to a list of titles'. Read \
                 the outer container first, then the value type.",
            ),
        ],
    )
}

fn updating_removing() -> SeedLesson {
    SeedLesson::new(
        "Updating and Removing Entries",
        vec![
            labelled(
                "THE IDEA",
                "Inserting a key that already exists replaces the old value. To ask whether a key is \
                 present without changing anything, use `contains_key`. To take an entry out, use \
                 `remove`, which returns the value if there was one.",
            ),
            code(
                "rust",
                r#"use std::collections::HashMap;

fn main() {
    let mut durations: HashMap<String, u32> = HashMap::new();

    durations.insert(String::from("Nude"), 255);
    durations.insert(String::from("Nude"), 260); // replaces

    println!("contains Blackbird: {}", durations.contains_key("Blackbird"));

    let removed = durations.remove("Nude");
    println!("removed: {:?}, now contains Nude: {}", removed, durations.contains_key("Nude"));
}"#,
            ),
            gap_fill(
                "Check for a key, then take it out. Fill in the two methods.",
                r#"if durations.____("Nude") {
    let old = durations.____("Nude");
    println!("{:?}", old);
}"#,
                &[
                    ("c1", "contains_key", "Ask without changing anything."),
                    ("r1", "remove", "Take the entry out, returning it."),
                ],
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`insert` is 'put this value under this key', replacing whatever was there. \
                 `contains_key` borrows and returns a `bool` without touching the map. `remove` takes \
                 the key, deletes the entry, and returns `Option<V>` — `Some(old_value)` if it \
                 existed, `None` if not.",
            ),
            callout(
                "Replacement is silent",
                "Unlike a `Vec`, a map happily accepts an insert over an existing key and quietly \
                 discards the old value. If losing the old value matters, check with `contains_key` \
                 or use `entry` to decide deliberately.",
            ),
        ],
    )
}

fn when_order_matters() -> SeedLesson {
    SeedLesson::new(
        "When Order Matters",
        vec![
            labelled(
                "THE IDEA",
                "`HashMap` gives no ordering. If you need entries kept sorted by key, Rust has \
                 `BTreeMap`. It has the same methods, and iterates in key order.",
            ),
            code(
                "rust",
                r#"use std::collections::BTreeMap;

fn main() {
    let mut durations: BTreeMap<&str, u32> = BTreeMap::new();
    durations.insert("Nude", 255);
    durations.insert("Blackbird", 208);
    durations.insert("Karma Police", 261);

    for (title, secs) in &durations {
        println!("{title}: {secs}");
    }
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "The same code with `BTreeMap` prints the songs in alphabetical key order — always. \
                 The cost is that lookups are a little slower in theory, because the map keeps its \
                 keys sorted rather than placing them by hash. For a beginner program, either is \
                 fine; the point is that the choice is deliberate.",
            ),
            table(
                &["Need", "Use"],
                &[
                    &["lookup by key, order irrelevant", "`HashMap`"],
                    &["lookup by key, kept sorted", "`BTreeMap`"],
                    &["order of insertion", "a `Vec` of pairs"],
                ],
            ),
            callout(
                "Do not make a `HashMap` act sorted",
                "Collecting keys and sorting them on every print is a sign you wanted a `BTreeMap`. \
                 Name the requirement — 'I need this in order' — and pick the container that \
                 provides it.",
            ),
        ],
    )
}
