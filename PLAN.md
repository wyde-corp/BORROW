# BORROW_ — Implementation Plan

## 1. Architecture Overview

```
┌─────────────────────────────────────────────────┐
│  GPUI Desktop App (Rust)                        │
│  ┌─────────────┐  ┌──────────────┐              │
│  │ UI Layer    │  │ State Store  │              │
│  │ (components)│  │ (lessons,    │              │
│  │             │  │  progress)   │              │
│  └──────┬──────┘  └──────┬───────┘              │
│         │                │                       │
│         └────────┬───────┘                       │
│                  │ HTTP/JSON                     │
└──────────────────┼──────────────────────────────┘
                   │
┌──────────────────┼──────────────────────────────┐
│  Axum Backend (Rust)                              │
│  ┌───────────────▼────────────┐                  │
│  │  REST API                  │                  │
│  │  /api/lessons              │                  │
│  │  /api/exercises            │                  │
│  │  /api/progress             │                  │
│  │  /api/projects             │                  │
│  └───────────────┬────────────┘                  │
│                  │                               │
│  ┌───────────────▼────────────┐                  │
│  │  SQLx (SQLite)             │                  │
│  │  lessons, exercises,       │                  │
│  │  progress, projects        │                  │
│  └────────────────────────────┘                  │
└──────────────────────────────────────────────────┘
```

**Key decision:** Local-first. SQLite database bundled with the app. Axum runs embedded (localhost) so the GPUI frontend and backend share one binary. No accounts required for MVP.

---

## 2. Data Model (SQLx + SQLite)

```sql
modules
  id, slug, title, description, order_index, status

lessons
  id, module_id, title, content (JSON), order_index

exercises
  id, module_id, lesson_id, type (mcq | code), 
  prompt, code, options (JSON), correct_answer, 
  explanation, xp_reward, order_index

projects
  id, module_id, title, description, 
  requirements (JSON), order_index

progress
  user_id, module_id, lesson_id, exercise_id,
  completed_at, xp_earned, streak_data
```

---

## 3. GPUI Component Hierarchy

Maps directly to the design spec's component system (Section 32):

```
AppShell
├── Navbar (desktop) / MobileNavigation
│   ├── Logo ("BORROW_")
│   ├── NavLinks (Learn, Practice, Projects)
│   └── ProgressIndicator + Profile
│
├── Pages
│   ├── HomePage
│   │   ├── Hero (typographic, geometric accents)
│   │   └── StatsBar (modules / exercises / projects)
│   │
│   ├── DashboardPage
│   │   ├── ContinueLearning (ModuleCard w/ progress)
│   │   └── CurriculumList (ModuleCard[])
│   │
│   ├── LessonPage
│   │   ├── LessonHeader (module label, title)
│   │   ├── LessonSection[] (content blocks)
│   │   ├── CodeBlock[] (syntax highlighted)
│   │   ├── Callout[] (key ideas)
│   │   └── Pagination (prev/next, progress)
│   │
│   ├── ExercisePage
│   │   ├── Question (prompt + code)
│   │   ├── MultipleChoice (options)
│   │   ├── AnswerFeedback (correct/incorrect + explanation)
│   │   └── XPReward
│   │
│   ├── ChallengePage
│   │   ├── InstructionsPanel
│   │   ├── CodeEditor (editable, themed)
│   │   └── RunButton + Output
│   │
│   ├── ProjectPage
│   │   ├── ProjectHeader
│   │   ├── RequirementsList
│   │   ├── ProjectProgress
│   │   └── StartButton
│   │
│   └── ProgressPage
│       ├── OverallProgressBar
│       ├── StatsGrid (modules, exercises, projects, streak, XP)
│       └── AchievementList
│
└── Shared Components
    ├── Button (primary / secondary / ghost)
    ├── Card (bordered, elevated surface)
    ├── ProgressBar (segmented, animated)
    ├── Badge
    ├── StatusIndicator (locked / available / in-progress / complete)
    ├── CompletionPanel (module complete celebration)
    └── CodeBlock (syntax highlight, line numbers, copy)
```

---

## 4. Design System Implementation

### Theme (GPUI style tokens)

```rust
struct Theme {
    // Backgrounds
    background: Color,      // #0B0A08
    surface: Color,         // #14120F
    elevated: Color,        // #1C1915
    border: Color,          // #34302A

    // Accents
    accent: Color,          // #FF6B35 (burnt orange)
    accent_secondary: Color,// #F2C14E (retro yellow)
    success: Color,         // #39FFB6 (phosphor green)
    info: Color,            // #5CC8FF (electric cyan)
    rare: Color,            // #A66CFF (retro violet)

    // Text
    text: Color,            // #F4EFE6
    text_muted: Color,      // #A9A197

    // Shape
    radius_sm: f32,         // 4px
    radius_md: f32,         // 6px
    radius_lg: f32,         // 8px
}
```

### Typography

- **IBM Plex Mono** — body, nav, buttons, code, labels (bundled as asset)
- **VT323** — logo, section titles, badges (sparingly)
- Load via GPUI's font loading; fallback to system monospace

### Decorative Elements (subtle, per spec Section 24)

- Thin grid lines on hero (CSS-like background pattern in GPUI)
- Corner marks on cards (small L-shaped accents)
- Coordinate labels ("RUST FUNDAMENTALS / 05")
- Segmented progress bars (discrete segments, not continuous)
- Optional: very subtle noise texture overlay at ~2% opacity

### Animations (150–250ms, per spec Section 25)

- Progress bar fill transitions
- Button hover (subtle bg shift)
- Page transitions (fade/slide)
- Completion panel (scale-in)
- Respect `prefers-reduced-motion`

---

## 5. Page-by-Page Plan

### 5.1 Homepage
- Left-aligned typographic hero: "LEARN RUST / WITHOUT THE 700-PAGE DETOUR."
- Subtitle: "Short lessons. Real practice. Small projects."
- Primary CTA: [START LEARNING]
- Bottom stats row: 11 MODULES | 40+ EXERCISES | 5 PROJECTS
- Decorative: thin grid background, geometric corner marks, orange rule lines

### 5.2 Dashboard ("Your Rust Journey")
- "Continue Learning" card: current module, animated progress bar, [CONTINUE]
- Curriculum list: numbered modules with status indicators
  - ✓ completed (phosphor green)
  - ● in progress (orange, partial bar)
  - ○ available (accent outline)
  - ○ locked (muted gray)

### 5.3 Lesson Page
- Header: module number label, title, lesson X/Y
- Content sections: readable prose, generous line-height
- Code blocks: dark elevated surface, syntax highlighting, line numbers, copy button
- Callouts: left-accent-bordered "The important idea" boxes
- Footer: [← PREV] [CONTINUE →]

### 5.4 Exercise Page
- Question prompt + code context
- Multiple choice: radio-style options, 44px+ touch targets
- [CHECK ANSWER] → immediate feedback
- Feedback: ✓/✗ icon + text (never color-only), explanation, +XP
- Smooth transition to next exercise

### 5.5 Challenge Page (Tier 2)
- Two-column: instructions left, code editor right
- Editor: monospace, syntax highlight, BORROW_ theme
- [RUN] button → compiles via backend, shows output/errors
- Mobile: stacked layout

### 5.6 Project Page
- Project title, description
- "You'll use:" checklist (skills)
- Segmented progress bar
- [START PROJECT] → unlocks after required modules

### 5.7 Progress Page
- Overall progress bar (segmented)
- Stats grid: modules, exercises, projects, streak, XP
- Achievements (Tier 2)

---

## 6. Backend API Design (Axum)

```
GET  /api/curriculum              → modules + lessons + status
GET  /api/lessons/:id             → lesson content
GET  /api/exercises/:module_id    → exercises for module
POST /api/exercises/:id/submit    → check answer, award XP
GET  /api/projects                → projects + status
POST /api/progress                → update progress
GET  /api/progress                → full progress state
POST /api/challenges/:id/run      → compile + run code (Tier 2)
```

All responses JSON. GPUI app fetches on launch and caches locally.

---

## 7. Curriculum Content (11 Modules)

| # | Module | Exercises | Project |
|---|--------|-----------|---------|
| 01 | Getting Started | 4 | — |
| 02 | Variables & Types | 5 | — |
| 03 | Control Flow | 5 | — |
| 04 | Functions | 4 | — |
| 05 | Ownership | 5 | — |
| 06 | Borrowing | 5 | — |
| 07 | Structs & Enums | 5 | CLI Tool |
| 08 | Collections | 4 | — |
| 09 | Option & Result | 4 | — |
| 10 | Generics & Traits | 4 | CLI Game |
| 11 | Iterators | 4 | Final Project |

Content stored as structured JSON (sections, code blocks, callouts) so the GPUI renderer is generic.

---

## 8. Phased Roadmap

### Phase 1 — Foundation (Week 1)
- [ ] Project scaffold: Cargo workspace (gpui-app, server)
- [ ] Theme system + font loading
- [ ] AppShell + Navbar + routing
- [ ] SQLx setup + migrations
- [ ] Seed curriculum data (module 01 content)

### Phase 2 — Core Learning Loop (Week 2)
- [ ] Homepage
- [ ] Dashboard with curriculum list
- [ ] Lesson page renderer (sections, code blocks, callouts)
- [ ] Exercise page (MCQ + feedback)
- [ ] Progress tracking (local SQLite)

### Phase 3 — Practice & Projects (Week 3)
- [ ] Code editor component (Tier 2)
- [ ] Challenge page with compile/run
- [ ] Project pages
- [ ] Progress page
- [ ] Completion panel + XP

### Phase 4 — Polish (Week 4)
- [ ] Animations (progress, transitions, feedback)
- [ ] Mobile/responsive layout pass
- [ ] Accessibility audit (contrast, keyboard, screen reader)
- [ ] Decorative elements (grid, corner marks, labels)
- [ ] Full curriculum content (all 11 modules)
- [ ] Empty states, error states
- [ ] Reduced motion support

---

## 9. Key Technical Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| UI framework | GPUI | Native Rust, GPU-accelerated, fast |
| Backend | Axum | Async, ergonomic, same language |
| Database | SQLite via SQLx | Local-first, zero config, fast |
| State management | GPUI Entity + Reactivity | Built-in, no extra deps |
| Code editor | GPUI custom (Tier 2) | Full control over theme |
| Syntax highlight | tree-sitter (Rust grammar) | Fast, accurate |
| Code execution | Backend subprocess (Tier 2) | Sandboxed cargo run |
| Fonts | Bundled assets | Consistent rendering |

---

## 10. Risks & Mitigations

| Risk | Mitigation |
|------|-----------|
| GPUI learning curve | Start with simple components, iterate |
| Code execution security | Sandbox: timeout, no network, temp dir |
| Content authoring effort | JSON-based content, write incrementally |
| Mobile layout complexity | Design mobile-first breakpoints early |
| GPUI platform limitations | Fallback: webview for complex views if needed |
