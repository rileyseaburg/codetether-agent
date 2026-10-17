//! Convert permission decisions into structured tool errors without side effects.

use crate::{ToolPolicyDecision, ToolPolicyOutcome, code};
use codetether_tool_core::ToolResult;
use serde_json::json;

/// Build the blocking result for a denied or approval-gated invocation.
///
/// # Arguments
/// * `tool_name` — Tool identifier for diagnostics.
/// * `decision` — Evaluated policy decision.
/// # Returns
/// A structured error, or `None` if the invocation is allowed.
/// # Examples
/// ```
/// use codetether_runtime_policy::{blocking, DecisionReason, ToolKind, ToolPolicyDecision, ToolPolicyOutcome};
/// let decision = ToolPolicyDecision::new(ToolPolicyOutcome::Allow, DecisionReason::ReadOnlyTool, ToolKind::ReadOnly);
/// assert!(blocking::blocking_result("read", decision).is_none());
/// ```

pub fn blocking_result(tool_name: &str, decision: ToolPolicyDecision) -> Option<ToolResult> {
    let (code, message) = match decision.outcome {
        ToolPolicyOutcome::Allow => return None,
        ToolPolicyOutcome::RequireApproval => (
            "TOOL_APPROVAL_REQUIRED",
            "Tool execution requires approval before it can run.",
        ),
        ToolPolicyOutcome::Deny => ("TOOL_POLICY_DENIED", "Tool execution was denied by policy."),
    };
    Some(
        ToolResult::structured_error(code, tool_name, message, None, None)
            .with_metadata("policy_outcome", json!(code::outcome(decision.outcome)))
            .with_metadata("policy_reason", json!(code::reason(decision.reason))),
    )
}
