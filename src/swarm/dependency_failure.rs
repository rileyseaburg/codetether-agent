//! Prevent execution when a subtask prerequisite has failed.

use crate::swarm::{SubTask, SubTaskResult};
use std::collections::HashSet;

pub(super) fn partition(
    tasks: &[SubTask],
    failed: &HashSet<String>,
) -> (Vec<SubTask>, Vec<SubTaskResult>) {
    let mut runnable = Vec::new();
    let mut blocked = Vec::new();
    for task in tasks {
        let dependencies = task
            .dependencies
            .iter()
            .filter(|dependency| failed.contains(*dependency))
            .cloned()
            .collect::<Vec<_>>();
        if dependencies.is_empty() {
            runnable.push(task.clone());
        } else {
            blocked.push(blocked_result(task.clone(), dependencies));
        }
    }
    (runnable, blocked)
}

fn blocked_result(task: SubTask, dependencies: Vec<String>) -> SubTaskResult {
    SubTaskResult {
        subtask_id: task.id.clone(),
        subagent_id: format!("blocked-{}", task.id),
        success: false,
        result: String::new(),
        steps: 0,
        tool_calls: 0,
        execution_time_ms: 0,
        error: Some(format!(
            "Blocked by failed dependencies: {}",
            dependencies.join(", ")
        )),
        artifacts: Vec::new(),
        retry_count: 0,
    }
}

#[cfg(test)]
#[path = "dependency_failure_tests.rs"]
mod tests;
