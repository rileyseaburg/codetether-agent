//! Native base-prompt builder for shared verifier system-prompt assembly.
use crate::session::tasks::GoalStatus;
pub use codetether_goal_verification::user_prompt;
use std::path::Path;
pub fn system_prompt(workspace: &Path, model: &str, claimed: GoalStatus) -> String {
    codetether_goal_verification::system_prompt(
        workspace,
        model,
        claimed,
        crate::tool::swarm_execute::agent_prompt::build,
    )
}
