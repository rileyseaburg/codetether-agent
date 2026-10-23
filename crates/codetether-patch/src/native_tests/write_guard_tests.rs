//! Temporary destinations must be refused before any patch writes.
use super::test_support::{ORIGINAL, execute, sample_patch, seed};
use serde_json::json;

#[tokio::test]
async fn temporary_workspace_is_denied_even_with_approval_id() {
    let dir = tempfile::tempdir().unwrap();
    seed(dir.path());
    for extra in [json!({}), json!({"approval_id": "not-a-bypass"})] {
        let mut params = extra;
        params["patch"] = json!(sample_patch());
        let result = execute(dir.path(), params).await;
        assert!(!result.success);
        assert_eq!(result.metadata["error_code"], "TEMP_DIR_WRITE_BLOCKED");
        assert_eq!(
            std::fs::read_to_string(dir.path().join("file.txt")).unwrap(),
            ORIGINAL
        );
    }
}

#[cfg(unix)]
#[tokio::test]
async fn symlinked_temp_destination_prevents_all_writes() {
    let dir = tempfile::tempdir_in(std::env::current_dir().unwrap()).unwrap();
    let temp = tempfile::tempdir().unwrap();
    seed(dir.path());
    std::os::unix::fs::symlink(temp.path(), dir.path().join("linked")).unwrap();
    let patch = format!(
        "{}\n--- /dev/null\n+++ b/linked/new.txt\n@@ -0,0 +1 @@\n+blocked\n",
        sample_patch()
    );
    let result = execute(dir.path(), json!({"patch": patch})).await;
    assert!(!result.success);
    assert_eq!(result.metadata["error_code"], "TEMP_DIR_WRITE_BLOCKED");
    assert_eq!(
        std::fs::read_to_string(dir.path().join("file.txt")).unwrap(),
        ORIGINAL
    );
    assert!(!temp.path().join("new.txt").exists());
}
