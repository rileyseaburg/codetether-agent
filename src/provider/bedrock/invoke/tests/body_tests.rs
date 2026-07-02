//! Tests for request-body construction and model routing.

use crate::provider::bedrock::BedrockProvider;
use crate::provider::bedrock::invoke::body::build_anthropic_messages_body;
use crate::provider::{CompletionRequest, ContentPart, Message, Role, ToolDefinition};

pub(super) fn req(messages: Vec<Message>, tools: Vec<ToolDefinition>) -> CompletionRequest {
    CompletionRequest {
        model: "us.anthropic.claude-fable-5".into(),
        messages,
        tools,
        temperature: None,
        top_p: None,
        max_tokens: Some(16),
        stop: vec![],
    }
}

#[test]
fn fable_uses_invoke_model_adapter() {
    assert!(BedrockProvider::should_use_invoke_model(
        "us.anthropic.claude-fable-5"
    ));
    assert!(!BedrockProvider::should_use_invoke_model(
        "us.anthropic.claude-sonnet-4-20250514-v1:0"
    ));
}

#[test]
fn builds_anthropic_messages_body() {
    let mut r = req(
        vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: "hi".into() }],
        }],
        vec![],
    );
    r.temperature = Some(0.7);
    r.max_tokens = Some(32);
    let body = build_anthropic_messages_body(&r, "us.anthropic.claude-fable-5");
    assert_eq!(body["anthropic_version"], "bedrock-2023-05-31");
    assert_eq!(body["max_tokens"], 32000);
    assert!(body.get("temperature").is_none());
    // Converse-only extensions must NOT leak into the native Messages body.
    assert!(body.get("thinking").is_none());
    assert!(body.get("output_config").is_none());
}
