//! Direct todo writes require and consume exact approval once.

use super::support;
use crate::approval::{test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use crate::tool::{Tool, todo::TodoWriteTool};
use serde_json::json;

#[tokio::test]
async fn todo_write_is_blocked_then_executes_once() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let root = tempfile::tempdir().expect("root");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let tool = TodoWriteTool::with_root(root.path().to_path_buf());
    let mut args = json!({"action": "add", "content": "exactly once"});
    let first = tool.execute(args.clone()).await.expect("blocked");
    assert!(!first.success);
    assert!(!root.path().join(".codetether-todos.json").exists());

    let id = support::id(&first);
    support::approve(&id);
    args["approval_id"] = json!(id);
    let approved = tool.execute(args.clone()).await.expect("approved");
    assert!(approved.success, "{}", approved.output);
    let path = root.path().join(".codetether-todos.json");
    let saved: Vec<serde_json::Value> =
        serde_json::from_str(&std::fs::read_to_string(&path).expect("todos")).expect("json");
    assert_eq!(saved.len(), 1);

    let replay = tool.execute(args).await.expect("replay");
    assert!(!replay.success);
    let saved: Vec<serde_json::Value> =
        serde_json::from_str(&std::fs::read_to_string(path).expect("todos")).expect("json");
    assert_eq!(saved.len(), 1);
}