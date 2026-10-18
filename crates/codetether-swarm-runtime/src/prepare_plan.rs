//! Shared readiness and cancellation around decomposition and provider setup.

use crate::Host;
use codetether_swarm_core::{DecompositionStrategy, SubTask};
use codetether_swarm_orchestrator::Orchestrator;

pub(super) async fn run<H: Host>(
    executor: &H,
    task: &str,
    strategy: DecompositionStrategy,
) -> anyhow::Result<(Orchestrator<H::Planning>, Vec<SubTask>)> {
    let control = executor.control();
    let plan = async {
        if let Some(control) = control {
            control.ready().await?;
        }
        let mut orchestrator = Orchestrator::<H::Planning>::new(executor.config().clone()).await?;
        if let Some(control) = control {
            control.ready().await?;
        }
        let subtasks = orchestrator.decompose(task, strategy).await?;
        Ok((orchestrator, subtasks))
    };
    match control {
        Some(control) => tokio::select! {
            biased;
            _ = control.cancelled() => anyhow::bail!("Swarm execution cancelled"),
            result = plan => result,
        },
        None => plan.await,
    }
}
