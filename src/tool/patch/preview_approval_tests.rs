//! Preview receipts are mode-scoped and consumed exactly once.

use super::test_support::{ORIGINAL, execute, sample_patch, seed};
use crate::approval::{ApprovalStore, test_env::{ScopedEnv, lock_env}};
use crate::config::AccessMode;
use serde_json::json;

#[tokio::test]
async fn approved_preview_executes_once_without_writing() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let dir = tempfile::tempdir().expect("workspace");
    seed(dir.path());
    let mut args = json!({"patch": sample_patch(), "preview": true});
    let blocked = execute(dir.path(), args.clone()).await;
    let id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("approval id")
        .to_string();
    ApprovalStore::open_default()
        .expect("store")
        .approve(&id, "test", "preview")
        .expect("approve");
    args["approval_id"] = json!(id);

    let first = execute(dir.path(), args.clone()).await;
    assert!(first.success, "{first:?}");
    assert_eq!(
        std::fs::read_to_string(dir.path().join("file.txt")).expect("read"),
        ORIGINAL
    );
    let replay = execute(dir.path(), args).await;
    assert!(!replay.success);
}