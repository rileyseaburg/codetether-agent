//! Synthetic Kubernetes subtask failures.

use crate::swarm::SubTaskResult;

pub(super) fn result(id: &str, error: String, elapsed_ms: u64) -> SubTaskResult {
    SubTaskResult {
        subtask_id: id.into(),
        subagent_id: format!("agent-{id}"),
        success: false,
        result: String::new(),
        steps: 0,
        tool_calls: 0,
        execution_time_ms: elapsed_ms,
        error: Some(error),
        artifacts: Vec::new(),
        retry_count: 0,
    }
}
