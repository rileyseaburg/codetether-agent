//! Decisions returned to a paused invocation by an approval client.

/// Live decision returned to a paused tool call.
///
/// # Examples
/// ```
/// use codetether_approval_core::interaction::LiveApprovalDecision;
/// match LiveApprovalDecision::denied() {
///     LiveApprovalDecision::Denied { reason } => assert!(reason.is_none()),
///     other => panic!("unexpected decision: {other:?}"),
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiveApprovalDecision {
    /// Resume the original tool call.
    Approved,
    /// Return an approval-denied tool result, optionally carrying the
    /// user's reason so it reaches the agent as the tool result.
    Denied { reason: Option<String> },
    /// Resume with a user-edited argument patch. The approval gate merges
    /// these fields into the original tool arguments before execution.
    Revised {
        arguments: serde_json::Value,
        approval_id: String,
    },
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
