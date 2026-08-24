//! `todo_write` claims exactly once under its reviewed alias identity.

use crate::approval::{ApprovalStore, test_env::{ScopedEnv, lock_env}};
use crate::config::AccessMode;
use crate::tool::{Tool, todo::TodoWriteTool};
use serde_json::json;

#[tokio::test]
async fn todo_alias_approval_executes_once() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let root = tempfile::tempdir().expect("root");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let tool = TodoWriteTool::with_root(root.path().to_path_buf());
    let mut args = json!({"action": "add", "content": "alias once"});
    let blocked = crate::tool::alias::scoped("todo_write", tool.execute(args.clone()))
        .await
        .expect("blocked");
    let id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("approval id")
        .to_string();
    let store = ApprovalStore::open_default().expect("store");
    assert_eq!(store.request(&id).unwrap().unwrap().tool, "todo_write");
    store.approve(&id, "test", "once").expect("approve");
    args["approval_id"] = json!(id);

    let first = crate::tool::alias::scoped("todo_write", tool.execute(args.clone()))
        .await
        .expect("first");
    assert!(first.success, "{first:?}");
    let replay = crate::tool::alias::scoped("todo_write", tool.execute(args))
        .await
        .expect("replay");
    assert!(!replay.success);
}