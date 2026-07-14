//! Per-stage statistics and lifecycle events.

use super::state::Run;
use crate::swarm::{StageStats, SubTaskResult};
use crate::tui::swarm_view::SwarmEvent;
use std::time::Duration;

pub(super) fn record(
    run: &mut Run<'_>,
    stage: usize,
    elapsed: Duration,
    results: &[SubTaskResult],
) {
    run.orchestrator.stats_mut().stages.push(StageStats {
        stage,
        subagent_count: results.len(),
        max_steps: results.iter().map(|result| result.steps).max().unwrap_or(0),
        total_steps: results.iter().map(|result| result.steps).sum(),
        execution_time_ms: elapsed.as_millis() as u64,
    });
    for result in results {
        run.orchestrator
            .complete_subtask(&result.subtask_id, result.clone());
        if !result.success {
            run.failed.insert(result.subtask_id.clone());
        }
    }
    run.executor.try_send_event(SwarmEvent::StageComplete {
        stage,
        completed: results.iter().filter(|result| result.success).count(),
        failed: results.iter().filter(|result| !result.success).count(),
    });
}
