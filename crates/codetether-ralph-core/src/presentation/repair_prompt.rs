//! Format targeted repair instructions after a failed quality check.
use crate::UserStory;
use std::path::Path;
/// Build a repair prompt retaining the original attempt numbering and scope.
///
/// # Arguments
/// * `qg_attempt` - Zero-based current quality attempt.
/// * `max_quality_retries` - Configured repair retry limit.
/// * `story` - Story being repaired.
/// * `error_output` - Failed verification or quality-check output.
/// * `story_working_dir` - Workspace for the repair agent.
/// # Returns
/// The original quality-gate repair instructions and errors.
/// # Examples
/// ```
/// # fn example(story: &codetether_ralph_core::UserStory) {
/// let prompt = codetether_ralph_core::presentation::build_repair_prompt(0, 2, story, "error", std::path::Path::new("."));
/// assert!(prompt.contains("attempt 1/2"));
/// # }
/// ```
pub fn build_repair_prompt(
    qg_attempt: usize,
    max_quality_retries: usize,
    story: &UserStory,
    error_output: &str,
    story_working_dir: &Path,
) -> String {
    let repair_prompt = format!(
        "# QUALITY GATE REPAIR (attempt {}/{})\n\n\
                                             The previous implementation attempt for story {} had quality gate failures.\n\n\
                                             ## Errors:\n```\n{}\n```\n\n\
                                             ## Instructions:\n\
                                             1. Read the files mentioned in the errors\n\
                                             2. Fix ONLY the compile/lint errors shown above\n\
                                             3. Run `cargo check 2>&1` to verify your fix\n\
                                             4. Do NOT refactor or change unrelated code\n\n\
                                             Working directory: {}",
        qg_attempt + 1,
        max_quality_retries,
        story.id,
        error_output,
        story_working_dir.display()
    );

    repair_prompt
}
