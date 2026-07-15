//! Serializable outcome for one `swarm_execute` worker.

use super::model_selection::ModelSelection;
use crate::swarm::executor::AgentLoopExit;

#[cfg(test)]
pub(super) fn completed(exit: AgentLoopExit, output: &str) -> bool {
    failure(exit, output).is_none()
}

pub(super) fn failure(exit: AgentLoopExit, output: &str) -> Option<String> {
    match exit {
        AgentLoopExit::Completed => crate::swarm::tool_policy::deliverable_error(output),
        AgentLoopExit::MaxStepsReached => Some("Sub-agent reached its step limit".into()),
        AgentLoopExit::TimedOut => Some("Sub-agent timed out".into()),
    }
}

#[derive(serde::Serialize)]
pub(super) struct TaskResult {
    pub(super) task_id: String,
    pub(super) task_name: String,
    pub(super) success: bool,
    pub(super) output: String,
    pub(super) error: Option<String>,
    pub(super) steps: usize,
    pub(super) tool_calls: usize,
    pub(super) model: ModelSelection,
}

impl TaskResult {
    pub(super) fn failed(
        task_id: String,
        task_name: String,
        error: String,
        model: ModelSelection,
    ) -> Self {
        Self {
            task_id,
            task_name,
            success: false,
            output: String::new(),
            error: Some(error),
            steps: 0,
            tool_calls: 0,
            model,
        }
    }
}
