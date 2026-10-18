//! Forward a planned local stage through shared executor bindings.
use super::{LocalStage, LocalStageHost};
use crate::SwarmExecutor;
use codetether_swarm_core::{SubTask, SubTaskResult};
use codetether_swarm_orchestrator::{Host as PlanningHost, Orchestrator};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
/// Execute local jobs using the planner's providers and shared executor services.
/// # Arguments
/// Supply the executor, planner, stage tasks, completed outputs and swarm ID.
/// # Returns
/// Cached and executed task results from the existing local-stage driver.
/// # Errors
/// Propagates the driver's provider selection and stage setup failures.
/// # Examples
/// ```no_run
/// # async fn example<H, P>(executor: &codetether_swarm_runtime::SwarmExecutor<H>, planner: &codetether_swarm_orchestrator::Orchestrator<P>, completed: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, String>>>) -> anyhow::Result<()>
/// # where H: codetether_swarm_runtime::LocalStageHost, P: codetether_swarm_orchestrator::Host<Providers = H::Providers> {
/// let _results = codetether_swarm_runtime::local_stage::execute(
///     executor, planner, vec![], completed, "swarm",
/// ).await?;
/// # Ok(()) }
/// ```
pub async fn execute<H: LocalStageHost, P: PlanningHost<Providers = H::Providers>>(
    executor: &SwarmExecutor<H>,
    orchestrator: &Orchestrator<P>,
    tasks: Vec<SubTask>,
    completed: Arc<RwLock<HashMap<String, String>>>,
    swarm: &str,
) -> anyhow::Result<Vec<SubTaskResult>> {
    codetether_swarm_local_stage::execute(
        &LocalStage(executor),
        orchestrator.providers(),
        orchestrator.provider(),
        orchestrator.model(),
        tasks,
        completed,
        swarm,
    )
    .await
}
