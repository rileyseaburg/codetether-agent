//! Serializable outcome for one `swarm_execute` worker.

use super::model_selection::ModelSelection;

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
