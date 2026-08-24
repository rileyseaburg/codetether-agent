//! Direct patch backends honor runtime policy and consume approval once.

use super::test_support::{ORIGINAL, execute, sample_patch, seed};
use crate::approval::{ApprovalStore, test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use serde_json::json;

#[tokio::test]
async fn direct_patch_is_blocked_then_executes_once() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let root = tempfile::tempdir().expect("root");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    seed(root.path());
    let mut args = json!({"patch": sample_patch()});
    let blocked = execute(root.path(), args.clone()).await;
    assert!(!blocked.success);
    assert_eq!(std::fs::read_to_string(root.path().join("file.txt")).unwrap(), ORIGINAL);
    let id = blocked.metadata["approval_request_id"].as_str().expect("id");
    ApprovalStore::open_default()
        .expect("store")
        .approve(id, "test", "direct patch")
        .expect("approve");
    args["approval_id"] = json!(id);

    let applied = execute(root.path(), args.clone()).await;
    assert!(applied.success, "{}", applied.output);
    assert!(std::fs::read_to_string(root.path().join("file.txt")).unwrap().contains("new line"));
    seed(root.path());
    let replay = execute(root.path(), args).await;
    assert!(!replay.success);
    assert_eq!(std::fs::read_to_string(root.path().join("file.txt")).unwrap(), ORIGINAL);
}