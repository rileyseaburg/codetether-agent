//! Record process-start failures and return a structured error.
use super::bash_unsandboxed_metadata;
use super::{Call, ToolResult};
use anyhow::Result;
use codetether_telemetry::{TOOL_EXECUTIONS, ToolExecution, record_persistent};
use serde_json::json;
pub(super) fn finish(
    call: &Call<'_>,
    unsafe_reason: &str,
    e: std::io::Error,
) -> Result<ToolResult> {
    let duration = call.exec_start.elapsed();
    let exec = ToolExecution::start(
        "bash",
        json!({
            "command": call.command,
            "cwd": call.cwd,
        }),
    )
    .complete_error(format!("Failed to execute: {}", e), duration);
    TOOL_EXECUTIONS.record(exec.success);
    let _ = record_persistent(
        "tool_execution",
        &serde_json::to_value(&exec).unwrap_or_default(),
    );

    Ok(bash_unsandboxed_metadata::mark(
        ToolResult::structured_error(
            "EXECUTION_FAILED",
            "bash",
            &format!("Failed to execute command: {}", e),
            None,
            Some(json!({"command": call.command})),
        ),
        call.effective_cwd.as_deref(),
        unsafe_reason,
    ))
}
