//! TUI approval command intents.

use crate::approval::LiveApprovalDecision;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ApprovalIntent {
    ApproveOnce,
    ApproveForSession,
    Deny,
    Abort,
}

impl ApprovalIntent {
    pub(super) fn approves(self) -> bool {
        matches!(self, Self::ApproveOnce | Self::ApproveForSession)
    }

    pub(super) fn session_scoped(self) -> bool {
        matches!(self, Self::ApproveForSession)
    }

    pub(super) fn live_decision_with_reason(self, reason: &str) -> LiveApprovalDecision {
        if self.approves() {
            LiveApprovalDecision::Approved
        } else {
            LiveApprovalDecision::denied_with(reason)
        }
    }

    pub(super) fn fallback_reason(self) -> &'static str {
        match self {
            Self::ApproveOnce => "approved once from TUI",
            Self::ApproveForSession => "approved for session from TUI",
            Self::Deny => "denied from TUI",
            Self::Abort => "aborted from TUI",
        }
    }

    pub(super) fn label(self) -> &'static str {
        match self {
            Self::ApproveOnce => "Approved once",
            Self::ApproveForSession => "Approved for session",
            Self::Deny => "Denied",
            Self::Abort => "Aborted",
        }
    }
}
