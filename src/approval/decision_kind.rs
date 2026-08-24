#[path = "decision_kind_session.rs"]
mod session;

use super::{ApprovalStatus, LiveApprovalDecision};
use serde::{Deserialize, Serialize};

/// Parsed review decision accepted by approval clients.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
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

    pub fn live(self, reason: Option<&str>) -> LiveApprovalDecision {
        if self.approves() {
            LiveApprovalDecision::Approved
        } else {
            reason.map_or_else(
                LiveApprovalDecision::denied,
                LiveApprovalDecision::denied_with,
            )
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
}
