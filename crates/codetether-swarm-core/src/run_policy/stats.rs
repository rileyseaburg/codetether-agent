use crate::{SubTaskResult, SwarmStats};
/// Finalize statistics and determine whether every planned task succeeded.
///
/// # Arguments
/// * `stats` - Accumulated statistics, updated in place.
/// * `results` - Recorded execution outcomes.
/// * `expected_count` - Number of planned tasks.
/// * `settled` - Whether all coordinator tasks reached terminal state.
/// * `elapsed_ms` - Duration measured by the host.
///
/// # Returns
/// Overall success and a clone of the final statistics.
///
/// # Examples
/// ```
/// let mut stats = codetether_swarm_core::SwarmStats::default();
/// let (success, final_stats) = codetether_swarm_core::run_policy::finalize_stats(&mut stats, &[], 0, true, 12);
/// assert!(success);
/// assert_eq!(final_stats.execution_time_ms, 12);
/// ```
pub fn finalize_stats(
    stats: &mut SwarmStats,
    results: &[SubTaskResult],
    expected_count: usize,
    settled: bool,
    elapsed_ms: u64,
) -> (bool, SwarmStats) {
    stats.execution_time_ms = elapsed_ms;
    stats.sequential_time_estimate_ms = results.iter().map(|result| result.execution_time_ms).sum();
    stats.calculate_critical_path();
    stats.calculate_speedup();
    let success =
        results.len() == expected_count && results.iter().all(|result| result.success) && settled;
    (success, stats.clone())
}
