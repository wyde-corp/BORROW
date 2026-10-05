use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProgressEntry {
    pub id: i64,
    pub user_id: String,
    pub module_id: i64,
    pub lesson_id: Option<i64>,
    pub exercise_id: Option<i64>,
    pub completed_at: String,
    pub xp_earned: i64,
}
