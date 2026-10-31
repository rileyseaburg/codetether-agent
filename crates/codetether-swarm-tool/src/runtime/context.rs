//! Prepare a worker identity and required working directory before spawning.
use super::{SelectedProvider, WorkerContext};
use crate::{support, task_input::TaskInput, worktrees::SwarmWorktrees};
use std::path::Path;
pub(super) fn prepare(
    task: &TaskInput,
    idx: usize,
    worktrees: &SwarmWorktrees,
    parent: &Path,
    selected: &SelectedProvider,
) -> (String, anyhow::Result<WorkerContext>) {
    let task_access = crate::support_task::build(
        task.intent_name(),
        &task.instruction,
        task.specialty.as_deref(),
        task.needs_worktree,
    );
    let access = codetether_swarm_core::tool_policy::TaskAccess::for_task(
        &task_access,
        worktrees.is_shared(),
    );
    let working_dir = worktrees
        .dir(idx)
        .and_then(|isolated| support::working_directory(isolated, access.read_only, parent));
    let task_id = task.id_for_index(idx);
    let context = working_dir.map(|working_dir| WorkerContext {
        shared_workspace: worktrees.is_shared(),
        read_only: access.read_only,
        verification: worktrees.is_verification(idx),
        expects_changes: worktrees.expects_changes(idx),
        working_dir,
        task_id: task_id.clone(),
        provider: selected.provider.clone(),
        model: selected.model.resolved_model.clone(),
    });

    (task_id, context)
}
