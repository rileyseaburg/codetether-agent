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
    let read_only = support::is_read_only(
        task.intent_name(),
        &task.instruction,
        task.specialty.as_deref(),
        task.needs_worktree,
    );
    let working_dir = worktrees
        .dir(idx)
        .and_then(|isolated| support::working_directory(isolated, read_only, parent));
    let task_id = task.id_for_index(idx);
    let context = working_dir.map(|working_dir| WorkerContext {
        read_only,
        verification: worktrees.is_verification(idx),
        expects_changes: worktrees.expects_changes(idx),
        working_dir,
        task_id: task_id.clone(),
        provider: selected.provider.clone(),
        model: selected.model.resolved_model.clone(),
    });

    (task_id, context)
}
