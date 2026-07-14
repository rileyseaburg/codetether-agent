//! Tests for parent-facing swarm result aggregation.

use super::results;
use crate::swarm::SubTaskResult;

fn result(success: bool, output: &str, error: Option<&str>) -> SubTaskResult {
    SubTaskResult {
        subtask_id: "task".into(),
        subagent_id: "agent".into(),
        success,
        result: output.into(),
        steps: 1,
        tool_calls: 0,
        execution_time_ms: 1,
        error: error.map(str::to_string),
        artifacts: Vec::new(),
        retry_count: 0,
    }
}

#[test]
fn labels_successes_and_failures() {
    let output = results(&[
        result(true, "complete", None),
        result(false, "", Some("broken")),
    ]);
    assert!(output.contains("=== Subtask 1 ===\ncomplete"));
    assert!(output.contains("=== Subtask 2 (FAILED) ===\nError: broken"));
}
