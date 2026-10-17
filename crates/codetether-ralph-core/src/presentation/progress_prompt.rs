//! Render the original progress_prompt text.
use crate::{Prd, UserStory};
/// Build a story implementation prompt.
///
/// # Arguments
/// * `story` - Story to implement.
/// * `prd` - Project context.
/// * `progress` - Previously recorded progress text.
/// # Returns
/// Prompt text with the original workflow and formatting.
/// # Examples
/// ```
/// # fn example(story: &codetether_ralph_core::UserStory, prd: &codetether_ralph_core::Prd) {
/// let prompt = codetether_ralph_core::presentation::build_prompt(story, prd, String::new());
/// assert!(prompt.contains("None yet"));
/// # }
/// ```
pub fn build_prompt(story: &UserStory, prd: &Prd, progress: String) -> String {
    format!(
        include_str!("progress_prompt.txt"),
        prd.project,
        prd.feature,
        story.id,
        story.title,
        story.description,
        story
            .acceptance_criteria
            .iter()
            .map(|c| format!("- {}", c))
            .collect::<Vec<_>>()
            .join("\n"),
        super::verification_steps::format_verification_steps(story),
        if progress.is_empty() {
            "None yet".to_string()
        } else {
            progress
        },
        story.id
    )
}
