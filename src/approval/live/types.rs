use serde::{Deserialize, Serialize};

use crate::approval::{ExecPolicyAmendment, ReviewDecision};

/// Live approval request emitted while a tool call is paused.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LiveApprovalRequest {
    /// Approval id that must be answered by the UI.
    pub approval_id: String,
    /// Provider tool-call id associated with this request.
    pub tool_call_id: String,
    /// Tool awaiting approval.
    pub tool: String,
    /// Requested action, for example `execute` or `write`.
    pub action: String,
    /// Resource governed by the approval.
    pub resource: String,
    /// Human-readable reason shown to the user.
    pub reason: String,
    /// Proposed exec-policy amendment that can allow similar commands.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proposed_execpolicy_amendment: Option<ExecPolicyAmendment>,
    /// Ordered decisions a client can present for this approval.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub available_decisions: Vec<ReviewDecision>,
}

/// Live decision returned to a paused tool call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiveApprovalDecision {
    /// Resume the original tool call.
    Approved,
    /// Return an approval-denied tool result, optionally carrying the
    /// user's reason so it reaches the agent as the tool result.
    Denied { reason: Option<String> },
}

impl LiveApprovalDecision {
    /// Convenience constructor for a denial without a reason.
    pub fn denied() -> Self {
        Self::Denied { reason: None }
    }

    /// Convenience constructor for a denial that carries a reason.
    pub fn denied_with(reason: impl Into<String>) -> Self {
        let reason = reason.into();
        let reason = reason.trim().to_string();
        Self::Denied {
            reason: (!reason.is_empty()).then_some(reason),
        }
    }
}
