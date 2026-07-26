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
fn non_deprecated_models_return_false() {
    assert!(!temperature_is_deprecated("claude-sonnet-4"));
    assert!(!temperature_is_deprecated("claude-opus-4-6"));
    assert!(!temperature_is_deprecated("gpt-4o"));
}
