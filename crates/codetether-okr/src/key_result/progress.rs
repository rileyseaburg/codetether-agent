//! KeyResult progress behavior.

use crate::KeyResult;
use crate::KeyResultStatus;
impl KeyResult {
    /// Calculate progress as a ratio (0.0 to 1.0)
    ///
    /// For KRs with a target of zero (e.g. "No critical errors" with target 0),
    /// progress is 1.0 when the current value also meets that target (zero
    /// errors) and 0.0 otherwise. This keeps `progress()` consistent with
    /// [`is_complete`](Self::is_complete).
    ///
    /// # Examples
    /// ```
    /// use codetether_okr::{KeyResult};
    /// let mut value = KeyResult::new(uuid::Uuid::nil(), "Checks", 10.0, "count");
    /// value.update_progress(5.0);
    /// assert_eq!(value.progress(), 0.5);
    /// ```
    pub fn progress(&self) -> f64 {
        if self.target_value == 0.0 {
            return if self.current_value <= 0.0 { 1.0 } else { 0.0 };
        }
        (self.current_value / self.target_value).clamp(0.0, 1.0)
    }

    /// Check if the key result is complete
    ///
    /// For zero-target KRs (e.g. "No critical errors"), completion means the
    /// current value is at or below the target (zero errors), not above it.
    ///
    /// # Examples
    /// ```
    /// use codetether_okr::{KeyResult};
    /// let mut value = KeyResult::new(uuid::Uuid::nil(), "Checks", 10.0, "count");
    /// assert!(!value.is_complete());
    /// ```
    pub fn is_complete(&self) -> bool {
        self.status == KeyResultStatus::Completed
            || if self.target_value == 0.0 {
                self.current_value <= 0.0
            } else {
                self.current_value >= self.target_value
            }
    }
}
