//! Budget and residual-history fit decision for a compression candidate.

/// Test whether the current history satisfies both compression thresholds.
///
/// # Arguments
/// * `message_count` - Retained history length.
/// * `est` - Measured request token cost.
/// * `keep_last` - Current candidate retained-tail size.
/// * `history_trigger` - Configured history threshold; zero disables it.
/// * `safety_budget` - Usable request token budget.
///
/// # Returns
/// True when cost fits and the remaining history is short enough.
///
/// # Examples
/// ```
/// use codetether_context_policy::compression::budget::attempt_fits;
/// assert!(attempt_fits(2, 80, 3, 0, 90));
/// assert!(!attempt_fits(10, 80, 3, 10, 90));
/// ```
pub fn attempt_fits(
    message_count: usize,
    est: usize,
    keep_last: usize,
    history_trigger: usize,
    safety_budget: usize,
) -> bool {
    let still_long =
        history_trigger > 0 && message_count > history_trigger.saturating_sub(keep_last);
    est <= safety_budget && !still_long
}
