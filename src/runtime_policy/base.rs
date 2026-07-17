//! Broad runtime policy decisions after explicit rules.

use super::{DecisionReason, ToolKind, ToolPolicyOutcome};
use crate::config::{ApprovalPolicy, PermissionProfile};

pub(super) fn decide(
    trusted: bool,
    approval: ApprovalPolicy,
    profile: PermissionProfile,
    kind: ToolKind,
) -> (ToolPolicyOutcome, DecisionReason) {
    if matches!(kind, ToolKind::ReadOnly) {
        return (ToolPolicyOutcome::Allow, DecisionReason::ReadOnlyTool);
    }
    if matches!(kind, ToolKind::SessionTransport) {
        return (ToolPolicyOutcome::Allow, DecisionReason::SessionApproval);
    }
    if matches!(profile, PermissionProfile::Disabled) {
        return (
            ToolPolicyOutcome::Deny,
            DecisionReason::PermissionProfileDisabled,
        );
    }
    match (trusted, approval) {
        (true, ApprovalPolicy::Never) => (
            ToolPolicyOutcome::Allow,
            DecisionReason::TrustedNeverApproval,
        ),
        (false, ApprovalPolicy::Never) => (
            ToolPolicyOutcome::Allow,
            DecisionReason::ApprovalPolicyAllows,
        ),
        (_, ApprovalPolicy::OnFailure) => (
            ToolPolicyOutcome::Allow,
            DecisionReason::ApprovalPolicyAllows,
        ),
        (_, ApprovalPolicy::OnRequest | ApprovalPolicy::Untrusted) => (
            ToolPolicyOutcome::RequireApproval,
            DecisionReason::MutatingTool,
        ),
    }
}
