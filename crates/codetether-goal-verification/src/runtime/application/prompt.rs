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
    crate::system_prompt(workspace, model, claimed, verifier_base)
}

fn verifier_base(
    id: &str,
    specialty: Option<&str>,
    workspace: &Path,
    model: &str,
    instruction: &str,
    read_only: bool,
    expects_changes: bool,
) -> String {
    codetether_swarm_tool::prompt::application::build(
        id,
        specialty,
        workspace,
        model,
        instruction,
        read_only,
        expects_changes,
        false,
    )
}
