//! Default effective context policy.

use crate::session::derive_policy::DerivePolicy;

pub(super) fn non_override(persisted: DerivePolicy) -> DerivePolicy {
    if matches!(persisted, DerivePolicy::Legacy) {
        DerivePolicy::Incremental {
            budget_tokens: super::incremental::DEFAULT_INCREMENTAL_BUDGET,
        }
    } else {
        persisted
    }
}
