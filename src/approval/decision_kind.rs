use super::{ApprovalReceipt, ApprovalStatus, LiveApprovalDecision};

/// Parsed review decision accepted by approval clients.
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

    pub fn live(self) -> LiveApprovalDecision {
        if self.approves() {
            LiveApprovalDecision::Approved
        } else {
            LiveApprovalDecision::Denied
        }
    }

    pub fn status(self) -> ApprovalStatus {
        match self {
            Self::ApproveOnce | Self::ApproveForSession | Self::ApproveWithAmendment => {
                ApprovalStatus::Approved
            }
            Self::Deny => ApprovalStatus::Denied,
        }
    }

    pub fn approves(self) -> bool {
        !matches!(self, Self::Deny)
    }

    pub fn grant_session(self, receipt: &ApprovalReceipt) {
        if matches!(self, Self::ApproveForSession | Self::ApproveWithAmendment) {
            super::session_grants::grant(receipt);
            super::session_command_grants::grant_for_request(&receipt.approval_id);
        }
    }
}
