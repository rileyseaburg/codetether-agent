//! Existing cache behavior checks, using an explicit temporary directory.
use super::*;
use tempfile::tempdir;
mod basic_operations;
mod bypass;
mod clear;
mod different_tasks;
mod disk_removal;
mod failed_results_not_cached;

fn create_test_task(name: &str, instruction: &str) -> SubTask {
    SubTask::new(name, instruction)
}

fn create_test_result(success: bool) -> SubTaskResult {
    SubTaskResult {
        subtask_id: "test-123".to_string(),
        subagent_id: "agent-123".to_string(),
        success,
        result: "test result".to_string(),
        steps: 5,
        tool_calls: 3,
        execution_time_ms: 1000,
        error: None,
        artifacts: vec![],
        retry_count: 0,
    }
}
