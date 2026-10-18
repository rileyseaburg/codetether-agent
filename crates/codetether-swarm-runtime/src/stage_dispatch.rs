//! Select the configured local or Kubernetes stage backend.
use crate::Host;
use codetether_swarm_core::{ExecutionMode, SubTask, SubTaskResult};
use codetether_swarm_orchestrator::Orchestrator;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
pub(super) async fn execute<H: Host>(
    host: &H,
    orchestrator: &Orchestrator<H::Planning>,
    tasks: Vec<SubTask>,
    completed: Arc<RwLock<HashMap<String, String>>>,
    swarm: &str,
) -> anyhow::Result<Vec<SubTaskResult>> {
    match host.config().execution_mode {
        ExecutionMode::KubernetesPod => {
            host.kubernetes(orchestrator, tasks, completed, swarm).await
        }
        ExecutionMode::LocalThread => host.local(orchestrator, tasks, completed, swarm).await,
    }
}
