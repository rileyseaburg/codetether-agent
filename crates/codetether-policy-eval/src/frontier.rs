//! Stable nondominated policy selection.
use crate::PolicyRunResult;

/// Drop dominated points and return references to the frontier.
///
/// Stable in input order among surviving points so downstream callers
/// can render the frontier with policy labels intact.
/// # Arguments
/// * `results` — Policy samples in presentation order.
/// # Returns
/// Borrowed nondominated samples, preserving their input order.
/// # Examples
/// ```
/// assert!(codetether_policy_eval::pareto_frontier(&[]).is_empty());
/// ```
pub fn pareto_frontier(results: &[PolicyRunResult]) -> Vec<&PolicyRunResult> {
    results
        .iter()
        .enumerate()
        .filter(|(i, candidate)| {
            !results
                .iter()
                .enumerate()
                .any(|(j, other)| *i != j && candidate.is_dominated_by(other))
        })
        .map(|(_, r)| r)
        .collect()
}
