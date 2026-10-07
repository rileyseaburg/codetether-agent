//! Action-aware default decision for one tool invocation.

use super::{DecisionReason, RuntimeToolPolicy, ToolKind, ToolPolicyDecision, ToolPolicyOutcome};
use serde_json::Value;

impl RuntimeToolPolicy {
    /// Decide like [`RuntimeToolPolicy::decide_tool`], but allow invocations
    /// whose `action` only reads state (e.g. `session_task` `list`).
    ///
    /// Explicit `[permissions]` rules for the tool still take precedence.
    pub(crate) fn decide_tool_action(&self, tool_name: &str, args: &Value) -> ToolPolicyDecision {
        let decision = self.decide_tool(tool_name);
        if !self.has_tool_rule(tool_name)
            && matches!(decision.outcome, ToolPolicyOutcome::RequireApproval)
            && super::action_read_only::allowed(tool_name, args)
        {
            return ToolPolicyDecision::new(
                ToolPolicyOutcome::Allow,
                DecisionReason::ReadOnlyTool,
                ToolKind::ReadOnly,
            );
        }
        decision
    }

    fn has_tool_rule(&self, tool_name: &str) -> bool {
        super::permissions::tool_outcome(&self.permissions, tool_name).is_some()
    }
}
