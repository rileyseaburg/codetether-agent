//! Unit tests for the Bedrock provider module.

use super::{BedrockProvider, CompletionRequest, parse_converse_response};

#[test]
fn resolve_speculative_claude_4_aliases_fall_back_to_stable_sonnet_4_profile() {
    for alias in [
        "claude-opus-4.7",
        "claude-opus-4-7",
        "claude-4.7-opus",
        "us.anthropic.claude-opus-4-7",
        "claude-opus-4.6",
        "claude-opus-4-6",
        "claude-4.6-opus",
        "us.anthropic.claude-opus-4-6",
        "us.anthropic.claude-opus-4-6-v1",
        "us.anthropic.claude-opus-4-6-v1:0",
        "claude-opus-4",
        "claude-4-opus",
        "us.anthropic.claude-opus-4",
        "claude-sonnet-4.6",
        "claude-sonnet-4-6",
        "us.anthropic.claude-sonnet-4-6",
        "us.anthropic.claude-sonnet-4-6-v1",
        "us.anthropic.claude-sonnet-4-6-v1:0",
        "claude-sonnet-4",
        "claude-4-sonnet",
        "us.anthropic.claude-sonnet-4",
        "claude-haiku-4.5",
        "us.anthropic.claude-haiku-4-5",
    ] {
        assert_eq!(
            BedrockProvider::resolve_model_id(alias),
            "us.anthropic.claude-sonnet-4-20250514-v1:0",
            "alias {alias} should not resolve to a speculative Bedrock model ID"
        );
    }
}

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

#[test]
fn sigv4_canonicalizes_model_suffix_in_path() {
    let url = "https://bedrock-runtime.us-east-1.amazonaws.com/model/amazon.nova-lite-v1:0/converse-stream";
    let canonical = super::sigv4::canonicalize_url(url).unwrap();
    assert_eq!(
        canonical.canonical_uri,
        "/model/amazon.nova-lite-v1%3A0/converse-stream"
    );
}

#[test]
fn sigv4_does_not_double_encode_model_suffix() {
    let url =
        "https://bedrock-runtime.us-east-1.amazonaws.com/model/amazon.nova-lite-v1%3A0/converse";
    let canonical = super::sigv4::canonicalize_url(url).unwrap();
    assert_eq!(
        canonical.canonical_uri,
        "/model/amazon.nova-lite-v1%3A0/converse"
    );
}

#[test]
fn sigv4_sorts_and_encodes_query_parameters() {
    let url = "https://bedrock.us-east-1.amazonaws.com/inference-profiles?typeEquals=SYSTEM_DEFINED&maxResults=200";
    let canonical = super::sigv4::canonicalize_url(url).unwrap();
    assert_eq!(
        canonical.canonical_querystring,
        "maxResults=200&typeEquals=SYSTEM_DEFINED"
    );
}

#[test]
fn parse_converse_response_handles_multibyte_error_prefix_without_panicking() {
    let prefix = "a".repeat(299);
    let body = format!("{prefix}\u{2014}{{");

    let err = parse_converse_response(&body).unwrap_err();
    let message = err.to_string();

    assert!(message.contains("Failed to parse Bedrock response:"));
    assert!(!message.contains("byte index 300 is not a char boundary"));
}
