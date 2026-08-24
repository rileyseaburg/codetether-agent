use super::test_support::{EnvGuard, ORIGINAL, execute, sample_patch, seed};
use crate::approval::{ApprovalStore, test_env::lock_env};
use serde_json::json;

#[tokio::test]
async fn approval_cannot_authorize_different_content_for_same_file() {
    let _lock = lock_env();
    let _required = EnvGuard::set("CODETETHER_PATCH_APPROVAL_REQUIRED", "1");
    let data = tempfile::tempdir().expect("tempdir");
    let _data = EnvGuard::set("CODETETHER_DATA_DIR", data.path().to_str().unwrap());
    let dir = tempfile::tempdir().expect("tempdir");
    seed(dir.path());
    let store = ApprovalStore::open_default().expect("store");
    let resource = super::approval_resource_for_root(dir.path(), sample_patch());
    let request = store
        .create_request("apply_patch", "write", &resource, "patch write")
        .expect("request");
    store
        .approve(&request.id, "test", "reviewed")
        .expect("approve");
    let changed = sample_patch().replace("+new line", "+different content");

    let result = execute(
        dir.path(),
        json!({"patch": changed, "approval_id": request.id}),
    )
    .await;

    assert!(!result.success);
    assert_eq!(
        std::fs::read_to_string(dir.path().join("file.txt")).unwrap(),
        ORIGINAL
    );
    assert_eq!(result.metadata["error_code"], "APPROVAL_RECEIPT_REJECTED");
}