//! Deliver shared approval handoffs through the native session event channel.
use tokio::sync::mpsc;

use crate::session::SessionEvent;

use super::{LiveApprovalDecision, LiveApprovalRequest};

pub async fn request(
    event_tx: &mpsc::Sender<SessionEvent>,
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
