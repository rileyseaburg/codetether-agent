//! Summary statistics for a direct swarm monitor run.

use super::super::task_result::TaskResult;
use crate::swarm::SwarmStats;
use std::time::Duration;

pub(super) fn build(results: &[TaskResult], elapsed: Duration) -> (bool, SwarmStats) {
    let completed = results.iter().filter(|result| result.success).count();
    let stats = SwarmStats {
        subagents_spawned: results.len(),
        subagents_completed: completed,
        subagents_failed: results.len() - completed,
        total_tool_calls: results.iter().map(|result| result.tool_calls).sum(),
        execution_time_ms: elapsed.as_millis() as u64,
        ..Default::default()
    };
    (completed == results.len(), stats)
}
