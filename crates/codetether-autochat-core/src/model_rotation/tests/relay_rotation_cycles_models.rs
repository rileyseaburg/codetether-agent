//! Existing model selection regression coverage.
use super::*;

#[test]
fn relay_rotation_cycles_models() {
    let mut rotation = RelayModelRotation {
        model_refs: vec!["a/x".to_string(), "b/y".to_string()],
        cursor: 0,
    };
    assert_eq!(rotation.next_model_ref("fallback"), "a/x");
    assert_eq!(rotation.next_model_ref("fallback"), "b/y");
    assert_eq!(rotation.next_model_ref("fallback"), "a/x");
}
