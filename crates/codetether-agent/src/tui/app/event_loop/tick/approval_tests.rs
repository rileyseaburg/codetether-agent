//! Tests for externally resolved approval reconciliation.

use crate::approval::{LiveApprovalDecision, LiveApprovalRequest};
use crate::tui::app::state::{App, approval_queue};

struct Guard;
impl Drop for Guard {
    fn drop(&mut self) {
        approval_queue::reset();
    }
}

#[tokio::test]
async fn external_decision_removes_stale_overlay() {
    approval_queue::reset();
    let _guard = Guard;
    let id = uuid::Uuid::new_v4().to_string();
    let request = LiveApprovalRequest::new(
        id.clone(),
        "call".into(),
        "bash".into(),
        "execute".into(),
        "bash:echo".into(),
        "test".into(),
    );
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    let waiter = tokio::spawn(async move { crate::approval::live::request(&tx, request).await });
    let crate::session::SessionEvent::ApprovalRequest(request) = rx.recv().await.unwrap() else {
        panic!("expected approval request")
    };
    approval_queue::push(request);
    let mut app = App::default();
    app.state.approval_waiting = true;
    app.state.approval_preview_scroll = 9;

    assert!(crate::approval::live::decide(
        &id,
        LiveApprovalDecision::Approved
    ));
    assert_eq!(waiter.await.unwrap(), LiveApprovalDecision::Approved);
    assert!(super::reconcile(&mut app));
    assert!(approval_queue::active().is_none());
    assert!(!app.state.approval_waiting);
    assert_eq!(app.state.approval_preview_scroll, 0);
}
