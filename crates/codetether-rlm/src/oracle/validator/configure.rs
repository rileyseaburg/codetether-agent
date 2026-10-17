//! Trace validator configure.

use super::Config;
use super::TraceValidator;
impl TraceValidator {
    /// Create a new trace validator with default configuration.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{TraceValidator};
    /// let stats = TraceValidator::new().batch_validate(Vec::new());
    /// assert_eq!(stats.total(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    /// Set the confidence threshold for golden classification.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{TraceValidator};
    /// let validator = TraceValidator::new().with_confidence_threshold(0.8);
    /// assert_eq!(validator.batch_validate(Vec::new()).total(), 0);
    /// ```
    pub fn with_confidence_threshold(mut self, threshold: f32) -> Self {
        self.config.confidence_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Set consensus threshold for semantic query verification.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{TraceValidator, OracleResult};
    /// let validator = TraceValidator::new().with_consensus_threshold(1.0);
    /// assert!(matches!(validator.validate_with_consensus(&[], "", None, None, None), OracleResult::Unverified { .. }));
    /// ```
    pub fn with_consensus_threshold(mut self, threshold: f32) -> Self {
        self.config.consensus_threshold = threshold.clamp(0.0, 1.0);
        self
    }
}
