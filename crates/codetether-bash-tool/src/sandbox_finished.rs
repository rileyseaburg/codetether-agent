//! Successful sandbox execution telemetry and tool result assembly.
use super::bash_sandbox_metadata;
use super::{Call, ToolResult};
use anyhow::Result;
use codetether_sandbox_policy::{SandboxPolicy, SandboxResult};
use codetether_telemetry::{TOOL_EXECUTIONS, ToolExecution, record_persistent};
use serde_json::json;
use std::path::Path;
pub(super) fn finish(
    call: &Call<'_>,
    policy: &SandboxPolicy,
    work_dir: &Path,
    r: SandboxResult,
) -> Result<ToolResult> {
    let duration = call.exec_start.elapsed();
    let exec = ToolExecution::start(
        "bash",
        json!({ "command": call.command, "sandboxed": true }),
    );
    let exec = if r.success {
        exec.complete_success(format!("exit_code={:?}", r.exit_code), duration)
    } else {
        exec.complete_error(format!("exit_code={:?}", r.exit_code), duration)
    };
    TOOL_EXECUTIONS.record(exec.success);
    let data = serde_json::json!({
        "tool": "bash",
        "command": call.command,
        "success": r.success,
        "exit_code": r.exit_code,
    });
    let _ = record_persistent("tool_execution", &data);

    Ok(bash_sandbox_metadata::sandbox_result(&policy, work_dir, r))
}
