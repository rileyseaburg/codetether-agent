//! Error-triggered retry selection and pending restore protection.
use crate::{RetryState, temporary_fallback};
/// Select and record a retry after a provider error.
/// # Arguments
/// Receives retry state, current model/catalog, optional inflight prompt, status and error.
/// # Returns
/// Preserves a pending restoration, records a selected retry, or clears retry state.
/// # Examples
/// ```
/// let mut state = codetether_model_failover::RetryState::default();
/// let mut status = String::new();
/// codetether_model_failover::schedule::schedule(&mut state, None, &[], None, &mut status, "unrelated error");
/// assert!(state.pending.is_none());
/// ```
pub fn schedule(
    retry: &mut RetryState,
    current_model: Option<&str>,
    models: &[String],
    inflight: Option<&str>,
    status: &mut String,
    err: &str,
) {
    if retry
        .pending
        .as_ref()
        .is_some_and(|pending| pending.restore_from.is_some())
    {
        *status = "Fallback failed; retaining the Codex restore timer".into();
        return;
    }
    let current_provider = current_model.and_then(|m| m.split('/').next());
    let prompt = inflight.map(str::to_owned).unwrap_or_default();
    let pending = retry.select(err, current_model, current_provider, models, &prompt);
    if let Some(pending) = pending {
        let pending =
            temporary_fallback::prepare(pending, err, current_provider, current_model, models);
        retry.count += 1;
        retry.remember_attempt(current_model, &pending.target_model);
        *status = temporary_fallback::status(&pending, retry.count);
        retry.pending = Some(pending);
        return;
    }
    retry.clear();
}
