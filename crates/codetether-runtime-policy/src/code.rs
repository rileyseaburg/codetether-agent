//! Stable string codes for runtime policy metadata.

use super::{DecisionReason, ToolPolicyOutcome};

/// Return the stable metadata spelling of an outcome.
///
/// # Arguments
/// * `outcome` — Structured policy outcome.
/// # Returns
/// Its stable string code.
/// # Examples
/// ```
/// use codetether_runtime_policy::{code, ToolPolicyOutcome};
/// assert_eq!(code::outcome(ToolPolicyOutcome::Allow), "allow");
/// ```
pub fn outcome(outcome: ToolPolicyOutcome) -> &'static str {
    match outcome {
        ToolPolicyOutcome::Allow => "allow",
        ToolPolicyOutcome::RequireApproval => "require_approval",
        ToolPolicyOutcome::Deny => "deny",
    }
}

/// Return the stable metadata spelling of a decision reason.
///
/// # Arguments
/// * `reason` — Structured policy reason.
/// # Returns
/// Its stable string code.
/// # Examples
/// ```
/// use codetether_runtime_policy::{code, DecisionReason};
/// assert_eq!(code::reason(DecisionReason::ReadOnlyTool), "read_only_tool");
/// ```
pub fn reason(reason: DecisionReason) -> &'static str {
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
