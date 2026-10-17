//! Existing search regression case.
use super::*;

#[tokio::test]
async fn grep_honors_result_limit() {
    let dir = tempfile::tempdir().expect("tempdir");
    tokio::fs::write(dir.path().join("a.txt"), "needle one\nneedle two\n")
        .await
        .expect("write fixture");
    tokio::fs::write(dir.path().join("b.txt"), "needle three\n")
        .await
        .expect("write fixture");

    let result = GrepTool::new()
        .execute(json!({
            "pattern": "needle",
            "path": dir.path().to_string_lossy(),
            "limit": 1
        }))
        .await
        .expect("grep executes");

    assert!(result.success);
    assert_eq!(result.metadata["count"], json!(1));
    assert_eq!(result.metadata["truncated"], json!(true));
    assert!(result.output.contains("needle"));
}
