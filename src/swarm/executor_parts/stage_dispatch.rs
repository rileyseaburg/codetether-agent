//! Selection of the configured stage execution backend.

use super::SwarmExecutor;
use crate::swarm::{ExecutionMode, Orchestrator, SubTask, SubTaskResult};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

impl SwarmExecutor {
    pub(super) async fn execute_stage(
        &self,
        orchestrator: &Orchestrator,
        subtasks: Vec<SubTask>,
        completed: Arc<RwLock<HashMap<String, String>>>,
        swarm_id: &str,
    ) -> Result<Vec<SubTaskResult>> {
        match self.config.execution_mode {
            ExecutionMode::KubernetesPod => {
                super::k8s_stage::execute(self, orchestrator, subtasks, completed, swarm_id).await
            }
            ExecutionMode::LocalThread => {
                super::local_stage::execute(self, orchestrator, subtasks, completed, swarm_id).await
            }
        }
    }
}
