//! Noninteractive command admission checks.
use super::{bash_interactive, interactive_auth_risk_reason};
use codetether_tool_core::ToolResult;
use serde_json::Value;
pub(super) fn validate(command: &str, args: &Value) -> Option<ToolResult> {
    if let Some(blocked) = codetether_command_guard::result_for_args("bash", args) {
        return Some(blocked);
    }

    if let Some(reason) = bash_interactive::shell_reason(command) {
        return Some(ToolResult::error(format!(
            "{reason} Use a noninteractive command such as `bash -c '...'`."
        )));
    }
    if let Some(reason) = interactive_auth_risk_reason(command) {
        tracing::warn!(target: "codetether_agent::tool::bash", reason, "Interactive auth risk detected");
    }

    None
}
