//! Sonnet and Haiku aliases retain their respective inference profiles.
use super::BedrockProvider;

#[test]
fn resolve_sonnet_and_haiku_aliases_to_their_profiles() {
    for (aliases, expected) in [
        (
            &[
                "claude-sonnet-4.6",
                "claude-sonnet-4-6",
                "us.anthropic.claude-sonnet-4-6",
            ][..],
            "us.anthropic.claude-sonnet-4-6",
        ),
        (
            &[
                "claude-sonnet-4",
                "claude-4-sonnet",
                "us.anthropic.claude-sonnet-4",
            ],
            "us.anthropic.claude-sonnet-4-20250514-v1:0",
        ),
        (
            &["claude-haiku-4.5", "us.anthropic.claude-haiku-4-5"],
            "us.anthropic.claude-haiku-4-5-20251001-v1:0",
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
}

#[test]
fn resolve_explicit_sonnet_ids_without_rewriting_them() {
    // Unknown explicit IDs pass through; the provider decides availability.
    for model in [
        "us.anthropic.claude-sonnet-4-6-v1",
        "us.anthropic.claude-sonnet-4-6-v1:0",
    ] {
        assert_eq!(BedrockProvider::resolve_model_id(model), model);
    }
}
