//! policy implicit rules.
//! Implicit context-policy migration and resolution.

use crate::derivation::DerivePolicy;

pub struct Inputs {
    pub persisted: DerivePolicy,
    pub has_reset_marker: bool,
    pub reset_threshold: usize,
    pub incremental_budget: usize,
}

#[cfg(test)]
#[path = "policy_default_tests/mod.rs"]
mod tests;

pub fn resolve(inputs: Inputs) -> DerivePolicy {
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
#[path = "policy_zero_budget_tests/mod.rs"]
mod zero_budget_tests;
