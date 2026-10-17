//! reset markers still take semantic priority derivation policy.
use super::*;

#[test]
pub(super) fn reset_markers_still_take_semantic_priority() {
    let policy = resolve(Inputs {
        persisted: DerivePolicy::Legacy,
        has_reset_marker: true,
        reset_threshold: 90_000,
        incremental_budget: 16_000,
    });
    assert!(matches!(
        policy,
        DerivePolicy::Reset {
            threshold_tokens: 90_000
        }
    ));
}
