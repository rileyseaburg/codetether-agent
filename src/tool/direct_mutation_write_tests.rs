//! Direct file writes require and consume exact approval once.

use super::support;
use crate::approval::{test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use crate::tool::{Tool, file::{ReadTool, WriteTool}};
use serde_json::json;

#[tokio::test]
async fn write_is_blocked_then_executes_once() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let target = std::env::current_dir()
        .expect("cwd")
        .join("target")
        .join(format!("approval-write-{}.txt", uuid::Uuid::new_v4()));
    let mut args = json!({"path": target, "content": "approved once"});
    let tool = WriteTool::new();
    let first = tool.execute(args.clone()).await.expect("blocked");
    assert!(!first.success);
    assert!(!target.exists());
    let id = support::id(&first);
    support::approve(&id);
    args["approval_id"] = json!(id);

    let result = tool.execute(args.clone()).await.expect("write");
    assert!(result.success, "{}", result.output);
    assert_eq!(std::fs::read_to_string(&target).expect("content"), "approved once");
    let replay = tool.execute(args).await.expect("replay");
    assert!(!replay.success);
    std::fs::remove_file(target).expect("cleanup");
}

#[tokio::test]
async fn read_is_not_misclassified_as_write() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let path = data.path().join("readable.txt");
    std::fs::write(&path, "visible").expect("fixture");
    let result = ReadTool::new().execute(json!({"path": path})).await.expect("read");
    assert!(result.success, "{}", result.output);
}