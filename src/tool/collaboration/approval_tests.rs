//! First-class collaboration claims under its reviewed identity exactly once.

use crate::approval::{ApprovalStore, test_env::{ScopedEnv, lock_env}};
use crate::config::AccessMode;
use crate::tool::{ToolRegistry, network_access};
use serde_json::{Value, json};

#[tokio::test]
async fn send_input_receipt_is_claimed_before_child_lookup() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let workspace = tempfile::tempdir().expect("workspace");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let mut args = json!({
        "target":"missing-child", "message":"hello",
        "__ct_session_id":"parent-session",
        "__ct_parent_workspace":workspace.path(),
    });
    network_access::bind_trusted(&mut args, true);
    approve("send_input", &mut args);
    let registry = ToolRegistry::with_defaults();
    let tool = registry.get("send_input").expect("tool");

    let first = tool.execute(args.clone()).await.expect("first");
    assert!(!first.output.contains("APPROVAL_CLAIM_FAILED"), "{first:?}");
    let replay = tool.execute(args).await.expect("replay");
    assert_eq!(replay.metadata.get("error_code"), Some(&json!("APPROVAL_RECEIPT_REJECTED")));
}

pub(super) fn approve(tool: &str, args: &mut Value) {
    let scope = crate::runtime_policy::invocation_scope::for_tool(tool, args);
    let store = ApprovalStore::open_default().expect("store");
    let request = store.create_request(tool, scope.action, &scope.resource, "test").unwrap();
    store.approve(&request.id, "test", "once").unwrap();
    args["approval_id"] = json!(request.id);
}