//! Review request data transported by an application to an approval client.

use super::{ExecPolicyAmendment, ReviewDecision};
use serde::{Deserialize, Serialize};

/// Live approval request emitted while a tool call is paused.
///
/// # Examples
/// ```
/// use codetether_approval_core::interaction::LiveApprovalRequest;
/// let request = LiveApprovalRequest::new("id".into(), "call".into(), "write".into(), "write".into(), "note.txt".into(), "save requested notes".into());
/// assert_eq!(request.approval_id, "id");
/// ```
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
    /// Model-supplied justification explaining why the action is needed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
    /// Full invocation detail shown in interactive approval clients.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
    /// Raw tool arguments for file mutations so clients can run diagnostics
    /// against the proposed content before deciding.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<serde_json::Value>,
    /// Proposed exec-policy amendment that can allow similar commands.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proposed_execpolicy_amendment: Option<ExecPolicyAmendment>,
    /// Ordered decisions a client can present for this approval.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub available_decisions: Vec<ReviewDecision>,
}
