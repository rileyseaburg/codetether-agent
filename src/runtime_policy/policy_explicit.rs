//! Terminal explicit-denial projection for specialized policy paths.

use super::{DecisionReason, RuntimeToolPolicy, ToolKind, ToolPolicyDecision, ToolPolicyOutcome};

pub(crate) fn denial(policy: &RuntimeToolPolicy, tool_name: &str) -> Option<ToolPolicyDecision> {
    super::permissions::tool_outcome(&policy.permissions, tool_name)
        .filter(|outcome| matches!(outcome, ToolPolicyOutcome::Deny))
        .map(|outcome| {
            ToolPolicyDecision::new(
                outcome,
                DecisionReason::MutatingTool,
                ToolKind::for_name(tool_name),
            )
        })
}
