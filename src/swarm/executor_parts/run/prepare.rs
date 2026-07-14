//! Decomposition and initialization of a swarm run.

use super::state::Run;
use crate::swarm::{DecompositionStrategy, Orchestrator, SwarmResult};
use crate::tui::swarm_view::SwarmEvent;
use anyhow::Result;

pub(super) enum Prepared<'a> {
    Empty(SwarmResult),
    Ready(Run<'a>),
}

pub(super) async fn run<'a>(
    executor: &'a super::super::SwarmExecutor,
    task: &str,
    strategy: DecompositionStrategy,
) -> Result<Prepared<'a>> {
    let strategy_name = format!("{strategy:?}");
    let mut orchestrator = Orchestrator::new(executor.config.clone()).await?;
    let subtasks = orchestrator.decompose(task, strategy).await?;
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
        executor.config.max_steps_per_subagent,
    ));
    let run = Run::new(executor, orchestrator, subtasks);
    executor
        .telemetry
        .start_swarm(&run.swarm_id, run.subtasks.len(), &strategy_name)
        .await;
    Ok(Prepared::Ready(run))
}
