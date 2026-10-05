# BORROW — Module 08: Collections

> **Status:** DRAFT SPEC — for review. No implementation starts until this is
> approved. This document is the source of truth the lessons, exercises, and
> interactive blocks are built from.

---

## 0. Decisions locked

| Decision | Choice |
|---|---|
| Position | Module 08, slug `collections`, id 8, ID block 8000 |
| Depth | Deep dive (~34 lessons) |
| Continuous project | **Music Playlist** — a new codebase that grows across the module |
| Breadth | `Vec`, `HashMap`, `HashSet` in depth; `BTreeMap` and `VecDeque` mentioned in a decision table |
| Interactive UI | Reuse the three widgets from Module 07 (`gap_fill`, `step_through`, `guess_run`) — no new front-end code |
| Build order | Spec → content → verify (no component phase this time) |

---

## 1. Module Purpose

So far every value has been one thing: a number, a string, a struct you built
yourself. Real programs work with *many* things — a playlist of songs, an index
of artists, a set of genres — and they need the right container for the shape of
the data.

This module teaches the three containers a beginner reaches for first:

- **`Vec<T>`** — an ordered list. Things in a row, reached by position.
- **`HashMap<K, V>`** — a lookup table. Things reached by key, order irrelevant.
- **`HashSet<T>`** — a bag of unique items. "Is this in here?" and set arithmetic.

The beginner failure this module must prevent is using the wrong container out
of habit: a `Vec` scanned with a loop where a `HashMap` would be a direct
lookup, or a `HashMap` used to keep order that matters. Choosing the container
is the skill; the syntax is small.

Central idea:

> A `Vec` is for **position**. A `HashMap` is for **lookup**. A `HashSet` is
> for **membership**.

---

## 2. Audience & Prerequisites

Assume the learner can already:

- read and write all the types from Modules 01–06 (`String`, `Vec`, references,
  ownership, borrowing)
- define structs and enums, derive `Debug`/`Clone`/`PartialEq`, and write methods
  (Module 07)
- read a borrow-checker error and say which two accesses overlap

Do **not** re-teach `Vec` from scratch — it appeared as `Vec<Topping>` in Module
07 and as `Vec<String>` in Module 02. This module explains *why* and *when*, and
teaches the two containers that are genuinely new.

Do **not** teach, and explicitly defer:

- iterators as a topic (Module 11). This module uses `for` loops deliberately;
  `.iter().map()` appears only where Module 07 already used it, without
  explanation
- traits and generics machinery (Module 10) — use `<T>` types, do not explain
  bounds
- `Option`/`Result` deeply (Module 09) — `.get()` returning `Option` is shown
  and named, but `?` and error handling are Module 09's job

---

## 3. Beginner-First Principles

Carried over from Module 07, and enforced here:

1. **Motivation before syntax.** Open with the problem ("scanning this list
  every time is getting silly"), not "a `HashMap` is…".
2. **One idea per lesson.**
3. **The Music Playlist never resets.** Every lesson continues one program.
4. **Predict before reveal.** Every lesson asks a question first.
5. **Plain language; define before use.** First use of *key*, *value*, *hash*,
  *capacity*, *entry* is defined in the same breath.
6. **Gentle failure.** Hints always available, no timers, no locking.
7. **Show, then say.** A 4-line example beats a paragraph.
8. **No incorrect simplifications.** Never: "a `HashMap` is sorted", "`Vec`
  indexing and `get` are interchangeable", "a `HashSet` is just a `Vec` of
  unique things under the hood", "`.len()` tells you the capacity".

---

## 4. The Continuous Project — Music Playlist

One program grows across the module. The learner always sees the current whole
file and the new lines.

### The domain (introduced piece by piece)

```rust
// Appears in this order over the module:
struct Song {
    title: String,
    artist: String,
    duration_secs: u32,
    genre: String,
}

// A playlist is an ordered list of songs.
let mut playlist: Vec<Song> = Vec::new();

// An index from artist name to that artist's songs (lookup).
let mut by_artist: HashMap<String, Vec<String>> = HashMap::new();

// The set of genres present (membership, no duplicates).
let mut genres: HashSet<String> = HashSet::new();
```

### Lesson-by-lesson growth (high level)

| Stage | Feature added |
|---|---|
| Vectors | a playlist as a `Vec<Song>`; push, index, `get`, iterate, remove |
| HashMaps | an artist index: `HashMap<String, Vec<String>>`; insert, `get`, `entry` |
| HashSets | a genre set: dedupe, `contains`, union/intersection |
| Choosing | a table that explains why each list is the shape it is |
| Real world | the complete playlist program: add, search, report, de-duplicate |

### Why Music Playlist

- **Relatable.** Everyone has made a playlist; no domain explanation needed.
- **Naturally uses all three containers.** Order (playlist), lookup (artist
  index), membership (genres) — each is a genuine fit, not a contrived one.
- **Carries Module 07 forward.** `Song` is a struct; a `Vec<Song>` is exactly
  the "collection of structs" pattern, now expanded.
- **Sets up interesting questions.** Duplicate songs, songs by an artist,
  genres shared between playlists — real reasons to reach for each container.

---

## 5. Interactive Components (reused)

No new front-end code. The three widgets from Module 07 carry the interactive
load, used where they are the natural fit:

### `gap_fill` — method and type recall
- the method that adds to a `Vec` (`push`), to a map (`insert`), to a set
  (`insert`)
- the return type of `.get(...)` (`Option<&V>`)
- the `entry(...).or_insert(...)` chain
- `HashMap<String, Vec<String>>` as a type

### `step_through` — what the container actually does
- **`Vec` growth**: pushing past capacity, the reallocation, why a reference
  into the vector cannot survive it (ties back to Module 06)
- **`HashMap` lookup**: key hashed, bucket found, value returned — the mental
  model without the implementation detail
- **`HashSet` dedupe**: inserting a value already present
- **iterate-while-mutate**: the borrow conflict, step by step

### `guess_run` — predict, then compile
- does this compile / what prints — for `get` vs indexing, iteration order,
  mutate-while-iterating, and duplicates in a set

All three keep their no-JS fallbacks; a lesson with scripting disabled still
reads completely.

---

## 6. Module Structure — 34 Lessons

One lesson per part. "Hook" names the interactive block the lesson is built
around.

### Vectors (`vectors.rs`, 10 lessons)

1. **From One Value to Many** — a hard-coded four-song program that begs for a
   list. Hook: observe.
2. **Creating a `Vec`** — `Vec::new`, `vec![]`, and the type annotation.
   Hook: gap_fill.
3. **`push` and `pop`** — growing and shrinking from the end. Hook: gap_fill.
4. **Indexing vs `get`** — `playlist[0]` panics; `.get(0)` returns
   `Option`. Hook: guess_run.
5. **Iterating by Reference** — `for song in &playlist`. Hook: step_through.
6. **Iterating Mutably** — `for song in &mut playlist`, changing songs in place.
   Hook: step_through.
7. **Iterating by Value** — `for song in playlist`, and what it consumes.
   Hook: guess_run.
8. **Inserting and Removing in the Middle** — `insert`, `remove`, and why they
   are more expensive than push/pop. Hook: gap_fill.
9. **A `Vec` of Structs** — the playlist proper. Hook: step_through.
10. **Capacity and Reallocation** — why a reference into a `Vec` cannot survive
    a `push`. Hook: step_through.

### HashMaps (`hashmaps.rs`, 10 lessons)

11. **The Scan Problem** — finding a song by title by looping is getting slow to
    read; a key lookup is the fix. Hook: observe.
12. **Creating a `HashMap` and Inserting** — `HashMap::new`, `insert`. Hook:
    gap_fill.
13. **Looking Up with `get`** — `Option<&V>`, and `get_mut`. Hook: guess_run.
14. **Who Owns the Keys and Values** — moving a `String` key in; borrowed
    lookups with `&str`. Hook: step_through.
15. **The `entry` API** — `entry(key).or_insert(value)`, and counting. Hook:
    gap_fill.
16. **Iterating a `HashMap`** — order is not guaranteed; iterate pairs. Hook:
    guess_run.
17. **Counting with a Map** — the artist → song-count example. Hook: gap_fill.
18. **A Map of Vectors** — `HashMap<String, Vec<String>>`, the artist index.
    Hook: step_through.
19. **Updating and Removing Entries** — overwrite, `remove`, `contains_key`.
    Hook: gap_fill.
20. **When Order Matters** — `BTreeMap` as the ordered alternative. Hook:
    observe/table.

### HashSets (`sets.rs`, 6 lessons)

21. **Uniqueness** — "which genres are in this playlist?" Hook: observe.
22. **`insert` and `contains`** — membership, and inserting a duplicate.
    Hook: gap_fill.
23. **Iterating a Set** — unordered, like a map's keys. Hook: guess_run.
24. **Set Operations** — `union`, `intersection`, `difference`. Hook:
    step_through.
25. **De-duplicating a `Vec`** — collect into a `HashSet` and back. Hook:
    gap_fill.
26. **Sets and Ownership** — a set owns its items; borrowed lookups. Hook:
    step_through.

### Choosing (`choosing.rs`, 4 lessons)

27. **The Decision Table** — position vs lookup vs membership. Hook: gap_fill.
28. **`BTreeMap` and `VecDeque`** — the ordered map and the double-ended queue,
    named and placed, not taught in depth. Hook: table.
29. **A Little Performance Intuition** — `O(1)` vs `O(n)` in plain language, no
    formal analysis. Hook: table.
30. **Common Mistakes** — ten, each with a broken example, prediction, compiler
    behaviour, fix, and rule. Hook: guess_run.

### Real World (`real_world.rs`, 3 lessons)

31. **The Playlist, Assembled** — the whole program: `Vec<Song>`, an artist
    index, a genre set, and counting. Hook: all three.
32. **Searching and Reporting** — a method that finds by title, lists an
    artist's songs, and reports the genres. Hook: step_through.
33. **Change the Design** — extend the program: duplicate detection, merging two
    playlists, "songs longer than three minutes". Hook: all three.

### Model (`model.rs`, 1 lesson)

34. **Final Mental Model** — position / lookup / membership, the decision
    procedure, and a checklist. Points ahead to `Option`/`Result` (Module 09).

---

## 7. Exercise Plan

- **1 exercise per lesson** (~34 total), in lesson order.
- **Formats**: gap-fill (method and type recall), predict-the-output,
  predict-the-compiler, fix-code, trace-the-container, choosing-the-collection.
- Each exercise is the *application* of the lesson; recognition practice lives
  in the lesson's interactive blocks.
- Every exercise has a one-sentence prompt, at least one hint, and an
  explanation that states the rule.
- No exercise requires writing a whole program; open-ended work lives in the
  challenges and projects.

---

## 8. Challenges (open-ended, `.code` + hints ≥ 2)

1. **WORD FREQUENCY** — count how many times each word appears in a block of
   text using a `HashMap<&str, u32>` and the `entry` API, then print the top
   three. The classic map exercise, made concrete.
2. **PLAYLIST TOOLKIT** — given a `Vec<Song>`, build an artist index and a genre
   set, then answer: how many distinct genres, which artists have more than one
   song, and is a given song already present. Requires choosing a container for
   each question.
3. **CONTAINER DETECTIVE** — given several short scenarios ("track the order
   songs were added", "check whether a genre is present", "find the song with a
   given id"), choose `Vec`, `HashMap`, or `HashSet` for each and justify it in
   one sentence. No code required; the reasoning is the deliverable.

Each ships with progressive hints and a full walkthrough as the final hint.

---

## 9. Projects (6, escalating)

1. **Vector Playground** — deliberately repetitive: build, grow, index, iterate,
   and remove from a list; record what panics and what returns `Option`.
2. **Song List** — a `Vec<Song>` with add, remove, and a total-duration method.
3. **Artist Index** — a `HashMap<String, Vec<String>>` built from a song list.
4. **Genre Set** — a `HashSet<String>` with membership and set operations
   between two playlists.
5. **Playlist Manager** — a small CLI combining all three containers with
   search and reporting.
6. **Music Playlist (capstone)** — the full running program, plus the written
   task: *for every collection in your program, say why it is a `Vec`, a
   `HashMap`, or a `HashSet` — and why not the other two.*

Each includes name, difficulty, estimated time, goal, requirements, concepts,
constraints, example interaction, and stretch goals (matching `SeedProject`).

---

## 10. Diagram Requirements

Diagrams answer a question, never decorate.

- **`Vec` as a row** — indexed slots, and what happens when it grows past
  capacity (a new, larger buffer).
- **`HashMap` as a lookup** — key → hashed → bucket → value; order not implied.
- **`HashSet` as membership** — a value either is or is not in the set;
  duplicates collapse.
- **The wrong tool** — scanning a `Vec` for a key vs a direct `HashMap` lookup.
- **Container decision** — position / lookup / membership as three boxes.

---

## 11. Content Style & Tone

- Short sentences, concrete nouns, the voice of an experienced Rust developer.
- Every lesson's first line names the problem, not the feature.
- Every lesson ends by advancing the Music Playlist program.
- Prefer a 4-line example over a paragraph; a diagram when the idea is spatial;
  a table for comparisons.
- Avoid: fake enthusiasm, vague "this is important", jargon before definition,
  walls of prose.

---

## 12. Teaching Requirements

1. Motivation before syntax, always.
2. Make the learner predict before revealing.
3. Explain compiler errors in terms of the rule enforced and the design
   alternative.
4. Never teach a simplification you will have to un-teach (see §3.8).
5. Prefer the smallest realistic example that shows the idea.

---

## 13. Module Outcome

At the end, given a new problem, the learner can:

- choose `Vec`, `HashMap`, or `HashSet` from the *shape* of the data and say why
- create, grow, index, iterate, and remove from a `Vec`, and explain the
  difference between indexing and `get`
- build and query a `HashMap`, use `entry`, and explain why iteration order is
  not guaranteed
- use a `HashSet` for membership and do union/intersection/difference
- explain, in plain language, why a lookup beats a scan
- recognise the borrow conflict of mutating a collection while iterating it
- know that `BTreeMap` exists for when order matters

And they should hold the module's core idea without syntax:

> A `Vec` is for **position**. A `HashMap` is for **lookup**. A `HashSet` is
> for **membership**.

---

## 14. Build Order & Verification

1. **Spec** — this document, reviewed and approved.
2. **Content** — `src/content/collections/` (lessons, exercises, challenges,
   projects), registered in `content/mod.rs`; remove the placeholder.
3. **Verify** — `cargo test` (content integrity + widget rendering), the ignored
   `sweep_every_seeded_fragment` (every valid Rust panel compiles), a
   full-program compile check, and a fresh-DB / HTTP smoke test.

---

## 15. Resolved decisions

1. **`entry` API** — taught. `entry(key).or_insert(value)` and
   `or_insert_with` are the idiomatic way to count, and they appear in the
   counting lessons and the word-frequency challenge.
2. **Challenges** — three as listed.
3. **`Vec` internals** — go deeper. Lesson 10 covers capacity, the growth
   strategy, the pointer move, and why a reference into the vector cannot
   survive a reallocation, with a diagram.
4. **Hashing** — keys must be hashable: say so plainly ("a key must be a type
   Rust can hash — text and numbers are"), without teaching the `Hash`/`Eq`
   traits themselves.
