//! Smoke tests for the tool-schema retry path.

use crate::{
    provider::{ContentPart, Message, Role, ToolDefinition},
    session::Session,
};
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn worker_retries_context_window_error_with_compact_tool_schema() {
    let provider = super::test_support::provider_arc();
    let provider_dyn: Arc<dyn crate::provider::Provider> = provider.clone();
    let mut session = Session::new().await.expect("session");
    session.add_message(Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: "Run cronjob \"rule:Retry Failed Conversion Forwards\".".to_string(),
        }],
    });
    let tools = vec![ToolDefinition {
        name: "large_tool".to_string(),
        description: "Large tool schema".to_string(),
        parameters: json!({"type": "object", "properties": {"payload": {"type": "string", "description": "x".repeat(20_000)}}}),
    }];
    let response = super::complete_worker_step_with_context_fallback(
        provider_dyn,
        &session,
        "mock-model",
        "system",
        &tools,
        Some(0.7),
    )
    .await
    .expect("compact tool-schema retry should recover");
    let text: String = response
        .message
        .content
        .iter()
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(text, "ok");
    assert_eq!(provider.calls(), 6);
    assert!(provider.saw_compact_schema());
}
