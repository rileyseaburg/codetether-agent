//! Original textual tool-call normalization regression case.
use crate::normalize_textual_tool_calls;
use codetether_provider_core::{
    CompletionResponse, ContentPart, FinishReason, Message, Role, ToolDefinition, Usage,
};
use serde_json::json;

#[test]
fn normalize_textual_tool_calls_ignores_unknown_tools() {
    let response = CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: "<tool_call>{\"name\":\"not_a_tool\",\"arguments\":{}}</tool_call>"
                    .to_string(),
            }],
        },
        usage: Usage::default(),
        finish_reason: FinishReason::Stop,
    };
    let tools = vec![ToolDefinition {
        name: "bash".to_string(),
        description: "shell".to_string(),
        parameters: json!({"type":"object"}),
    }];

    let normalized = normalize_textual_tool_calls(response, &tools);
    assert_eq!(normalized.finish_reason, FinishReason::Stop);
    assert!(
        normalized
            .message
            .content
            .iter()
            .all(|p| !matches!(p, ContentPart::ToolCall { .. }))
    );
}
