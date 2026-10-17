//! Completion fixture containing inspection and final calls.
use super::provider::text;
pub(super) fn response() -> crate::traits::LlmResponse {
    let mut tools = text("").unwrap();
    tools.tool_calls = vec![
        crate::traits::ToolCall {
            id: "one".into(),
            name: "inspect".into(),
            arguments: serde_json::json!({}),
        },
        crate::traits::ToolCall {
            id: "two".into(),
            name: "FINAL".into(),
            arguments: serde_json::json!({}),
        },
    ];
    tools
}
