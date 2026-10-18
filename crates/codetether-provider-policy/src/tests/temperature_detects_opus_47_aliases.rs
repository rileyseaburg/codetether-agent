//! Original provider policy regression case.
use crate::temperature_is_deprecated;

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
