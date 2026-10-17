//! Render the original story_prompt text.
use crate::UserStory;
use std::path::Path;
/// Build a story implementation prompt.
///
/// # Arguments
/// * `story` - Story to implement.
/// * `prd_info` - Project and feature names.
/// * `working_dir` - Workspace displayed in the prompt.
/// # Returns
/// Prompt text with the original workflow and formatting.
/// # Examples
/// ```
/// # fn example(story: &codetether_ralph_core::UserStory) {
/// let prompt = codetether_ralph_core::presentation::build_story_prompt(story, &("project".into(), "feature".into()), std::path::Path::new("."));
/// assert!(prompt.contains(&story.id));
/// # }
/// ```
pub fn build_story_prompt(
    story: &UserStory,
    prd_info: &(String, String),
    working_dir: &Path,
) -> String {
    let wd = working_dir.display();
    format!(
        include_str!("story_prompt.txt"),
        prd_info.0,
        prd_info.1,
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
        story.id,
        story.id
    )
}
