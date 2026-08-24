//! Environment cleanup for Ctrl+D approval tests.

use crate::tui::app::state::approval_queue;

pub(super) async fn waiter(
    id: String,
) -> (
    tokio::task::JoinHandle<crate::approval::LiveApprovalDecision>,
    tokio::sync::mpsc::Receiver<crate::session::SessionEvent>,
) {
    let live = crate::approval::LiveApprovalRequest::new(
        id,
        "call-1".into(),
        "bash".into(),
        "execute".into(),
        "bash:pwd".into(),
        "test".into(),
    );
    let (event_tx, mut events) = tokio::sync::mpsc::channel(4);
    let waiter = tokio::spawn(async move { crate::approval::live::request(&event_tx, live).await });
    let crate::session::SessionEvent::ApprovalRequest(live) = events.recv().await.unwrap() else {
        panic!("request event")
    };
    approval_queue::push(live);
    (waiter, events)
}

pub(super) struct Guard;

impl Drop for Guard {
    fn drop(&mut self) {
        unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
        approval_queue::reset();
    }
}
