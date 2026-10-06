//! Live Bedrock check: a tool-less request replaying tool history (the TUI
//! `/ask` shape) must not be rejected with "The toolConfig field must be
//! defined when using toolUse and toolResult content blocks".
//!
//! Skipped unless `BEDROCK_LIVE_TESTS=1`. The provider is resolved through
//! `ProviderRegistry::from_vault()` exactly as the TUI does. Override the
//! model with `BEDROCK_LIVE_MODEL`. Run with:
//!
//! ```text
//! BEDROCK_LIVE_TESTS=1 cargo test --test bedrock_ask_tool_history_live -- --nocapture
//! ```

use codetether_agent::provider::{CompletionRequest, ContentPart, Message, ProviderRegistry, Role};

fn text(role: Role, text: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}

fn ask_shaped_messages() -> Vec<Message> {
    vec![
        text(Role::User, "List the files."),
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: "toolu_live_hist".into(),
                name: "list".into(),
                arguments: r#"{"path":"."}"#.into(),
                thought_signature: None,
            }],
        },
        Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: "toolu_live_hist".into(),
                content: "Cargo.toml\nsrc/".into(),
            }],
        },
        text(
            Role::User,
            "[SIDE QUESTION — no tools]\nName one listed file.",
        ),
    ]
}

#[tokio::test]
async fn ask_with_tool_history_and_no_tools_is_accepted() {
    if std::env::var("BEDROCK_LIVE_TESTS").as_deref() != Ok("1") {
        eprintln!("skipping: BEDROCK_LIVE_TESTS != 1");
        return;
    }
    let registry = ProviderRegistry::from_vault().await.expect("registry");
    let Some(provider) = registry.get("bedrock") else {
        eprintln!("skipping: no bedrock provider configured");
        return;
    };
    let model = std::env::var("BEDROCK_LIVE_MODEL")
        .unwrap_or_else(|_| "us.anthropic.claude-sonnet-4-20250514-v1:0".into());
    let request = CompletionRequest {
        messages: ask_shaped_messages(),
        tools: vec![],
        model: model.clone(),
        temperature: None,
        top_p: None,
        max_tokens: Some(64),
        stop: vec![],
    };
    let response = provider.complete(request).await;
    assert!(response.is_ok(), "{model}: {:?}", response.err());
}
