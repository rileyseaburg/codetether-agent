//! Allocate workspaces, schedule workers, and finalize the aggregate.
use super::{Host, Observer, SelectedProvider, collect, schedule};
use crate::{
    aggregate, aggregate_response, execution_config::ExecutionConfig, support,
    task_input::TaskInput, worktree_specs, worktrees::SwarmWorktrees,
};
use anyhow::Result;
use codetether_swarm_core::result_store::ResultStore;
use codetether_tool_core::ToolResult;
use serde_json::Value;
use std::sync::Arc;

pub(super) async fn execute<H: Host>(
    params: Value,
    tasks: Vec<TaskInput>,
    config: ExecutionConfig,
    mut observer: H::Observer,
    selected: SelectedProvider,
) -> Result<ToolResult> {
    let parent_workspace = support::workspace(&params);
    let shared_results = ResultStore::new_arc();
    let specs = worktree_specs::from_tasks(&tasks);
    let worktrees = SwarmWorktrees::create(&parent_workspace, &specs).await;
    let semaphore = Arc::new(tokio::sync::Semaphore::new(config.concurrency));
    let handles = schedule::spawn::<H>(
        &tasks,
        &worktrees,
        &parent_workspace,
        &selected,
        &config,
        &observer,
        shared_results,
        semaphore,
    );
    let (results, failures) = collect::join(handles, &worktrees).await;
    observer.complete(&results);
    let response = aggregate_response::build(results, failures, &config.aggregation_strategy);
    Ok(aggregate::result(response, failures))
}
