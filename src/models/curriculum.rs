use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Module {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub order_index: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Lesson {
    pub id: i64,
    pub module_id: i64,
    pub title: String,
    pub content: String,
    pub order_index: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Exercise {
    pub id: i64,
    pub module_id: i64,
    pub lesson_id: Option<i64>,
    pub exercise_type: String,
    pub prompt: String,
    pub code: Option<String>,
    pub code_b: Option<String>,
    pub options: Option<String>,
    pub correct_answer: String,
    pub explanation: String,
    pub xp_reward: i64,
    pub order_index: i64,
}

impl Exercise {
    /// True when there is no single right answer to check against.
    ///
    /// Answering exercises is not in the desktop app yet; until it is, this
    /// reads as unused in that target only.
    #[allow(dead_code)]
    ///
    /// The submission flow, the result view, and the graded practice run all
    /// have to agree on this, so it is defined once here. Previously the test
    /// was written out three times, which is exactly the kind of thing that
    /// drifts the first time a new kind is added.
    pub fn is_open_ended(&self) -> bool {
        is_open_ended_type(&self.exercise_type)
    }
}

/// Free-standing form of [`Exercise::is_open_ended`], for callers that have
/// only a type string (a SQL row, a run item).
#[allow(dead_code)] // see Exercise::is_open_ended
pub fn is_open_ended_type(exercise_type: &str) -> bool {
    matches!(exercise_type, "challenge" | "build_it")
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Project {
    pub id: i64,
    pub module_id: i64,
    pub title: String,
    pub description: String,
    pub requirements: String,
    pub order_index: i64,
}
