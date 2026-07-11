//! Serializable outcome for one `swarm_execute` worker.

#[derive(serde::Serialize)]
pub(super) struct TaskResult {
    pub(super) task_id: String,
    pub(super) task_name: String,
    pub(super) success: bool,
    pub(super) output: String,
    pub(super) error: Option<String>,
    pub(super) steps: usize,
    pub(super) tool_calls: usize,
}

impl TaskResult {
    pub(super) fn failed(task_id: String, task_name: String, error: String) -> Self {
        Self {
            task_id,
            task_name,
            success: false,
            output: String::new(),
            error: Some(error),
            steps: 0,
            tool_calls: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TaskResult;

    #[test]
    fn failed_result_preserves_diagnostics() {
        let result = TaskResult::failed("task-1".into(), "inspect".into(), "boom".into());
        assert_eq!(result.task_id, "task-1");
        assert_eq!(result.task_name, "inspect");
        assert_eq!(result.error.as_deref(), Some("boom"));
        assert!(!result.success);
    }
}
