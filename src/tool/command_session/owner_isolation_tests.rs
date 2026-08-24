//! Persistent command sessions are isolated by authoritative owner identity.

use super::{Registry, command};
use crate::approval::{ApprovalStore, test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use crate::tool::{Tool, bash_shell, write_stdin::WriteStdinTool};
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn wrong_owner_is_rejected_without_consuming_receipt() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let registry = Arc::new(Registry::default());
    let (program, args) = shell_args("read value");
    let running = command(&program, &args, data.path(), false, &[], None)
        .await
        .expect("command");
    let session_id = registry.insert(running, "owner-a".into()).await.expect("insert");
    let mut invocation = json!({
        "session_id": session_id,
        "chars": "secret\n",
        "__ct_session_id": "owner-b"
    });
    let scope = crate::runtime_policy::invocation_scope::for_tool("write_stdin", &invocation);
    let store = ApprovalStore::open_default().expect("store");
    let request = store
        .create_request("write_stdin", scope.action, &scope.resource, "owner test")
        .expect("request");
    store.approve(&request.id, "test", "allow").expect("approve");
    invocation["approval_id"] = json!(request.id);

    let result = WriteStdinTool::new(registry).execute(invocation).await.expect("result");
    assert!(!result.success);
    assert!(result.output.contains("unknown or completed"));
    assert!(store.verify(&request.id, "write_stdin", scope.action, &scope.resource).is_ok());
}

fn shell_args(script: &str) -> (String, Vec<String>) {
    let shell = bash_shell::resolve();
    let mut args = shell.prefix_args;
    args.push(script.to_string());
    (shell.program, args)
}