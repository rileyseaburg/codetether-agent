//! Native session-context and sandbox probes for shared invocation evaluation.
use crate::{
    runtime_policy::{RuntimeToolPolicy, ToolPolicyDecision},
    tool::ToolResult,
};
use serde_json::Value;
pub(super) struct Native;
impl codetether_runtime_policy::evaluation::Host for Native {
    fn prior_context_block(tool_name: &str, args: &Value) -> Option<ToolResult> {
        codetether_prior_context_access::runtime_context(tool_name, args)
    }
    fn sandbox(
        policy: &RuntimeToolPolicy,
        tool_name: &str,
        args: &Value,
    ) -> Option<ToolPolicyDecision> {
        super::super::sandbox_preflight::decision(policy, tool_name, args)
    }
}
