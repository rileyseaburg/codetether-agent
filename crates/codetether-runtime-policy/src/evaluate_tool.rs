//! Configuration-backed evaluation when only a tool name is available.

use crate::RuntimeToolPolicy;
use crate::result_approval::blocking_result_with_approval_request;
use codetether_config::Config;
use codetether_tool_core::ToolResult;

/// Load default configuration and evaluate a tool without invocation arguments.
///
/// # Arguments
/// * `tool_name` — Tool identifier.
/// # Returns
/// A blocking result with an approval request when required, or `None`.
/// # Examples
/// ```no_run
/// # async fn example() {
/// assert!(codetether_runtime_policy::evaluate_tool::evaluate_tool("read").await.is_none());
/// # }
/// ```
pub async fn evaluate_tool(tool_name: &str) -> Option<ToolResult> {
    let config = Config::load().await.unwrap_or_default();
    evaluate_tool_with_config(&config, tool_name)
}

/// Evaluate a tool using supplied configuration, without invocation arguments.
///
/// # Arguments
/// * `config` — Effective configuration.
/// * `tool_name` — Tool identifier.
/// # Returns
/// A blocking result with an approval request when required, or `None`.
/// # Examples
/// ```no_run
/// let config = codetether_config::Config::default();
/// assert!(codetether_runtime_policy::evaluate_tool::evaluate_tool_with_config(&config, "read").is_none());
/// ```
pub fn evaluate_tool_with_config(config: &Config, tool_name: &str) -> Option<ToolResult> {
    let policy = RuntimeToolPolicy::from_config(config);
    blocking_result_with_approval_request(
        tool_name,
        policy.decide_tool(tool_name),
        "execute",
        tool_name,
    )
}
