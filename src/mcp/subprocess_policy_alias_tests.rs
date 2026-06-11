use super::{ApprovalStore, ENV_LOCK, EnvGuard, guard, policy_args};

#[tokio::test]
async fn approved_mcp_bridge_alias_allows_subprocess_spawn_gate() {
    let _lock = ENV_LOCK.lock().expect("env lock");
    let data = tempfile::tempdir().expect("tempdir");
    let _env = EnvGuard::data_dir(data.path());
    let store = ApprovalStore::open(data.path().join("approvals")).expect("store");
    let args = policy_args("npx", &["server"], None);
    let blocked = crate::runtime_policy::evaluate_tool_invocation("mcp_bridge", &args)
        .await
        .expect("approval required");
    let request_id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("request id")
        .to_string();
    store.approve(&request_id, "riley", "ok").expect("approve");
    assert!(guard("npx", &["server"], Some(&request_id)).await.is_ok());
}
