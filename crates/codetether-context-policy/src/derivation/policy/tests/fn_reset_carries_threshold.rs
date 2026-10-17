//! reset carries threshold derivation policy.
use super::*;

#[test]
pub(super) fn reset_carries_threshold() {
    let p = DerivePolicy::Reset {
        threshold_tokens: 8192,
    };
    if let DerivePolicy::Reset { threshold_tokens } = p {
        assert_eq!(threshold_tokens, 8192);
    } else {
        panic!("expected Reset");
    }
}
