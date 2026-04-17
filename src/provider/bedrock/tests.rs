//! Unit tests for the Bedrock provider module.

use super::{BedrockProvider, CompletionRequest};

#[test]
fn resolve_opus_46_alias_includes_profile_suffix() {
    assert_eq!(
        BedrockProvider::resolve_model_id("claude-opus-4.6"),
        "us.anthropic.claude-opus-4-6-v1:0"
    );
    assert_eq!(
        BedrockProvider::resolve_model_id("claude-opus-4-6"),
        "us.anthropic.claude-opus-4-6-v1:0"
    );
}

#[test]
fn resolve_model_id_passes_through_full_id() {
    let model_id = "us.anthropic.claude-opus-4-6-v1:0";
    assert_eq!(BedrockProvider::resolve_model_id(model_id), model_id);
}

#[test]
fn resolve_opus_47_aliases() {
    assert_eq!(
        BedrockProvider::resolve_model_id("claude-opus-4.7"),
        "us.anthropic.claude-opus-4-7-v1:0"
    );
    assert_eq!(
        BedrockProvider::resolve_model_id("claude-opus-4-7"),
        "us.anthropic.claude-opus-4-7-v1:0"
    );
    assert_eq!(
        BedrockProvider::resolve_model_id("claude-4.7-opus"),
        "us.anthropic.claude-opus-4-7-v1:0"
    );
    assert_eq!(
        BedrockProvider::resolve_model_id("us.anthropic.claude-opus-4-7"),
        "us.anthropic.claude-opus-4-7-v1:0"
    );
    let full_id = "us.anthropic.claude-opus-4-7-v1:0";
    assert_eq!(BedrockProvider::resolve_model_id(full_id), full_id);
}

#[test]
fn opus_47_request_omits_temperature() {
    let provider = BedrockProvider::new("test-key".into()).unwrap();
    let model_id = BedrockProvider::resolve_model_id("claude-opus-4-7");
    let request = CompletionRequest {
        model: "claude-opus-4-7".to_string(),
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
        config.get("temperature").is_none(),
        "temperature should be absent for Opus 4.7 but was {:?}",
        config.get("temperature")
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
