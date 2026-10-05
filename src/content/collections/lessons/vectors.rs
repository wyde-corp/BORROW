//! Module 08 — Collections, lessons 1–10.
//!
//! Vectors. The ordered list: creating, growing, indexing versus `get`,
//! iterating three ways, editing in the middle, and finally what actually
//! happens when a vector grows past its capacity.

use crate::content::types::{
    broken, callout, code, diagram, gap_fill, guess_run, labelled, paragraphs, step_through,
    table, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![
        one_to_many(),
        creating_a_vec(),
        push_and_pop(),
        indexing_vs_get(),
        iterate_by_ref(),
        iterate_mutably(),
        iterate_by_value(),
        insert_and_remove(),
        vec_of_structs(),
        capacity(),
    ]
}

fn one_to_many() -> SeedLesson {
    SeedLesson::new(
        "From One Value to Many",
        vec![
            labelled(
                "CONCEPT",
                "A playlist is many songs, and so far the only tool you have for 'many' is to name \
                 each one. Let us write a four-song playlist that way, then notice the shape of the \
                 problem.",
            ),
            code(
                "rust",
                r#"fn main() {
    let title1 = String::from("Blackbird");
    let artist1 = String::from("The Beatles");

    let title2 = String::from("Karma Police");
    let artist2 = String::from("Radiohead");

    let title3 = String::from("Nude");
    let artist3 = String::from("Radiohead");

    let title4 = String::from("Weird Fishes");
    let artist4 = String::from("Radiohead");

    println!("{title1} — {artist1}");
    println!("{title2} — {artist2}");
    println!("{title3} — {artist3}");
    println!("{title4} — {artist4}");
}"#,
            ),
            labelled(
                "EXPLANATION",
                "This works, and it does not scale. Adding a fifth song means four more lines. There \
                 is no way to ask 'how many songs', no way to loop over them, and no way to pass the \
                 whole playlist to a function without listing eight variables in the signature. The \
                 data is a list; the code is not.",
            ),
            diagram(
                "A list pretending to be separate variables",
                "title1 artist1   title2 artist2   title3 artist3   title4 artist4\n   │      │         │      │         │      │         │      │\n   └──────┴─────────┴──────┴─────────┴──────┴─────────┴──────┘\n                    one playlist, four names for it",
            ),
            callout(
                "The container is the fix",
                "A collection is a single value that holds many values, in a defined shape. Rust's \
                 most basic one is `Vec<T>` — an ordered list where every item has the same type. \
                 That is the next ten lessons.",
            ),
            paragraphs(&[
                "Notice the phrase *ordered*: a `Vec` remembers the order you put things in, and lets \
                 you reach an item by its position. Keep that word, because the whole module turns on \
                 it — position, lookup, membership.",
            ]),
        ],
    )
}

fn creating_a_vec() -> SeedLesson {
    SeedLesson::new(
        "Creating a `Vec`",
        vec![
            labelled(
                "CONCEPT",
                "A `Vec<T>` holds zero or more values of type `T`, in order. You make an empty one \
                 with `Vec::new()`, or a full one with the `vec!` macro.",
            ),
            gap_fill(
                "Make an empty vector, then add two songs to it. Fill in the type, the constructor, \
                 and the method that adds to the end.",
                r#"let mut playlist: ____<String> = Vec::____();
playlist.____(String::from("Blackbird"));
playlist.____(String::from("Karma Police"));

println!("{} songs", playlist.len());"#,
                &[
                    ("t1", "Vec", "The ordered-list type."),
                    ("c1", "new", "The empty constructor."),
                    ("m1", "push", "Adds to the end."),
                    ("m2", "push", "You already know this one."),
                ],
            ),
            code(
                "rust",
                r#"fn main() {
    let mut playlist: Vec<String> = Vec::new();
    playlist.push(String::from("Blackbird"));
    playlist.push(String::from("Karma Police"));

    println!("{} songs", playlist.len());
}"#,
            ),
            labelled(
                "EXPLANATION",
                "The type is `Vec<String>` — a vector of owned strings. It must be `mut` to `push`, \
                 exactly as in Module 02. When the type is not obvious, you write it on the binding, \
                 as above.",
            ),
            paragraphs(&["The `vec!` macro builds a full one in a single expression:"]),
            code(
                "rust",
                r#"fn main() {
    let playlist = vec![
        String::from("Blackbird"),
        String::from("Karma Police"),
    ];

    println!("{} songs", playlist.len());
}"#,
            ),
            guess_run(
                "Both of these are empty vectors. Predict what the compiler does with the second.",
                r#"fn main() {
    let a: Vec<i32> = Vec::new();
    let b = Vec::new();

    println!("{}", b.len());
}"#,
                &[
                    "Both compile — the type is inferred later",
                    "The second does not compile without a type, because nothing says what `b` holds",
                    "The second defaults to `Vec<i32>`",
                ],
                1,
                "`a` is fine: its type annotation says `Vec<i32>`. `b` has nothing to infer from, so \
                 the compiler cannot know the element type. The fix is a `let b: Vec<String> = \
                 Vec::new();` annotation, or adding a value with `push` so inference has something \
                 to work with.",
            ),
        ],
    )
}

fn push_and_pop() -> SeedLesson {
    SeedLesson::new(
        "`push` and `pop`",
        vec![
            labelled(
                "CONCEPT",
                "A `Vec` grows and shrinks from the end. `push` adds an item and `pop` removes the \
                 last one, handing it back (or `None` if the vector is empty).",
            ),
            gap_fill(
                "Add a song, then take the last one back off. Fill in the two methods.",
                r#"let mut playlist = vec![String::from("Blackbird")];

playlist.____(String::from("Nude"));

let last = playlist.____();
println!("{:?}", last);"#,
                &[
                    ("p1", "push", "Add to the end."),
                    ("p2", "pop", "Remove from the end, returning it."),
                ],
            ),
            code(
                "rust",
                r#"fn main() {
    let mut playlist = vec![String::from("Blackbird")];

    playlist.push(String::from("Nude"));

    let last = playlist.pop();
    println!("{:?} — {} left", last, playlist.len());
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`pop` returns an `Option<T>`, not a `T`, because a vector can be empty. The last \
                 item here is `Some(\"Nude\")`, and the vector is back to one song. This is the same \
                 `Option` from Module 07 — absence is a real possibility, so the type says so.",
            ),
            callout(
                "The end of a `Vec` is cheap",
                "Adding and removing at the end is the fast operation. Doing it in the middle means \
                 shifting everything after it, which is lesson 8. For now, prefer the end when the \
                 order does not matter.",
            ),
        ],
    )
}

fn indexing_vs_get() -> SeedLesson {
    SeedLesson::new(
        "Indexing vs `get`",
        vec![
            labelled(
                "CONCEPT",
                "There are two ways to read an item by position, and they differ in what happens \
                 when the position does not exist. `playlist[0]` panics. `playlist.get(0)` returns \
                 `Option<&T>`.",
            ),
            guess_run(
                "The playlist has one song. Predict what this program does.",
                r#"fn main() {
    let playlist = vec![String::from("Blackbird")];

    println!("{}", playlist[3]);
}"#,
                &[
                    "Prints an empty string",
                    "Panics at runtime with an index out of bounds",
                    "Does not compile",
                ],
                1,
                "Square-bracket indexing panics when the index is past the end. The code compiles, \
                 because the compiler does not track how long the vector is — so this is a runtime \
                 crash, which is exactly what `get` is for.",
            ),
            paragraphs(&["The same read with `get` does not panic; it returns `None`:"]),
            code(
                "rust",
                r#"fn main() {
    let playlist = vec![String::from("Blackbird")];

    match playlist.get(3) {
        Some(song) => println!("found {song}"),
        None => println!("no song at position 3"),
    }

    if let Some(first) = playlist.get(0) {
        println!("first: {first}");
    }
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`get` returns `Option<&T>`: `Some(&song)` when the position exists, `None` when it \
                 does not. Because `None` is a real value, `match` and `if let` handle it without any \
                 risk of crashing. Indexing is fine when you *know* the position exists; `get` is for \
                 when you do not.",
            ),
            table(
                &["You write", "Missing position gives", "Use when"],
                &[
                    &["`v[i]`", "a panic at runtime", "the index is guaranteed"],
                    &["`v.get(i)`", "`None`", "the position might not exist"],
                ],
            ),
            callout(
                "Panics are not errors you catch",
                "A panic ends the program. `get` is how you avoid that by turning 'maybe missing' \
                 into a shape the type system makes you handle — the same idea as `Option` for \
                 absence everywhere else.",
            ),
        ],
    )
}

fn iterate_by_ref() -> SeedLesson {
    SeedLesson::new(
        "Iterating by Reference",
        vec![
            labelled(
                "CONCEPT",
                "The usual way to visit every item is `for song in &playlist`. The `&` borrows the \
                 vector, so you get a reference to each song and the vector is untouched.",
            ),
            step_through(
                "Walking a playlist without taking it",
                r#"let playlist = vec![String::from("Blackbird"), String::from("Nude")];

for song in &playlist {
    println!("{song}");
}

println!("still {} songs", playlist.len());"#,
                &[
                    (&[1][..], "The playlist owns two strings."),
                    (&[3][..], "`&playlist` borrows the whole vector for the duration of the loop."),
                    (&[4][..], "`song` is a `&String` — a reference to one item inside the vector."),
                    (&[7][..], "The borrow ended when the loop did, so the playlist is usable again."),
                ],
            ),
            code(
                "rust",
                r#"fn main() {
    let playlist = vec![String::from("Blackbird"), String::from("Nude")];

    for song in &playlist {
        println!("{song}");
    }

    println!("still {} songs", playlist.len());
}"#,
            ),
            labelled(
                "EXPLANATION",
                "Iterating by reference is the default you want most of the time: you read each item \
                 and keep the collection. It is Module 06's shared borrow, applied to a container. \
                 Because the loop only reads, it can run as long as it likes without conflict.",
            ),
            callout(
                "`&playlist` yields `&String`",
                "When you iterate by reference, each item is a reference, not an owned value. \
                 `println!` handles that transparently, but if you wanted to keep an item past the \
                 loop you would need to clone it or restructure.",
            ),
        ],
    )
}

fn iterate_mutably() -> SeedLesson {
    SeedLesson::new(
        "Iterating Mutably",
        vec![
            labelled(
                "CONCEPT",
                "To change every item in place, iterate with `&mut playlist`. Each item is a mutable \
                 reference, and the vector is borrowed exclusively for the loop — so nothing else may \
                 touch it until the loop ends.",
            ),
            step_through(
                "Changing every song in place",
                r#"let mut playlist = vec![String::from("blackbird"), String::from("nude")];

for song in &mut playlist {
    *song = song.to_uppercase();
}

println!("{:?}", playlist);"#,
                &[
                    (&[1][..], "The vector is `mut`, because its items will change."),
                    (&[3][..], "`&mut playlist` borrows exclusively for the loop."),
                    (&[4][..], "`song` is a `&mut String`; `*song` reaches through it to assign the new value."),
                    (&[7][..], "The exclusive borrow ended with the loop, so the vector is usable again."),
                ],
            ),
            code(
                "rust",
                r#"fn main() {
    let mut playlist = vec![String::from("blackbird"), String::from("nude")];

    for song in &mut playlist {
        *song = song.to_uppercase();
    }

    println!("{:?}", playlist);
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`song` is a `&mut String`, so `song.to_uppercase()` reads through it and `*song = \
                 ...` writes through it. This is the mutable borrow from Module 06, once per item, \
                 released before the next. The `mut` is required on the binding because the items \
                 change, not because the loop rebinds anything.",
            ),
            broken(
                r#"fn main() {
    let mut playlist = vec![String::from("blackbird")];

    for song in &mut playlist {
        playlist.push(String::from("nude"));
        println!("{song}");
    }
}"#,
            ),
            labelled(
                "EXPLANATION",
                "Pushing inside the loop needs a second, mutable borrow of the same vector while the \
                 loop already holds one exclusively — the collection conflict from Module 06, and the \
                 same wall you hit in Module 07. Collect the additions and push after the loop.",
            ),
        ],
    )
}

fn iterate_by_value() -> SeedLesson {
    SeedLesson::new(
        "Iterating by Value",
        vec![
            labelled(
                "CONCEPT",
                "Write `for song in playlist` with no `&`, and the loop *consumes* the vector. Each \
                 item is an owned value, and the playlist is gone afterwards.",
            ),
            guess_run(
                "The loop has no `&`, and the playlist is used after it. Predict what happens.",
                r#"fn main() {
    let playlist = vec![String::from("Blackbird")];

    for song in playlist {
        println!("{song}");
    }

    println!("{} songs", playlist.len());
}"#,
                &[
                    "It compiles and prints the count",
                    "It does not compile — the loop moved the vector",
                    "It compiles but the count is zero",
                ],
                1,
                "`for song in playlist` takes ownership of the vector and moves each item out. Using \
                 `playlist` afterwards is a use-after-move. Add `&` if you want to keep the \
                 playlist; iterate by value only when you genuinely mean to consume it.",
            ),
            table(
                &["Loop", "Each item is", "Vector afterwards"],
                &[
                    &["`for x in &list`", "a `&T`", "yours, unchanged"],
                    &["`for x in &mut list`", "a `&mut T`", "yours, possibly changed"],
                    &["`for x in list`", "a `T`", "consumed"],
                ],
            ),
            callout(
                "The rule is the same as every function call",
                "Iterating by value is passing the vector to a function that takes it by value. \
                 Nothing new — just the move/battle from Module 05 wearing a `for` loop.",
            ),
        ],
    )
}

fn insert_and_remove() -> SeedLesson {
    SeedLesson::new(
        "Inserting and Removing in the Middle",
        vec![
            labelled(
                "CONCEPT",
                "`push` and `pop` work at the end. To change an item in the middle you use `insert` \
                 and `remove`, which take a position — and cost more, because everything after that \
                 position must shift along.",
            ),
            gap_fill(
                "Put a song at position 1, then take it back out. Fill in the two methods.",
                r#"let mut playlist = vec![String::from("Blackbird"), String::from("Nude")];

playlist.____(1, String::from("Karma Police"));
println!("{:?}", playlist);

let removed = playlist.____(1);
println!("removed: {:?}", removed);"#,
                &[
                    ("i1", "insert", "Puts a value at a position, shifting the rest right."),
                    ("r1", "remove", "Takes the value at a position, shifting the rest left."),
                ],
            ),
            code(
                "rust",
                r#"fn main() {
    let mut playlist = vec![String::from("Blackbird"), String::from("Nude")];

    playlist.insert(1, String::from("Karma Police"));
    println!("{:?}", playlist);

    let removed = playlist.remove(1);
    println!("removed: {:?}", removed);
}"#,
            ),
            labelled(
                "EXPLANATION",
                "`insert(1, value)` slides every item from position 1 onward one slot to the right, \
                 then drops the new value into the gap. `remove(1)` returns the item and slides the \
                 rest left. Both take `&mut self`, because they change the vector.",
            ),
            diagram(
                "Shifting is the cost",
                "before:  [ Blackbird ][ Nude ][      ?      ]\ninsert(1, Karma Police)\n                  └──────────────┐\n                                 ▼\nafter:   [ Blackbird ][ Karma Police ][ Nude ]\n           (Nude moved one slot right)",
            ),
            callout(
                "Prefer the end when order does not matter",
                "Push and pop never shift anything. If you are choosing a collection and order is not \
                 important, that is one more reason to think before reaching for a `Vec` — the next \
                 lessons introduce two that do not have this cost for lookup and membership.",
            ),
        ],
    )
}

fn vec_of_structs() -> SeedLesson {
    SeedLesson::new(
        "A `Vec` of Structs",
        vec![
            labelled(
                "CONCEPT",
                "A `Vec<T>` holds any single type, including a struct you defined. A playlist is a \
                 `Vec<Song>`, and every song is a full value with its own fields.",
            ),
            step_through(
                "Holding songs, not strings",
                r#"let mut playlist: Vec<Song> = Vec::new();
playlist.push(Song {
    title: String::from("Blackbird"),
    artist: String::from("The Beatles"),
    duration_secs: 208,
    genre: String::from("Folk"),
});
println!("{} — {}", playlist[0].title, playlist[0].artist);"#,
                &[
                    (&[1][..], "The vector's element type is `Song`, so it owns whole songs."),
                    (&[2][..], "A `Song` is built and pushed; the vector now owns it."),
                    (&[7][..], "Reaching a field of an item chains: position, then field."),
                ],
            ),
            code(
                "rust",
                r#"#[derive(Debug)]
struct Song {
    title: String,
    artist: String,
    duration_secs: u32,
    genre: String,
}

fn main() {
    let mut playlist: Vec<Song> = Vec::new();

    playlist.push(Song {
        title: String::from("Blackbird"),
        artist: String::from("The Beatles"),
        duration_secs: 208,
        genre: String::from("Folk"),
    });

    playlist.push(Song {
        title: String::from("Karma Police"),
        artist: String::from("Radiohead"),
        duration_secs: 261,
        genre: String::from("Alternative"),
    });

    let mut total = 0;
    for song in &playlist {
        total += song.duration_secs;
    }

    println!("{} songs, {total} seconds", playlist.len());
}"#,
            ),
            labelled(
                "EXPLANATION",
                "This is the 'collection of structs' shape from Module 07, now the backbone of the \
                 playlist. `playlist[0].title` reads a field of the first song. The loop borrows the \
                 vector and reads each `song.duration_secs`, summing them without moving anything.",
            ),
            callout(
                "This is the shape real programs use",
                "A vector of structs, iterated to compute something, is one of the most common \
                 patterns in Rust. Everything from here — maps of songs, sets of genres — sits on \
                 top of it.",
            ),
        ],
    )
}

fn capacity() -> SeedLesson {
    SeedLesson::new(
        "Capacity and Reallocation",
        vec![
            labelled(
                "CONCEPT",
                "A `Vec` knows two different sizes: its *length* (how many items it holds) and its \
                 *capacity* (how many it could hold before it has to ask for more memory). When a \
                 push runs out of capacity, the vector reallocates: it asks for a bigger block, \
                 copies everything over, and frees the old one.",
            ),
            step_through(
                "Growing past capacity",
                r#"let mut v: Vec<i32> = Vec::with_capacity(2);
v.push(1);
v.push(2);
let first = &v[0];
v.push(3);
println!("{first}");"#,
                &[
                    (&[1][..], "`with_capacity(2)` reserves room for two items without growing."),
                    (&[2][..], "Two pushes fill the reserved space exactly; no reallocation yet."),
                    (&[4][..], "`first` borrows an item inside the vector's current buffer."),
                    (&[5][..], "The third push runs out of capacity, so the vector reallocates — the items move to a new, larger buffer, and the old one is freed."),
                    (&[6][..], "`first` now points at freed memory. The compiler refuses this before it can run."),
                ],
            ),
            code(
                "rust",
                r#"fn main() {
    let mut v: Vec<i32> = Vec::with_capacity(2);
    v.push(1);
    v.push(2);

    println!("length {} capacity {}", v.len(), v.capacity());

    v.push(3);
    println!("length {} capacity {}", v.len(), v.capacity());
}"#,
            ),
            diagram(
                "Reallocation moves the buffer",
                "before push(3):        after push(3):\n\nold buffer (cap 2)      old buffer freed\n [1][2]                 (dangling)\n                          new buffer (cap 4)\n                           [1][2][3][ ]",
            ),
            labelled(
                "EXPLANATION",
                "The takeaway is not the allocator, it is the rule it produces: **a reference into a \
                 vector cannot survive an operation that might reallocate it**. Rust does not guess \
                 whether this particular push reallocates — it treats any `push` as potentially \
                 invalidating a live borrow, and rejects the program in the `step_through` above.",
            ),
            broken(
                r#"fn main() {
    let mut v = vec![1, 2];
    let first = &v[0];

    v.push(3);

    println!("{first}");
}"#,
            ),
            labelled(
                "EXPLANATION",
                "This is the same error in miniature, and the same one you met in Module 06's \
                 collection section. The fix is to end the borrow first — copy the value out, or do \
                 the push before taking the reference.",
            ),
            callout(
                "Two numbers, two jobs",
                "`len` is how many items there are; `capacity` is how many fit before growing. You \
                 rarely call `with_capacity`, but knowing it exists explains why a `Vec` can be fast \
                 to push to even though it has no fixed size.",
            ),
        ],
    )
}
