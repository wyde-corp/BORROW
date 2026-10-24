# BORROW_ — UI Design Specification

## Product

**BORROW_** is a lightweight Rust fundamentals learning platform.

The product teaches Rust through a simple loop:

> **Learn → Practice → Build → Progress**

The first version is intentionally small. It focuses strictly on the fundamentals of the Rust programming language and should feel fast, approachable, and enjoyable.

---

# 1. Core Design Direction

## Retro-futuristic, not terminal-themed

The visual identity should combine:

- 1970s educational computing
- 1980s personal computers
- early 1980s/1990s sci-fi interfaces
- retro arcade aesthetics
- futuristic industrial technology
- subtle punk / experimental design

However, **BORROW_ must NOT look like a terminal emulator**.

Do not make the UI look like:

```text
$ cargo run
> SYSTEM ONLINE
> USER: GUS
> MEMORY: SAFE
```

The product is a modern web application with a retro-futuristic visual identity.

Think:

> **A beautifully designed learning app from an alternate future where 1980s computer design evolved instead of being replaced.**

The interface should be:

- clean
- responsive
- spacious
- highly readable
- modern in interaction
- visually distinctive
- lightweight
- fast

Retro elements should be used as visual seasoning rather than as the entire interface.

---

# 2. Design Principles

### 1. Learning comes first

The UI should never make the learner fight the interface.

Content must remain the most important element.

### 2. Modern UX, retro visual language

Use modern:

- responsive layouts
- navigation
- buttons
- cards
- progress indicators
- interactive exercises
- mobile layouts
- transitions

But style them using the BORROW_ visual identity.

### 3. Minimalism

Avoid unnecessary:

- dashboards full of statistics
- excessive cards
- excessive icons
- huge illustrations
- unnecessary animations
- complicated navigation
- social features

Every element should have a purpose.

### 4. Strong visual hierarchy

A user should immediately understand:

1. Where they are
2. What they are learning
3. What they need to do
4. How much progress they have made
5. What comes next

### 5. Make the product feel alive

Use subtle motion and micro-interactions:

- progress bars animate when updated
- buttons have small hover transitions
- completed lessons have a satisfying completion animation
- exercise feedback appears smoothly
- navigation transitions are subtle

Never use excessive motion.

---

# 3. Brand

## Name

# BORROW_

The underscore is part of the visual identity.

It references:

- programming
- cursors
- Rust's ownership/borrowing model
- old computer interfaces

But the site itself should **not** behave like a terminal.

### Logo

Primary:

```text
BORROW_
```

Alternative compact mark:

```text
B_
```

The logo should be simple enough to work in:

- navbar
- favicon
- mobile header
- loading state
- completion badges

---

# 4. Color System

The base should be dark, warm, and slightly industrial.

Avoid the generic:

> dark navy + purple gradient + glowing blue

developer SaaS aesthetic.

## Primary background

```text
#0B0A08
```

Very dark warm charcoal.

## Surface

```text
#14120F
```

Used for cards, panels, navigation surfaces.

## Elevated surface

```text
#1C1915
```

Used for active cards and interactive areas.

## Border

```text
#34302A
```

Thin, understated borders.

## Primary accent — Burnt Orange

```text
#FF6B35
```

Use for:

- primary buttons
- active states
- progress highlights
- important visual accents

## Secondary accent — Retro Yellow

```text
#F2C14E
```

Use sparingly for:

- achievements
- milestones
- important highlights

## Success — Phosphor Green

```text
#39FFB6
```

Use for:

- correct answers
- completed lessons
- successful exercises
- positive feedback

## Secondary accent — Electric Cyan

```text
#5CC8FF
```

Use for:

- links
- information
- secondary highlights

## Rare accent — Retro Violet

```text
#A66CFF
```

Use very sparingly.

It should not become the dominant color.

## Main text

```text
#F4EFE6
```

Warm off-white.

## Secondary text

```text
#A9A197
```

Muted warm gray.

---

# 5. Typography

Use a combination of a readable modern monospace font and an optional retro display font.

## Primary UI font

Preferred:

**IBM Plex Mono**

Alternative:

**JetBrains Mono**

Use for:

- body text
- navigation
- buttons
- code
- labels
- metadata

## Display font

Optional:

**VT323**
or
**Press Start 2P**

Use very sparingly.

Good uses:

- BORROW_ logo
- major section title
- occasional badge
- special achievement

Do NOT use pixel fonts for large amounts of body text.

Readability is more important than nostalgia.

---

# 6. Shape Language

Avoid making every component extremely rounded.

Use a mixture of:

- slightly rounded corners
- square corners
- thin borders
- hard-edged panels

Suggested border radius:

```text
Small:   4px
Medium:  6px
Large:   8px
```

Avoid:

```text
border-radius: 24px
border-radius: 9999px
```

unless used intentionally for pills or small status indicators.

The design should feel **precise and engineered**.

---

# 7. Borders

Borders are an important part of the visual language.

Use subtle 1px borders:

```text
#34302A
```

Active elements may use the primary accent.

Do not put borders around absolutely everything.

Use borders to establish hierarchy.

---

# 8. Background

The main background should be mostly flat.

Optional subtle texture:

- extremely subtle grain
- extremely subtle CRT scanlines
- subtle noise

These effects must be almost invisible.

The user should notice the aesthetic, not the effect.

Never cover the entire UI with an obvious CRT overlay.

---

# 9. Layout

The application should use a responsive layout.

## Desktop

Recommended maximum content width:

```text
1200px
```

Main content should remain centered.

Example:

```text
┌─────────────────────────────────────────────────────────────┐
│ BORROW_                            Progress   Profile       │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│                MAIN CONTENT AREA                            │
│                                                             │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Tablet

Collapse navigation where necessary.

Maintain generous spacing.

## Mobile

The application should feel intentionally designed for mobile rather than simply shrinking the desktop layout.

Use:

- compact top navigation
- bottom navigation if useful
- stacked content
- full-width exercise controls
- large touch targets

---

# 10. Navigation

The navigation should be simple.

Desktop:

```text
BORROW_     Learn     Practice     Projects          Progress
```

Optional right side:

```text
[Progress] [Profile]
```

Do not create a huge navigation system.

Mobile:

```text
BORROW_                         ☰
```

or a simple bottom navigation.

---

# 11. Homepage

The homepage **is** the download page. One screen, one action: get the app.

Direction: **"gilded CRT"** — a dark umber void (starfield + film grain) with
Renaissance art duotoned to the palette. Luxury restraint: generous space,
hairline gold rules, letter-spaced small caps. Executed like a product page,
not a fan page.

```text
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│  BORROW_                                                    │
│  ラストの遠回りを、やめよう。                                  │
│                                                             │
│  LEARN RUST                       ┌───────────────────────┐ │
│  WITHOUT THE                      │ ● ● ●  creation_of_  │ │
│  700-PAGE DETOUR.                 │  adam.dpt — 1512      │ │
│                                   │  [duotone Michelangelo│ │
│  Short lessons. Real practice.     │   hands — public      │ │
│  A native desktop trainer for      │   domain]             │ │
│  macOS, Windows & Linux.           └───────────────────────┘ │
│  ──────────── ◆ ────────────            caption (credit)     │
│  ▢ AVAILABLE NOW                                         │
│  [ Download for macOS ]   ← the one CTA, OS-aware            │
│  Also for Windows · Linux                                   │
│  V2.1 · MACOS · WINDOWS · LINUX · FREE                      │
│                                                             │
│  ───────────────────────────────────────────────────────────│
│  BORROW_ · GitHub · wyde industries · MMXXVI                 │
└─────────────────────────────────────────────────────────────┘
```

## Type

- Display: **Cormorant Garamond** (roman caps; the Renaissance voice).
- Body / UI: **IBM Plex Mono**.
- Outlier: **VT323**, exactly two slots — the wordmark underscore and the
  AVAILABLE NOW badge (the two "live" blinking signals).
- JP tagline: Noto Serif JP (subset), the display serif's Japanese companion.
  No pixel fonts.

## Color discipline

- Gold (`--gold`) owns **all ornament**: hairline frames, rules, small caps.
- Orange (`--accent`) is reserved for **the download CTA and the two live
  signals** — nothing else may be orange.
- Ivory text on umber-black. Palette lives in `site/tokens.css` (OKLCH).

## Rules

- **One CTA.** OS-aware label + direct-asset href via download-widget; falls
  back to the GitHub releases page. Mobile (< 960px) gets a sticky bottom bar
  and the art becomes a dimmed backdrop behind the text.
- **No sections beyond hero + one-line footer.** No tickers, no guestbook, no
  hit counter, no PRO pitch, no related links.
- **Window chrome only on the art plate** — thin gold titlebar, hollow gold
  dots. It is a framed painting, not a fake OS.
- **Motion**: one entrance reveal, the two blinks, a CTA hover lift.
  `prefers-reduced-motion` honored everywhere.
- Art asset: `site/assets/creation-of-adam-duotone.jpg` (960w variant for
  mobile) — public domain, processed to a 4-stop gold gradient map + grain.

---

# 12. Learning Dashboard

The dashboard should feel like a **training map**, not an analytics dashboard.

Example:

```text
YOUR RUST JOURNEY

Continue Learning
┌──────────────────────────────────────────────────────┐
│ Ownership                                             │
│                                                      │
│ Understanding how Rust manages memory.               │
│                                                      │
│ ███████████████████░░░░░  76%                       │
│                                                      │
│ [ CONTINUE ]                                         │
└──────────────────────────────────────────────────────┘


CURRICULUM

01  Getting Started       ✓
02  Control Flow          ✓
03  Functions             ●
04  Ownership             ○
05  Borrowing             ○
06  Structs & Enums       ○
07  Collections           ○
08  Error Handling        ○
09  Generics & Traits     ○
10  Iterators             ○
```

The curriculum should visually communicate progression.

Avoid turning every module into a giant card.

---

# 13. Curriculum

The Rust fundamentals curriculum should initially contain:

```text
01  Getting Started
02  Variables & Types
03  Control Flow
04  Functions
05  Ownership
06  Borrowing
07  Structs & Enums
08  Collections
09  Option & Result
10  Generics & Traits
11  Iterators
```

The curriculum can later be adjusted.

Each module should have:

- short lessons
- examples
- practice exercises
- a challenge
- optional mini project

---

# 14. Lesson UI

The lesson page is the most important screen.

It should be extremely readable.

Example:

```text
OWNERSHIP

Understanding who owns a value.


Every value in Rust has an owner.

When the owner goes out of scope,
Rust automatically cleans up the value.


┌───────────────────────────────────────────────┐
│ let name = String::from("Rust");              │
│                                               │
│ println!("{name}");                           │
└───────────────────────────────────────────────┘


The important idea:

A value can only have one owner at a time.


                              03 / 06

                    [ CONTINUE → ]
```

Use visual callouts rather than terminal-like output.

---

# 15. Code Blocks

Code blocks should feel like polished developer tooling.

They should have:

- syntax highlighting
- line numbers where useful
- copy button
- clear contrast
- subtle border
- optional highlighted lines

Example:

```text
┌───────────────────────────────────────────────┐
│ Rust                              COPY        │
├───────────────────────────────────────────────┤
│ 1 │ let name = String::from("Rust");         │
│ 2 │ let other = name;                        │
│ 3 │ println!("{name}");                      │
└───────────────────────────────────────────────┘
```

Do not make code blocks look like terminal windows.

---

# 16. Practice Exercises

Practice should be interactive and focused.

Example:

```text
PRACTICE

What happens when this code runs?

┌───────────────────────────────────────────────┐
│ let x = String::from("hello");                │
│ let y = x;                                    │
│ println!("{x}");                              │
└───────────────────────────────────────────────┘

○ It prints hello
○ It prints nothing
○ It fails to compile
○ Rust automatically clones x


                         [ CHECK ANSWER ]
```

After answering:

```text
✓ CORRECT

`x` moved into `y`, so `x` can no longer
be used.

+10 XP
```

Keep feedback concise.

---

# 17. Coding Challenges

When implementing actual code editing:

The editor should look like a modern code editor with the BORROW_ theme.

Not like a fake terminal.

Layout:

```text
┌─────────────────────────────────────────────────────────────┐
│ OWNERSHIP CHALLENGE                                         │
├───────────────────────────────┬─────────────────────────────┤
│                               │                             │
│ Instructions                  │ Rust Editor                 │
│                               │                             │
│ Fix the function so it        │ fn greet(name: String) {   │
│ returns the borrowed value.   │     ...                     │
│                               │ }                           │
│                               │                             │
│                               │                             │
├───────────────────────────────┴─────────────────────────────┤
│                                      [ RUN ]                │
└─────────────────────────────────────────────────────────────┘
```

On mobile, stack the instruction panel above the editor.

---

# 18. Projects

Projects should feel like the learner is building something real.

Do not make them look like another lesson.

Example:

```text
PROJECT

NUMBER GUESSING GAME

Build your first Rust command-line game.

You'll use:

✓ Variables
✓ Control flow
✓ Loops
✓ Random numbers
✓ Input
✓ Pattern matching


PROJECT STATUS

░░░░░░░░░░░░░░░░░░░░  0%


[ START PROJECT ]
```

Projects should provide a clear sense of achievement.

---

# 19. Progress

Keep progress simple.

Useful metrics:

- modules completed
- exercises completed
- projects completed
- current streak
- XP

Avoid analytics overload.

Example:

```text
YOUR PROGRESS

RUST FUNDAMENTALS

████████████████░░░░░░  72%


7 / 11 MODULES

42 exercises completed
3 projects completed
5 day streak
680 XP
```

---

# 20. Gamification

Gamification should be lightweight.

Use:

- XP
- progress bars
- completion states
- streaks
- small achievements

Do NOT build:

- complicated economies
- virtual currency
- loot boxes
- excessive badges
- huge leaderboards

The main reward should be learning.

Example completion:

```text
┌───────────────────────────────────────────────┐
│                                               │
│              MODULE COMPLETE                 │
│                                               │
│              OWNERSHIP                        │
│                                               │
│              +100 XP                          │
│                                               │
│        NEXT: BORROWING                        │
│                                               │
│              [ CONTINUE ]                     │
│                                               │
└───────────────────────────────────────────────┘
```

---

# 21. Status Indicators

Use simple visual states.

### Locked

Muted gray.

```text
○
```

### Available

Accent outline.

```text
●
```

### In progress

Progress ring/bar.

### Completed

Small green check.

```text
✓
```

Do not overuse glowing neon.

---

# 22. Buttons

Buttons should feel modern.

Primary:

```text
[ START LEARNING ]
```

Secondary:

```text
[ VIEW CURRICULUM ]
```

Ghost:

```text
[ BACK ]
```

Buttons should have:

- clear hover state
- clear focus state
- subtle transition
- accessible contrast
- comfortable touch targets

Avoid giant pill-shaped buttons.

---

# 23. Cards

Cards should be used sparingly.

Good:

- current lesson
- project
- achievement
- module overview

Bad:

Putting every sentence inside a separate card.

Cards should feel like physical panels in a retro-futuristic machine.

Use:

```text
border: 1px solid
background: slightly elevated
```

rather than heavy shadows.

---

# 24. Decorative Elements

Use small decorative elements to establish the identity.

Examples:

- thin grid lines
- geometric corner marks
- small coordinate-like labels
- subtle scanline texture
- tiny dots
- thin orange/cyan rules
- segmented progress indicators
- retro technical diagrams

Example:

```text
MODULE 05
───────────────
OWNERSHIP

RUST FUNDAMENTALS / 05
```

These should support the design rather than dominate it.

---

# 25. Animation

Animations should be subtle.

Use:

- 150–250ms transitions
- small hover movement
- progress animation
- page transitions
- success feedback

Avoid:

- constant floating objects
- spinning elements
- excessive glitch effects
- flashing text
- aggressive CRT distortion

The application should feel calm and focused.

---

# 26. Responsive Design

Mobile is a first-class experience.

## Desktop

Use two-column layouts where helpful.

## Tablet

Reduce spacing and collapse secondary navigation.

## Mobile

Use one-column layouts.

Example:

```text
OWNERSHIP

Understanding who owns a value.

[lesson content]

[code example]

[practice]

[ CONTINUE ]
```

Never require horizontal scrolling for normal content.

Code editors may scroll horizontally when necessary.

Touch targets should be at least approximately 44px tall.

---

# 27. Accessibility

The retro aesthetic must never compromise accessibility.

Ensure:

- sufficient color contrast
- keyboard navigation
- visible focus states
- semantic HTML
- accessible buttons
- accessible form controls
- reduced-motion support
- readable font sizes
- screen-reader-friendly labels

Do not use color alone to communicate correct/incorrect states.

For example:

```text
✓ Correct
```

rather than only changing the border to green.

---

# 28. Empty States

Empty states should remain simple.

Example:

```text
NO PROJECTS STARTED

Your first project will appear here
after you complete the required modules.

[ VIEW CURRICULUM ]
```

Avoid illustrations unless they add genuine value.

---

# 29. Error States

Errors should be friendly and useful.

Example:

```text
SOMETHING WENT WRONG

We couldn't load this lesson.

[ TRY AGAIN ]
```

Do not expose raw technical errors to learners unless the error is part of the Rust exercise.

---

# 30. Overall Visual Balance

The final product should sit between these ideas:

```text
1970s
Educational computing
       ↓
1980s
Personal computers
       ↓
1990s
Developer tooling
       ↓
2020s
Modern responsive UX
       ↓
BORROW_
       ↓
Future
```

The UI should look nostalgic without looking old.

It should look futuristic without looking like generic cyberpunk.

It should look technical without looking like a terminal.

It should look playful without looking childish.

---

# 31. What BORROW_ Should NOT Look Like

Avoid:

- generic Tailwind SaaS templates
- excessive purple gradients
- glassmorphism
- giant rounded cards
- excessive neon
- fake terminal windows everywhere
- terminal command prompts as decoration
- excessive ASCII art
- cyberpunk city backgrounds
- stock illustrations
- 3D renders
- overly animated dashboards
- dark-mode-only developer clichés

The retro identity should come from:

**typography + color + spacing + geometry + subtle texture + interaction design**

not from pretending the whole website is a terminal.

---

# 32. Recommended Component System

Build reusable components around the learning experience:

```text
AppShell
Navbar
MobileNavigation

Hero
SectionHeader

ModuleCard
ModuleProgress
LessonCard

Lesson
LessonSection
CodeBlock
Callout

Question
MultipleChoice
AnswerFeedback

Exercise
CodeEditor
RunButton

ProjectCard
ProjectProgress

ProgressBar
Achievement
CompletionPanel

Button
Badge
StatusIndicator
```

Keep components small and reusable.

---

# 33. MVP Priority

The first version should prioritize:

### Tier 1 — Must Have

- responsive application shell
- homepage
- curriculum
- lesson pages
- code examples
- practice questions
- progress tracking
- projects
- mobile layout

### Tier 2 — Nice to Have

- XP
- streaks
- achievements
- subtle animations
- theme details
- code editor

### Tier 3 — Later

- accounts
- cloud progress
- subscriptions
- social features
- leaderboards
- AI tutor
- advanced Rust code execution
- certificates

Do not let Tier 3 features delay the MVP.

---

# 34. Product Feeling

When someone opens BORROW_, the intended reaction should be:

> "This feels different."

Then:

> "This is clean."

Then:

> "I know exactly what I'm supposed to do."

And finally:

> "Let me try the next Rust challenge."

The design should create curiosity without distracting from learning.

---

# 35. Final Design Summary

**BORROW_ = modern learning UX + retro-futuristic visual identity.**

### Visual keywords

```text
RETRO
FUTURE
COMPUTING
EDUCATION
INDUSTRIAL
PUNK
MINIMAL
WARM
TECHNICAL
PLAYFUL
CLEAN
```

### Primary colors

```text
Warm Charcoal    #0B0A08
Surface          #14120F
Elevated         #1C1915
Border           #34302A
Burnt Orange     #FF6B35
Retro Yellow     #F2C14E
Phosphor Green   #39FFB6
Electric Cyan    #5CC8FF
Retro Violet     #A66CFF
Warm White       #F4EFE6
Muted Text       #A9A197
```

### The single most important rule

> **BORROW_ should look like a modern, beautifully designed learning application that happens to have been influenced by the visual language of retro computing — not like a terminal emulator.**
