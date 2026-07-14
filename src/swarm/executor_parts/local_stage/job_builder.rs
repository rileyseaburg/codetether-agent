//! Translation of a subtask into an owned local-agent job.

use super::{context, delegation, job::Job, state::State};
use crate::swarm::{SubTask, tool_policy};
use std::sync::Arc;

pub(super) async fn build(state: &mut State<'_>, task: SubTask, index: usize) -> Job {
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
    let read_only = tool_policy::is_read_only_task(&task);
    let verification = task.is_verification();
    let expects_changes = task.expects_file_changes();
    Job {
        id: task.id,
        name: task.name,
        instruction: task.instruction,
        specialty,
        context,
        provider,
        model,
        read_only,
        verification,
        expects_changes,
        workspace,
        result_store: Arc::clone(&state.executor.result_store),
        bus: state.executor.bus.clone(),
        events: state.executor.event_tx.clone(),
        semaphore: Arc::clone(&state.semaphore),
        stagger_ms: state.executor.config.request_delay_ms * index as u64,
        max_steps: state.executor.config.max_steps_per_subagent,
        timeout_secs: state.executor.config.subagent_timeout_secs,
        max_retries: state.executor.config.max_retries,
        base_delay_ms: state.executor.config.base_delay_ms,
        max_delay_ms: state.executor.config.max_delay_ms,
    }
}
