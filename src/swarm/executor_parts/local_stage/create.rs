//! Construction of local-stage runtime state.

use super::super::{SwarmExecutor, cache_result, change_expectations, workspace_registry};
use super::{create_resources, state::State};
use crate::swarm::{Orchestrator, SubTask};
use anyhow::Result;
use futures::stream::FuturesUnordered;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub(super) fn state<'a>(
    executor: &'a SwarmExecutor,
    orchestrator: &'a Orchestrator,
    subtasks: &[SubTask],
    completed: Arc<RwLock<HashMap<String, String>>>,
    swarm_id: &'a str,
) -> Result<State<'a>> {
    let provider_name = orchestrator.provider().to_string();
    let (providers, fallback_provider) = create_resources::provider(orchestrator, &provider_name)?;
    let workspace = workspace_registry::resolve_root(executor.config.working_dir.as_deref());
    let manager = create_resources::manager(executor, &workspace);
    Ok(State {
        executor,
        swarm_id,
        prior_results: completed,
        providers,
        fallback_provider,
        provider_name,
        model: orchestrator.model().to_string(),
        workspace,
        manager,
        cache_tasks: cache_result::read_only_tasks(subtasks),
        change_expectations: change_expectations::for_tasks(subtasks),
        handles: FuturesUnordered::new(),
        aborts: HashMap::new(),
        task_ids: HashMap::new(),
        active_worktrees: HashMap::new(),
        all_worktrees: HashMap::new(),
        ready_worktrees: HashMap::new(),
        immediate_results: Vec::new(),
        completed: Vec::new(),
        assignments: HashMap::new(),
        kill_reasons: HashMap::new(),
        promoted: None,
        semaphore: Arc::new(tokio::sync::Semaphore::new(
            executor.config.max_concurrent_requests,
        )),
    })
}
