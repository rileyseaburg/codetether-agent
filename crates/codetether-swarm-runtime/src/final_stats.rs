//! Host timing and coordinator state supplied to shared run statistics.
use super::state::Run;
pub(super) fn calculate(
    run: &mut Run<'_, impl crate::Host>,
) -> (bool, codetether_swarm_core::SwarmStats) {
    let settled = run.orchestrator.is_complete();
    let stats = run.orchestrator.stats_mut();
    let elapsed_ms = run.started_at.elapsed().as_millis() as u64;
    codetether_swarm_core::run_policy::finalize_stats(
        stats,
        &run.results,
        run.subtasks.len(),
        settled,
        elapsed_ms,
    )
}
