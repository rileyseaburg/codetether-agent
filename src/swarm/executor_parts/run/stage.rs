//! Execution of one dependency stage.

use super::super::{dependency_failure, dependency_failure_events};
use super::state::Run;
use crate::swarm::SubTask;
use anyhow::Result;
use std::time::Instant;

pub(super) async fn execute(run: &mut Run<'_>, stage: usize) -> Result<()> {
    let started = Instant::now();
    let tasks: Vec<SubTask> = run
        .orchestrator
        .subtasks_for_stage(stage)
        .into_iter()
        .cloned()
        .collect();
    if tasks.is_empty() {
        return Ok(());
    }
    tracing::info!(provider = %run.orchestrator.provider(), stage, count = tasks.len(),
        "Executing swarm stage");
    let (runnable, mut results) = dependency_failure::partition(&tasks, &run.failed);
    dependency_failure_events::report(&tasks, &results, |event| {
        run.executor.try_send_event(event);
    });
    let mut executed = run
        .executor
        .execute_stage(
            &run.orchestrator,
            runnable,
            run.completed.clone(),
            &run.swarm_id,
        )
        .await?;
    results.append(&mut executed);
    super::store::publish(run, stage, &results).await;
    super::stage_stats::record(run, stage, started.elapsed(), &results);
    run.results.extend(results);
    Ok(())
}
