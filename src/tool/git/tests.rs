//! Tests for the structured git tool using a real temporary repository.

use super::GitTool;
use super::tests_helpers::init_repo;
use crate::tool::Tool;
use serde_json::json;

#[tokio::test]
async fn status_diff_and_commit_roundtrip() {
    let dir = init_repo().await;
    let cwd = dir.path().to_string_lossy().to_string();
    std::fs::write(dir.path().join("a.txt"), "hello\n").unwrap();

    let tool = GitTool::new();
    let status = tool
        .execute(json!({ "op": "status", "cwd": cwd }))
        .await
        .unwrap();
    assert!(status.success);
    assert!(status.output.contains("a.txt"));

    let commit = tool
        .execute(json!({ "op": "commit", "message": "add a", "cwd": cwd }))
        .await
        .unwrap();
    assert!(commit.success, "commit failed: {}", commit.output);

    let log = tool
        .execute(json!({ "op": "log", "cwd": cwd }))
        .await
        .unwrap();
    assert!(log.output.contains("add a"));
}

#[tokio::test]
async fn unknown_op_and_missing_message_error() {
    let dir = init_repo().await;
    let cwd = dir.path().to_string_lossy().to_string();
    let tool = GitTool::new();

    let bad = tool
        .execute(json!({ "op": "frobnicate", "cwd": cwd }))
        .await
        .unwrap();
    assert!(!bad.success);

    let no_msg = tool
        .execute(json!({ "op": "commit", "cwd": cwd }))
        .await
        .unwrap();
    assert!(!no_msg.success);
}
