//! Review decision parsing and application to shared approval state.

use super::LiveApprovalDecision;
use crate::{ApprovalReceipt, ApprovalStatus};

/// Parsed review decision accepted by approval clients.
///
/// Variants approve once, approve for the session, approve with an amendment,
/// or deny. Applying a decision is separate from parsing it.
///
/// # Examples
/// ```
/// use codetether_approval_core::interaction::ApprovalDecisionKind;
/// assert_eq!(ApprovalDecisionKind::parse("approve").unwrap(), ApprovalDecisionKind::ApproveOnce);
/// assert!(!ApprovalDecisionKind::Deny.approves());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApprovalDecisionKind {
    ApproveOnce,
    ApproveForSession,
    ApproveWithAmendment,
    Deny,
}

impl ApprovalDecisionKind {
    /// Parse local and Codex-compatible decision names.
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "approve" | "approved" => Ok(Self::ApproveOnce),
            "approve_for_session" | "approved_for_session" => Ok(Self::ApproveForSession),
            "approved_with_amendment" | "approved_execpolicy_amendment" => {
                Ok(Self::ApproveWithAmendment)
            }
            "deny" | "denied" | "abort" | "timed_out" => Ok(Self::Deny),
            _ => Err("decision must be approve, approved_for_session, amendment, or deny".into()),
        }
    }

    /// Convert this choice into a live approve/deny response.
    pub fn live(self) -> LiveApprovalDecision {
        if self.approves() {
            LiveApprovalDecision::Approved
        } else {
            LiveApprovalDecision::denied()
        }
    }

    /// Return the persistent approval status for this choice.
    pub fn status(self) -> ApprovalStatus {
        match self {
            Self::ApproveOnce | Self::ApproveForSession | Self::ApproveWithAmendment => {
                ApprovalStatus::Approved
            }
            Self::Deny => ApprovalStatus::Denied,
        }
    }

    /// Return whether this choice authorizes the requested operation.
    pub fn approves(self) -> bool {
        !matches!(self, Self::Deny)
    }

    /// Apply session tuple and command grants for the two session-wide choices.
    ///
    /// # Panics
    /// Panics if either session-grant mutex was poisoned.
    pub fn grant_session(self, receipt: &ApprovalReceipt) {
        if matches!(self, Self::ApproveForSession | Self::ApproveWithAmendment) {
            crate::session_grants::grant(receipt);
            crate::session_command_grants::grant_for_request(&receipt.approval_id);
        }
    }
}
