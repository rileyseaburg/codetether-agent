use super::{blocked, policy_args};
use crate::approval::{ApprovalStore, test_env::lock_env};

struct EnvGuard;

impl Drop for EnvGuard {
    fn drop(&mut self) {
        unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
    }
}

#[tokio::test]
async fn bridge_policy_block_includes_approval_id() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", data.path()) };
    let _env = EnvGuard;
    let result = blocked("npx", &["github-mcp"], None)
        .await
        .expect("approval required");
    assert!(result.metadata["approval_request_id"].is_string());
}

#[tokio::test]
async fn bridge_policy_accepts_approved_id() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", data.path()) };
    let _env = EnvGuard;
    let request = blocked("npx", &["github-mcp"], None).await.unwrap();
    let id = request.metadata["approval_request_id"].as_str().unwrap();
    ApprovalStore::open(data.path().join("approvals"))
        .unwrap()
        .approve(id, "test", "ok")
        .unwrap();
    assert!(blocked("npx", &["github-mcp"], Some(id)).await.is_none());
}

#[tokio::test]
async fn bridge_policy_accepts_alias_approved_id() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", data.path()) };
    let _env = EnvGuard;
    let args = policy_args("npx", &["github-mcp"], None);
    let request = crate::runtime_policy::evaluate_tool_invocation("mcp_bridge", &args)
        .await
        .unwrap();
    let id = request.metadata["approval_request_id"].as_str().unwrap();
    ApprovalStore::open(data.path().join("approvals"))
        .unwrap()
        .approve(id, "test", "ok")
        .unwrap();
    assert!(blocked("npx", &["github-mcp"], Some(id)).await.is_none());
}
