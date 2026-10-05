use serde_json::json;

use crate::models::{Exercise, Lesson, Module, Project};

#[derive(Debug, Clone)]
pub struct SeedModule {
    pub slug: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub lessons: Vec<SeedLesson>,
    pub exercises: Vec<SeedExercise>,
    pub projects: Vec<SeedProject>,
}

#[derive(Debug, Clone)]
pub struct SeedLesson {
    pub title: String,
    pub content: serde_json::Value,
}

/// Exercise formats. Each renders differently in the UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExerciseKind {
    /// Multiple choice with radio options
    Mcq,
    /// Show code, ask what it prints
    PredictOutput,
    /// Show broken code, ask what the compiler complains about
    FixCode,
    /// Show two snippets, ask what's different
    SpotDifference,
    /// Show code, ask for the inferred type of a variable
    TypeDetective,
    /// Show a compiler error, ask what Rust thinks the problem is
    CompilerChallenge,
    /// Requirements only, learner writes the code
    BuildIt,
    /// Guided debugging challenge with hints
    Challenge,
    /// Blanks in the code, learner supplies the missing expression
    FillGap,
    /// Working code to convert into a different construct
    Refactor,
    /// Given a signature, identify the name, parameters, types, and return type
    SignatureReading,
    /// Given a value passed into a function, decide whether the caller keeps it
    PredictOwnership,
    /// Given a snippet, decide whether it compiles and why
    PredictCompile,
    /// Given a snippet, fill in the stack and heap state it produces
    MemoryDiagram,
    /// Given a snippet, identify owner, borrower, borrow type, and when the borrow ends
    TraceBorrow,
    /// Given a function, decide whether it should take ownership, `&T`, or `&mut T`
    ApiDesign,
}

impl ExerciseKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ExerciseKind::Mcq => "mcq",
            ExerciseKind::PredictOutput => "predict_output",
            ExerciseKind::FixCode => "fix_code",
            ExerciseKind::SpotDifference => "spot_difference",
            ExerciseKind::TypeDetective => "type_detective",
            ExerciseKind::CompilerChallenge => "compiler_challenge",
            ExerciseKind::BuildIt => "build_it",
            ExerciseKind::Challenge => "challenge",
            ExerciseKind::FillGap => "fill_gap",
            ExerciseKind::Refactor => "refactor",
            ExerciseKind::SignatureReading => "signature_reading",
            ExerciseKind::PredictOwnership => "predict_ownership",
            ExerciseKind::PredictCompile => "predict_compile",
            ExerciseKind::MemoryDiagram => "memory_diagram",
            ExerciseKind::TraceBorrow => "trace_borrow",
            ExerciseKind::ApiDesign => "api_design",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SeedExercise {
    pub lesson_index: Option<usize>,
    pub kind: ExerciseKind,
    pub prompt: String,
    pub code: Option<String>,
    /// Second snippet, only used by SpotDifference
    pub code_b: Option<String>,
    pub options: Vec<String>,
    pub correct_answer: usize,
    pub explanation: String,
    pub xp_reward: i64,
    /// Progressive hints, revealed one at a time
    pub hints: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

impl Difficulty {
    pub fn as_str(&self) -> &'static str {
        match self {
            Difficulty::Beginner => "Beginner",
            Difficulty::Intermediate => "Intermediate",
            Difficulty::Advanced => "Advanced",
        }
    }

    pub fn slug(&self) -> &'static str {
        match self {
            Difficulty::Beginner => "beginner",
            Difficulty::Intermediate => "intermediate",
            Difficulty::Advanced => "advanced",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SeedProject {
    pub title: String,
    pub description: String,
    pub difficulty: Difficulty,
    pub estimated_time: String,
    pub requirements: Vec<String>,
    pub concepts: Vec<String>,
    pub constraints: Vec<String>,
    pub example_interaction: String,
    pub stretch_goals: Vec<String>,
}

impl SeedLesson {
    pub fn new(title: &str, sections: Vec<serde_json::Value>) -> Self {
        Self {
            title: title.to_string(),
            content: json!({ "sections": sections }),
        }
    }
}

impl SeedExercise {
    /// Open-ended constructor. Options and hints are taken as &str and
    /// converted here, so content files stay readable.
    pub fn new(
        lesson_index: Option<usize>,
        kind: ExerciseKind,
        prompt: &str,
        options: &[&str],
        correct_answer: usize,
        explanation: &str,
    ) -> Self {
        Self {
            lesson_index,
            kind,
            prompt: prompt.to_string(),
            code: None,
            code_b: None,
            options: options.iter().map(|s| s.to_string()).collect(),
            correct_answer,
            explanation: explanation.to_string(),
            xp_reward: 10,
            hints: vec![],
        }
    }

    pub fn code(mut self, code: &str) -> Self {
        self.code = Some(code.to_string());
        self
    }

    pub fn hints(mut self, hints: &[&str]) -> Self {
        self.hints = hints.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn xp(mut self, xp: i64) -> Self {
        self.xp_reward = xp;
        self
    }

    pub fn mcq(
        lesson_index: usize,
        prompt: &str,
        options: &[&str],
        correct: usize,
        explanation: &str,
    ) -> Self {
        Self {
            lesson_index: Some(lesson_index),
            kind: ExerciseKind::Mcq,
            prompt: prompt.to_string(),
            code: None,
            code_b: None,
            options: options.iter().map(|s| s.to_string()).collect(),
            correct_answer: correct,
            explanation: explanation.to_string(),
            xp_reward: 10,
            hints: vec![],
        }
    }

    pub fn with_code(mut self, code: &str) -> Self {
        self.code = Some(code.to_string());
        self
    }

    pub fn with_code_b(mut self, code: &str) -> Self {
        self.code_b = Some(code.to_string());
        self
    }

    pub fn with_hints(mut self, hints: &[&str]) -> Self {
        self.hints = hints.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn kind(mut self, kind: ExerciseKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn detached(mut self) -> Self {
        self.lesson_index = None;
        self
    }
}

impl SeedProject {
    // Nine arguments, all required by the spec for a project. A builder would
    // spread a literal across a dozen lines and make the content harder to
    // scan, which is the opposite of what a content file is for.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        title: &str,
        description: &str,
        difficulty: Difficulty,
        estimated_time: &str,
        requirements: &[&str],
        concepts: &[&str],
        constraints: &[&str],
        example_interaction: &str,
        stretch_goals: &[&str],
    ) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            difficulty,
            estimated_time: estimated_time.to_string(),
            requirements: requirements.iter().map(|s| s.to_string()).collect(),
            concepts: concepts.iter().map(|s| s.to_string()).collect(),
            constraints: constraints.iter().map(|s| s.to_string()).collect(),
            example_interaction: example_interaction.to_string(),
            stretch_goals: stretch_goals.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl SeedModule {
    pub fn new(
        slug: &'static str,
        title: &'static str,
        description: &'static str,
        lessons: Vec<SeedLesson>,
        exercises: Vec<SeedExercise>,
        projects: Vec<SeedProject>,
    ) -> Self {
        Self {
            slug,
            title,
            description,
            lessons,
            exercises,
            projects,
        }
    }

    pub fn to_module(&self, id: i64, order_index: i64) -> Module {
        Module {
            id,
            slug: self.slug.to_string(),
            title: self.title.to_string(),
            description: self.description.to_string(),
            order_index,
        }
    }

    pub fn to_lessons(&self, module_id: i64, id_base: i64) -> Vec<Lesson> {
        self.lessons
            .iter()
            .enumerate()
            .map(|(i, l)| Lesson {
                id: id_base + i as i64 + 1,
                module_id,
                title: l.title.clone(),
                content: serde_json::to_string(&l.content).unwrap_or_default(),
                order_index: i as i64 + 1,
            })
            .collect()
    }

    pub fn to_exercises(&self, module_id: i64, lesson_id_base: i64, id_base: i64) -> Vec<Exercise> {
        self.exercises
            .iter()
            .enumerate()
            .map(|(i, e)| Exercise {
                id: id_base + i as i64 + 1,
                module_id,
                lesson_id: e.lesson_index.map(|idx| lesson_id_base + idx as i64 + 1),
                exercise_type: e.kind.as_str().to_string(),
                prompt: e.prompt.clone(),
                code: e.code.clone(),
                code_b: e.code_b.clone(),
                options: Some(build_exercise_payload(e)),
                correct_answer: e.correct_answer.to_string(),
                explanation: e.explanation.clone(),
                xp_reward: e.xp_reward,
                order_index: i as i64 + 1,
            })
            .collect()
    }

    pub fn to_projects(&self, module_id: i64, id_base: i64) -> Vec<Project> {
        self.projects
            .iter()
            .enumerate()
            .map(|(i, p)| Project {
                id: id_base + i as i64 + 1,
                module_id,
                title: p.title.clone(),
                description: p.description.clone(),
                requirements: serde_json::to_string(&json!({
                    "requirements": p.requirements,
                    "difficulty": p.difficulty.as_str(),
                    "difficulty_slug": p.difficulty.slug(),
                    "estimated_time": p.estimated_time,
                    "concepts": p.concepts,
                    "constraints": p.constraints,
                    "example_interaction": p.example_interaction,
                    "stretch_goals": p.stretch_goals,
                }))
                .unwrap_or_default(),
                order_index: i as i64 + 1,
            })
            .collect()
    }
}

fn build_exercise_payload(e: &SeedExercise) -> String {
    let options: Vec<String> = if e.options.is_empty() {
        Vec::new()
    } else {
        e.options.clone()
    };

    let hints: Vec<String> = if e.hints.is_empty() {
        Vec::new()
    } else {
        e.hints.clone()
    };

    if options.is_empty() && hints.is_empty() {
        return String::from("[]");
    }

    json!({
        "options": options,
        "hints": hints,
    })
    .to_string()
}

pub fn text(value: &str) -> serde_json::Value {
    json!({ "type": "text", "text": value })
}

pub fn callout(title: &str, value: &str) -> serde_json::Value {
    json!({ "type": "callout", "title": title, "text": value })
}

pub fn code(language: &str, value: &str) -> serde_json::Value {
    json!({ "type": "code", "language": language, "code": value })
}

/// Multi-paragraph text block. Each item becomes its own paragraph.
pub fn paragraphs(items: &[&str]) -> serde_json::Value {
    json!({
        "type": "paragraphs",
        "items": items,
    })
}

/// ASCII diagram / visual block rendered in a monospace panel.
pub fn diagram(title: &str, value: &str) -> serde_json::Value {
    json!({ "type": "diagram", "title": title, "code": value })
}

/// Comparison table. `rows` are `[cell, cell]` pairs; `headers` is the header row.
pub fn table(headers: &[&str], rows: &[&[&str]]) -> serde_json::Value {
    let rows: Vec<serde_json::Value> = rows
        .iter()
        .map(|r| json!(r))
        .collect();

    json!({
        "type": "table",
        "headers": headers,
        "rows": rows,
    })
}

/// Numbered list of things to look for, e.g. compiler error anatomy.
pub fn steps(items: &[&str]) -> serde_json::Value {
    json!({ "type": "steps", "items": items })
}

/// A paragraph used verbatim. Blank lines inside it split into separate paragraphs.
pub fn text_block(text: &str) -> serde_json::Value {
    json!({ "type": "text", "text": text })
}

/// A paragraph tagged with a 6-step label, e.g. "CONCEPT — ...".
pub fn labelled(label: &str, body: &str) -> serde_json::Value {
    json!({ "type": "text", "text": format!("{label} — {body}") })
}

/// Marks a code panel as intentionally broken, so the learner knows before running it.
pub fn broken(code: &str) -> serde_json::Value {
    json!({
        "type": "code",
        "language": "rust",
        "code": code,
        "status": "broken",
    })
}

/// The corrected version of a panel previously marked `broken`.
pub fn fixed(code: &str) -> serde_json::Value {
    json!({
        "type": "code",
        "language": "rust",
        "code": code,
        "status": "corrected",
    })
}

/// A broken panel in a language other than Rust, e.g. compiler output.
pub fn broken_text(code: &str) -> serde_json::Value {
    json!({
        "type": "code",
        "language": "text",
        "code": code,
        "status": "broken",
    })
}

/// Source paired with a per-line explanation of what each part does.
pub fn breakdown(title: &str, code: &str, items: &[&str]) -> serde_json::Value {
    json!({
        "type": "breakdown",
        "title": title,
        "code": code,
        "items": items,
    })
}

/// A ticked list of things the learner can now do.
pub fn checklist(title: &str, items: &[&str]) -> serde_json::Value {
    json!({
        "type": "checklist",
        "title": title,
        "items": items,
    })
}

/// A code panel with inline inputs the learner fills in.
///
/// `blanks` are `(id, answer, hint)` in the order the `____` markers appear in
/// `code`. The learner can check each blank on its own.
pub fn gap_fill(intro: &str, code: &str, blanks: &[(&str, &str, &str)]) -> serde_json::Value {
    let blanks: Vec<serde_json::Value> = blanks
        .iter()
        .map(|(id, answer, hint)| {
            json!({ "id": id, "answer": answer, "hint": hint })
        })
        .collect();

    json!({
        "type": "gap_fill",
        "intro": intro,
        "language": "rust",
        "code": code,
        "blanks": blanks,
    })
}

/// A snippet walked through one step at a time.
///
/// `steps` are `(lines, note)`, where `lines` are 1-based line numbers the step
/// highlights.
pub fn step_through(title: &str, code: &str, steps: &[(&[usize], &str)]) -> serde_json::Value {
    let steps: Vec<serde_json::Value> = steps
        .iter()
        .map(|(lines, note)| json!({ "lines": lines, "note": note }))
        .collect();

    json!({
        "type": "step_through",
        "title": title,
        "language": "rust",
        "code": code,
        "steps": steps,
    })
}

/// Predict what a program does, then run the real compiler and compare.
///
/// `correct` is the index of the right option in `options`.
pub fn guess_run(
    prompt: &str,
    code: &str,
    options: &[&str],
    correct: usize,
    explanation: &str,
) -> serde_json::Value {
    json!({
        "type": "guess_run",
        "prompt": prompt,
        "language": "rust",
        "code": code,
        "options": options,
        "correct": correct,
        "explanation": explanation,
    })
}

/// Static versus dynamic dispatch over the same call, advanced in tandem.
///
/// `steps` are `(lines, static_note, dynamic_note)`, where `lines` are 1-based
/// line numbers the step highlights.
pub fn dispatch(
    title: &str,
    code: &str,
    steps: &[(&[usize], &str, &str)],
) -> serde_json::Value {
    let steps: Vec<serde_json::Value> = steps
        .iter()
        .map(|(lines, static_note, dynamic_note)| {
            json!({
                "lines": lines,
                "static_note": static_note,
                "dynamic_note": dynamic_note,
            })
        })
        .collect();

    json!({
        "type": "dispatch",
        "title": title,
        "language": "rust",
        "code": code,
        "steps": steps,
    })
}
