//! Focused adapter/result contract tests.
use crate::{InvalidTool, Tool};
use serde_json::json;

#[tokio::test]
async fn invalid_tool_limits_suggestions_in_input_order() {
    let names = ["read", "reed", "raid", "road"]
        .map(str::to_string)
        .to_vec();
    let tool = InvalidTool::with_context("fallback".into(), names);
    let result = tool
        .execute(json!({"requested_tool":"readx", "args":{"x":1}}))
        .await
        .unwrap();
    assert!(!result.success);
    assert!(result.output.contains("Tool 'readx' not found."));
    assert!(result.output.contains("Did you mean: read, reed, raid?"));
    assert!(
        result
            .output
            .contains("All available tools: read, reed, raid, road")
    );
    assert!(result.output.contains("\"x\": 1"));
    assert!(result.metadata.is_empty());
}

#[tokio::test]
async fn invalid_tool_uses_unicode_distance_and_context_fallback() {
    let tool = InvalidTool::with_context("cafe".into(), vec!["café".into()]);
    let result = tool.execute(json!({})).await.unwrap();
    assert!(result.output.contains("Tool 'cafe' not found."));
    assert!(result.output.contains("Did you mean: café?"));
    assert!(
        InvalidTool::new()
            .execute(json!({}))
            .await
            .unwrap()
            .output
            .contains("No tool list provided")
    );
}
