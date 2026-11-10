//! Regression coverage for the Fable 5.2 Converse request and stream path.
mod frame;
mod heartbeat;
mod history;
mod incremental;
mod server;
mod server_io;
mod tools;
use crate::provider::BedrockProvider;
use codetether_provider_core::{CompletionRequest, ContentPart, Message, Role};

fn request() -> CompletionRequest {
    CompletionRequest {
        model: "us.anthropic.claude-fable-5-2".into(),
        messages: vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: "hi".into() }],
        }],
        tools: vec![],
        temperature: Some(0.7),
        top_p: None,
        max_tokens: Some(10),
        stop: vec![],
    }
}

#[test]
fn fable_stream_request_uses_converse_controls() {
    let provider = BedrockProvider::new("fixture-token".into()).unwrap();
    let request = request();
    let body = provider.build_converse_body(&request, &request.model);
    assert!(body.get("anthropic_version").is_none());
    assert!(body.get("max_tokens").is_none());
    assert_eq!(body["messages"][0]["content"][0]["text"], "hi");
    assert_eq!(body["inferenceConfig"]["maxTokens"], 32_000);
    assert!(body["inferenceConfig"].get("temperature").is_none());
    assert_eq!(
        body["additionalModelRequestFields"]["thinking"]["type"],
        "adaptive"
    );
    assert!(body["additionalModelRequestFields"]["output_config"]["effort"].is_string());
}
