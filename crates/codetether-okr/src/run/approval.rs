//! OkrRun approval behavior.

use crate::OkrRun;
use crate::{ApprovalChoice, ApprovalDecision, OkrRunStatus, OkrValidationError};
use chrono::Utc;
impl OkrRun {
    /// Submit for approval
    ///
    /// # Errors
    /// Returns an error unless the run is in draft status.
    ///
    /// # Examples
    /// ```
    /// use codetether_okr::{OkrRun, OkrRunStatus};
    /// let mut value = OkrRun::new(uuid::Uuid::nil(), "Quarterly run");
    /// value.submit_for_approval().unwrap();
    /// assert_eq!(value.status, OkrRunStatus::PendingApproval);
    /// ```
    pub fn submit_for_approval(&mut self) -> Result<(), OkrValidationError> {
        if self.status != OkrRunStatus::Draft {
            return Err(OkrValidationError::InvalidStatusTransition);
        }
        self.status = OkrRunStatus::PendingApproval;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Record an approval decision
    ///
    /// # Examples
    /// ```
    /// use codetether_okr::{OkrRun, ApprovalDecision, OkrRunStatus};
    /// let mut value = OkrRun::new(uuid::Uuid::nil(), "Quarterly run");
    /// value.record_decision(ApprovalDecision::deny(value.id, "Needs review"));
    /// assert_eq!(value.status, OkrRunStatus::Denied);
    /// ```
    pub fn record_decision(&mut self, decision: ApprovalDecision) {
        self.approval = Some(decision.clone());
        self.updated_at = Utc::now();
        match decision.decision {
            ApprovalChoice::Approved => {
                self.status = OkrRunStatus::Approved;
            }
            ApprovalChoice::Denied => {
                self.status = OkrRunStatus::Denied;
            }
        }
    }
}
