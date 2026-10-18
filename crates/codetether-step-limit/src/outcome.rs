//! Read whether the current thread still has an active budget.
use super::state::BUDGET_ACTIVE;

/// Returns `true` if the step budget was active (not cleared) after the
/// agentic loop exited, indicating the run hit the `max_steps` ceiling.
/// # Returns
/// True when the current thread's budget was set and has not been cleared.
/// # Examples
/// ```
/// codetether_step_limit::begin(None);
/// assert!(!codetether_step_limit::was_budget_exhausted());
/// ```
pub fn was_budget_exhausted() -> bool {
    BUDGET_ACTIVE.with(|c| c.get())
}
