use super::Registry;
use crate::approval::{ApprovalStore, test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use crate::tool::{Tool, exec_command::ExecCommandTool};
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn approval_cannot_cross_implicit_default_workdirs() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let first = data.path().join("first");
    let second = data.path().join("second");
    std::fs::create_dir_all(&first).expect("first cwd");
    std::fs::create_dir_all(&second).expect("second cwd");
    let sessions = Arc::new(Registry::default());
    let reviewed = ExecCommandTool::new(Arc::clone(&sessions), Some(first.clone()));
    let args = json!({
        "cmd": "touch approved-cwd",
        "sandbox_permissions": "require_escalated",
    });
    let blocked = reviewed
        .execute(args.clone())
        .await
        .expect("approval result");
    let request_id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("approval request id");
    ApprovalStore::open_default()
        .expect("store")
        .approve(request_id, "test", "first cwd only")
        .expect("approve");
    let mut approved = args;
    approved["approval_id"] = json!(request_id);

    let changed = ExecCommandTool::new(sessions, Some(second.clone()))
        .execute(approved.clone())
        .await
        .expect("changed cwd result");
    assert!(!changed.success);
    assert!(!second.join("approved-cwd").exists());

    let intended = reviewed
        .execute(approved)
        .await
        .expect("intended cwd result");
    assert!(intended.success, "{}", intended.output);
    assert!(first.join("approved-cwd").exists());
}