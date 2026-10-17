//! resolved explicit persisted policies are preserved derivation policy.
use super::*;

#[test]
pub(super) fn resolved_explicit_persisted_policies_are_preserved() {
    let persisted = DerivePolicy::Incremental {
        budget_tokens: 32_000,
    };
    assert!(matches!(
        resolve(Inputs {
            persisted,
            has_reset_marker: false,
            reset_threshold: 90_000,
            incremental_budget: 32_000
        }),
        DerivePolicy::Incremental {
            budget_tokens: 32_000
        }
    ));
}
