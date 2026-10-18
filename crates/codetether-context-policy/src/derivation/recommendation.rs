//! Evaluation-based default derivation policy recommendations.
//!
//! Picks the policy that Pareto-dominates [`DerivePolicy::Legacy`] on a
//! recorded fixture sweep. Returns `None` when no alternative dominates.

use super::DerivePolicy;
use codetether_policy_eval::{PolicyRunResult, pareto_frontier};

const LEGACY: &str = "legacy";

/// Recommend the [`DerivePolicy`] that should become the new default.
///
/// Returns the first non-legacy policy on the Pareto frontier that strictly
/// dominates the legacy entry. `None` keeps the existing default.
///
/// # Arguments
/// * `results` - Recorded policy samples to compare against the legacy entry.
/// # Returns
/// A reset or incremental strategy with its zero-sentinel budget, if a supported
/// non-legacy frontier entry strictly dominates the legacy sample.
/// # Examples
/// ```
/// use codetether_context_policy::derivation::recommendation::recommend_default;
/// assert!(recommend_default(&[]).is_none());
/// ```
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
#[path = "recommendation_tests.rs"]
mod tests;
