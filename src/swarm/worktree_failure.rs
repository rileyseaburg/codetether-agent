//! Failed result construction for required worktree preflight.

use crate::swarm::{SubTask, SubTaskResult};

pub(super) fn required(subtask: &SubTask) -> SubTaskResult {
    let error = "Failed to create required isolated worktree".to_string();
    SubTaskResult {
        subtask_id: subtask.id.clone(),
        subagent_id: "worktree-preflight".to_string(),
        success: false,
        result: error.clone(),
        steps: 0,
        tool_calls: 0,
        execution_time_ms: 0,
        error: Some(error),
        artifacts: Vec::new(),
        retry_count: 0,
    }
}
