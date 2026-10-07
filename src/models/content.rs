use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonContent {
    pub sections: Vec<LessonSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LessonSection {
    /// Prose. Blank lines separate paragraphs.
    Text { text: String },
    /// A visually distinct aside: idea, practice, challenge, or project.
    Callout { title: String, text: String },
    /// A syntax-coloured code panel. `status` flags deliberately invalid
    /// examples, and the corrected version that follows them.
    Code {
        language: String,
        code: String,
        #[serde(default)]
        status: CodeStatus,
    },
    /// Source paired with a per-line explanation of what each part does.
    Breakdown {
        title: String,
        code: String,
        items: Vec<String>,
    },
    /// A ticked list — module recaps and in-lesson requirement lists.
    Checklist { title: String, items: Vec<String> },
    /// A monospace diagram panel — ASCII art, visualisations, syntax breakdowns.
    Diagram { title: String, code: String },
    /// A data table. `rows` is a list of equally-sized cell lists.
    Table {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    },
    /// An ordered list — used for "read the error like this" walkthroughs.
    Steps { items: Vec<String> },
    /// A list of paragraphs rendered as separate blocks.
    Paragraphs { items: Vec<String> },
    /// A code panel with inline inputs the learner fills in. Each blank is
    /// checked on its own, so a partially-complete answer still gives feedback.
    GapFill {
        intro: String,
        #[serde(default = "default_language")]
        language: String,
        code: String,
        blanks: Vec<GapBlank>,
        #[serde(default)]
        after: String,
    },
    /// A snippet walked through one step at a time. Each step highlights the
    /// lines it is talking about, so the learner drives the explanation instead
    /// of reading a finished one.
    StepThrough {
        title: String,
        #[serde(default = "default_language")]
        language: String,
        code: String,
        steps: Vec<Step>,
    },
    /// Predict what a program does, then run the real compiler and compare.
    GuessRun {
        prompt: String,
        #[serde(default = "default_language")]
        language: String,
        code: String,
        options: Vec<String>,
        correct: usize,
        explanation: String,
    },
    /// Static versus dynamic dispatch over the same call, advanced in tandem.
    Dispatch {
        title: String,
        #[serde(default = "default_language")]
        language: String,
        code: String,
        steps: Vec<DispatchStep>,
    },
    /// A pipeline shown first as lazy adapters, then driven by a consumer.
    Pipeline {
        title: String,
        #[serde(default = "default_language")]
        language: String,
        code: String,
        steps: Vec<PipelineStep>,
    },
}

/// One fillable blank in a [`LessonSection::GapFill`].
///
/// The answer travels to the browser in a data attribute rather than being
/// graded by the server: per-blank instant feedback would otherwise need a
/// round trip per keystroke, and this is a local learning tool, not an exam.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GapBlank {
    pub id: String,
    pub answer: String,
    #[serde(default)]
    pub hint: String,
}

/// One step in a [`LessonSection::StepThrough`]. `lines` are 1-based line
/// numbers the step highlights.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    pub lines: Vec<usize>,
    pub note: String,
}

/// One step in a [`LessonSection::Dispatch`]. `lines` are 1-based line numbers
/// the step highlights, and the two notes narrate the same step under static
/// and dynamic dispatch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispatchStep {
    pub lines: Vec<usize>,
    pub static_note: String,
    pub dynamic_note: String,
}

/// One step in a [`LessonSection::Pipeline`]. `phase` is `"lazy"` or `"eager"`,
/// which drives the badge: the adapters describe work first and only run once a
/// consumer pulls.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStep {
    pub lines: Vec<usize>,
    pub phase: String,
    pub note: String,
}

fn default_language() -> String {
    "rust".to_string()
}

/// Whether a code panel is valid, deliberately broken, or a correction.
///
/// Teaching a compiler error means showing the error. Rendering it the same way
/// as working code makes learners think BORROW is broken rather than the
/// example, so broken and corrected panels carry their own banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum CodeStatus {
    /// Ordinary, working code.
    #[default]
    Valid,
    /// Deliberately does not compile.
    Broken,
    /// The fixed version of a panel marked broken.
    Corrected,
}
