//! Pending approval display snapshot.

use super::ApprovalReport;
use codetether_approval_core::interaction::LiveApprovalRequest;
use codetether_approval_core::review::ReviewVerdict;

/// Snapshot of a pending request and its current display/review state.
/// # Examples
/// ```
/// let request = codetether_approval_core::interaction::LiveApprovalRequest::new("id".into(), "call".into(), "write".into(), "write".into(), "note".into(), "save".into());
/// let snapshot = codetether_approval_queue::ApprovalSnapshot::from(request);
/// assert_eq!(snapshot.id, "id");
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApprovalSnapshot {
    /// Stable approval request ID.
    pub id: String,
    /// Tool requesting approval.
    pub tool: String,
    /// Requested action.
    pub action: String,
    /// Requested resource.
    pub resource: String,
    /// Request reason shown to the user.
    pub reason: String,
    /// Optional model-supplied justification.
    pub justification: Option<String>,
    /// Optional preview text.
    pub preview: Option<String>,
    /// Rendered-row limit measured at the current popup dimensions.
    pub scroll_limit: u16,
    /// Original tool arguments.
    pub arguments: Option<serde_json::Value>,
    /// Latest source analysis report.
    pub report: ApprovalReport,
    /// Reviewer state: `None` = not requested, `Some(None)` = running.
    pub review: Option<Option<ReviewVerdict>>,
}

impl From<LiveApprovalRequest> for ApprovalSnapshot {
    fn from(request: LiveApprovalRequest) -> Self {
        Self {
            id: request.approval_id,
            tool: request.tool,
            action: request.action,
            resource: request.resource,
            reason: request.reason,
            justification: request.justification,
            preview: request.preview,
            scroll_limit: u16::MAX,
            arguments: request.arguments,
            report: ApprovalReport::default(),
            review: None,
        }
    }
}
