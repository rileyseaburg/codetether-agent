//! Helper predicates for [`QualityChecks`].

use super::types::QualityChecks;

impl QualityChecks {
    /// Returns `true` when no quality-check command is configured.
    ///
    /// An empty set means Ralph has nothing to run for this PRD, so a story
    /// cannot be considered verified on quality checks alone.
    ///
    /// # Examples
    ///
    /// ```
    /// use codetether_agent::ralph::QualityChecks;
    ///
    /// let mut checks = QualityChecks::default();
    /// assert!(checks.is_empty());
    /// checks.test = Some("cargo test".to_string());
    /// assert!(!checks.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.typecheck.is_none()
            && self.test.is_none()
            && self.lint.is_none()
            && self.build.is_none()
    }
}
