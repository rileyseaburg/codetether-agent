//! Original provider policy regression case.
use crate::temperature_is_deprecated;

#[test]
fn non_deprecated_models_return_false() {
    assert!(!temperature_is_deprecated("claude-sonnet-4"));
    assert!(!temperature_is_deprecated("claude-haiku-4-5"));
    assert!(!temperature_is_deprecated("claude-opus-4-6"));
    assert!(!temperature_is_deprecated("gpt-4o"));
}
