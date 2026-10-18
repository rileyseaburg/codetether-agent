//! Status text for normal retries and temporary overload fallbacks.
use crate::PendingSmartSwitchRetry;
use std::time::Instant;

/// Describe a normal retry or temporary overload fallback.
/// # Arguments
/// * `pending` - Pending retry and optional restore metadata.
/// * `count` - Number of retries attempted.
/// # Returns
/// Status text with the selected model and remaining cooldown where applicable.
/// # Examples
/// ```no_run
/// # fn example(pending: &codetether_model_failover::PendingSmartSwitchRetry) {
/// let _ = codetether_model_failover::temporary_fallback::status(pending, 1);
/// # }
/// ```
pub fn status(pending: &PendingSmartSwitchRetry, count: u32) -> String {
    let Some(restore) = pending.restore_model.as_ref() else {
        return format!(
            "Smart switch retry {}/{} → {}",
            count,
            crate::smart_switch_max_retries(),
            pending.target_model
        );
    };
    let seconds = pending
        .restore_at
        .map(|at| at.saturating_duration_since(Instant::now()).as_secs())
        .unwrap_or_default();
    format!(
        "Codex overloaded; retrying with {} in {}s; restore {} afterward…",
        pending.target_model, seconds, restore
    )
}
