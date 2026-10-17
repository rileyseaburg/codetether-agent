//! Record direct-command completion telemetry.
use super::Call;
use codetether_telemetry::{TOOL_EXECUTIONS, ToolExecution, record_persistent};
use serde_json::json;

pub(super) fn record(
    call: &Call<'_>,
    success: bool,
    exit_code: i32,
    source_len: usize,
    output_str: &str,
) {
    let duration = call.exec_start.elapsed();

    // Record telemetry
    let exec = ToolExecution::start(
        "bash",
        json!({
            "command": call.command,
            "cwd": call.effective_cwd
                .as_ref()
                .map(|dir| dir.display().to_string()),
            "timeout": call.timeout_secs,
        }),
    );
    let exec = if success {
        exec.complete_success(
            format!("exit_code={}, output_len={}", exit_code, source_len),
            duration,
        )
    } else {
        exec.complete_error(
            format!(
                "exit_code={}: {}",
                exit_code,
                output_str.lines().next().unwrap_or("(no output)")
            ),
            duration,
        )
    };
    TOOL_EXECUTIONS.record(exec.success);
    let _ = record_persistent(
        "tool_execution",
        &serde_json::to_value(&exec).unwrap_or_default(),
    );
}
