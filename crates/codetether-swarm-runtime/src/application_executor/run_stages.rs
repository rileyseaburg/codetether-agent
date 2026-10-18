//! Forward stage execution to the existing native backend adapters.
use super::{ExecutionHost, SwarmExecutor};
use codetether_swarm_core::{SubTask, SubTaskResult};
use codetether_swarm_orchestrator::Orchestrator;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
impl<H: ExecutionHost> crate::Stages for SwarmExecutor<H> {
    async fn local(
        &self,
        orchestrator: &Orchestrator<H::Planning>,
        tasks: Vec<SubTask>,
        completed: Arc<RwLock<HashMap<String, String>>>,
        swarm: &str,
    ) -> anyhow::Result<Vec<SubTaskResult>> {
        super::local::execute(self, orchestrator, tasks, completed, swarm).await
    }
    async fn kubernetes(
        &self,
        orchestrator: &Orchestrator<H::Planning>,
        tasks: Vec<SubTask>,
        completed: Arc<RwLock<HashMap<String, String>>>,
        swarm: &str,
    ) -> anyhow::Result<Vec<SubTaskResult>> {
        super::kubernetes::execute(self, orchestrator, tasks, completed, swarm).await
    }
}
