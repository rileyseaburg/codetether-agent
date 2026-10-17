//! Render declared verification steps for prompt display.
use crate::UserStory;

pub(super) fn format_verification_steps(story: &UserStory) -> String {
    if story.verification_steps.is_empty() {
        "None".to_string()
    } else {
        serde_json::to_string_pretty(&story.verification_steps)
            .unwrap_or_else(|_| "Unable to render verification steps".to_string())
    }
}
