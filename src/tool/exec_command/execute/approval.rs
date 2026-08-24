//! Final sandbox and exact-authority validation before command spawn.

use crate::tool::{ToolResult, sandbox::SandboxPolicy};
use serde_json::Value;
use std::path::Path;

pub(super) async fn validate(
    args: &Value,
    command: (&str, &[String]),
    policy: Option<&SandboxPolicy>,
    cwd: &Path,
) -> Result<(), ToolResult> {
    if super::super::policy::direct_disallowed(args, policy) {
        return Err(ToolResult::error(
            "direct exec_command requires exact approval and network authority",
        ));
    }
    if let Some(policy) = policy {
        crate::tool::sandbox::sandbox_preflight::validate(command.0, command.1, policy, cwd)
            .await
            .map_err(|error| ToolResult::error(format!("sandbox preflight failed: {error}")))?;
    }
    crate::approval::use_once::claim("exec_command", args)
        .map_err(|error| ToolResult::error(format!("approval claim failed: {error}")))
}