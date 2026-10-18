//! Bind the shared Kubernetes stage backend to native executor services.
mod host;
use super::SwarmExecutor;
use crate::swarm::{Orchestrator, SubTask, SubTaskResult};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
pub(super) async fn execute(
    executor: &SwarmExecutor,
    orchestrator: &Orchestrator,
    tasks: Vec<SubTask>,
    completed: Arc<RwLock<HashMap<String, String>>>,
    swarm: &str,
) -> anyhow::Result<Vec<SubTaskResult>> {
    codetether_swarm_k8s::execute(
        &host::Native(executor),
        orchestrator.provider(),
        orchestrator.model(),
        tasks,
        completed,
        swarm,
    )
    .await
}
