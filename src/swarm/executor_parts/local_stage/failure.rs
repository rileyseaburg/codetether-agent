//! Synthetic results for local task and join failures.

use crate::swarm::SubTaskResult;

pub(super) fn result(id: String, error: String) -> SubTaskResult {
    SubTaskResult {
        subtask_id: id.clone(),
        subagent_id: format!("agent-{id}"),
        success: false,
        result: String::new(),
        steps: 0,
        tool_calls: 0,
        execution_time_ms: 0,
        error: Some(error),
        artifacts: Vec::new(),
        retry_count: 0,
    }
}
