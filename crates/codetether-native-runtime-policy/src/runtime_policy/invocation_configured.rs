//! Native specialization of configured invocation policy.
use crate::{config::Config, tool::ToolResult};
use serde_json::Value;
pub fn evaluate_tool_invocation_with_config(
    config: &Config,
    tool_name: &str,
    args: &Value,
) -> Option<ToolResult> {
    codetether_runtime_policy::evaluation::with_config::<super::host::Native>(
        config, tool_name, args,
    )
}
