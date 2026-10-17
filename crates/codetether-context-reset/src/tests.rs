//! Reset marker and summary validation regression tests.
use codetether_tool_core::Tool;
use serde_json::json;

use super::*;

#[tokio::test]
async fn execute_requires_non_empty_summary() {
    let tool = ContextResetTool;
    for summary in ["", "   "] {
        let result = tool.execute(json!({"summary": summary})).await.unwrap();
        assert!(!result.success);
    }
}

#[tokio::test]
async fn execute_wraps_summary_with_marker() {
    let tool = ContextResetTool;
    let result = tool
        .execute(json!({"summary": "goal: ship it; open: review"}))
        .await
        .unwrap();
    assert!(result.success);
    assert!(result.output.starts_with(RESET_MARKER_PREFIX));
    assert!(result.output.contains("goal: ship it"));
}

#[test]
fn format_reset_marker_has_stable_prefix_and_body() {
    let body = format_reset_marker("hello", 5);
    assert_eq!(RESET_MARKER_PREFIX, "[CONTEXT RESET]");
    assert!(body.starts_with(RESET_MARKER_PREFIX));
    assert!(body.contains("hello"));
    assert!(body.contains("bytes=5"));
    assert!(body.contains("session_recall"));
}
