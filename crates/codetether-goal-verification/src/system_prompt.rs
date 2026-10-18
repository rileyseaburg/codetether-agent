//! System-prompt assembly around a caller-provided base prompt.
use crate::charter;
use codetether_goal_core::GoalStatus;
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
/// * `build_base` — Existing runtime-aware base-prompt builder.
///
/// # Returns
/// The base prompt and the unchanged verification charter.
///
/// # Examples
///
/// ```no_run
/// fn build(workspace: &std::path::Path, model: &str, claimed: codetether_goal_core::GoalStatus) {
///     let _prompt = codetether_goal_verification::system_prompt(workspace, model, claimed, |_, _, _, _, _, _, _| String::new());
/// }
/// ```
pub fn system_prompt(
    workspace: &Path,
    model: &str,
    claimed: GoalStatus,
    build_base: impl FnOnce(&str, Option<&str>, &Path, &str, &str, bool, bool) -> String,
) -> String {
    let charter = charter(claimed);
    let specialty = match claimed {
        GoalStatus::Blocked => "Controlled opposition to a blocked claim",
        _ => "Independent completion verifier",
    };
    let base = build_base(
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
