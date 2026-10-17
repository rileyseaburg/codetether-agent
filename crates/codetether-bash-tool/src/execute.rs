//! Validate input and select the effective execution path.
use super::{BashTool, codetether_wrapped_command};
use super::{Call, ToolResult};
use anyhow::Result;
use serde_json::{Value, json};
use std::{path::PathBuf, time::Instant};
pub(super) async fn execute(tool: &BashTool, args: Value) -> Result<ToolResult> {
    let exec_start = Instant::now();

    let command = match args["command"].as_str() {
        Some(c) => c,
        None => {
            return Ok(ToolResult::structured_error(
                "INVALID_ARGUMENT",
                "bash",
                "command is required",
                Some(vec!["command"]),
                Some(json!({"command": "ls -la", "cwd": "."})),
            ));
        }
    };
    let cwd = args["cwd"].as_str().map(PathBuf::from);
    let effective_cwd = cwd.clone().or_else(|| tool.default_cwd.clone());
    let timeout_secs = args["timeout"].as_u64().unwrap_or(tool.timeout_secs);
    let wrapped_command = codetether_wrapped_command(command);

    if let Some(blocked) = super::validation::validate(command, &args) {
        return Ok(blocked);
    }
    let call = Call {
        command,
        cwd,
        effective_cwd,
        timeout_secs,
        wrapped_command,
        exec_start,
        args: &args,
    };
    let sandboxed =
        codetether_bash_policy::config::enabled_for_args(tool.sandboxed, command, &args).await;
    if sandboxed {
        super::sandbox::execute(call).await
    } else {
        let reason =
            codetether_bash_policy::config::unsafe_reason_for_args(tool.sandboxed, command, &args)
                .await;
        super::direct::execute(call, reason).await
    }
}
