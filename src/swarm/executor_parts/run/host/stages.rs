//! Forward stage execution to the existing native backend adapters.
use super::Native;
use crate::swarm::{Orchestrator, SubTask, SubTaskResult};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
impl codetether_swarm_runtime::Stages for Native<'_> {
    async fn local(
        &self,
        orchestrator: &Orchestrator,
        tasks: Vec<SubTask>,
        completed: Arc<RwLock<HashMap<String, String>>>,
        swarm: &str,
    ) -> anyhow::Result<Vec<SubTaskResult>> {
        super::super::super::local_stage::execute(self.0, orchestrator, tasks, completed, swarm)
            .await
    }
    async fn kubernetes(
        &self,
        orchestrator: &Orchestrator,
        tasks: Vec<SubTask>,
        completed: Arc<RwLock<HashMap<String, String>>>,
        swarm: &str,
    ) -> anyhow::Result<Vec<SubTaskResult>> {
        super::super::super::k8s_stage::execute(self.0, orchestrator, tasks, completed, swarm).await
    }
}
