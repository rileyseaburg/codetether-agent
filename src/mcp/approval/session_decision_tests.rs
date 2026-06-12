use crate::approval::{ApprovalStore, session_command_grants, test_env::lock_env};
use serde_json::json;

struct EnvGuard;

impl Drop for EnvGuard {
    fn drop(&mut self) {
        session_command_grants::reset();
        unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
    }
}

#[tokio::test]
async fn mcp_execpolicy_decision_grants_session_prefix() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", data.path()) };
    session_command_grants::reset();
    let _env = EnvGuard;
    let store = ApprovalStore::open_default().expect("store");
    let request = store
        .create_request("bash", "execute", "bash:abc", "needs approval")
        .expect("request");
    session_command_grants::remember_request(&request.id, vec!["cargo test".into()]);

    super::handle(
        "approvals/decision",
        Some(json!({"approval_id": request.id, "decision": "approved_execpolicy_amendment"})),
    )
    .await
    .expect("decision");

    assert!(session_command_grants::allowed("cargo test --lib mcp"));
}
