//! Tests for the structured git tool using a real temporary repository.

#[path = "tests_validation.rs"]
mod validation;

use super::GitTool;
use super::tests_helpers::init_repo;
use crate::tool::Tool;
use serde_json::json;

#[tokio::test]
#[ignore = "requires enforced OS sandbox; run in the mandatory sandbox CI lane"]
async fn status_diff_and_commit_roundtrip() {
    let dir = init_repo().await;
    let cwd = dir.path().to_string_lossy().to_string();
    std::fs::write(dir.path().join("a.txt"), "hello\n").unwrap();
    super::tests_helpers::require_sandbox!();

    let tool = GitTool::new();
    super::tests_helpers::environment!(dir);
    let status = tool
        .execute(super::tests_helpers::scoped(
            &cwd,
            json!({ "op": "status" }),
        ))
        .await
        .unwrap();
    assert!(status.success);
    assert!(status.output.contains("a.txt"));

    let commit = tool
        .execute(super::tests_helpers::scoped(
            &cwd,
            json!({ "op": "commit", "message": "add a" }),
        ))
        .await
        .unwrap();
    assert!(commit.success, "commit failed: {}", commit.output);

    let log = tool
        .execute(super::tests_helpers::scoped(&cwd, json!({ "op": "log" })))
        .await
        .unwrap();
    assert!(log.output.contains("add a"));
}
