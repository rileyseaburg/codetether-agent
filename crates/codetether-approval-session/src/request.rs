//! Deliver shared approval handoffs through a session event channel.
use tokio::sync::mpsc;

use codetether_session_events::SessionEvent;

use super::{LiveApprovalDecision, LiveApprovalRequest};

/// Deliver an approval event and wait for its decision.
/// # Arguments
/// * `event_tx` — Channel carrying events for a session snapshot type.
/// * `request` — Approval details and decision identifier.
/// # Returns
/// The submitted decision, or denial if delivery or the response channel fails.
/// # Panics
/// Panics if the shared pending-request mutex is poisoned.
/// # Examples
/// ```no_run
/// # async fn example(request: codetether_approval_session::LiveApprovalRequest) {
/// let (tx, _rx) = tokio::sync::mpsc::channel::<codetether_session_events::SessionEvent<()>>(1);
/// let _decision = codetether_approval_session::request(&tx, request).await;
/// # }
/// ```
pub async fn request<S: Send>(
    event_tx: &mpsc::Sender<SessionEvent<S>>,
    request: LiveApprovalRequest,
) -> LiveApprovalDecision {
    codetether_approval_core::interaction::handoff::request(request, |request| async move {
        event_tx
            .send(SessionEvent::ApprovalRequest(request))
            .await
            .is_ok()
    })
    .await
}
