//! Prepare a temporary Codex fallback with its restoration deadline.
use crate::{
    PendingSmartSwitchRetry, codex_overload, codex_overload_cooldown, codex_overload_fallback,
};
use std::time::Instant;

/// Add a timed restore request when a Codex overload requires temporary fallback.
/// # Arguments
/// * `pending` - Scheduled retry to enrich.
/// * `error` - Provider error text.
/// * `provider` - Current provider identifier.
/// * `current` - Current full model identifier.
/// * `available_models` - Candidate models available to the caller.
/// # Returns
/// The retry with fallback/restore metadata, or the original retry for other errors.
/// # Examples
/// ```no_run
/// # fn example(pending: codetether_model_failover::PendingSmartSwitchRetry) {
/// let _ = codetether_model_failover::temporary_fallback::prepare(pending, "error", None, None, &[]);
/// # }
/// ```
pub fn prepare(
    mut pending: PendingSmartSwitchRetry,
    error: &str,
    provider: Option<&str>,
    current: Option<&str>,
    available_models: &[String],
) -> PendingSmartSwitchRetry {
    if codex_overload(error) && provider == Some("openai-codex") {
        if let Some(fallback) = codex_overload_fallback(current, available_models) {
            pending.target_model = fallback;
        }
        pending.restore_at = Some(Instant::now() + codex_overload_cooldown(error));
        pending.not_before = None;
        pending.restore_from = Some(pending.target_model.clone());
        pending.restore_model = current.map(ToString::to_string);
    }
    pending
}
