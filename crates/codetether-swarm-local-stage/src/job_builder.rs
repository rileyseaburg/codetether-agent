//! Translation of a subtask into an owned local-agent job.

use super::{context, delegation, job::Job, state::State};
use codetether_swarm_core::SubTask;
use std::sync::Arc;

pub(super) async fn build(
    state: &mut State<'_, impl crate::Host>,
    task: SubTask,
    index: usize,
) -> Job {
    let (provider_name, model, provider) = delegation::select(state, &task);
    let specialty = task.specialty.clone().unwrap_or_else(|| "General".into());
    state
        .assignments
        .insert(task.id.clone(), (provider_name, specialty.clone()));
    let completed = state.prior_results.read().await;
    let context = context::build(&task, &completed);
    drop(completed);
    let worktree = state.ready_worktrees.remove(&task.id);
    let workspace = worktree
        .as_ref()
        .map(|item| item.path.clone())
        .unwrap_or_else(|| state.workspace.clone());
    let shared_workspace = !state.executor.config().worktree_enabled;
    let access = codetether_swarm_core::tool_policy::TaskAccess::for_task(&task, shared_workspace);
    Job {
        id: task.id,
        name: task.name,
        instruction: task.instruction,
        specialty,
        context,
        provider,
        model,
        shared_workspace,
        read_only: access.read_only,
        verification: access.verification,
        expects_changes: access.expects_changes,
        workspace,
        result_store: Arc::clone(state.executor.result_store()),
        bus: state.executor.bus().clone(),
        events: state.executor.events().clone(),
        semaphore: Arc::clone(&state.semaphore),
        stagger_ms: state.executor.config().request_delay_ms * index as u64,
        max_steps: state.executor.config().max_steps_per_subagent,
        timeout_secs: state.executor.config().subagent_timeout_secs,
        max_retries: state.executor.config().max_retries,
        base_delay_ms: state.executor.config().base_delay_ms,
        max_delay_ms: state.executor.config().max_delay_ms,
        control: state.executor.control().cloned(),
    }
}
