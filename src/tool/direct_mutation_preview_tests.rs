//! Preview-only confirmation branches consume exact approval once.

use super::support;
use crate::approval::{test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use crate::tool::{Tool, confirm_edit::ConfirmEditTool, confirm_multiedit::ConfirmMultiEditTool};
use serde_json::json;

#[tokio::test]
async fn edit_preview_is_approval_bound() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let root = tempfile::tempdir().expect("root");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let path = root.path().join("file.txt");
    std::fs::write(&path, "old\n").expect("fixture");
    let mut args = json!({"path": path, "old_string": "old", "new_string": "new"});
    let tool = ConfirmEditTool::new();
    let blocked = tool.execute(args.clone()).await.expect("blocked");
    let id = support::id(&blocked);
    support::approve(&id);
    args["approval_id"] = json!(id);
    assert!(tool.execute(args.clone()).await.expect("preview").success);
    assert!(!tool.execute(args).await.expect("replay").success);
}

#[tokio::test]
async fn multiedit_preview_is_approval_bound() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let root = tempfile::tempdir().expect("root");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let path = root.path().join("file.txt");
    std::fs::write(&path, "old\n").expect("fixture");
    let mut args = json!({"edits": [{
        "file": path, "old_string": "old", "new_string": "new"
    }]});
    let tool = ConfirmMultiEditTool::new();
    let blocked = tool.execute(args.clone()).await.expect("blocked");
    let id = support::id(&blocked);
    support::approve(&id);
    args["approval_id"] = json!(id);
    assert!(tool.execute(args.clone()).await.expect("preview").success);
    assert!(!tool.execute(args).await.expect("replay").success);
}