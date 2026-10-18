//! Native checks used around the shared runtime policy evaluator.
use crate::{RuntimeToolPolicy, ToolPolicyDecision};
use codetether_tool_core::ToolResult;
use serde_json::Value;
/// Application-bound prior-context and sandbox checks.
/// # Implementors
/// Hosts retain active-session lookup and operating-system sandbox probes.
pub trait Host {
    /// Return a context-access denial before evaluating configured policy.
    fn prior_context_block(tool_name: &str, args: &Value) -> Option<ToolResult>;
    /// Probe the sandbox only when requested by shared policy evaluation.
    fn sandbox(
        policy: &RuntimeToolPolicy,
        tool_name: &str,
        args: &Value,
    ) -> Option<ToolPolicyDecision>;
}
