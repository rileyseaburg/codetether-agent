use crate::approval::{
    ApprovalStore, LiveApprovalDecision, LiveApprovalRequest, test_env::lock_env,
};
use serde_json::json;

struct EnvGuard;

impl EnvGuard {
    fn data_dir(path: &std::path::Path) -> Self {
        unsafe { std::env::set_var("CODETETHER_DATA_DIR", path) };
        Self
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
    }
}

#[tokio::test]
async fn mcp_decision_resumes_live_approval_request() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = EnvGuard::data_dir(data.path());
    let store = ApprovalStore::open(data.path().join("approvals")).expect("store");
    let request = store
        .create_request("bash", "execute", "bash:abc", "needs approval")
        .expect("request");
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    let live = LiveApprovalRequest::new(
        request.id.clone(),
        "call-1".into(),
        "bash".into(),
        "execute".into(),
        "bash:abc".into(),
        "needs approval".into(),
    );
    let waiter = tokio::spawn(async move { crate::approval::live::request(&tx, live).await });
    rx.recv().await.expect("live event");
    let response = super::handle(
        "approvals/decision",
        Some(json!({"approval_id": request.id, "decision": "approve"})),
    )
    .await
    .expect("decision");
    assert_eq!(response["status"], "approved");
    assert_eq!(
        waiter.await.expect("waiter"),
        LiveApprovalDecision::Approved
    );
}
