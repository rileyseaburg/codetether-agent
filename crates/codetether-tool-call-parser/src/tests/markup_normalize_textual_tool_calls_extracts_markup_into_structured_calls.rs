//! Original textual tool-call normalization regression case.
use crate::normalize_textual_tool_calls;
use codetether_provider_core::{
    CompletionResponse, ContentPart, FinishReason, Message, Role, ToolDefinition, Usage,
};
use serde_json::json;

#[test]
fn normalize_textual_tool_calls_extracts_markup_into_structured_calls() {
    let response = CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: "<tool_call>{\"name\":\"lsp\",\"arguments\":{\"action\":\"workspaceSymbol\",\"query\":\"Session\"}}</tool_call>".to_string(),
            }],
        },
        usage: Usage::default(),
        finish_reason: FinishReason::Stop,
    };
    let tools = vec![ToolDefinition {
        name: "lsp".to_string(),
        description: "LSP operations".to_string(),
        parameters: json!({"type":"object"}),
    }];

    let normalized = normalize_textual_tool_calls(response, &tools);
    assert_eq!(normalized.finish_reason, FinishReason::ToolCalls);
    assert!(
        normalized
            .message
            .content
            .iter()
            .any(|p| matches!(p, ContentPart::ToolCall { name, .. } if name == "lsp"))
    );
    assert!(
        normalized
            .message
            .content
            .iter()
            .all(|p| !matches!(p, ContentPart::Text { text } if text.contains("<tool_call>")))
    );
}
