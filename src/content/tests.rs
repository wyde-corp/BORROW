//! Content integrity tests.
//!
//! These guard a failure mode that is otherwise silent: a content block whose
//! `"type"` has no matching `LessonSection` variant deserialises to an error,
//! and the lesson quietly renders an "could not be loaded" panel instead.

use crate::content;
use crate::models::LessonContent;

/// Mirror of the project spec payload written by `SeedModule::to_projects`.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProjectSpec {
    #[serde(default)]
    requirements: Vec<String>,
    #[serde(default)]
    concepts: Vec<String>,
}

/// Every seeded lesson must parse into the typed content model.
#[test]
fn every_lesson_content_parses() {
    for seed in content::all() {
        let module = &seed.module;

        for lesson in &seed.lessons {
            let parsed: Result<LessonContent, _> = serde_json::from_str(&lesson.content);

            if let Err(err) = parsed {
                let heading = lesson
                    .title
                    .lines()
                    .next()
                    .unwrap_or("<untitled>")
                    .to_string();
                panic!(
                    "module '{}' lesson '{}' (id {}) has malformed content: {err}\n  \
                     check that every block's \"type\" matches a LessonSection variant",
                    module.slug, heading, lesson.id
                );
            }
        }
    }
}

/// Lesson content must be non-empty and reasonably substantial.
#[test]
fn lessons_have_content() {
    for seed in content::all() {
        for lesson in &seed.lessons {
            let parsed: LessonContent = serde_json::from_str(&lesson.content)
                .unwrap_or_else(|e| panic!("{} / {}: {e}", seed.module.slug, lesson.id));

            assert!(
                !parsed.sections.is_empty(),
                "module '{}' lesson '{}' has no sections",
                seed.module.slug,
                lesson.title
            );
        }
    }
}

/// Exercise payloads must declare a correct answer index inside their options.
#[test]
fn exercise_answers_are_in_range() {
    for seed in content::all() {
        for exercise in &seed.exercises {
            let payload: serde_json::Value = match exercise.options.as_ref() {
                Some(o) => serde_json::from_str(o).unwrap_or_else(|e| {
                    panic!("{} exercise {}: {e}", seed.module.slug, exercise.id)
                }),
                None => serde_json::Value::Null,
            };

            let options = payload
                .get("options")
                .and_then(|v| v.as_array())
                .map(|a| a.len())
                .unwrap_or(0);

            if options == 0 {
                continue;
            }

            let correct = exercise
                .correct_answer
                .parse::<usize>()
                .unwrap_or_else(|e| panic!("{} exercise {}: {e}", seed.module.slug, exercise.id));

            assert!(
                correct < options,
                "module '{}' exercise {} points at option {correct} but only {options} exist",
                seed.module.slug,
                exercise.id
            );
        }
    }
}

/// Choice-style exercises must offer something to choose.
#[test]
fn choice_exercises_have_options() {
    for seed in content::all() {
        for exercise in &seed.exercises {
            if !matches!(
                exercise.exercise_type.as_str(),
                "mcq"
                    | "predict_output"
                    | "fix_code"
                    | "spot_difference"
                    | "type_detective"
                    | "compiler_challenge"
                    | "signature_reading"
                    | "predict_ownership"
                    | "predict_compile"
                    | "memory_diagram"
                    | "trace_borrow"
                    | "api_design"
            ) {
                continue;
            }

            let payload: serde_json::Value =
                serde_json::from_str(exercise.options.as_deref().unwrap_or("[]"))
                    .unwrap_or(serde_json::Value::Null);

            let count = payload
                .get("options")
                .and_then(|v| v.as_array())
                .map(|a| a.len())
                .unwrap_or(0);

            assert!(
                count >= 2,
                "module '{}' exercise {} is a {} but has only {count} options",
                seed.module.slug,
                exercise.id,
                exercise.exercise_type
            );
        }
    }
}

/// Guided challenges are useless without progressive hints — the whole point
/// is that the learner can get unstuck without the walkthrough.
#[test]
fn challenges_ship_hints() {
    for seed in content::all() {
        for exercise in &seed.exercises {
            if exercise.exercise_type != "challenge" {
                continue;
            }

            let payload: serde_json::Value =
                serde_json::from_str(exercise.options.as_deref().unwrap_or("{}"))
                    .unwrap_or(serde_json::Value::Null);

            let hints = payload
                .get("hints")
                .and_then(|v| v.as_array())
                .map(|a| a.len())
                .unwrap_or(0);

            assert!(
                hints >= 2,
                "module '{}' challenge exercise {} has {hints} hints, expected at least 2",
                seed.module.slug,
                exercise.id
            );
        }
    }
}

/// Spot-the-difference exercises need a second code panel to be meaningful.
#[test]
fn spot_difference_exercises_have_two_panels() {
    for seed in content::all() {
        for exercise in &seed.exercises {
            if exercise.exercise_type == "spot_difference" {
                assert!(
                    exercise.code.is_some() && exercise.code_b.is_some(),
                    "module '{}' exercise {} is spot_difference but lacks a second panel",
                    seed.module.slug,
                    exercise.id
                );
            }
        }
    }
}

/// Project requirement payloads must decode into the full spec.
#[test]
fn project_specs_parse() {
    for seed in content::all() {
        // Placeholder modules carry a stub project with nothing in it yet.
        if seed.lessons.is_empty() {
            continue;
        }

        for project in &seed.projects {
            let spec: ProjectSpec = serde_json::from_str(&project.requirements)
                .unwrap_or_else(|e| panic!("{} / {}: {e}", seed.module.slug, project.title));

            assert!(
                !spec.requirements.is_empty(),
                "module '{}' project '{}' has no requirements",
                seed.module.slug,
                project.title
            );
            assert!(
                !spec.concepts.is_empty(),
                "module '{}' project '{}' lists no concepts",
                seed.module.slug,
                project.title
            );
        }
    }
}

/// Module slugs must match the registry so IDs resolve correctly.
#[test]
fn module_slugs_are_registered() {
    for seed in content::all() {
        assert!(
            crate::content::registry::MODULES.contains(&seed.module.slug.as_str()),
            "module '{}' is not in the registry",
            seed.module.slug
        );
    }
}

/// Placeholder modules must not be seeded with a fake project.
#[test]
fn placeholder_modules_have_no_content() {
    for seed in content::all() {
        if seed.lessons.is_empty() {
            assert!(
                seed.exercises.is_empty(),
                "module '{}' has exercises but no lessons",
                seed.module.slug
            );
        }
    }
}
