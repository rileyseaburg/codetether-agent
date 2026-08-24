//! Direct confirmed edits require and consume exact approval once.

use super::support;
use crate::approval::{test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use crate::tool::{Tool, confirm_edit::ConfirmEditTool};
use serde_json::json;

#[tokio::test]
async fn confirmed_edit_is_blocked_then_executes_once() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let root = tempfile::tempdir().expect("root");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let path = root.path().join("file.txt");
    std::fs::write(&path, "old value\n").expect("fixture");
    let mut args = json!({
        "path": path.display().to_string(),
        "old_string": "old value",
        "new_string": "new value",
        "confirm": true,
    });
    let tool = ConfirmEditTool::new();
    let first = tool.execute(args.clone()).await.expect("blocked");
    assert!(!first.success);
    assert_eq!(std::fs::read_to_string(&path).expect("file"), "old value\n");

    let id = support::id(&first);
    support::approve(&id);
    args["approval_id"] = json!(id);
    let approved = tool.execute(args.clone()).await.expect("approved");
    assert!(approved.success, "{}", approved.output);
    assert_eq!(std::fs::read_to_string(&path).expect("file"), "new value\n");

    std::fs::write(&path, "old value\n").expect("reset");
    let replay = tool.execute(args).await.expect("replay");
    assert!(!replay.success);
    assert_eq!(std::fs::read_to_string(path).expect("file"), "old value\n");
}