//! Tests for [`crate::session::helper::temperature::temperature_is_deprecated`].

use super::temperature_is_deprecated;

#[test]
fn detects_opus_47_aliases() {
    assert!(temperature_is_deprecated("claude-opus-4-7"));
    assert!(temperature_is_deprecated("claude-opus-4.7"));
    assert!(temperature_is_deprecated("claude-4.7-opus"));
    assert!(temperature_is_deprecated("claude-4-7-opus"));
    assert!(temperature_is_deprecated("claude-opus_4_7"));
    assert!(temperature_is_deprecated("claude-opus_47"));
    assert!(temperature_is_deprecated("us.anthropic.claude-opus-4-7"));
}

#[test]
fn detects_opus_5_aliases() {
    assert!(temperature_is_deprecated("claude-opus-5"));
    assert!(temperature_is_deprecated("claude-opus-5@default"));
    assert!(temperature_is_deprecated("vertex-anthropic/claude-opus-5"));
    assert!(temperature_is_deprecated("global.anthropic.claude-opus-5"));
    assert!(temperature_is_deprecated("claude-5-opus"));
}

#[test]
fn detects_newer_families_that_reject_temperature() {
    for model in [
        "bedrock/us.openai.gpt-6-astra",
        "us.anthropic.claude-fable-5",
        "global.anthropic.claude-sonnet-5-5",
        "us.anthropic.claude-haiku-5-5",
        "claude-5-haiku",
    ] {
        assert!(temperature_is_deprecated(model), "{model}");
    }
}

#[test]
fn non_deprecated_models_return_false() {
    assert!(!temperature_is_deprecated("claude-sonnet-4"));
    assert!(!temperature_is_deprecated("claude-haiku-4-5"));
    assert!(!temperature_is_deprecated("claude-opus-4-6"));
    assert!(!temperature_is_deprecated("gpt-4o"));
}
