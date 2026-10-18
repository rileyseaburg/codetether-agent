//! Construction of local-stage runtime state.

use super::{create_resources, state::State};
use crate::Host;
use anyhow::Result;
use codetether_swarm_core::SubTask;
use codetether_swarm_core::{cache as cache_result, run_policy};
use codetether_workspace_tools as workspace_registry;
use futures::stream::FuturesUnordered;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub(super) fn state<'a, H: Host>(
    executor: &'a H,
    providers: &'a H::Providers,
    provider_name: &str,
    model: &str,
    subtasks: &[SubTask],
    completed: Arc<RwLock<HashMap<String, String>>>,
    swarm_id: &'a str,
) -> Result<State<'a, H>> {
    let provider_name = provider_name.to_string();
    let (providers, fallback_provider) =
        create_resources::provider::<H>(providers, &provider_name)?;
    let workspace = workspace_registry::resolve_root(executor.config().working_dir.as_deref());
    let manager = create_resources::manager(executor, &workspace);
    Ok(State {
        executor,
        swarm_id,
        prior_results: completed,
        providers,
        fallback_provider,
        provider_name,
        model: model.to_string(),
        workspace,
        manager,
        cache_tasks: cache_result::read_only_tasks(subtasks),
        change_expectations: run_policy::expected_changes(subtasks),
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
            executor.config().max_concurrent_requests,
        )),
    })
}
