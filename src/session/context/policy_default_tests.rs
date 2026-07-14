//! Regression tests for proactive RLM policy selection.

use super::{Inputs, resolve};
use crate::session::derive_policy::DerivePolicy;

#[test]
fn legacy_sessions_use_incremental_rlm_by_default() {
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

#[test]
fn reset_markers_still_take_semantic_priority() {
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

#[test]
fn resolved_explicit_persisted_policies_are_preserved() {
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
