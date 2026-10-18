//! TUI approval command intents.

use crate::interaction::LiveApprovalDecision;

/// Command decision, retaining separate labels for denial and abort.
///
/// # Examples
/// ```
/// use codetether_approval_core::command::ApprovalIntent;
/// assert!(ApprovalIntent::ApproveOnce.approves());
/// assert!(!ApprovalIntent::Abort.approves());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApprovalIntent {
    /// Approve only this request.
    ApproveOnce,
    /// Approve this request and grant its session scope.
    ApproveForSession,
    /// Deny the request.
    Deny,
    /// Abort with the existing denial semantics and distinct label.
    Abort,
}

impl ApprovalIntent {
    /// Whether this intent approves the requested operation.
    ///
    /// # Examples
    /// ```
    /// let intent = codetether_approval_core::command::ApprovalIntent::Deny;
    /// assert!(!intent.approves());
    /// ```
    pub fn approves(self) -> bool {
        matches!(self, Self::ApproveOnce | Self::ApproveForSession)
    }

    /// Whether this intent requests session-scoped grants.
    ///
    /// # Examples
    /// ```
    /// let intent = codetether_approval_core::command::ApprovalIntent::Deny;
    /// assert!(!intent.session_scoped());
    /// ```
    pub fn session_scoped(self) -> bool {
        matches!(self, Self::ApproveForSession)
    }

    /// Convert the intent to a live decision retaining a denial reason.
    ///
    /// # Examples
    /// ```
    /// let intent = codetether_approval_core::command::ApprovalIntent::Deny;
    /// let _decision = intent.live_decision_with_reason("stop");
    /// ```
    pub fn live_decision_with_reason(self, reason: &str) -> LiveApprovalDecision {
        if self.approves() {
            LiveApprovalDecision::Approved
        } else {
            LiveApprovalDecision::denied_with(reason)
        }
    }

    /// Default reason for commands without explicit feedback.
    ///
    /// # Examples
    /// ```
    /// let intent = codetether_approval_core::command::ApprovalIntent::Deny;
    /// assert_eq!(intent.fallback_reason(), "denied from TUI");
    /// ```
    pub fn fallback_reason(self) -> &'static str {
        match self {
            Self::ApproveOnce => "approved once from TUI",
            Self::ApproveForSession => "approved for session from TUI",
            Self::Deny => "denied from TUI",
            Self::Abort => "aborted from TUI",
        }
    }

    /// User-facing label for the decision.
    ///
    /// # Examples
    /// ```
    /// let intent = codetether_approval_core::command::ApprovalIntent::Deny;
    /// assert_eq!(intent.label(), "Denied");
    /// ```
    pub fn label(self) -> &'static str {
        match self {
            Self::ApproveOnce => "Approved once",
            Self::ApproveForSession => "Approved for session",
            Self::Deny => "Denied",
            Self::Abort => "Aborted",
        }
    }
}
