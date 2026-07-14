//! Finalization of telemetry, statistics, and aggregate output.

use super::state::Run;
use crate::swarm::SwarmResult;
use crate::tui::swarm_view::SwarmEvent;
use anyhow::Result;

pub(super) async fn execute(mut run: Run<'_>) -> Result<SwarmResult> {
    run.executor
        .telemetry
        .record_swarm_latency("total_execution", run.started_at.elapsed())
        .await;
    let provider = run.orchestrator.provider().to_string();
    let (success, stats) = super::final_stats::calculate(&mut run);
    let _ = run.executor.telemetry.complete_swarm(success).await;
    let result = super::aggregate::results(&run.results);
    tracing::info!(provider = %provider, subtasks = run.results.len(),
        speedup = stats.speedup_factor, "Swarm execution complete");
    let error = (!success).then(|| {
        let failed = run.results.iter().filter(|result| !result.success).count();
        format!(
            "Swarm incomplete: {}/{} subtasks returned, {failed} failed",
            run.results.len(),
            run.subtasks.len()
        )
    });
    run.executor.try_send_event(SwarmEvent::Complete {
        success,
        stats: stats.clone(),
    });
    Ok(SwarmResult {
        success,
        result,
        subtask_results: run.results,
        stats,
        artifacts: run.artifacts,
        error,
    })
}
