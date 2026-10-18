//! Pairwise Pareto dominance across cost, faults, oracle gap and reuse.
use crate::PolicyRunResult;

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
