//! Original prose recovery regression case.
use crate::extract_tool_call_from_prose;
use codetether_provider_core::ToolDefinition;

#[test]
fn returns_none_when_bash_tool_unavailable() {
    let tools = vec![ToolDefinition {
        name: "read".to_string(),
        description: "read".to_string(),
        parameters: serde_json::json!({"type": "object"}),
    }];
    assert!(extract_tool_call_from_prose("I'll use bash to run `ls`", &tools).is_none());
}
