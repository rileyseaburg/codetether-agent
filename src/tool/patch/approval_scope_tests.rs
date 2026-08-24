use super::test_support::{EnvGuard, ORIGINAL, execute, sample_patch, seed};
use crate::approval::{ApprovalStore, test_env::lock_env};
use serde_json::json;

#[tokio::test]
async fn approval_for_one_workspace_cannot_write_another() {
    let _lock = lock_env();
    let _required = EnvGuard::set("CODETETHER_PATCH_APPROVAL_REQUIRED", "1");
    let data = tempfile::tempdir().expect("tempdir");
    let _data = EnvGuard::set("CODETETHER_DATA_DIR", data.path().to_str().unwrap());
    let first = tempfile::tempdir().expect("first");
    let second = tempfile::tempdir().expect("second");
    seed(first.path());
    seed(second.path());
    let store = ApprovalStore::open_default().expect("store");
    let resource = super::approval_resource_for_root(first.path(), sample_patch());
    let request = store
        .create_request("apply_patch", "write", &resource, "patch write")
        .expect("request");
    store
        .approve(&request.id, "test", "reviewed")
        .expect("approve");

    let result = execute(
        second.path(),
        json!({"patch": sample_patch(), "approval_id": request.id}),
    )
    .await;

    assert!(!result.success);
    assert_eq!(
        std::fs::read_to_string(second.path().join("file.txt")).unwrap(),
        ORIGINAL
    );
    assert_eq!(result.metadata["error_code"], "APPROVAL_RECEIPT_REJECTED");
}