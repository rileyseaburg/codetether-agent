//! Warm-hit reuse ratio calculation.

/// Compute the reuse rate from `(warm_hits, total)`.
///
/// Returns `0.0` when `total == 0` to avoid NaN so downstream
/// aggregations (averages, min/max, Pareto comparisons) stay clean.
/// # Arguments
/// * `counts` — Warm-hit count followed by total entry count.
/// # Returns
/// Warm hits divided by total, or zero when the total is zero.
/// # Examples
/// ```
/// assert_eq!(codetether_policy_eval::reuse_rate(&[4, 8]), 0.5);
/// ```
pub fn reuse_rate(counts: &[usize; 2]) -> f64 {
    let [warm, total] = *counts;
    if total == 0 {
        return 0.0;
    }
    warm as f64 / total as f64
}
