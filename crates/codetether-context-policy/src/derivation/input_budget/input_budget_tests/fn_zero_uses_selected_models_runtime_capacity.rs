//! zero uses selected models runtime capacity derivation policy.
use super::*;

#[test]
pub(super) fn zero_uses_selected_models_runtime_capacity() {
    assert_eq!(resolve("mistral", 0), usable("mistral"));
    assert_eq!(resolve("gpt-5.6-sol", 0), usable("gpt-5.6-sol"));
    assert!(usable("gpt-5.6-sol") > usable("mistral"));
}
