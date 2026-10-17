//! incremental carries budget derivation policy.
use super::*;

#[test]
pub(super) fn incremental_carries_budget() {
    let p = DerivePolicy::Incremental {
        budget_tokens: 16_384,
    };
    if let DerivePolicy::Incremental { budget_tokens } = p {
        assert_eq!(budget_tokens, 16_384);
    } else {
        panic!("expected Incremental");
    }
}
