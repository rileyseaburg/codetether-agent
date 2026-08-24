//! End-to-end execution through a scoped session command grant.

use super::super::{BashTool, Tool};
use crate::approval::{ApprovalStore, test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use serde_json::json;

#[tokio::test]
async fn scoped_session_prefix_grant_executes_inside_reviewed_workspace() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let tool = BashTool::sandboxed();
    let initial = json!({
        "command": "touch initial.txt",
        "cwd": data.path(),
        "prefix_rule": ["touch"],
        "__ct_session_id": "bash-session-grant-test",
    });
    let blocked = tool.execute(initial).await.expect("approval result");
    let id = blocked.metadata["approval_request_id"].as_str().expect("id");
    ApprovalStore::open_default()
        .expect("store")
        .approve(id, "test", "session prefix")
        .expect("approve");
    crate::approval::session_command_grants::grant_for_request(id);

    let result = tool
        .execute(json!({
            "command": "touch session-grant.txt",
            "cwd": data.path(),
            "__ct_session_id": "bash-session-grant-test",
        }))
        .await
        .expect("granted execution");

    assert!(result.success, "{}", result.output);
    assert_eq!(result.metadata["sandboxed"], true);
    assert!(data.path().join("session-grant.txt").is_file());
    let outside = tempfile::tempdir().expect("outside");
    let rejected = tool.execute(json!({
        "command": "touch escaped.txt",
        "cwd": outside.path(),
        "__ct_session_id": "bash-session-grant-test",
    })).await.expect("scope rejection");
    assert!(!rejected.success);
    assert!(rejected.metadata["approval_request_id"].is_string());
    assert!(!outside.path().join("escaped.txt").exists());
}