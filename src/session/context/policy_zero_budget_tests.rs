//! Regression coverage for persisted zero incremental budgets.

use super::{Inputs, resolve};
use crate::session::derive_policy::DerivePolicy;

#[test]
fn zero_incremental_sentinel_uses_resolved_model_budget() {
    let policy = resolve(Inputs {
        persisted: DerivePolicy::Incremental { budget_tokens: 0 },
        has_reset_marker: false,
        reset_threshold: 90_000,
        incremental_budget: 221_184,
    });
    assert!(matches!(
        policy,
        DerivePolicy::Incremental {
            budget_tokens: 221_184
        }
    ));
}
