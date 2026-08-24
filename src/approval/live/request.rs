use tokio::sync::{mpsc, oneshot};

use crate::session::SessionEvent;

use super::{LiveApprovalDecision, LiveApprovalRequest, state};

pub async fn request(
    event_tx: &mpsc::Sender<SessionEvent>,
    request: LiveApprovalRequest,
) -> LiveApprovalDecision {
    let (tx, rx) = oneshot::channel();
    let id = request.approval_id.clone();
    let tool_call_id = request.tool_call_id.clone();
    let tool = request.tool.clone();
    state::insert(
        id.clone(),
        state::Pending::new(
            tx,
            event_tx.clone(),
            request.tool_call_id.clone(),
            request.tool.clone(),
        ),
    );
    let _guard = PendingGuard(id.clone());
    if event_tx
        .send(SessionEvent::ApprovalRequest(request))
        .await
        .is_err()
    {
        state::remove(&id);
        return LiveApprovalDecision::denied();
    }
    super::poll::wait(&id, rx, event_tx, &tool_call_id, &tool).await
}

struct PendingGuard(String);

impl Drop for PendingGuard {
    fn drop(&mut self) {
        state::remove(&self.0);
        super::pending::cancel(&self.0);
    }
}
