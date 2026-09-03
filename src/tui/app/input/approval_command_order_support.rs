//! Live-waiter fixtures for approval ordering tests.

use crate::approval::{LiveApprovalDecision, LiveApprovalRequest};
use crate::session::SessionEvent;
use crate::tui::app::state::approval_queue;

pub(super) fn request(id: &str, call: &str) -> LiveApprovalRequest {
    LiveApprovalRequest::new(
        id.into(),
        call.into(),
        "bash".into(),
        "execute".into(),
        format!("bash:{call}"),
        "runtime policy".into(),
    )
}

pub(super) async fn queue(
    tx: &tokio::sync::mpsc::Sender<SessionEvent>,
    rx: &mut tokio::sync::mpsc::Receiver<SessionEvent>,
    item: LiveApprovalRequest,
) -> tokio::task::JoinHandle<LiveApprovalDecision> {
    approval_queue::push(item.clone());
    let tx = tx.clone();
    let waiter = tokio::spawn(async move { crate::approval::live::request(&tx, item).await });
    rx.recv().await.unwrap();
    waiter
}
