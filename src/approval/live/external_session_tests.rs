use crate::approval::test_env::{ScopedEnv, lock_env};
use crate::approval::{ApprovalDecisionKind, ApprovalStore, LiveApprovalDecision};
#[path = "external_session_test_support.rs"]
mod support;

#[tokio::test]
async fn durable_session_decision_activates_in_waiting_process() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), crate::config::AccessMode::Ask);
    crate::approval::session_grants::reset();
    let _reset = support::Reset;
    let store = ApprovalStore::open_default().expect("store");
    let request = store
        .create_request("generic_mutator", "execute", "resource", "external")
        .expect("request");
    crate::approval::session_grants::remember_request(&request.id, Some("session-a"));
    let live = super::LiveApprovalRequest::new(
        request.id.clone(),
        "call".into(),
        "generic_mutator".into(),
        "execute".into(),
        "resource".into(),
        "reason".into(),
    );
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    let waiter = tokio::spawn(async move { super::request(&tx, live).await });
    rx.recv().await.expect("approval event");
    store
        .approve_review(
            &request.id,
            "external",
            "session",
            ApprovalDecisionKind::ApproveForSession,
        )
        .expect("approve for session");

    assert_eq!(
        waiter.await.expect("waiter"),
        LiveApprovalDecision::Approved
    );
    support::assert_session_scope();
}
