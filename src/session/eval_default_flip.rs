//! Phase B step 24 — default-flip recommendation.
//!
//! Picks the policy that Pareto-dominates [`DerivePolicy::Legacy`] on a
//! recorded fixture sweep. Returns `None` when no alternative dominates.

use super::derive_policy::DerivePolicy;
use super::eval::{PolicyRunResult, pareto_frontier};

const LEGACY: &str = "legacy";

/// Recommend the [`DerivePolicy`] that should become the new default.
///
/// Returns the first non-legacy policy on the Pareto frontier that strictly
/// dominates the legacy entry. `None` keeps the existing default.
pub fn recommend_default(results: &[PolicyRunResult]) -> Option<DerivePolicy> {
    let legacy = results.iter().find(|r| r.policy == LEGACY)?;
    let frontier = pareto_frontier(results);
    let winner = frontier
        .iter()
        .find(|r| r.policy != LEGACY && legacy.is_dominated_by(r))?;
    match winner.policy {
        "reset" => Some(DerivePolicy::Reset {
            threshold_tokens: 0,
        }),
        "incremental" => Some(DerivePolicy::Incremental { budget_tokens: 0 }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(policy: &'static str, tokens: usize, faults: usize) -> PolicyRunResult {
        PolicyRunResult {
            policy,
            kept_messages: 0,
            context_tokens: tokens,
            fault_count: faults,
            oracle_gap: 0,
            reuse_rate: 0.5,
        }
    }

    #[test]
    fn flips_when_reset_dominates() {
        let res = vec![r("legacy", 24_000, 12), r("reset", 6_000, 4)];
        assert!(matches!(
            recommend_default(&res),
            Some(DerivePolicy::Reset { .. })
        ));
    }

    #[test]
    fn keeps_legacy_when_no_dominator() {
        let res = vec![r("legacy", 24_000, 12), r("reset", 30_000, 4)];
        assert!(recommend_default(&res).is_none());
    }
}
