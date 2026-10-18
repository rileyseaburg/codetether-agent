//! Native specialization of shared owned prompt execution.
use super::{PromptRequest, SessionNotice, active_cancel::ActiveCancel};
use crate::session::SessionEvent;
use std::sync::Arc;
use tokio::sync::{Notify, mpsc};
pub(super) async fn run(
    request: PromptRequest,
    event_tx: mpsc::Sender<SessionEvent>,
    notice_tx: mpsc::Sender<SessionNotice>,
    cancel: Arc<Notify>,
    active: ActiveCancel,
) {
    codetether_session_runtime::execute::run::<super::execute_host::Native, _>(
        request, event_tx, notice_tx, cancel, active,
    )
    .await;
}
