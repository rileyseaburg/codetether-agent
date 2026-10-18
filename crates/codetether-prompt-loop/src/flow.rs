//! Control-flow decisions between provider steps.

/// Decision returned after handling one completion response.
/// # Examples
/// ```
/// let flow = codetether_prompt_loop::StepFlow::Finish;
/// assert!(matches!(flow, codetether_prompt_loop::StepFlow::Finish));
/// ```
pub enum StepFlow {
    /// Continue with another provider step.
    Continue,
    /// Continue an active goal with a fresh per-turn step budget.
    ContinueGoal,
    /// Persist the session and return the accumulated answer.
    Finish,
}
