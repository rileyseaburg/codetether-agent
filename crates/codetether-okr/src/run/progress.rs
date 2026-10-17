//! OkrRun progress behavior.

use crate::OkrRun;
use crate::OkrRunStatus;
use chrono::Utc;
impl OkrRun {
    /// Update key result progress
    ///
    /// # Examples
    /// ```
    /// use codetether_okr::{OkrRun};
    /// let mut value = OkrRun::new(uuid::Uuid::nil(), "Quarterly run");
    /// value.update_kr_progress("checks", 0.5);
    /// assert_eq!(value.kr_progress["checks"], 0.5);
    /// ```
    pub fn update_kr_progress(&mut self, kr_id: &str, progress: f64) {
        self.kr_progress.insert(kr_id.to_string(), progress);
        self.updated_at = Utc::now();
    }

    /// Check if run can be resumed
    ///
    /// # Examples
    /// ```
    /// use codetether_okr::{OkrRun};
    /// let mut value = OkrRun::new(uuid::Uuid::nil(), "Quarterly run");
    /// assert!(!value.is_resumable());
    /// ```
    pub fn is_resumable(&self) -> bool {
        matches!(
            self.status,
            OkrRunStatus::Running | OkrRunStatus::Paused | OkrRunStatus::WaitingApproval
        )
    }
}
