//! Structured result handling for Kubernetes swarm sub-agents.

use super::{kubernetes_executor::result_from_logs, subtask::SubTaskResult};

/// Builds the parent-facing result for a finished Kubernetes sub-agent pod.
pub fn final_result(
    subtask_id: &str,
    elapsed_ms: u64,
    exit_code: Option<i32>,
    reason: Option<&str>,
    logs: &str,
) -> SubTaskResult {
    if let Some(mut result) = result_from_logs(logs) {
        if result.execution_time_ms == 0 {
            result.execution_time_ms = elapsed_ms;
        }
        return result;
    }
    let missing = exit_code == Some(127);
    let error = if missing {
        "Kubernetes sub-agent runner missing: image lacks codetether swarm-subagent support"
            .to_string()
    } else {
        format!(
            "Kubernetes sub-agent did not emit structured result; exit={}; reason={}",
            exit_code
                .map(|code| code.to_string())
                .unwrap_or_else(|| "unknown".to_string()),
            reason.unwrap_or("unknown")
        )
    };
    SubTaskResult {
        subtask_id: subtask_id.to_string(),
        subagent_id: format!("agent-{subtask_id}"),
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
