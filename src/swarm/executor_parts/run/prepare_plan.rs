//! Shared readiness and cancellation around decomposition and provider setup.

use super::super::SwarmExecutor;
use crate::swarm::{DecompositionStrategy, Orchestrator, SubTask};

pub(super) async fn run(
    executor: &SwarmExecutor,
    task: &str,
    strategy: DecompositionStrategy,
) -> anyhow::Result<(Orchestrator, Vec<SubTask>)> {
    let control = executor.control.as_ref();
    let plan = async {
        if let Some(control) = control {
            control.ready().await?;
        }
        let mut orchestrator = Orchestrator::new(executor.config.clone()).await?;
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
