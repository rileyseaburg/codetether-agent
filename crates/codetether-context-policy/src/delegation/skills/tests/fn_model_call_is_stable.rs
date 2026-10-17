//! model call is stable derivation policy.
use super::*;

#[test]
pub(super) fn model_call_is_stable() {
    // Changing this string would silently fragment existing
    // DelegationState sidecars. Unit-test it so the rename is
    // caught in review, not in production.
    assert_eq!(MODEL_CALL, "model_call");
}
