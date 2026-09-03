//! Tests for deterministic pending-request ordering.

use crate::approval::{LiveApprovalDecision, LiveApprovalRequest};

fn request(id: &str) -> LiveApprovalRequest {
    LiveApprovalRequest::new(
        id.into(),
        format!("call-{id}"),
        "bash".into(),
        "execute".into(),
        "bash:test".into(),
        "test".into(),
    )
}

async fn register(
    tx: tokio::sync::mpsc::Sender<crate::session::SessionEvent>,
    request: LiveApprovalRequest,
) -> tokio::task::JoinHandle<LiveApprovalDecision> {
    let waiter = tokio::spawn(async move { super::request(&tx, request).await });
    tokio::task::yield_now().await;
    waiter
}

#[tokio::test]
async fn latest_falls_back_to_older_pending_request() {
    let _lock = crate::approval::test_env::lock_env();
    let (tx, mut rx) = tokio::sync::mpsc::channel(2);
    let first_id = uuid::Uuid::new_v4().to_string();
    let second_id = uuid::Uuid::new_v4().to_string();
    let first = register(tx.clone(), request(&first_id)).await;
    rx.recv().await.unwrap();
    let second = register(tx, request(&second_id)).await;
    rx.recv().await.unwrap();

    assert_eq!(super::latest_id().as_deref(), Some(&*second_id));
    assert!(super::decide(&second_id, LiveApprovalDecision::Approved));
    assert_eq!(super::latest_id().as_deref(), Some(&*first_id));
    assert_eq!(second.await.unwrap(), LiveApprovalDecision::Approved);
    assert!(super::decide(&first_id, LiveApprovalDecision::denied()));
    assert_eq!(first.await.unwrap(), LiveApprovalDecision::denied());
    assert!(super::latest_id().is_none());
}
