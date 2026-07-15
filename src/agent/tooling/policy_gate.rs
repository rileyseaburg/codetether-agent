//! Runtime policy gate for agent tool dispatch.

#[cfg(test)]
use crate::config::Config;
use crate::tool::ToolResult;
use serde_json::Value;

pub(super) async fn blocked(tool_name: &str, args: &Value) -> Option<ToolResult> {
    crate::runtime_policy::evaluate_tool_invocation(tool_name, args).await
}

#[cfg(test)]
pub(super) fn blocked_with_config(config: &Config, tool_name: &str) -> Option<ToolResult> {
    crate::runtime_policy::evaluate_tool_with_config(config, tool_name)
}

#[cfg(test)]
#[path = "policy_gate_tests.rs"]
mod tests;
