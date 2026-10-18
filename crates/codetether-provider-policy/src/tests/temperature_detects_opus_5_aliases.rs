//! Original provider policy regression case.
use crate::temperature_is_deprecated;

#[test]
fn detects_opus_5_aliases() {
    assert!(temperature_is_deprecated("claude-opus-5"));
    assert!(temperature_is_deprecated("claude-opus-5@default"));
    assert!(temperature_is_deprecated("vertex-anthropic/claude-opus-5"));
    assert!(temperature_is_deprecated("global.anthropic.claude-opus-5"));
    assert!(temperature_is_deprecated("claude-5-opus"));
}
