//! Tests for approval-gated patch execution.

use super::test_support::{EnvGuard, ORIGINAL, execute, sample_patch, seed};
use crate::approval::test_env::ENV_LOCK;
use serde_json::json;

#[tokio::test]
async fn patch_approval_required_does_not_write() {
    let _lock = ENV_LOCK.lock().expect("env lock");
    let _env = EnvGuard::set("CODETETHER_PATCH_APPROVAL_REQUIRED", "1");
    let data = tempfile::tempdir().expect("tempdir");
    let _data_env = EnvGuard::set("CODETETHER_DATA_DIR", data.path().to_str().unwrap());
    let dir = tempfile::tempdir().expect("tempdir");
    seed(dir.path());

    let result = execute(dir.path(), json!({"patch": sample_patch()})).await;

    let content = std::fs::read_to_string(dir.path().join("file.txt")).unwrap();
    assert!(!result.success);
    assert_eq!(content, ORIGINAL);
    assert_eq!(
        result.metadata["error_code"],
        json!("PATCH_APPROVAL_REQUIRED")
    );
    assert_eq!(result.metadata["patch_files"], json!(["file.txt"]));
    assert_eq!(result.metadata["patch_hunks"], json!(1));
    assert_eq!(result.metadata["approval_required"], json!(true));
    assert!(result.metadata["approval_request_id"].as_str().is_some());
    assert!(
        result.metadata["patch_preview"]
            .as_str()
            .unwrap()
            .contains("+new line")
    );
}
