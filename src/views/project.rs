use super::components::segmented_progress;
use super::layout::{escape, page};
use crate::models::Project;

struct Spec {
    requirements: Vec<String>,
    difficulty: String,
    difficulty_slug: String,
    estimated_time: String,
    concepts: Vec<String>,
    constraints: Vec<String>,
    example_interaction: String,
    stretch_goals: Vec<String>,
}

impl Spec {
    fn parse(raw: &str) -> Self {
        let v: serde_json::Value = serde_json::from_str(raw).unwrap_or(serde_json::json!({}));

        let list = |key: &str| -> Vec<String> {
            v.get(key)
                .and_then(|x| x.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|s| s.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default()
        };

        Self {
            requirements: list("requirements"),
            difficulty: v
                .get("difficulty")
                .and_then(|x| x.as_str())
                .unwrap_or("Beginner")
                .to_string(),
            difficulty_slug: v
                .get("difficulty_slug")
                .and_then(|x| x.as_str())
                .unwrap_or("beginner")
                .to_string(),
            estimated_time: v
                .get("estimated_time")
                .and_then(|x| x.as_str())
                .unwrap_or("—")
                .to_string(),
            concepts: list("concepts"),
            constraints: list("constraints"),
            example_interaction: v
                .get("example_interaction")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string(),
            stretch_goals: list("stretch_goals"),
        }
    }
}

pub fn show(project: &Project, module: Option<&crate::models::Module>, unlocked: bool) -> String {
    let spec = Spec::parse(&project.requirements);
    let module_title = module.map(|m| m.title.clone()).unwrap_or_default();

    let meta = format!(
        r##"<div class="project-meta">
    <span class="difficulty {slug}">{difficulty}</span>
    <span class="meta-sep" aria-hidden="true">·</span>
    <span class="time">{time}</span>
</div>"##,
        slug = spec.difficulty_slug,
        difficulty = escape(&spec.difficulty),
        time = escape(&spec.estimated_time)
    );

    let concepts = if spec.concepts.is_empty() {
        String::new()
    } else {
        let chips: String = spec
            .concepts
            .iter()
            .map(|c| format!(r##"<span class="chip">{}</span>"##, escape(c)))
            .collect();
        format!(
            r##"<section class="module-section">
    <h2 class="section-heading">CONCEPTS PRACTICED</h2>
    <div class="chips">{chips}</div>
</section>"##
        )
    };

    let requirements = bullet_section("REQUIREMENTS", &spec.requirements);
    let constraints = bullet_section("CONSTRAINTS", &spec.constraints);
    let stretch = bullet_section("STRETCH GOALS", &spec.stretch_goals);

    let example = if spec.example_interaction.is_empty() {
        String::new()
    } else {
        format!(
            r##"<section class="module-section">
    <h2 class="section-heading">EXAMPLE INTERACTION</h2>
    <pre class="example-output">{example}</pre>
</section>"##,
            example = escape(&spec.example_interaction)
        )
    };

    let status = if unlocked {
        format!(
            r##"<div class="project-unlocked">
    <span class="check" aria-hidden="true">✓</span>
    <span>{module} unlocked — you can start this project.</span>
</div>
<a href="#" class="btn btn-primary">START PROJECT</a>"##,
            module = escape(&module_title)
        )
    } else {
        format!(
            r##"<div class="project-locked">
    <span class="lock-icon" aria-hidden="true">🔒</span>
    <span>Complete the {module} module to unlock this project.</span>
</div>"##,
            module = escape(&module_title)
        )
    };

    let body = format!(
        r##"<header class="module-header">
    <span class="module-label">PROJECT</span>
    <h1>{title}</h1>
    {meta}
    <p class="lesson-subtitle">{description}</p>
</header>

{requirements}
{constraints}
{concepts}
{example}
{stretch}

<section class="module-section">
    <h2 class="section-heading">PROJECT STATUS</h2>
    {progress}
    <span class="progress-text">0%</span>
    {status}
</section>"##,
        title = escape(&project.title),
        meta = meta,
        description = escape(&project.description),
        requirements = requirements,
        constraints = constraints,
        concepts = concepts,
        example = example,
        stretch = stretch,
        progress = segmented_progress(0, 12),
        status = status
    );

    page("projects", &project.title, &body)
}

fn bullet_section(title: &str, items: &[String]) -> String {
    if items.is_empty() {
        return String::new();
    }

    let list: String = items
        .iter()
        .map(|i| {
            let text = escape(i);
            let text = text
                .strip_prefix("✓ ")
                .map(|rest| format!(r##"<span class="check" aria-hidden="true">✓</span>{rest}"##))
                .unwrap_or_else(|| format!(r##"<span class="bullet" aria-hidden="true">▸</span>{text}"##));
            format!("<li>{text}</li>")
        })
        .collect();

    format!(
        r##"<section class="module-section">
    <h2 class="section-heading">{title}</h2>
    <ul class="requirements">{list}</ul>
</section>"##
    )
}
