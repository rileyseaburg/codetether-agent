//! Launch and tracking of all runnable local subtasks.

use super::{job_builder, job_run, state::State};
use crate::swarm::SubTask;

pub(super) async fn all(state: &mut State<'_>, tasks: Vec<SubTask>) {
    for (index, task) in tasks.into_iter().enumerate() {
        let id = task.id.clone();
        let job = job_builder::build(state, task, index).await;
        tracing::debug!(subtask_id = %id, swarm_id = %state.swarm_id, "Launching sub-agent");
        let handle = tokio::spawn(job_run::execute(job));
        state.aborts.insert(id.clone(), handle.abort_handle());
        state.task_ids.insert(handle.id(), id);
        state.handles.push(handle);
    }
}
