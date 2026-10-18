//! Opus aliases retain their generation instead of falling back to Sonnet.
use super::BedrockProvider;

#[test]
fn resolve_opus_aliases_to_their_profiles() {
    for (aliases, expected) in [
        (
            &["claude-opus-4.7", "claude-opus-4-7", "claude-4.7-opus"][..],
            "us.anthropic.claude-opus-4-7",
        ),
        (
            &["claude-opus-4.6", "claude-opus-4-6", "claude-4.6-opus"],
            "us.anthropic.claude-opus-4-6-v1",
        ),
        (
            &[
                "claude-opus-4",
                "claude-4-opus",
                "us.anthropic.claude-opus-4",
            ],
            "us.anthropic.claude-opus-4-20250514-v1:0",
        ),
    ] {
        for alias in aliases {
            assert_eq!(
                BedrockProvider::resolve_model_id(alias),
                expected,
                "{alias}"
            );
        }
    }
    assert_eq!(
        BedrockProvider::resolve_model_id("us.anthropic.claude-opus-4-6"),
        "us.anthropic.claude-opus-4-6-v1"
    );
}

#[test]
fn resolve_explicit_opus_ids_without_rewriting_them() {
    // Passthrough is not a claim that every explicit ID is invokable upstream.
    for model in [
        "us.anthropic.claude-opus-4-7",
        "us.anthropic.claude-opus-4-6-v1",
        "us.anthropic.claude-opus-4-6-v1:0",
    ] {
        assert_eq!(BedrockProvider::resolve_model_id(model), model);
    }
}
