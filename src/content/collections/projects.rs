//! Module 08 — Collections projects.
//!
//! Six projects escalating from deliberate repetition to the Music Playlist
//! capstone. Each asks for a container decision that the learner can justify.

use crate::content::types::{Difficulty, SeedProject};

pub fn projects() -> Vec<SeedProject> {
    vec![
        SeedProject::new(
            "Vector Playground",
            "Deliberately repetitive. Build, grow, index, iterate, and remove from a list, and record which operations panic and which return an `Option`.",
            Difficulty::Beginner,
            "30 min",
            &[
                "Create an empty `Vec<String>` and push five songs",
                "Read one by index and one with `get`, and explain the difference",
                "Iterate by reference and print each title",
                "Remove an item from the middle and print the list again",
                "Try to read an index past the end with `[i]`, then with `.get(i)`",
                "Write one sentence on when you would use each",
            ],
            &[
                "`Vec` creation and `push`",
                "indexing versus `get`",
                "iteration",
                "`insert` and `remove`",
            ],
            &[
                "No `HashMap` or `HashSet` in this project",
                "Every panic must be commented out before it stops you running the rest",
                "The final sentence comparing `[i]` and `get` is required",
                "Do not clone strings you do not need",
            ],
            "let mut songs: Vec<String> = Vec::new();\nsongs.push(String::from(\"Blackbird\"));\nprintln!(\"{:?}\", songs.get(0));\nprintln!(\"{}\", songs[0]);",
            &[
                "Print `len` and `capacity` as the vector grows and watch the capacity jump",
                "Insert a song at the front and count how the output changes",
            ],
        ),

        SeedProject::new(
            "Song List",
            "A `Vec<Song>` with add, remove, and a total-duration method. The playlist as an ordered list, using the struct you built in Module 07.",
            Difficulty::Beginner,
            "40 min",
            &[
                "Define `Song { title, artist, duration_secs }`",
                "Represent the playlist as a `Vec<Song>`",
                "Write `add(&mut self, song: Song)`",
                "Write `total_duration(&self) -> u32`",
                "Write `remove_first(&mut self) -> Option<Song>`",
                "Build a playlist of four songs and print the total",
            ],
            &[
                "struct in a vector",
                "`&mut self` methods",
                "iteration by reference",
                "`Option` from `pop`/`remove`",
            ],
            &[
                "The playlist must own its songs; no references in the struct",
                "No indexing without a length check",
                "`total_duration` must not move any song",
                "Every method gets a comment naming its receiver and why",
            ],
            "playlist.add(Song { title: String::from(\"Nude\"), artist: String::from(\"Radiohead\"), duration_secs: 255 });\nprintln!(\"{} seconds\", playlist.total_duration());",
            &[
                "Add a method that returns the longest song as `Option<&Song>`",
                "Add a method that returns all songs by a given artist as `Vec<&Song>`",
            ],
        ),

        SeedProject::new(
            "Artist Index",
            "Build a `HashMap` from artist name to that artist's song titles, from a list of songs. The lookup container, applied for real.",
            Difficulty::Intermediate,
            "1 h",
            &[
                "Start from a `Vec<Song>`",
                "Write `fn by_artist<'a>(songs: &'a [Song]) -> HashMap<&'a str, Vec<&'a str>>`",
                "Use `entry(...).or_insert_with(Vec::new).push(...)` to build it in one pass",
                "Print each artist and how many songs they have",
                "Write a second function that returns one artist's titles, as `Option<&Vec<&str>>`",
                "Say in a comment why the keys and values borrow instead of owning",
            ],
            &[
                "`HashMap` construction",
                "the `entry` API",
                "map of vectors",
                "borrowed keys and values",
            ],
            &[
                "Do not clone titles into the map — borrow them",
                "Build the whole index in a single pass over the songs",
                "No scanning the `Vec` to answer 'what is by this artist'",
                "The comment on borrowing is required",
            ],
            "let index = by_artist(&songs);\nif let Some(titles) = index.get(\"Radiohead\") {\n    println!(\"Radiohead: {} songs\", titles.len());\n}",
            &[
                "Add a function that finds the artist with the most songs",
                "Make a second index keyed by genre and compare the two",
            ],
        ),

        SeedProject::new(
            "Genre Set",
            "A `HashSet` of genres, plus set operations between two playlists. Membership and uniqueness, applied to real questions.",
            Difficulty::Intermediate,
            "1 h",
            &[
                "Write `fn genres<'a>(songs: &'a [Song]) -> HashSet<&'a str>`",
                "Build two playlists and get each one's genre set",
                "Print the distinct genres of each",
                "Print the genres they share (intersection)",
                "Print the genres in the first but not the second (difference)",
                "Use `insert`'s `bool` to report whether a genre was already seen",
            ],
            &[
                "`HashSet` membership",
                "`insert` returning whether a value was new",
                "set operations",
                "borrowing from a collection",
            ],
            &[
                "No `Vec` for membership questions",
                "The shared/difference output must use the set operations, not nested loops",
                "Say in a comment why the set borrows its genres",
                "No panic on an empty playlist",
            ],
            "let a = genres(&playlist_a);\nlet b = genres(&playlist_b);\nlet shared: Vec<&&str> = a.intersection(&b).collect();\nprintln!(\"shared: {shared:?}\");",
            &[
                "Report the union as a sorted `Vec` and explain why sorting was needed",
                "Add a check for whether a specific genre appears in *either* playlist",
            ],
        ),

        SeedProject::new(
            "Playlist Manager",
            "A small CLI combining all three containers: songs in a `Vec`, an artist index in a `HashMap`, and genres in a `HashSet`. Search and report.",
            Difficulty::Advanced,
            "2 h",
            &[
                "Model `Song` and a `Playlist` holding `Vec<Song>`",
                "Derive the artist index and genre set from the songs",
                "Write `find(&self, title: &str) -> Option<&Song>`",
                "Write `by_artist(&self) -> HashMap<&str, Vec<&str>>`",
                "Write `genres(&self) -> HashSet<&str>`",
                "Write a report that prints totals, the artist index, and the genre count",
            ],
            &[
                "choosing a container per question",
                "methods returning borrowed collections",
                "`Option` from lookups",
                "iteration and reporting",
            ],
            &[
                "Each collection must be justified in a comment",
                "No cloning the song list to build the indexes",
                "No scanning the `Vec` to answer a lookup or membership question",
                "The report must not mutate the playlist",
            ],
            "match playlist.find(\"Nude\") {\n    Some(s) => println!(\"found {} by {}\", s.title, s.artist),\n    None => println!(\"not in the playlist\"),\n}\n\nfor (artist, titles) in playlist.by_artist() {\n    println!(\"{artist}: {}\", titles.len());\n}",
            &[
                "Add 'songs longer than N seconds' as a `Vec<&Song>` filter",
                "Add 'merge with another playlist' and report the combined distinct genres",
            ],
        ),

        SeedProject::new(
            "Music Playlist",
            "The capstone: the program built across the module, finished and justified. A `Vec` of songs, a `HashMap` artist index, a `HashSet` of genres, and a written defense of every container.",
            Difficulty::Advanced,
            "3 h",
            &[
                "Model `Song` and a `Playlist` that owns its songs in order",
                "Provide lookups, the artist index, and the genre set",
                "Add a count of songs per artist using the `entry` API",
                "Add duplicate detection when adding a song",
                "Print a full report: count, total duration, artist index, distinct genres",
                "Write a design note: for every collection, say why it is a `Vec`, a `HashMap`, or a `HashSet`, and why not the other two",
            ],
            &[
                "all three containers",
                "the `entry` API",
                "borrowed keys and values",
                "methods and receivers",
                "API design and justification",
            ],
            &[
                "No wrong-container choices — each must match its question",
                "No scans for lookup or membership questions",
                "No clone unless the design note justifies it",
                "The design note is required — working code alone is not the deliverable",
            ],
            "println!(\"{} songs, {} seconds, {} genres\", playlist.len(), playlist.total_duration(), playlist.genres().len());\n\nprintln!(\"Radiohead: {} songs\", playlist.by_artist().get(\"Radiohead\").map(|v| v.len()).unwrap_or(0));",
            &[
                "Add 'top artist by song count' and explain which containers it touches",
                "Add 'merge two playlists without duplicate songs' and decide what 'duplicate' means",
                "Write the design note a second time, after the code, and compare the two",
            ],
        ),
    ]
}
