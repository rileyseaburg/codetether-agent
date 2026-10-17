//! KrOutcome evidence behavior.

use crate::KrOutcome;

impl KrOutcome {
    /// Create a metric outcome with a value
    ///
    /// # Examples
    /// ```
    /// use codetether_okr::{KrOutcome};
    /// let value = KrOutcome::new(uuid::Uuid::nil(), "Passing checks");
    /// assert_eq!(value.with_value(5.0).value, Some(5.0));
    /// ```
    pub fn with_value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Add evidence link
    ///
    /// # Examples
    /// ```
    /// use codetether_okr::{KrOutcome};
    /// let value = KrOutcome::new(uuid::Uuid::nil(), "Passing checks");
    /// assert_eq!(value.add_evidence("report.json").evidence, vec!["report.json"]);
    /// ```
    #[allow(dead_code)]
    pub fn add_evidence(mut self, evidence: impl Into<String>) -> Self {
        self.evidence.push(evidence.into());
        self
    }
}
