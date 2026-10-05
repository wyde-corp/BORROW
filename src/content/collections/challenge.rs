//! Module 08 — the three module challenges.
//!
//! Open-ended by design: one counting exercise, one toolkit that requires
//! choosing a container per question, and one reasoning-only detective. The
//! walkthrough is always the last hint.

use crate::content::types::{ExerciseKind, SeedExercise};

const FREQUENCY_START: &str = r#"use std::collections::HashMap;

// Count how many times each word appears, then print the three most common.
// Words are separated by spaces; punctuation may be ignored.
fn main() {
    let text = "the quick brown fox jumps over the lazy dog the fox sleeps";

    let mut counts: HashMap<&str, u32> = HashMap::new();

    // TODO: count each word with the entry API.

    // TODO: print the top three words and their counts.
    println!("{:?}", counts);
}"#;

const TOOLKIT_START: &str = r#"use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Song {
    title: String,
    artist: String,
    duration_secs: u32,
    genre: String,
}

// Given a list of songs, answer three questions:
//   1. how many distinct genres are there?
//   2. which artists have more than one song?
//   3. is a song with a given title present?
fn main() {
    let songs = vec![
        Song { title: String::from("Nude"), artist: String::from("Radiohead"), duration_secs: 255, genre: String::from("Alternative") },
        Song { title: String::from("Karma Police"), artist: String::from("Radiohead"), duration_secs: 261, genre: String::from("Alternative") },
        Song { title: String::from("Blackbird"), artist: String::from("The Beatles"), duration_secs: 208, genre: String::from("Folk") },
    ];

    // TODO: choose a container for each question, then answer it.
    let _ = songs;
}"#;

const DETECTIVE_SCENARIOS: &str = r#"// For each scenario, choose Vec, HashMap, or HashSet, and say why in one
// sentence. No code is required — the reasoning is the deliverable.
//
// 1. Track the order in which songs were added to a session.
// 2. Find the duration of a song given its title.
// 3. Check whether a genre has appeared anywhere in the playlist.
// 4. Count how many songs belong to each artist.
// 5. Keep the playlist sorted by title for display.
// 6. Add and remove songs from the front of a queue of "next up".
// 7. Store each song once even if it is added twice.
// 8. Look up a song by a numeric id.
"#;

pub const FREQUENCY_WALKTHROUGH: &str = r#"The shape is the counting idiom from the module, applied to
words instead of genres:

  for word in text.split_whitespace() {
      *counts.entry(word).or_insert(0) += 1;
  }

`split_whitespace()` yields borrowed `&str` slices of `text`, which is why the
map is `HashMap<&str, u32>` — the keys borrow from the original string rather
than owning new ones. That is a real decision: it is cheap, but the map cannot
outlive `text`.

For the top three, the honest answer is that a `HashMap` has no order, so you
must collect the pairs and sort:

  let mut pairs: Vec<(&str, u32)> = counts.into_iter().collect();
  pairs.sort_by(|a, b| b.1.cmp(&a.1));
  for (word, n) in pairs.iter().take(3) { ... }

Notice the two choices: a map to count (lookup by word), a vector to rank
(position matters). That is the whole module in one program."#;

pub const TOOLKIT_WALKTHROUGH: &str = r#"Each question wants a different container, which is the
point.

1. Distinct genres — a `HashSet<&str>`, collected from `song.genre.as_str()`.
   Membership, no duplicates.

2. Artists with more than one song — first a `HashMap<&str, u32>` counting songs
   per artist with `entry(...).or_insert(0)`, then filter the map for counts
   greater than one. The count is a lookup by artist.

3. Is a title present — a `HashSet<&str>` of titles, or a `HashMap<&str, &Song>`
   if you also want the song back. Membership if you only need yes/no; a map if
   you need the value.

A common mistake is to reach for one container for all three. The exercise is to
notice that the containers differ, and to say why in a sentence each."#;

pub const DETECTIVE_WALKTHROUGH: &str = r#"1. Order of addition — a `Vec`: order is the question.
2. Duration by title — a `HashMap<&str, u32>`: lookup by key.
3. Genre present — a `HashSet<&str>`: membership.
4. Songs per artist — a `HashMap<&str, u32>`: a count per key.
5. Sorted by title — a `BTreeMap` (or a `Vec` you sort): ordered keys.
6. Fast at both ends — a `VecDeque`: front operations a `Vec` cannot do cheaply.
7. Each song once — a `HashSet`: uniqueness.
8. Lookup by id — a `HashMap<u32, &Song>`: lookup by key; the key is a number.

If you chose a `Vec` for 3, 4, or 8, ask the question again: those are membership
or lookup, and a `Vec` would force a scan. If you chose a map for 1, 5, 6, or 7,
the question is about order or uniqueness, which a map does not provide."#;

pub fn challenges() -> Vec<SeedExercise> {
    vec![
        SeedExercise::new(
            None,
            ExerciseKind::Challenge,
            "WORD FREQUENCY — Count how many times each word appears in a block of text, then print \
             the three most common. Use the `entry` API for the counting. Decide what the keys \
             should borrow from, and why.",
            &[
                "Split the text into words",
                "Use `entry(word).or_insert(0)` and increment",
                "Decide whether the keys should own their text or borrow it, and say why",
                "Print the three most common words",
                "Explain why you needed a second container to rank them",
            ],
            0,
            FREQUENCY_WALKTHROUGH,
        )
        .code(FREQUENCY_START)
        .xp(160)
        .hints(&[
            "Counting a key is `entry(key).or_insert(0)`, then increment.",
            "`split_whitespace()` gives borrowed `&str` slices — a natural map key.",
            "A `HashMap` cannot be sorted; collect to a `Vec` to rank.",
            "The walkthrough is the last hint; it shows one faithful solution.",
        ]),
        SeedExercise::new(
            None,
            ExerciseKind::Challenge,
            "PLAYLIST TOOLKIT — Given a list of songs, answer three questions: how many distinct \
             genres, which artists have more than one song, and whether a given title is present. \
             Choose a container for each question and justify it.",
            &[
                "Answer each question with a container chosen for its shape",
                "Do not use one container for all three",
                "For 'which artists have more than one song', count first, then filter",
                "Say in one sentence why each container fits",
                "Borrow from the songs rather than cloning titles",
            ],
            0,
            TOOLKIT_WALKTHROUGH,
        )
        .code(TOOLKIT_START)
        .xp(170)
        .hints(&[
            "Distinct genres is membership; counting per artist is a lookup; a title check is membership.",
            "`entry(...).or_insert(0)` counts songs per artist in one pass.",
            "A map of counts can be filtered afterwards into a list.",
            "The walkthrough names a container for each question.",
        ]),
        SeedExercise::new(
            None,
            ExerciseKind::Challenge,
            "CONTAINER DETECTIVE — Eight scenarios describe data needs. For each, choose `Vec`, \
             `HashMap`, or `HashSet` (or name the better-fitting cousin) and justify it in one \
             sentence. No code is required; the reasoning is the deliverable.",
            &[
                "For each scenario, name the question being asked",
                "Choose the container that answers it directly",
                "Give one sentence of justification per choice",
                "Call out any scenario where `BTreeMap` or `VecDeque` is the better fit",
                "Do not write code — the point is the decision",
            ],
            0,
            DETECTIVE_WALKTHROUGH,
        )
        .code(DETECTIVE_SCENARIOS)
        .xp(140)
        .hints(&[
            "Ask 'position, lookup, or membership?' for each scenario.",
            "Sorted display is a signal for `BTreeMap`; front operations for `VecDeque`.",
            "If you are tempted to scan a `Vec`, the question is probably a lookup or membership.",
            "The walkthrough lists a container for every scenario.",
        ]),
    ]
}
