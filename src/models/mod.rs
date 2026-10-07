mod content;
mod curriculum;

// These are the crate's public names for the content types, not imports that
// happen to go unused — both binaries address them through here. The lint reads
// `pub use` as unused in whichever target only matches on them structurally
// (the desktop app matches `LessonSection` variants rather than naming the
// per-step types), so it is allowed here rather than at each call site.
#[allow(unused_imports)]
pub use content::{
    CodeStatus, DispatchStep, GapBlank, LessonContent, LessonSection, PipelineStep, Step,
};
pub use curriculum::{Exercise, Lesson, Module, Project};
