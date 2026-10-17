//! Check references and self-dependencies for one PRD story.
use super::*;
pub(super) fn validate(story: &UserStory, story_ids: &HashSet<String>, errors: &mut Vec<String>) {
    for dep in &story.depends_on {
        if !story_ids.contains(dep) {
            errors.push(format!(
                "Story {} depends on non-existent story: {}",
                story.id, dep
            ));
        }
        if dep == &story.id {
            errors.push(format!("Story {} depends on itself", story.id));
        }
    }
}
