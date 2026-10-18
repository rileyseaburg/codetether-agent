//! Tests for preview-mode patch execution.

use super::test_support::{ORIGINAL, execute, sample_patch, seed};
use serde_json::json;

#[tokio::test]
async fn patch_preview_alias_does_not_write() {
    let dir = tempfile::tempdir().expect("tempdir");
    seed(dir.path());

    let result = execute(
        dir.path(),
        json!({"patch": sample_patch(), "preview": true}),
    )
    .await;

    let content = std::fs::read_to_string(dir.path().join("file.txt")).unwrap();
    assert!(result.success);
    assert_eq!(content, ORIGINAL);
    assert_eq!(result.metadata["patch_files"], json!(["file.txt"]));
    assert_eq!(result.metadata["patch_hunks"], json!(1));
    assert_eq!(result.metadata["approval_required"], json!(false));
    assert!(
        result.metadata["patch_preview"]
            .as_str()
            .unwrap()
            .contains("-old line")
    );
}
