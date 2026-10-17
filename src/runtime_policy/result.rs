//! Convert runtime policy decisions into tool results.

#[path = "result_approval.rs"]
mod approval_result;

pub use approval_result::blocking_result_with_approval_request;
pub(in crate::runtime_policy) use approval_result::blocking_result_with_approval_request_for_args;

use super::RuntimeToolPolicy;
use crate::config::Config;
use crate::tool::ToolResult;
pub use codetether_runtime_policy::blocking::blocking_result;

pub async fn evaluate_tool(tool_name: &str) -> Option<ToolResult> {
    let config = Config::load().await.unwrap_or_default();
    evaluate_tool_with_config(&config, tool_name)
}

pub fn evaluate_tool_with_config(config: &Config, tool_name: &str) -> Option<ToolResult> {
    let policy = RuntimeToolPolicy::from_config(config);
    blocking_result_with_approval_request(
        tool_name,
        policy.decide_tool(tool_name),
        "execute",
        tool_name,
    )
}
