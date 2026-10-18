//! Mutable failover state across a prompt's retry lifecycle.
//!
//! RetryState retains candidate history and the pending retry; native runtimes
//! remain responsible for actually running a selected model.
mod attempts;
mod runtime;
mod select;
use crate::PendingSmartSwitchRetry;
/// Pending retry, retry budget usage and models already attempted.
/// # Examples
/// ```
/// let state = codetether_model_failover::RetryState::default();
/// assert!(state.pending.is_none());
/// assert_eq!(state.count, 0);
/// ```
#[derive(Default)]
pub struct RetryState {
    /// Retry or delayed restoration request awaiting execution.
    pub pending: Option<PendingSmartSwitchRetry>,
    /// Number of retries recorded for the current prompt.
    pub count: u32,
    /// Previously attempted model identifiers in insertion order.
    pub attempted_models: Vec<String>,
}
