//! Bind the shared Kubernetes stage backend to native executor services.
use crate::{ExecutorHost, SwarmExecutor};
use codetether_swarm_core::{SubTask, SubTaskResult};
use codetether_swarm_orchestrator::{Host as PlanningHost, Orchestrator};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
/// Execute a Kubernetes stage using the executor's existing services.
/// # Arguments
/// Supply the executor, current planner, stage tasks, completed outputs and swarm ID.
/// # Returns
/// The stage's task results in the existing backend's order.
/// # Errors
/// Propagates Kubernetes setup, execution and result-collection failures.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_swarm_runtime::ExecutorHost, P: codetether_swarm_orchestrator::Host>(executor: &codetether_swarm_runtime::SwarmExecutor<H>, planner: &codetether_swarm_orchestrator::Orchestrator<P>, completed: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, String>>>) -> anyhow::Result<()> {
/// let _results = codetether_swarm_runtime::kubernetes::execute(
///     executor, planner, vec![], completed, "swarm",
/// ).await?;
/// # Ok(()) }
/// ```
pub async fn execute<H: ExecutorHost, P: PlanningHost>(
    executor: &SwarmExecutor<H>,
    orchestrator: &Orchestrator<P>,
    tasks: Vec<SubTask>,
    completed: Arc<RwLock<HashMap<String, String>>>,
    swarm: &str,
) -> anyhow::Result<Vec<SubTaskResult>> {
    codetether_swarm_k8s::execute(
        executor,
        orchestrator.provider(),
        orchestrator.model(),
        tasks,
        completed,
        swarm,
    )
    .await
}
