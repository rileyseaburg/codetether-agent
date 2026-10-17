//! Trace validator batch.

use super::TraceValidator;
use super::{BatchValidationStats, OracleResult, TraceStep};
use crate::RlmAnalysisResult;
impl TraceValidator {
    /// Batch validate multiple traces and return statistics.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{TraceValidator};
    /// assert_eq!(TraceValidator::new().batch_validate(Vec::new()).total(), 0);
    /// ```
    pub fn batch_validate<'a>(
        &self,
        traces: impl IntoIterator<Item = (RlmAnalysisResult, &'a str, Option<&'a str>)>,
    ) -> BatchValidationStats {
        self.batch_validate_with_options(traces, None, None)
    }

    /// Batch validate with additional options.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{TraceValidator};
    /// assert_eq!(TraceValidator::new().batch_validate_with_options(Vec::new(), Some("revision"), None).total(), 0);
    /// ```
    pub fn batch_validate_with_options<'a>(
        &self,
        traces: impl IntoIterator<Item = (RlmAnalysisResult, &'a str, Option<&'a str>)>,
        repo_revision: Option<&str>,
        trace_steps: Option<Vec<TraceStep>>,
    ) -> BatchValidationStats {
        let mut stats = BatchValidationStats::default();

        for (result, source, source_path) in traces {
            match self.validate(
                &result,
                source,
                source_path,
                repo_revision,
                trace_steps.clone(),
            ) {
                OracleResult::Golden(trace) => stats.golden.push(trace),
                OracleResult::Consensus { trace, .. } => stats.consensus.push(trace),
                OracleResult::Unverified { reason, trace } => {
                    stats.unverified.push((trace, reason));
                }
                OracleResult::Failed { reason, trace, .. } => {
                    stats.failed.push((trace, reason));
                }
            }
        }
        stats
    }
}
