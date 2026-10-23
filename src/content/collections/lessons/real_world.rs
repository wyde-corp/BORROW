//! Module 08 — Collections, lessons 31–33.
//!
//! The Music Playlist, assembled. All three containers in one program, then the
//! reporting methods, then a set of design changes to try.

use crate::content::types::{
    callout, code, guess_run, labelled, paragraphs, steps, table, SeedLesson,
};

pub fn lessons() -> Vec<SeedLesson> {
    vec![assembled(), searching_and_reporting(), change_the_design()]
}

fn assembled() -> SeedLesson {
    SeedLesson::new(
        "The Playlist, Assembled",
        vec![
            labelled(
                "THE IDEA",
                "Here is the whole program as it stands. A `Vec<Song>` keeps the playlist in order. A \
                 method builds a `HashMap` index from artist to titles. Another collects the distinct \
                 genres into a `HashSet`. Each container is chosen for the question it answers.",
            ),
            code(
                "rust",
                r#"use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
struct Song {
    title: String,
    artist: String,
    duration_secs: u32,
    genre: String,
}

struct Playlist {
    songs: Vec<Song>,
}

impl Playlist {
    fn new() -> Playlist {
        Playlist { songs: Vec::new() }
    }

    fn add(&mut self, song: Song) {
        self.songs.push(song);
    }

    fn total_duration(&self) -> u32 {
        self.songs.iter().map(|s| s.duration_secs).sum()
    }

    fn genres(&self) -> HashSet<&str> {
        self.songs.iter().map(|s| s.genre.as_str()).collect()
    }

    fn by_artist(&self) -> HashMap<&str, Vec<&str>> {
        let mut index: HashMap<&str, Vec<&str>> = HashMap::new();
        for song in &self.songs {
            index
                .entry(song.artist.as_str())
                .or_insert_with(Vec::new)
                .push(song.title.as_str());
        }
        index
    }
}

fn main() {
    let mut playlist = Playlist::new();

    playlist.add(Song {
        title: String::from("Blackbird"),
        artist: String::from("The Beatles"),
        duration_secs: 208,
        genre: String::from("Folk"),
    });
    playlist.add(Song {
        title: String::from("Nude"),
        artist: String::from("Radiohead"),
        duration_secs: 255,
        genre: String::from("Alternative"),
    });
    playlist.add(Song {
        title: String::from("Karma Police"),
        artist: String::from("Radiohead"),
        duration_secs: 261,
        genre: String::from("Alternative"),
    });

    println!("{} songs, {} seconds", playlist.songs.len(), playlist.total_duration());
    println!("{} distinct genres", playlist.genres().len());

    for (artist, titles) in playlist.by_artist() {
        println!("{artist}: {} song(s)", titles.len());
    }
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "Trace the containers, not the lines. `songs` is a `Vec` because order matters and \
                 we ask 'what is in the playlist'. `by_artist` is a `HashMap` because we ask 'which \
                 songs are by this artist'. `genres` is a `HashSet` because we ask 'which genres \
                 appear', with no duplicates and no order.",
            ),
            table(
                &["Field or method", "Container", "The question it answers"],
                &[
                    &["`songs`", "`Vec<Song>`", "what is in the playlist, in order"],
                    &["`genres()`", "`HashSet<&str>`", "which genres appear at all"],
                    &["`by_artist()`", "`HashMap<&str, Vec<&str>>`", "which songs are by this artist"],
                    &["`total_duration()`", "number", "how long is it"],
                ],
            ),
            callout(
                "The containers are the design",
                "Nothing here is decorative. Change `songs` to a `HashMap` and you lose the order; \
                 change `genres` to a `Vec` and you can have duplicates. The types are doing the \
                 work of keeping the data honest.",
            ),
        ],
    )
}

fn searching_and_reporting() -> SeedLesson {
    SeedLesson::new(
        "Searching and Reporting",
        vec![
            labelled(
                "THE IDEA",
                "Two more methods make the playlist useful: finding one song by title, and reporting \
                 the artist index. Both borrow the playlist and return references, so nothing is \
                 copied.",
            ),
            code(
                "rust",
                r#"use std::collections::HashMap;

struct Song {
    title: String,
    artist: String,
    duration_secs: u32,
}

struct Playlist {
    songs: Vec<Song>,
}

impl Playlist {
    fn find(&self, title: &str) -> Option<&Song> {
        self.songs.iter().find(|s| s.title == title)
    }

    fn by_artist(&self) -> HashMap<&str, Vec<&str>> {
        let mut index: HashMap<&str, Vec<&str>> = HashMap::new();
        for song in &self.songs {
            index
                .entry(song.artist.as_str())
                .or_insert_with(Vec::new)
                .push(song.title.as_str());
        }
        index
    }
}

fn main() {
    let playlist = Playlist {
        songs: vec![
            Song { title: String::from("Nude"), artist: String::from("Radiohead"), duration_secs: 255 },
            Song { title: String::from("Karma Police"), artist: String::from("Radiohead"), duration_secs: 261 },
        ],
    };

    if let Some(song) = playlist.find("Nude") {
        println!("found {} by {}", song.title, song.artist);
    }

    for (artist, titles) in playlist.by_artist() {
        println!("{artist} has {} songs", titles.len());
    }
}"#,
            ),
            labelled(
                "LET'S BREAK IT DOWN",
                "`find` scans the `Vec` because it is an ordered list and we are searching once — \
                 fine for a playlist. The result is `Option<&Song>`, borrowed from the playlist. \
                 `by_artist` builds a map whose keys and values borrow from the songs, so the index \
                 costs no string copies.",
            ),
            guess_run(
                "A second, identical song is added. Predict how many entries the artist index holds \
                 for that artist.",
                r#"use std::collections::HashMap;

struct Song { title: String, artist: String }

fn main() {
    let songs = vec![
        Song { title: String::from("Nude"), artist: String::from("Radiohead") },
        Song { title: String::from("Nude"), artist: String::from("Radiohead") },
    ];

    let mut index: HashMap<&str, Vec<&str>> = HashMap::new();
    for song in &songs {
        index.entry(song.artist.as_str()).or_insert_with(Vec::new).push(song.title.as_str());
    }

    println!("{:?}", index.get("Radiohead"));
}"#,
                &[
                    "One entry: [\"Nude\"]",
                    "One artist with two titles: [\"Nude\", \"Nude\"]",
                    "It does not compile",
                ],
                1,
                "A `HashMap<String, Vec<String>>` tracks one list per key, not one list per unique \
                 title. Two identical songs become two entries in Radiohead's list. If you wanted \
                 to drop duplicates, that list would be a `HashSet` of titles instead — a different \
                 question, a different container.",
            ),
            callout(
                "The value type encodes the question",
                "`Vec<&str>` means 'all of them, in order'. `HashSet<&str>` would mean 'each one \
                 once, order irrelevant'. The choice of the value type is as much of a decision as \
                 the outer container.",
            ),
        ],
    )
}

fn change_the_design() -> SeedLesson {
    SeedLesson::new(
        "Change the Design",
        vec![
            labelled(
                "THE IDEA",
                "Reading the program proves you can follow Rust. Changing it proves you can write it. \
                 Work through these, deciding the container each one needs.",
            ),
            steps(&[
                "Detect whether a song is already in the playlist before adding it.",
                "Merge two playlists and report the combined distinct genres.",
                "Find every song longer than three minutes.",
                "Count how many songs each genre contributes to the total duration.",
                "Print the playlist in alphabetical title order without losing the original order.",
            ]),
            paragraphs(&[
                "For each one, name the question first: order, lookup, or membership. The first is a \
                 membership question about a set of titles — a `HashSet` of titles lets you ask \
                 `insert` and know whether it was new. Merging and distinct genres is a union of two \
                 sets. The long songs are a filter over the `Vec`. The per-genre duration is a \
                 `HashMap<genre, u32>` with the `entry` API. Alphabetical printing is the one that \
                 wants a `BTreeMap`, or a `Vec` you sort on the way out.",
            ]),
            callout(
                "Decide before you type",
                "The exercise is choosing, not coding. Say the question, name the container, then \
                 write the three lines it takes. If you cannot name the container, the question is \
                 not clear yet.",
            ),
            labelled(
                "Where this goes next",
                "Every collection you have met returns an `Option` when a thing might be missing — \
                 `get`, `pop`, `find`. That 'might be absent' idea, and its sibling 'might fail', are \
                 the whole of Module 09.",
            ),
        ],
    )
}
