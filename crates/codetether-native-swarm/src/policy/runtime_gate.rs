//! Native runtime authorization for autonomous tool calls.
/// Return the existing policy denial text for a tool invocation.
/// # Arguments
/// * `tool_name` - Tool requested by the sub-agent.
/// * `args` - Arguments evaluated by the native runtime policy.
/// # Returns
/// A denial message, or None when the invocation may proceed.
/// # Examples
/// `runtime_denial(tool, args).await` preserves the shared policy decision.
pub async fn runtime_denial(tool_name: &str, args: &serde_json::Value) -> Option<String> {
    codetether_native_runtime_policy::evaluate_tool_invocation(tool_name, args)
        .await
        .map(|result| result.output)
}
