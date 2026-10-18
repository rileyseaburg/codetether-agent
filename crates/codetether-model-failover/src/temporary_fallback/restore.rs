//! Build the scheduled request that restores the original model.
use crate::PendingSmartSwitchRetry;

/// Convert a temporary fallback into its delayed model-restoration request.
/// # Arguments
/// * `pending` - The fallback request that was executed.
/// # Returns
/// A restore request if an original model was recorded, otherwise None.
/// # Examples
/// ```no_run
/// # fn example(pending: &codetether_model_failover::PendingSmartSwitchRetry) {
/// let _ = codetether_model_failover::temporary_fallback::after_fallback(pending);
/// # }
/// ```
pub fn after_fallback(pending: &PendingSmartSwitchRetry) -> Option<PendingSmartSwitchRetry> {
    let original = pending.restore_model.as_ref()?.clone();
    Some(PendingSmartSwitchRetry {
        prompt: pending.prompt.clone(),
        target_model: original.clone(),
        not_before: pending.restore_at,
        restore_from: Some(pending.target_model.clone()),
        restore_at: None,
        restore_model: Some(original),
    })
}
