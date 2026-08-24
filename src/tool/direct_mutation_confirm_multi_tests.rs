//! Direct confirmed multi-edits require one exact approval.

use super::support;
use crate::approval::{test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use crate::tool::{Tool, confirm_multiedit::ConfirmMultiEditTool};
use serde_json::json;

#[tokio::test]
async fn confirmed_multiedit_is_blocked_then_executes_once() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let root = tempfile::tempdir().expect("root");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let first_path = root.path().join("first.txt");
    let second_path = root.path().join("second.txt");
    std::fs::write(&first_path, "old one\n").expect("fixture");
    std::fs::write(&second_path, "old two\n").expect("fixture");
    let mut args = json!({"confirm": true, "edits": [
        {"file": first_path, "old_string": "old one", "new_string": "new one"},
        {"file": second_path, "old_string": "old two", "new_string": "new two"}
    ]});
    let tool = ConfirmMultiEditTool::new();
    let first = tool.execute(args.clone()).await.expect("blocked");
    assert!(!first.success);
    assert_eq!(std::fs::read_to_string(&first_path).expect("first"), "old one\n");

    let id = support::id(&first);
    support::approve(&id);
    args["approval_id"] = json!(id);
    let approved = tool.execute(args.clone()).await.expect("approved");
    assert!(approved.success, "{}", approved.output);
    assert_eq!(std::fs::read_to_string(&second_path).expect("second"), "new two\n");

    std::fs::write(&first_path, "old one\n").expect("reset");
    std::fs::write(&second_path, "old two\n").expect("reset");
    let replay = tool.execute(args).await.expect("replay");
    assert!(!replay.success);
    assert_eq!(std::fs::read_to_string(first_path).expect("first"), "old one\n");
}