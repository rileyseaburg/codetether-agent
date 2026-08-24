use super::blocked;
use crate::approval::{
    ApprovalStore,
    test_env::{ScopedEnv, lock_env},
};
use crate::config::AccessMode;
use serde_json::{Value, json};

#[path = "mcp_bridge_policy_alias_tests.rs"]
mod alias;

pub(super) fn invocation(approval_id: Option<&str>) -> Value {
    let mut args = json!({"action": "list_tools", "command": "npx github-mcp"});
    if let Some(id) = approval_id {
        args["approval_id"] = json!(id);
    }
    args
}

#[tokio::test]
async fn bridge_policy_block_includes_approval_id() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let result = blocked(&invocation(None), "npx", &["github-mcp"])
        .await
        .expect("approval required");
    assert!(result.metadata["approval_request_id"].is_string());
}

#[tokio::test]
async fn bridge_policy_accepts_approved_id() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let mut invocation = invocation(None);
    let request = blocked(&invocation, "npx", &["github-mcp"]).await.unwrap();
    let id = request.metadata["approval_request_id"].as_str().unwrap();
    ApprovalStore::open(data.path().join("approvals"))
        .unwrap()
        .approve(id, "test", "ok")
        .unwrap();
    invocation["approval_id"] = json!(id);
    assert!(blocked(&invocation, "npx", &["github-mcp"]).await.is_none());
}