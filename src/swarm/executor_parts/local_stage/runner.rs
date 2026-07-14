//! Lifecycle driver for a local-thread stage.

use super::super::SwarmExecutor;
use crate::swarm::{Orchestrator, SubTask, SubTaskResult};
use anyhow::Result;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

pub(in crate::swarm::executor) async fn execute(
    executor: &SwarmExecutor,
    orchestrator: &Orchestrator,
    tasks: Vec<SubTask>,
    completed: Arc<RwLock<HashMap<String, String>>>,
    swarm: &str,
) -> Result<Vec<SubTaskResult>> {
    let mut state = super::create::state(executor, orchestrator, &tasks, completed, swarm)?;
    let pending = super::cache_filter::pending(&mut state, tasks).await;
    let runnable = super::worktrees::prepare(&mut state, pending).await;
    super::launch::all(&mut state, runnable).await;
    super::collect::all(&mut state).await;
    Ok(super::integrate::all(&mut state).await)
}
