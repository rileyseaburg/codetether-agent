//! Native bindings for shared local-stage scheduling and worktree integration.
mod host;
mod job;
mod job_host;
mod job_run;
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
    codetether_swarm_local_stage::execute(
        &host::Native(executor),
        orchestrator.providers(),
        orchestrator.provider(),
        orchestrator.model(),
        tasks,
        completed,
        swarm,
    )
    .await
}
