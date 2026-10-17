//! Add previous-stage learnings to a parallel story prompt.
use crate::UserStory;
use std::path::Path;
/// Build a parallel story prompt with accumulated stage learnings.
///
/// # Arguments
/// * `story` - Story to implement.
/// * `prd_info` - Project and feature names.
/// * `working_dir` - Isolated workspace path.
/// * `stage_learnings` - Prior stage observations, in their original order.
/// # Returns
/// Base story prompt followed by the original learning section when nonempty.
/// # Examples
/// ```
/// # fn example(story: &codetether_ralph_core::UserStory) {
/// let prompt = codetether_ralph_core::presentation::build_parallel_prompt(story, &("p".into(), "f".into()), std::path::Path::new("."), &["learned X".into()]);
/// assert!(prompt.contains("- learned X"));
/// # }
/// ```
pub fn build_parallel_prompt(
    story: &UserStory,
    prd_info: &(String, String),
    working_dir: &Path,
    stage_learnings: &[String],
) -> String {
    // Build the prompt with worktree awareness + accumulated learnings
    let mut prompt = super::build_story_prompt(story, prd_info, working_dir);
    if !stage_learnings.is_empty() {
        prompt.push_str("\n## Learnings from Previous Stages:\n");
        for l in stage_learnings {
            prompt.push_str(&format!("- {l}\n"));
        }
    }

    prompt
}
