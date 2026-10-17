//! Existing search regression case.
use super::*;

#[tokio::test]
async fn grep_skips_oversized_files() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut file = tokio::fs::File::create(dir.path().join("large.txt"))
        .await
        .expect("create fixture");
    file.write_all(&vec![b'x'; (DEFAULT_GREP_MAX_FILE_BYTES + 1) as usize])
        .await
        .expect("write fixture");
    file.flush().await.expect("flush fixture");

    let result = GrepTool::new()
        .execute(json!({
            "pattern": "needle",
            "path": dir.path().to_string_lossy()
        }))
        .await
        .expect("grep executes");

    assert!(result.success);
    assert_eq!(result.metadata["count"], json!(0));
    assert_eq!(result.metadata["skipped_oversize"], json!(1));
    assert!(result.output.contains("No matches found"));
}
