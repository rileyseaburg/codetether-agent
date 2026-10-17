//! Okr progress behavior.

use crate::Okr;

impl Okr {
    /// Calculate overall progress (0.0 to 1.0) across all key results
    ///
    /// # Examples
    /// ```
    /// use codetether_okr::{Okr};
    /// let mut value = Okr::new("Reliability", "Reduce failures");
    /// assert_eq!(value.progress(), 0.0);
    /// ```
    pub fn progress(&self) -> f64 {
        if self.key_results.is_empty() {
            return 0.0;
        }
        let total: f64 = self.key_results.iter().map(|kr| kr.progress()).sum();
        total / self.key_results.len() as f64
    }

    /// Check if all key results are complete
    ///
    /// # Examples
    /// ```
    /// use codetether_okr::{Okr};
    /// let mut value = Okr::new("Reliability", "Reduce failures");
    /// assert!(value.is_complete());
    /// ```
    pub fn is_complete(&self) -> bool {
        self.key_results.iter().all(|kr| kr.is_complete())
    }
}
