//! Backend invocation and ordered application result recording.
use super::state::Run;
use anyhow::Result;
use codetether_swarm_core::{SubTask, SubTaskResult};
use std::time::Instant;
pub(super) async fn execute(
    run: &mut Run<'_, impl crate::Host>,
    tasks: Vec<SubTask>,
) -> Result<Vec<SubTaskResult>> {
    crate::stage_dispatch::execute(
        run.executor,
        &run.orchestrator,
        tasks,
        run.completed.clone(),
        &run.swarm_id,
    )
    .await
}
pub(super) async fn record(
    run: &mut Run<'_, impl crate::Host>,
    stage: usize,
    started: Instant,
    results: Vec<SubTaskResult>,
) {
    super::store::publish(run, stage, &results).await;
    super::stage_stats::record(run, stage, started.elapsed(), &results);
    run.results.extend(results);
}
