//! OkrRun execution behavior.

use crate::OkrRun;
use crate::{OkrRunStatus, OkrValidationError};
use chrono::Utc;
impl OkrRun {
    /// Start execution
    ///
    /// # Errors
    /// Returns an error unless the run is approved.
    ///
    /// # Examples
    /// ```
    /// use codetether_okr::{OkrRun, ApprovalDecision};
    /// let mut value = OkrRun::new(uuid::Uuid::nil(), "Quarterly run");
    /// value.record_decision(ApprovalDecision::approve(value.id, "Reviewed"));
    /// assert!(value.start().is_ok());
    /// ```
    pub fn start(&mut self) -> Result<(), OkrValidationError> {
        if self.status != OkrRunStatus::Approved {
            return Err(OkrValidationError::NotApproved);
        }
        self.status = OkrRunStatus::Running;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Mark as complete
    ///
    /// # Examples
    /// ```
    /// use codetether_okr::{OkrRun, OkrRunStatus};
    /// let mut value = OkrRun::new(uuid::Uuid::nil(), "Quarterly run");
    /// value.complete();
    /// assert_eq!(value.status, OkrRunStatus::Completed);
    /// ```
    pub fn complete(&mut self) {
        self.status = OkrRunStatus::Completed;
        self.completed_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
}
