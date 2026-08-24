use crate::approval::test_env::{ScopedEnv, lock_env};
use crate::approval::{ApprovalStore, LiveApprovalDecision, LiveApprovalRequest};
use crate::config::AccessMode;
use crate::session::SessionEvent;

#[tokio::test]
async fn unattended_approval_is_durably_denied() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let store = ApprovalStore::open_default().expect("store");
    let request = store
        .create_request("generic_mutator", "execute", "resource", "test")
        .expect("request");
    let live = LiveApprovalRequest {
        approval_id: request.id.clone(),
        tool_call_id: "call".into(),
        tool: "generic_mutator".into(),
        action: "execute".into(),
        resource: "resource".into(),
        reason: "test".into(),
        preview: None,
        proposed_execpolicy_amendment: None,
        available_decisions: Vec::new(),
    };
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    let waiter = tokio::spawn(async move { crate::approval::live::request(&tx, live).await });
    let SessionEvent::ApprovalRequest(received) = rx.recv().await.expect("event") else {
        panic!("approval request expected");
    };
    super::deny_unattended(&received);

    assert!(matches!(
        waiter.await.expect("waiter"),
        LiveApprovalDecision::Denied { .. }
    ));
    assert!(
        store
            .verify(&request.id, "generic_mutator", "execute", "resource")
            .is_err()
    );
}
