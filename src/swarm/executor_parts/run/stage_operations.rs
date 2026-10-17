//! Backend invocation and ordered application result recording.
use super::state::Run;
use crate::swarm::{SubTask, SubTaskResult};
use anyhow::Result;
use std::time::Instant;
pub(super) async fn execute(run: &mut Run<'_>, tasks: Vec<SubTask>) -> Result<Vec<SubTaskResult>> {
    run.executor
        .execute_stage(
            &run.orchestrator,
            tasks,
            run.completed.clone(),
            &run.swarm_id,
        )
        .await
}
pub(super) async fn record(
    run: &mut Run<'_>,
    stage: usize,
    started: Instant,
    results: Vec<SubTaskResult>,
) {
    super::store::publish(run, stage, &results).await;
    super::stage_stats::record(run, stage, started.elapsed(), &results);
    run.results.extend(results);
}
