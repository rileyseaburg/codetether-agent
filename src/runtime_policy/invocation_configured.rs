//! Session prior-context guard and host sandbox adapter for shared policy.

use crate::config::Config;
use crate::tool::ToolResult;
use serde_json::Value;

pub fn evaluate_tool_invocation_with_config(
    config: &Config,
    tool_name: &str,
    args: &Value,
) -> Option<ToolResult> {
    if let Some(blocked) =
        crate::session::helper::runtime::block_prior_context_from_runtime(tool_name, args)
    {
        return Some(blocked);
    }
    codetether_runtime_policy::evaluate::evaluate_tool_invocation_with_config(
        config,
        tool_name,
        args,
        |policy| super::super::sandbox_preflight::decision(policy, tool_name, args),
    )
}
