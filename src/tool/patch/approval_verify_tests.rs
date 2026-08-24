//! Tests for store-backed patch approval verification.

use super::test_support::{EnvGuard, ORIGINAL, execute, sample_patch, seed};
use crate::approval::{ApprovalStore, test_env::lock_env};
use serde_json::json;

#[tokio::test]
async fn approved_patch_id_allows_write() {
    let _lock = lock_env();
    let _env = EnvGuard::set("CODETETHER_PATCH_APPROVAL_REQUIRED", "1");
    let data = tempfile::tempdir().expect("tempdir");
    let _data_env = EnvGuard::set("CODETETHER_DATA_DIR", data.path().to_str().unwrap());
    let dir = tempfile::tempdir().expect("tempdir");
    seed(dir.path());
    let store = ApprovalStore::open(data.path().join("approvals")).expect("store");
    let resource = super::approval_resource_for_root(dir.path(), sample_patch());
    let request = store
        .create_request("apply_patch", "write", &resource, "patch write")
        .expect("request");
    store.approve(&request.id, "riley", "ok").expect("approve");

    let args = json!({"patch": sample_patch(), "approval_id": request.id});
    let result = execute(dir.path(), args.clone()).await;
    let content = std::fs::read_to_string(dir.path().join("file.txt")).unwrap();

    assert!(result.success, "{}", result.output);
    assert_ne!(content, ORIGINAL);
    assert!(result.metadata["approval_receipt"].is_object());
    std::fs::write(dir.path().join("file.txt"), ORIGINAL).expect("reset");
    let replay = execute(dir.path(), args).await;
    assert!(!replay.success);
    let content = std::fs::read_to_string(dir.path().join("file.txt")).expect("content");
    assert_eq!(content, ORIGINAL);
}