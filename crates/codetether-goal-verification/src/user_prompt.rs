//! Render persisted requirements and untrusted worker evidence for review.
use crate::VerificationRequest;

/// Build the verifier's task: the persisted goal plus the worker's claim.
///
/// # Arguments
///
/// * `request` — The goal requirements and the worker's claim.
///
/// # Examples
///
/// ```rust
/// use codetether_goal_core::GoalStatus;
/// use codetether_goal_verification::{VerificationRequest, user_prompt};
///
/// let request = VerificationRequest {
///     objective: "Add export".into(), success_criteria: vec!["tests pass".into()],
///     forbidden: vec![], claimed: GoalStatus::Blocked, evidence: "no creds".into(),
/// };
/// let prompt = user_prompt(&request);
/// assert!(prompt.contains("Claimed transition: blocked"));
/// assert!(prompt.contains("- tests pass"));
/// ```
/// # Returns
/// Task text containing the persisted scope and separately marked worker evidence.
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
