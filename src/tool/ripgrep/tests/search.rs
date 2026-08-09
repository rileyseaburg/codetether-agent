//! End-to-end tests that actually invoke the `rg` binary.

use crate::tool::Tool;
use crate::tool::ripgrep::RipgrepTool;
use serde_json::json;

/// The defect that motivated this tool: `grep` escapes its pattern unless
/// `is_regex` is set, so alternation silently finds nothing. `rg` must not.
#[tokio::test]
async fn alternation_matches_without_an_opt_in_flag() {
    let result = RipgrepTool::new()
        .execute(json!({
            "pattern": "start_session|start_record",
            "paths": ["src/mux"],
            "files_with_matches": true
        }))
        .await
        .expect("rg runs");
    assert!(result.success, "output: {}", result.output);
    assert!(
        result.output.contains("start.rs"),
        "expected src/mux/control/start.rs, got: {}",
        result.output
    );
}

#[tokio::test]
async fn no_matches_is_success_not_error() {
    // Search an isolated directory: searching the workspace would match this
    // test file's own source text.
    let dir = tempfile::tempdir().expect("tempdir");
    tokio::fs::write(dir.path().join("a.txt"), "hello\n")
        .await
        .expect("seed file");
    let result = RipgrepTool::with_root(dir.path().to_path_buf())
        .execute(json!({"pattern": "definitely-absent-token"}))
        .await
        .expect("rg runs");
    assert!(result.success, "output: {}", result.output);
    assert_eq!(result.metadata.get("matches"), Some(&json!(0)));
    assert!(result.output.contains("No matches found"));
}

#[tokio::test]
async fn empty_pattern_is_rejected() {
    let result = RipgrepTool::new()
        .execute(json!({"pattern": ""}))
        .await
        .expect("returns a result");
    assert!(!result.success);
    assert!(result.output.contains("INVALID_ARGUMENT"));
}
