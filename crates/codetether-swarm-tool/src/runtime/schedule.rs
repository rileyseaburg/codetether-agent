//! Spawn workers in input order under shared session control.
use super::{Host, Observer, SelectedProvider, context, pending::Pending, worker};
use crate::{
    abort_on_drop::AbortOnDrop, execution_config::ExecutionConfig, task_input::TaskInput,
    worktrees::SwarmWorktrees,
};
use codetether_swarm_core::result_store::ResultStore;
use std::{path::Path, sync::Arc};

pub(super) fn spawn<H: Host>(
    tasks: &[TaskInput],
    worktrees: &SwarmWorktrees,
    parent: &Path,
    selected: &SelectedProvider,
    config: &ExecutionConfig,
    observer: &H::Observer,
    shared_results: Arc<ResultStore>,
    semaphore: Arc<tokio::sync::Semaphore>,
) -> Vec<Pending> {
    let mut handles = Vec::new();
    for (idx, task) in tasks.to_vec().into_iter().enumerate() {
        let (task_id, context) = context::prepare(&task, idx, worktrees, parent, selected);
        let failed_task_name = task.name.clone();
        let future = worker::run::<H>(
            task,
            context,
            selected.model.clone(),
            Arc::clone(&shared_results),
            Arc::clone(&semaphore),
            observer.sender(),
            config.max_steps,
            config.timeout_secs,
        );
        let handle = codetether_session_control::ControlHandle::spawn(future);
        handles.push((
            idx,
            task_id,
            failed_task_name,
            selected.model.clone(),
            AbortOnDrop::new(handle),
        ));
    }
    handles
}
