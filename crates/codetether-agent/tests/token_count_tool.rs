//! Runtime test for the `token_count` tool.

use codetether_agent::tool::Tool;
use codetether_agent::tool::token_count::TokenCountTool;
use serde_json::json;

#[tokio::test]
async fn token_count_reports_tokens_and_missing_files() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("a.txt");
    std::fs::write(&file, "hello world\nsecond line\n".repeat(100)).unwrap();
    let path = file.to_string_lossy().to_string();
    let result = TokenCountTool
        .execute(json!({"paths": [path, "/nonexistent/zz"]}))
        .await
        .unwrap();
    assert!(result.success);
    assert!(
        result.output.contains("tokens, 2400 bytes, 200 lines"),
        "{}",
        result.output
    );
    assert!(result.output.contains("/nonexistent/zz: error"));
}
