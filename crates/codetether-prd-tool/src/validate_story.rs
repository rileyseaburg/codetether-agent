//! validate story for PRD generation and validation.
use super::*;
pub(super) fn validate(
    story: &UserStory,
    story_ids: &HashSet<String>,
    seen_ids: &mut HashSet<String>,
    errors: &mut Vec<String>,
    warnings: &mut Vec<String>,
) {
    if seen_ids.contains(&story.id) {
        errors.push(format!("Duplicate story ID: {}", story.id));
    }
    seen_ids.insert(story.id.clone());

    if story.id.is_empty() {
        errors.push("Story has empty ID".to_string());
    }
    if story.title.is_empty() {
        errors.push(format!("Story {} has empty title", story.id));
    }
    if story.description.is_empty() {
        warnings.push(format!("Story {} has empty description", story.id));
    }
    if story.acceptance_criteria.is_empty() {
        warnings.push(format!("Story {} has no acceptance criteria", story.id));
    }
    if story.verification_steps.is_empty() {
        warnings.push(format!(
            "Story {} has no verification_steps; it may pass on quality checks alone",
            story.id
        ));
    }

    if story.priority == 0 {
        warnings.push(format!("Story {} has priority 0 (should be 1+)", story.id));
    }
    if story.complexity == 0 || story.complexity > 5 {
        warnings.push(format!(
            "Story {} has complexity {} (should be 1-5)",
            story.id, story.complexity
        ));
    }

    validate_dependencies::validate(story, story_ids, errors);
}
