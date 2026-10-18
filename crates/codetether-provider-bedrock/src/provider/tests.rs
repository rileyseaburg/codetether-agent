//! Unit tests for the Bedrock provider module.

use super::BedrockProvider;
use codetether_provider_core::CompletionRequest;

#[path = "opus_alias_resolution_tests.rs"]
mod opus_alias_resolution;
#[path = "sonnet_haiku_alias_resolution_tests.rs"]
mod sonnet_haiku_alias_resolution;

#[test]
fn resolve_model_id_passes_through_full_id() {
    let model_id = "us.anthropic.claude-sonnet-4-20250514-v1:0";
    assert_eq!(BedrockProvider::resolve_model_id(model_id), model_id);
}

#[test]
fn runtime_model_url_encodes_model_id_path_segment() {
    let provider = BedrockProvider::with_region("test-key".into(), "us-east-1".into()).unwrap();
    assert_eq!(
        provider.runtime_model_url(
            "us.anthropic.claude-sonnet-4-20250514-v1:0",
            "converse-stream"
        ),
        "https://bedrock-runtime.us-east-1.amazonaws.com/model/us.anthropic.claude-sonnet-4-20250514-v1%3A0/converse-stream"
    );
}

#[test]
fn non_opus_47_request_includes_temperature() {
    let provider = BedrockProvider::new("test-key".into()).unwrap();
    let model_id = BedrockProvider::resolve_model_id("claude-sonnet-4");
    let request = CompletionRequest {
        model: "claude-sonnet-4".to_string(),
        messages: vec![],
        tools: vec![],
        temperature: Some(0.7),
        top_p: None,
        max_tokens: None,
        stop: vec![],
    };
    let body = provider.build_converse_body(&request, model_id);
    let config = &body["inferenceConfig"];
    assert!(
        config.get("temperature").is_some(),
        "temperature should be present for non-Opus-4.7 models"
    );
}
