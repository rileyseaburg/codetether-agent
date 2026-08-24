use super::Registry;
use crate::approval::test_env::{ScopedEnv, lock_env};
use crate::config::AccessMode;
use crate::tool::{Tool, bash::BashTool, exec_command::ExecCommandTool};
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn direct_tool_apis_fail_closed_without_approval() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let bash_marker = data.path().join("bash-unapproved");
    let exec_marker = data.path().join("exec-unapproved");

    let bash = BashTool::new()
        .execute(json!({
            "command": "printf denied > bash-unapproved",
            "cwd": data.path(),
        }))
        .await
        .expect("bash result");
    let exec = ExecCommandTool::new(Arc::new(Registry::default()), None)
        .execute(json!({
            "cmd": "printf denied > exec-unapproved",
            "workdir": data.path(),
        }))
        .await
        .expect("exec result");

    assert!(!bash.success);
    assert!(!exec.success);
    assert!(bash.metadata["approval_request_id"].is_string());
    assert!(exec.metadata["approval_request_id"].is_string());
    assert!(!bash_marker.exists());
    assert!(!exec_marker.exists());
}