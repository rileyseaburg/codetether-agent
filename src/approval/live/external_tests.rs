use crate::approval::test_env::{ScopedEnv, lock_env};
use crate::approval::{ApprovalStore, LiveApprovalDecision, LiveApprovalRequest};

#[tokio::test]
async fn durable_store_decision_resumes_waiter_without_live_delivery() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), crate::config::AccessMode::Ask);
    let store = ApprovalStore::open_default().expect("store");
    let request = store
        .create_request("bash", "execute", "bash:external", "external")
        .expect("request");
    let live = live_request(&request.id);
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    let waiter = tokio::spawn(async move { super::request(&tx, live).await });
    rx.recv().await.expect("approval event");

    store
        .approve(&request.id, "external", "approved")
        .expect("approve");
    let decision = tokio::time::timeout(std::time::Duration::from_secs(1), waiter)
        .await
        .expect("waiter timeout")
        .expect("waiter task");
    assert_eq!(decision, LiveApprovalDecision::Approved);
    let event = rx.recv().await.expect("decision event");
    assert!(matches!(
        event,
        crate::session::SessionEvent::ToolCallMetadata { .. }
    ));
}

fn live_request(id: &str) -> LiveApprovalRequest {
    LiveApprovalRequest::new(
        id.into(),
        "call".into(),
        "bash".into(),
        "execute".into(),
        "resource".into(),
        "reason".into(),
    )
}
