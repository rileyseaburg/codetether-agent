//! Trace validator consensus.

use super::TraceValidator;
use super::validate_with_consensus;
use super::{OracleResult, TraceStep};
use crate::RlmAnalysisResult;
impl TraceValidator {
    /// Validate multiple semantic runs using strict consensus.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{TraceValidator, OracleResult};
    /// assert!(matches!(TraceValidator::new().validate_with_consensus(&[], "", None, None, None), OracleResult::Unverified { .. }));
    /// ```
    pub fn validate_with_consensus(
        &self,
        results: &[RlmAnalysisResult],
        _source: &str,
        source_path: Option<&str>,
        repo_revision: Option<&str>,
        trace_steps: Option<Vec<TraceStep>>,
    ) -> OracleResult {
        validate_with_consensus(
            results,
            source_path,
            repo_revision,
            trace_steps,
            self.config.consensus_threshold,
        )
    }
}
