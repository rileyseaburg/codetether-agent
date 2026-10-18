//! Decomposition and initialization of a swarm run.

use super::state::Run;
use anyhow::Result;
use codetether_swarm_core::SwarmEvent;
use codetether_swarm_core::{DecompositionStrategy, SwarmResult};

pub(super) enum Prepared<'a, H: crate::Host> {
    Empty(SwarmResult),
    Ready(Run<'a, H>),
}

pub(super) async fn run<'a, H: crate::Host>(
    executor: &'a H,
    task: &str,
    strategy: DecompositionStrategy,
) -> Result<Prepared<'a, H>> {
    let strategy_name = format!("{strategy:?}");
    let (orchestrator, subtasks) = super::prepare_plan::run(executor, task, strategy).await?;
    if subtasks.is_empty() {
        executor.try_send_event(SwarmEvent::Error("No subtasks generated".into()));
        return Ok(Prepared::Empty(super::empty::result()));
    }
    tracing::info!(provider = %orchestrator.provider(), count = subtasks.len(), "Task decomposed");
    executor.try_send_event(SwarmEvent::Started {
        task: task.into(),
        total_subtasks: subtasks.len(),
    });
    executor.try_send_event(super::decomposed_event::build(
        &subtasks,
        executor.config().max_steps_per_subagent,
    ));
    let run = Run::new(executor, orchestrator, subtasks);
    executor
        .telemetry()
        .start_swarm(&run.swarm_id, run.subtasks.len(), &strategy_name)
        .await;
    Ok(Prepared::Ready(run))
}
