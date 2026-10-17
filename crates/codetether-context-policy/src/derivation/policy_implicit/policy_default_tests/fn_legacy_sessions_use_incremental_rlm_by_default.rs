//! legacy sessions use incremental rlm by default derivation policy.
use super::*;

#[test]
pub(super) fn legacy_sessions_use_incremental_rlm_by_default() {
    let policy = resolve(Inputs {
        persisted: DerivePolicy::Legacy,
        has_reset_marker: false,
        reset_threshold: 90_000,
        incremental_budget: 16_000,
    });
    assert!(matches!(
        policy,
        DerivePolicy::Incremental {
            budget_tokens: 16_000
        }
    ));
}
