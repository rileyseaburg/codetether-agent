use crate::provider::ToolDefinition;
use serde_json::json;
#[test]
fn compact_worker_tool_definitions_preserve_tool_identity() {
    let tools = vec![ToolDefinition {
        name: "large_tool".to_string(),
        description: "Large tool schema".to_string(),
        parameters: json!({"type": "object", "properties": {"payload": {"type": "string", "description": "x".repeat(20_000)}}}),
    }];
    let compact = super::compact_worker_tool_definitions(&tools);
    assert_eq!(compact[0].name, tools[0].name);
    assert_eq!(compact[0].description, tools[0].description);
    assert_eq!(
        compact[0].parameters.get("additionalProperties"),
        Some(&serde_json::Value::Bool(true))
    );
    assert!(super::tool_schema_bytes(&compact) < super::tool_schema_bytes(&tools) / 10);
}
