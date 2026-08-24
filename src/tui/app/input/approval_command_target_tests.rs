use crate::approval::{LiveApprovalDecision, LiveApprovalRequest};
use crate::tui::app::state::approval_queue;
use tokio::sync::mpsc;

fn request(id: &str) -> LiveApprovalRequest {
    LiveApprovalRequest::new(
        id.into(),
        format!("call-{id}"),
        "bash".into(),
        "execute".into(),
        format!("bash:{id}"),
        "test".into(),
    )
}

#[tokio::test]
async fn implicit_target_matches_visible_oldest_request() {
    let _lock = crate::approval::test_env::lock_env();
    approval_queue::reset();
    let (tx, mut rx) = mpsc::channel(2);
    let first = request("first");
    let second = request("second");
    let first_wait = tokio::spawn({
        let tx = tx.clone();
        let request = first.clone();
        async move { crate::approval::live::request(&tx, request).await }
    });
    rx.recv().await.expect("first event");
    let second_wait = tokio::spawn({
        let request = second.clone();
        async move { crate::approval::live::request(&tx, request).await }
    });
    rx.recv().await.expect("second event");
    approval_queue::push(first);
    approval_queue::push(second);

    assert_eq!(super::dispatch::target_id(None).as_deref(), Some("first"));

    crate::approval::live::decide("first", LiveApprovalDecision::denied());
    crate::approval::live::decide("second", LiveApprovalDecision::denied());
    first_wait.await.expect("first waiter");
    second_wait.await.expect("second waiter");
    approval_queue::reset();
}
