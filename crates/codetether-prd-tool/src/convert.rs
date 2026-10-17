//! convert for PRD generation and validation.
use super::*;
pub(super) fn story(s: StoryInput) -> UserStory {
    UserStory {
        id: s.id,
        title: s.title,
        description: s.description,
        acceptance_criteria: s.acceptance_criteria,
        verification_steps: s.verification_steps,
        passes: false,
        priority: s.priority.unwrap_or(1),
        depends_on: s.depends_on,
        complexity: s.complexity.unwrap_or(3),
    }
}
