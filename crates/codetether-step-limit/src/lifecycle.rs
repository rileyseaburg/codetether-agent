//! Begin and successfully clear a bounded prompt run.
use super::state::BUDGET_ACTIVE;

/// Records whether the next agentic run has an explicit step budget.
/// # Arguments
/// * `max_steps` — An explicit step budget, including zero, or None for unbounded.
/// # Examples
/// ```
/// codetether_step_limit::begin(Some(5));
/// assert!(codetether_step_limit::was_budget_exhausted());
/// codetether_step_limit::clear_budget();
/// ```
pub fn begin(max_steps: Option<usize>) {
    BUDGET_ACTIVE.with(|active| active.set(max_steps.is_some()));
}

/// Clear the step-limit flag because the run completed within budget.
/// # Examples
/// ```
/// codetether_step_limit::begin(Some(5));
/// codetether_step_limit::clear_budget();
/// assert!(!codetether_step_limit::was_budget_exhausted());
/// ```
pub fn clear_budget() {
    BUDGET_ACTIVE.with(|c| c.set(false));
}
