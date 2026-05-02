//! Pareto evaluation harness for [`DerivePolicy`].
//!
//! ## Role (Phase B step 23)
//!
//! Liu et al. (arXiv:2512.22087), Lu et al. (arXiv:2510.06727), and
//! ClawVM (arXiv:2604.10352) agree on one methodological point: a
//! policy is never flipped on aggregate accuracy alone. Pareto over
//! `(accuracy, context-cost, reuse-rate, oracle-gap)` is the minimum.
//! This module is the *comparator* half of that evaluation loop. The
//! *runner* half — actually executing [`derive_with_policy`] over a
//! fixture corpus and collecting [`PolicyRunResult`]s — is a
//! follow-up commit.
//!
//! ## Scope
//!
//! * [`PolicyRunResult`] — per-policy datapoint.
//! * [`pareto_frontier`] — drop dominated points, keep frontier.
//! * [`reuse_rate`] — Cognitive-Workspace-style warm-hit proxy.
//!
//! The ClawVM Tier-1 fault regression suite is a sibling concern;
//! those tests live as `#[test]` functions on the subsystems they
//! exercise (compression, journal, session_recall) rather than as
//! harness inputs.
//!
//! ## Examples
//!
//! ```rust
//! use codetether_agent::session::eval::{PolicyRunResult, pareto_frontier, reuse_rate};
//!
//! let results = vec![
//!     PolicyRunResult {
//!         policy: "legacy",
//!         kept_messages: 30,
//!         context_tokens: 24_000,
//!         fault_count: 12,
//!         oracle_gap: 4,
//!         reuse_rate: 0.50,
//!     },
//!     PolicyRunResult {
//!         policy: "reset",
//!         kept_messages: 8,
//!         context_tokens: 6_500,
//!         fault_count: 5,
//!         oracle_gap: 2,
//!         reuse_rate: 0.62,
//!     },
//! ];
//! let frontier = pareto_frontier(&results);
//! assert!(frontier.iter().any(|r| r.policy == "reset"));
//!
//! // 4 of 8 context entries were already warm from the prior turn.
//! assert!((reuse_rate(&[4, 8]) - 0.5).abs() < 1e-9);
//! ```

/// One Pareto sample for a single derivation policy run.
///
/// Lower is better on [`Self::context_tokens`], [`Self::fault_count`],
/// and [`Self::oracle_gap`]. Higher is better on [`Self::reuse_rate`].
/// [`Self::kept_messages`] is neutral (informational).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PolicyRunResult {
    /// Policy identifier (matches
    /// [`DerivePolicy::kind`](super::derive_policy::DerivePolicy::kind)).
    pub policy: &'static str,
    /// Size of the derived context in messages.
    pub kept_messages: usize,
    /// Estimated token cost of the derived context.
    pub context_tokens: usize,
    /// Explicit faults raised during the run (ClawVM Tier-1).
    pub fault_count: usize,
    /// Online-minus-oracle fault delta from [`replay_oracle`].
    ///
    /// [`replay_oracle`]: super::oracle::replay_oracle
    pub oracle_gap: usize,
    /// Fraction of derived-context entries that were already present
    /// in the previous turn's derived context (Cognitive-Workspace
    /// warm-hit proxy). `0.0 ≤ reuse_rate ≤ 1.0`.
    pub reuse_rate: f64,
}

impl PolicyRunResult {
    /// Returns `true` when `other` Pareto-dominates `self` — that is,
    /// `other` is at least as good on every axis and strictly better
    /// on at least one.
    pub fn is_dominated_by(&self, other: &PolicyRunResult) -> bool {
        let better_or_equal_on_all = other.context_tokens <= self.context_tokens
            && other.fault_count <= self.fault_count
            && other.oracle_gap <= self.oracle_gap
            && other.reuse_rate >= self.reuse_rate;
        let strictly_better_on_one = other.context_tokens < self.context_tokens
            || other.fault_count < self.fault_count
            || other.oracle_gap < self.oracle_gap
            || other.reuse_rate > self.reuse_rate;
        better_or_equal_on_all && strictly_better_on_one
    }
}

/// Drop dominated points and return references to the frontier.
///
/// Stable in input order among surviving points so downstream callers
/// can render the frontier with policy labels intact.
pub fn pareto_frontier(results: &[PolicyRunResult]) -> Vec<&PolicyRunResult> {
    results
        .iter()
        .filter(|candidate| {
            !results
                .iter()
                .any(|other| !std::ptr::eq(*candidate, other) && candidate.is_dominated_by(other))
        })
        .collect()
}

/// Compute the reuse rate from `(warm_hits, total)`.
///
/// Returns `0.0` when `total == 0` to avoid NaN so downstream
/// aggregations (averages, min/max, Pareto comparisons) stay clean.
pub fn reuse_rate(counts: &[usize; 2]) -> f64 {
    let [warm, total] = *counts;
    if total == 0 {
        return 0.0;
    }
    warm as f64 / total as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(
        policy: &'static str,
        tokens: usize,
        faults: usize,
        gap: usize,
        reuse: f64,
    ) -> PolicyRunResult {
        PolicyRunResult {
            policy,
            kept_messages: 0,
            context_tokens: tokens,
            fault_count: faults,
            oracle_gap: gap,
            reuse_rate: reuse,
        }
    }

    #[test]
    fn strictly_better_on_all_axes_dominates() {
        let a = sample("a", 1000, 5, 2, 0.5);
        let b = sample("b", 500, 1, 0, 0.9);
        assert!(a.is_dominated_by(&b));
        assert!(!b.is_dominated_by(&a));
    }

    #[test]
    fn tied_on_all_does_not_dominate() {
        let a = sample("a", 1000, 5, 2, 0.5);
        let b = sample("b", 1000, 5, 2, 0.5);
        assert!(!a.is_dominated_by(&b));
        assert!(!b.is_dominated_by(&a));
    }

    #[test]
    fn pareto_frontier_keeps_nondominated_points() {
        let results = vec![
            sample("legacy", 24_000, 12, 4, 0.50),
            sample("reset", 6_500, 5, 2, 0.62),
            sample("dominated", 30_000, 20, 10, 0.10),
        ];
        let frontier = pareto_frontier(&results);
        let labels: Vec<&str> = frontier.iter().map(|r| r.policy).collect();
        assert!(labels.contains(&"reset"));
        assert!(!labels.contains(&"dominated"));
    }

    #[test]
    fn reuse_rate_is_zero_when_total_is_zero() {
        assert_eq!(reuse_rate(&[0, 0]), 0.0);
    }

    #[test]
    fn reuse_rate_round_trips_typical_values() {
        assert!((reuse_rate(&[7, 10]) - 0.7).abs() < 1e-9);
        assert!((reuse_rate(&[10, 10]) - 1.0).abs() < 1e-9);
    }
}
