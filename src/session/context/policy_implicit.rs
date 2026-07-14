//! Implicit context-policy migration and resolution.

use crate::session::DerivePolicy;

pub(super) struct Inputs {
    pub persisted: DerivePolicy,
    pub has_reset_marker: bool,
    pub reset_threshold: usize,
    pub incremental_budget: usize,
}

pub(super) fn resolve(inputs: Inputs) -> DerivePolicy {
    match inputs.persisted {
        DerivePolicy::Legacy if inputs.has_reset_marker => DerivePolicy::Reset {
            threshold_tokens: inputs.reset_threshold,
        },
        DerivePolicy::Legacy => DerivePolicy::Incremental {
            budget_tokens: inputs.incremental_budget,
        },
        DerivePolicy::Incremental { .. } => DerivePolicy::Incremental {
            budget_tokens: inputs.incremental_budget,
        },
        policy => policy,
    }
}

#[cfg(test)]
#[path = "policy_default_tests.rs"]
mod tests;
#[cfg(test)]
#[path = "policy_zero_budget_tests.rs"]
mod zero_budget_tests;
