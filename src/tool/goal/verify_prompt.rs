//! Prompts sent to the independent goal verifier.

use super::{VerificationRequest, charter};
use crate::session::tasks::GoalStatus;
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
    format!("{base}\n\n{charter}")
}

/// Build the verifier's task: the persisted goal plus the worker's claim.
///
/// # Arguments
///
/// * `request` — The goal requirements and the worker's claim.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::tasks::GoalStatus;
/// use codetether_agent::tool::goal::verify::{VerificationRequest, user_prompt};
///
/// let request = VerificationRequest {
///     objective: "Add export".into(), success_criteria: vec!["tests pass".into()],
///     forbidden: vec![], claimed: GoalStatus::Blocked, evidence: "no creds".into(),
/// };
/// let prompt = user_prompt(&request);
/// assert!(prompt.contains("Claimed transition: blocked"));
/// assert!(prompt.contains("- tests pass"));
/// ```
pub fn user_prompt(request: &VerificationRequest) -> String {
    format!(
        "Claimed transition: {status}\n\n<objective>\n{objective}\n</objective>\n\n\
         Success criteria:\n{criteria}\n\nForbidden actions:\n{forbidden}\n\n\
         <worker_evidence>\n{evidence}\n</worker_evidence>",
        status = request.claimed.as_str(),
        objective = request.objective,
        criteria = bullets(&request.success_criteria),
        forbidden = bullets(&request.forbidden),
        evidence = request.evidence,
    )
}

fn bullets(items: &[String]) -> String {
    if items.is_empty() {
        return "- (none recorded; derive requirements from the objective)".into();
    }
    items
        .iter()
        .map(|item| format!("- {item}"))
        .collect::<Vec<_>>()
        .join("\n")
}
