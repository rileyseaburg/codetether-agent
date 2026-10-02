//! Credential-free approval correlation and effective isolation diagnostics.

use crate::tool::{ToolResult, sandbox::SandboxPolicy};
use serde_json::{Value, json};
use std::path::Path;

fn fields(args: &Value, mode: &str) -> Value {
    let (action, resource) = crate::runtime_policy::approval_binding("exec_command", args);
    json!({"approval_id": args.get("approval_id").and_then(Value::as_str),
        "approval_action": action, "approval_resource": resource,
        "effective_sandbox_mode": mode,
        "sandbox_permissions": args.get("sandbox_permissions").and_then(Value::as_str)
            .unwrap_or("use_default")})
}

pub(super) fn escalation_error(args: &Value) -> ToolResult {
    let diagnostics = fields(args, "not-started");
    let mut result = ToolResult::structured_error(
        "SANDBOX_ESCALATION_NOT_AUTHORIZED",
        "exec_command",
        "sandbox escalation requires an approved exec_command invocation",
        None,
        None,
    );
    result
        .output
        .push_str(&format!("\nExecution diagnostics: {diagnostics}"));
    result
        .metadata
        .insert("execution_diagnostics".into(), diagnostics);
    result
}

pub(super) fn execution(args: &Value, cwd: &Path, policy: Option<&SandboxPolicy>) -> Value {
    let mode = match policy {
        Some(policy) if policy.allowed_paths.is_empty() => "read-only",
        Some(_) => "workspace-write",
        None => "unsandboxed",
    };
    let diagnostics = fields(args, mode);
    tracing::info!(tool = "exec_command", approval_id = ?diagnostics["approval_id"],
        approval_resource = %diagnostics["approval_resource"], effective_sandbox_mode = mode,
        cwd = %cwd.display(), "Resolved command execution authority");
    diagnostics
}

#[cfg(test)]
#[path = "diagnostics_tests.rs"]
mod tests;
