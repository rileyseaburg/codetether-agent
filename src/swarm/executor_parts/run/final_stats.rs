//! Final swarm statistics calculation.

use super::state::Run;

pub(super) fn calculate(run: &mut Run<'_>) -> (bool, crate::swarm::SwarmStats) {
    let settled = run.orchestrator.is_complete();
    let stats = run.orchestrator.stats_mut();
    stats.execution_time_ms = run.started_at.elapsed().as_millis() as u64;
    stats.sequential_time_estimate_ms = run
        .results
        .iter()
        .map(|result| result.execution_time_ms)
        .sum();
    stats.calculate_critical_path();
    stats.calculate_speedup();
    let success = run.results.len() == run.subtasks.len()
        && run.results.iter().all(|result| result.success)
        && settled;
    (success, stats.clone())
}
