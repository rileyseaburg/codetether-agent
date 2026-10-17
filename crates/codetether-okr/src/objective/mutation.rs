//! Okr mutation behavior.

use crate::KeyResult;
use crate::Okr;
use chrono::Utc;
impl Okr {
    /// Add a key result to this OKR
    ///
    /// # Examples
    /// ```
    /// use codetether_okr::{Okr, KeyResult};
    /// let mut value = Okr::new("Reliability", "Reduce failures");
    /// value.add_key_result(KeyResult::new(value.id, "Checks", 10.0, "count"));
    /// assert_eq!(value.key_results.len(), 1);
    /// ```
    pub fn add_key_result(&mut self, kr: KeyResult) {
        self.key_results.push(kr);
        self.updated_at = Utc::now();
    }
}
