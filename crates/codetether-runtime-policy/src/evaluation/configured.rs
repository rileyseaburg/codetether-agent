//! Prior-context-first configured invocation evaluation.
use super::Host;
use codetether_config::Config;
use codetether_tool_core::ToolResult;
use serde_json::Value;
/// Evaluate a tool using existing configuration and native hooks.
/// # Arguments
/// Receives configuration, tool name and original arguments.
/// # Returns
/// A blocking result, or None when execution may proceed.
/// # Examples
/// ```no_run
/// fn check<H: codetether_runtime_policy::evaluation::Host>(config: &codetether_config::Config) {
///     let _ = codetether_runtime_policy::evaluation::with_config::<H>(config, "read", &serde_json::json!({}));
/// }
/// ```
pub fn with_config<H: Host>(config: &Config, tool_name: &str, args: &Value) -> Option<ToolResult> {
    if let Some(blocked) = H::prior_context_block(tool_name, args) {
        return Some(blocked);
    }
    crate::evaluate::evaluate_tool_invocation_with_config(config, tool_name, args, |policy| {
        H::sandbox(policy, tool_name, args)
    })
}
