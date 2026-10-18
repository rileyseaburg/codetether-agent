//! Verifier charter and mandatory goal-turn instructions in one system prompt.
pub use crate::user_prompt;
use codetether_goal_core::GoalStatus;
use std::path::Path;

/// Build the verifier system prompt with repository and goal instructions.
/// # Arguments
/// `workspace` and `model` identify the verifier context; `claimed` selects its charter.
/// # Returns
/// The direct-worker prompt followed by the independent-verification charter.
/// # Examples
/// ```no_run
/// let prompt = codetether_goal_verification::runtime::application::prompt::system_prompt(
///     std::path::Path::new("."), "model", codetether_goal_core::GoalStatus::Complete);
/// assert!(!prompt.is_empty());
/// ```
pub fn system_prompt(workspace: &Path, model: &str, claimed: GoalStatus) -> String {
    crate::system_prompt(
        workspace,
        model,
        claimed,
        codetether_swarm_tool::prompt::application::build,
    )
}
