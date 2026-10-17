//! KeyResult mutation behavior.

use crate::KeyResult;
use crate::{KeyResultStatus, KrOutcome};
use chrono::Utc;
impl KeyResult {
    /// Add an outcome to this key result
    ///
    /// # Examples
    /// ```
    /// use codetether_okr::{KeyResult, KrOutcome};
    /// let mut value = KeyResult::new(uuid::Uuid::nil(), "Checks", 10.0, "count");
    /// value.add_outcome(KrOutcome::new(value.id, "Passing checks"));
    /// assert_eq!(value.outcomes.len(), 1);
    /// ```
    pub fn add_outcome(&mut self, outcome: KrOutcome) {
        self.outcomes.push(outcome);
        self.updated_at = Utc::now();
    }

    /// Update current value and recalculate status
    ///
    /// # Examples
    /// ```
    /// use codetether_okr::{KeyResult};
    /// let mut value = KeyResult::new(uuid::Uuid::nil(), "Checks", 10.0, "count");
    /// value.update_progress(5.0);
    /// assert_eq!(value.current_value, 5.0);
    /// ```
    pub fn update_progress(&mut self, value: f64) {
        self.current_value = value;
        self.updated_at = Utc::now();
        if self.is_complete() {
            self.status = KeyResultStatus::Completed;
        } else if self.current_value > 0.0 {
            self.status = KeyResultStatus::InProgress;
        }
    }
}
