//! Native entry for shared ordered prompt submission.
#[cfg(test)]
#[path = "answer_review_submit_tests.rs"]
mod tests;
use super::{PromptRequest, SessionNotice, active_cancel::ActiveCancel};
use crate::session::SessionEvent;
use tokio::sync::mpsc;
pub(super) async fn submit(
    request: PromptRequest,
    cancel: &ActiveCancel,
    event_tx: &mpsc::Sender<SessionEvent>,
    notice_tx: &mpsc::Sender<SessionNotice>,
) -> bool {
    let mut host = super::dispatch_host::Native {
        cancel: cancel.clone(),
        event_tx: event_tx.clone(),
        notice_tx: notice_tx.clone(),
    };
    codetether_session_runtime::dispatch::submit(&mut host, request).await
}
