//! Feed completed swarm work back into provider calibration.
use crate::bucket_for_specialty;
use codetether_context_policy::delegation::{DelegationState, skills::SWARM_DISPATCH};
use std::sync::{Arc, Mutex};

/// Record `(provider, specialty, success)` against the shared sidecar.
///
/// No-op when delegation is disabled. Locking failure is logged and
/// otherwise ignored — losing one update never justifies crashing the
/// swarm loop.
/// # Arguments
/// Receives shared delegation state, provider, specialty and success status.
/// # Returns
/// Updates enabled calibration state; poisoned state is logged and skipped.
/// # Examples
/// ```no_run
/// fn record(state: &std::sync::Arc<std::sync::Mutex<codetether_context_policy::delegation::DelegationState>>) {
///     codetether_swarm_delegation::record_subtask_outcome(state, "provider", "review", true);
/// }
/// ```
pub fn record_subtask_outcome(
    state: &Arc<Mutex<DelegationState>>,
    provider: &str,
    specialty: &str,
    success: bool,
) {
    let bucket = bucket_for_specialty(specialty);
    match state.lock() {
        Ok(mut guard) => {
            if !guard.enabled() {
                return;
            }
            guard.update(provider, SWARM_DISPATCH, bucket, success);
        }
        Err(err) => {
            tracing::warn!(%err, "delegation state mutex poisoned; skipping swarm outcome update");
        }
    }
}
