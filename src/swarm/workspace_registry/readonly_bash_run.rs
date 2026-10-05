//! Execution of one verifier shell command with a read-only workspace.

use crate::tool::ToolResult;
use crate::tool::sandbox::{SandboxPolicy, execute_sandboxed, unavailable_reason};
use anyhow::Result;
use serde_json::Value;
use std::path::Path;

/// Run `args.command` under a sandbox policy with no writable paths.
///
/// Refuses (rather than running unconfined) when no sandbox backend exists.
///
/// # Errors
///
/// Propagates sandbox setup failures.
pub(super) async fn run(root: &Path, args: &Value) -> Result<ToolResult> {
    if let Some(reason) = unavailable_reason() {
        return Ok(ToolResult::error(format!(
            "read-only sandbox unavailable ({reason})"
        )));
    }
    let command = args
        .get("command")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let policy = SandboxPolicy {
        allow_exec: true,
        timeout_secs: 300,
        ..SandboxPolicy::default()
    };
    let argv = vec!["-c".to_string(), command.to_string()];
    let result = execute_sandboxed("bash", &argv, &policy, Some(root)).await?;
    Ok(ToolResult {
        output: result.output,
        success: result.success,
        metadata: Default::default(),
    })
}
