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
    Breakdown { title: String, code: String, items: Vec<String> },
    /// A ticked list — module recaps and in-lesson requirement lists.
    Checklist { title: String, items: Vec<String> },
    /// A monospace diagram panel — ASCII art, visualisations, syntax breakdowns.
    Diagram { title: String, code: String },
    /// A data table. `rows` is a list of equally-sized cell lists.
    Table { headers: Vec<String>, rows: Vec<Vec<String>> },
    /// An ordered list — used for "read the error like this" walkthroughs.
    Steps { items: Vec<String> },
    /// A list of paragraphs rendered as separate blocks.
    Paragraphs { items: Vec<String> },
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
