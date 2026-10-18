//! Native system-prompt construction and shared verifier task text.
use super::charter;
use crate::session::tasks::GoalStatus;
pub use codetether_goal_verification::user_prompt;
use std::path::Path;

/// Build the second agent's system prompt: repository context plus the
/// charter for the claimed transition (see [`charter`]).
///
/// # Arguments
///
/// * `workspace` — Directory the second agent inspects.
/// * `model` — Model identifier it runs on.
/// * `claimed` — The transition being challenged; `blocked` selects
///   controlled opposition.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::tasks::GoalStatus;
/// use codetether_agent::tool::goal::verify::system_prompt;
/// use std::path::Path;
///
/// let audit = system_prompt(Path::new("/repo"), "openai/gpt-5", GoalStatus::Complete);
/// assert!(audit.contains("VERDICT: PASS") && audit.contains("/repo"));
/// let opposition = system_prompt(Path::new("/repo"), "openai/gpt-5", GoalStatus::Blocked);
/// assert!(opposition.contains("controlled opposition"));
/// ```
pub fn system_prompt(workspace: &Path, model: &str, claimed: GoalStatus) -> String {
    let charter = charter(claimed);
    let specialty = match claimed {
        GoalStatus::Blocked => "Controlled opposition to a blocked claim",
        _ => "Independent completion verifier",
    };
    let base = crate::tool::swarm_execute::agent_prompt::build(
        "goal-verifier",
        Some(specialty),
        workspace,
        model,
        charter,
        false,
        false,
    );
    let runtime_rule = base
        .lines()
        .find(|line| line.contains("[runtime "))
        .unwrap_or("none");
    tracing::info!(rule = %runtime_rule, "Goal verifier runtime constraint");
    format!("{base}\n\n{charter}")
}
