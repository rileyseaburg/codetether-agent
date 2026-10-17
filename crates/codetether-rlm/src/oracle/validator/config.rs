//! Confidence and consensus thresholds.

/// Configuration for the trace validator.
///
/// # Examples
/// ```
/// use codetether_rlm::{TraceValidator};
/// let validator = TraceValidator::new().with_confidence_threshold(0.8);
/// assert_eq!(validator.batch_validate(Vec::new()).total(), 0);
/// ```
#[derive(Debug, Clone)]
pub struct Config {
    /// Minimum coverage ratio for golden classification (0.0-1.0)
    pub confidence_threshold: f32,
    /// Agreement threshold for semantic consensus checks (0.0-1.0)
    pub consensus_threshold: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            confidence_threshold: 0.95,
            consensus_threshold: 1.0,
        }
    }
}
