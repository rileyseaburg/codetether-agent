//! Boundary calculation for the worker wrap-up window.

/// Fraction of the budget reserved for finishing up (commit, push, report).
const WRAP_UP_FRACTION: f64 = 0.15;

/// Never reserve fewer steps than this for wrap-up on budgets that allow it.
const WRAP_UP_MIN_STEPS: usize = 8;

/// Returns the step at which the wrap-up nudge should be injected.
///
/// # Examples
///
/// ```rust
/// use codetether_a2a_worker_core::session_protocol::wrap_up_step;
/// assert_eq!(wrap_up_step(200), 170);
/// ```
/// # Arguments
/// * `max_steps` - Model step budget.
/// # Returns
/// The first step reserved for finalizing the task.
pub fn wrap_up_step(max_steps: usize) -> usize {
    let reserve = ((max_steps as f64) * WRAP_UP_FRACTION).ceil() as usize;
    let reserve = reserve
        .max(WRAP_UP_MIN_STEPS)
        .min(max_steps.saturating_sub(1));
    max_steps.saturating_sub(reserve).max(1)
}
