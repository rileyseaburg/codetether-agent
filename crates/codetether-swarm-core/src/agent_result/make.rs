use super::*;

pub(super) fn failed(
    spec: &ExecutionSpec<'_>,
    error: String,
    retries: u32,
    elapsed: Duration,
) -> (SubTaskResult, SubTaskStatus) {
    (
        make(
            spec,
            false,
            String::new(),
            0,
            0,
            Some(error),
            retries,
            elapsed,
        ),
        SubTaskStatus::Failed,
    )
}

pub(super) fn make(
    spec: &ExecutionSpec<'_>,
    success: bool,
    output: String,
    steps: usize,
    tools: usize,
    error: Option<String>,
    retries: u32,
    elapsed: Duration,
) -> SubTaskResult {
    SubTaskResult {
        subtask_id: spec.subtask_id.to_string(),
        subagent_id: format!("agent-{}", spec.subtask_id),
        success,
        result: output,
        steps,
        tool_calls: tools,
        execution_time_ms: elapsed.as_millis() as u64,
        error,
        artifacts: Vec::new(),
        retry_count: retries,
    }
}
