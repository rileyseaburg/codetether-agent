//! Pending approval registration, delivery, decision waiting, and cleanup.
//!
//! [request] owns the lifecycle in the shared pending registry. The application
//! supplies only request delivery; dropping the future removes its pending entry.
use super::{LiveApprovalDecision, LiveApprovalRequest, pending};
use std::future::Future;
use tokio::sync::oneshot;

/// Deliver an approval request and wait for its decision.
///
/// # Arguments
/// * `request` — Approval details and the identifier used to deliver a decision.
/// * `deliver` — Sends the request to a client and reports whether delivery succeeded.
/// # Returns
/// The client's decision, or denial if delivery or the decision channel fails.
/// Cancellation removes the pending entry through a drop guard.
/// # Panics
/// Panics if the process-wide pending-request mutex is poisoned.
/// # Examples
/// ```no_run
/// # async fn example(request: codetether_approval_core::interaction::LiveApprovalRequest) {
/// let decision = codetether_approval_core::interaction::handoff::request(
///     request, |_| async { false },
/// ).await;
/// assert_eq!(decision, codetether_approval_core::interaction::LiveApprovalDecision::denied());
/// # }
/// ```
pub async fn request<F, D>(request: LiveApprovalRequest, deliver: F) -> LiveApprovalDecision
where
    F: FnOnce(LiveApprovalRequest) -> D,
    D: Future<Output = bool>,
{
    let (tx, rx) = oneshot::channel();
    let id = request.approval_id.clone();
    pending::insert(id.clone(), tx);
    let _guard = PendingGuard(id.clone());
    if !deliver(request).await {
        pending::remove(&id);
        return LiveApprovalDecision::denied();
    }
    match rx.await {
        Ok(decision) => decision,
        Err(_) => LiveApprovalDecision::denied(),
    }
}

struct PendingGuard(String);
impl Drop for PendingGuard {
    fn drop(&mut self) {
        pending::remove(&self.0);
    }
}

#[cfg(test)]
#[path = "handoff_tests.rs"]
mod tests;
