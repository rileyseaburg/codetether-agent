//! Upstream adapter aliases must produce the same model-visible identity.
use super::resolve;

#[test]
fn routed_identity_matches_bedrock_dispatch_resolution() {
    for model in [
        "claude-sonnet-4.6",
        "claude-opus-4.5",
        "nova-pro",
        "us.anthropic.claude-sonnet-4-6",
        "custom-model-id",
    ] {
        assert_eq!(
            resolve("bedrock", model),
            crate::provider::bedrock::BedrockProvider::resolve_model_id(model)
        );
    }
    assert_ne!(resolve("bedrock", "nova-pro"), "nova-pro");
}

#[test]
fn routed_identity_strips_codex_request_options() {
    assert_eq!(resolve("openai-codex", "gpt-5.4:high"), "gpt-5.4");
    assert_eq!(resolve("openai-codex", "gpt-5.4-fast:high"), "gpt-5.4");
    assert_eq!(resolve("openai-codex", "gpt-5.4-fast"), "gpt-5.4");
    assert_eq!(resolve("openai-codex", "gpt-5.4"), "gpt-5.4");
    assert_eq!(
        resolve("openai-codex", "custom-model:unknown"),
        "custom-model:unknown"
    );
}
