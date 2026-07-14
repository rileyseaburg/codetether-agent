use crate::swarm::SubTaskResult;

pub(super) fn create() -> SubTaskResult {
    SubTaskResult {
        subtask_id: "task".into(),
        subagent_id: "agent".into(),
        success: true,
        result: "done".into(),
        steps: 1,
        tool_calls: 1,
        execution_time_ms: 1,
        error: None,
        artifacts: vec![],
        retry_count: 0,
    }
}
