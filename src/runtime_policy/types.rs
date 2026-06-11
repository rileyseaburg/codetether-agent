//! Public decision payloads for runtime tool policy.

use super::ToolKind;

/// Final policy result for a tool invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolPolicyOutcome {
    /// The invocation can proceed without an approval prompt.
    Allow,
    /// The caller should obtain approval before running the tool.
    RequireApproval,
    /// The invocation must not run.
    Deny,
}

/// Stable reason code explaining why an outcome was selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionReason {
    /// Known read-only tools are safe to run.
    ReadOnlyTool,
    /// A shell command matched a read-only command policy.
    ReadOnlyCommand,
    /// The permission profile disables mutating tool execution.
    PermissionProfileDisabled,
    /// The project is trusted and approval prompts are disabled.
    TrustedNeverApproval,
    /// The approval policy allows a best-effort run without pre-approval.
    ApprovalPolicyAllows,
    /// The invocation matched a user-approved session command prefix.
    SessionApproval,
    /// The invocation can mutate state and needs approval.
    MutatingTool,
    /// The OS sandbox runner is unavailable, so direct execution needs approval.
    SandboxUnavailable,
    /// Approval cannot be requested for an untrusted mutating invocation.
    ApprovalUnavailable,
}

/// Structured decision returned by [`crate::runtime_policy::RuntimeToolPolicy`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToolPolicyDecision {
    /// Final runtime outcome.
    pub outcome: ToolPolicyOutcome,
    /// Stable reason for the outcome.
    pub reason: DecisionReason,
    /// Side-effect classification of the requested tool.
    pub tool_kind: ToolKind,
}

impl ToolPolicyDecision {
    pub(crate) fn new(
        outcome: ToolPolicyOutcome,
        reason: DecisionReason,
        tool_kind: ToolKind,
    ) -> Self {
        Self {
            outcome,
            reason,
            tool_kind,
        }
    }
}
