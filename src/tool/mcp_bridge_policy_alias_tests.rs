use super::{AccessMode, ApprovalStore, ScopedEnv, blocked, invocation, json, lock_env};

#[tokio::test]
async fn outer_gate_and_bridge_claim_alias_approval_once() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    super::super::test_env::require_sandbox!();
    let mut args = invocation(None);
    let request = crate::runtime_policy::evaluate_tool_invocation("mcp_bridge", &args)
        .await
        .expect("approval required");
    let id = request.metadata["approval_request_id"]
        .as_str()
        .expect("approval id");
    ApprovalStore::open(data.path().join("approvals"))
        .expect("store")
        .approve(id, "test", "ok")
        .expect("approve");
    args["approval_id"] = json!(id);

    assert!(
        crate::runtime_policy::evaluate_tool_invocation("mcp_bridge", &args)
            .await
            .is_none()
    );
    assert!(blocked(&args, "npx", &["github-mcp"]).await.is_none());
    assert!(blocked(&args, "npx", &["github-mcp"]).await.is_some());
}