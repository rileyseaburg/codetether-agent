//! Preview authority cannot be upgraded into confirmed mutation authority.

use super::support;
use crate::approval::{test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use crate::tool::{Tool, confirm_edit::ConfirmEditTool};
use serde_json::json;

#[tokio::test]
async fn preview_receipt_rejects_confirmed_write() {
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
    });
    let tool = ConfirmEditTool::new();
    let blocked = tool.execute(args.clone()).await.expect("blocked");
    let id = support::id(&blocked);
    support::approve(&id);
    args["approval_id"] = json!(id);
    args["confirm"] = json!(true);

    let rejected = tool.execute(args).await.expect("rejected");
    assert!(!rejected.success);
    assert_eq!(
        std::fs::read_to_string(path).expect("file"),
        "old value\n"
    );
}