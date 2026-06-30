use tokio::sync::{mpsc, oneshot};

use crate::session::SessionEvent;

use super::{LiveApprovalDecision, LiveApprovalRequest, state};

pub async fn request(
    event_tx: &mpsc::Sender<SessionEvent>,
    request: LiveApprovalRequest,
) -> LiveApprovalDecision {
    let (tx, rx) = oneshot::channel();
    let id = request.approval_id.clone();
    state::insert(id.clone(), tx);
    let _guard = PendingGuard(id.clone());
    if event_tx
        .send(SessionEvent::ApprovalRequest(request))
        .await
        .is_err()
    {
        state::remove(&id);
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
        state::remove(&self.0);
    }
}
