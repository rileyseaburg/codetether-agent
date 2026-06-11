//! Stable string codes for runtime policy metadata.

use super::{DecisionReason, ToolPolicyOutcome};

pub(super) fn outcome(outcome: ToolPolicyOutcome) -> &'static str {
    match outcome {
        ToolPolicyOutcome::Allow => "allow",
        ToolPolicyOutcome::RequireApproval => "require_approval",
        ToolPolicyOutcome::Deny => "deny",
    }
}

pub(super) fn reason(reason: DecisionReason) -> &'static str {
    match reason {
        DecisionReason::ReadOnlyTool => "read_only_tool",
        DecisionReason::ReadOnlyCommand => "read_only_command",
        DecisionReason::PermissionProfileDisabled => "permission_profile_disabled",
        DecisionReason::TrustedNeverApproval => "trusted_never_approval",
        DecisionReason::ApprovalPolicyAllows => "approval_policy_allows",
        DecisionReason::SessionApproval => "session_approval",
        DecisionReason::MutatingTool => "mutating_tool",
        DecisionReason::SandboxUnavailable => "sandbox_unavailable",
        DecisionReason::ApprovalUnavailable => "approval_unavailable",
    }
}
