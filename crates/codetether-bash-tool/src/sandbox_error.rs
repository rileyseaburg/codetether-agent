//! Sandbox failure telemetry and metadata.
use super::bash_sandbox_error_metadata;
use super::{Call, ToolResult};
use anyhow::Result;
use codetether_sandbox_policy::SandboxPolicy;
use codetether_telemetry::{TOOL_EXECUTIONS, ToolExecution, record_persistent};
use serde_json::json;
use std::path::Path;
pub(super) fn finish(
    call: &Call<'_>,
    policy: &SandboxPolicy,
    work_dir: &Path,
    e: anyhow::Error,
) -> Result<ToolResult> {
    let duration = call.exec_start.elapsed();
    let exec = ToolExecution::start(
        "bash",
        json!({ "command": call.command, "sandboxed": true }),
    )
    .complete_error(e.to_string(), duration);
    TOOL_EXECUTIONS.record(exec.success);
    let data = serde_json::json!({
        "tool": "bash",
        "command": call.command,
        "success": false,
        "error": e.to_string(),
    });
    let _ = record_persistent("tool_execution", &data);
    Ok(bash_sandbox_error_metadata::sandbox_error(
        ToolResult::error(format!("Sandbox error: {}", e)),
        &policy,
        work_dir,
        e.to_string(),
    ))
}
