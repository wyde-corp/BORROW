-- Graded practice runs.
--
-- Deliberately separate from `progress`. That table means "this exercise was
-- completed correctly" -- `is_exercise_completed` is a bare EXISTS over
-- exercise_id -- so writing a wrong answer into it would mark the exercise
-- complete and silently award credit. Practice runs are formative: they record
-- every attempt, including wrong ones, and never touch `progress` or XP.

CREATE TABLE IF NOT EXISTS practice_runs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id TEXT NOT NULL DEFAULT 'local',
    module_id INTEGER NOT NULL REFERENCES modules(id),
    label TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    finished_at TEXT
);

-- The chosen questions, in the order they will be asked. `position` is 1-based
-- and is what the URL carries, so a run is navigable without recomputing an
-- offset every request.
CREATE TABLE IF NOT EXISTS practice_run_items (
    run_id INTEGER NOT NULL REFERENCES practice_runs(id) ON DELETE CASCADE,
    position INTEGER NOT NULL,
    exercise_id INTEGER NOT NULL REFERENCES exercises(id),
    PRIMARY KEY (run_id, position)
);

-- Latest answer per question per run. The composite key makes re-answering an
-- upsert, so a learner can jump back and fix a mistake without accumulating
-- duplicate rows. `given` is kept so the results screen can show what was
-- actually chosen next to the right answer.
CREATE TABLE IF NOT EXISTS practice_answers (
    run_id INTEGER NOT NULL REFERENCES practice_runs(id) ON DELETE CASCADE,
    exercise_id INTEGER NOT NULL REFERENCES exercises(id),
    correct INTEGER NOT NULL,
    given TEXT,
    answered_at TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (run_id, exercise_id)
);

CREATE INDEX IF NOT EXISTS idx_practice_runs_user ON practice_runs(user_id, module_id);
CREATE INDEX IF NOT EXISTS idx_practice_run_items_exercise ON practice_run_items(exercise_id);
CREATE INDEX IF NOT EXISTS idx_practice_answers_exercise ON practice_answers(exercise_id);
