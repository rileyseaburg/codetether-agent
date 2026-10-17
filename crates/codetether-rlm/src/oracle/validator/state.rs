//! Trace-validator settings.

use super::Config;

/// Main trace validator for RLM REPL outputs.
///
/// # Examples
/// ```
/// use codetether_rlm::{TraceValidator};
/// let stats = TraceValidator::new().batch_validate(Vec::new());
/// assert_eq!(stats.total(), 0);
/// ```
#[derive(Debug, Clone)]
pub struct TraceValidator {
    pub(super) config: Config,
}

impl Default for TraceValidator {
    fn default() -> Self {
        Self::new()
    }
}
