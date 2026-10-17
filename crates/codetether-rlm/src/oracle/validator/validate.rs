//! Prepare trace metadata and dispatch one analysis result.

use super::{FinalPayload, OracleResult, TraceStep, TraceValidator, build_base_trace};
use crate::RlmAnalysisResult;
use std::time::Instant;
impl TraceValidator {
    /// Validate an RLM analysis result against source code.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{TraceValidator, RlmAnalysisResult, OracleResult};
    /// let result = RlmAnalysisResult { answer: "not JSON".into(), iterations: 1,
    ///     sub_queries: vec![], stats: Default::default() };
    /// let verdict = TraceValidator::new().validate(&result, "", None, None, None);
    /// assert!(matches!(verdict, OracleResult::Unverified { .. }));
    /// ```
    pub fn validate(
        &self,
        result: &RlmAnalysisResult,
        source: &str,
        source_path: Option<&str>,
        repo_revision: Option<&str>,
        trace_steps: Option<Vec<TraceStep>>,
    ) -> OracleResult {
        let _start = Instant::now();

        let final_payload = FinalPayload::parse(&result.answer);
        let query = result
            .sub_queries
            .first()
            .map(|sq| sq.query.clone())
            .unwrap_or_else(|| "unknown query".to_string());

        let base_trace = || {
            build_base_trace(
                result,
                source_path,
                repo_revision,
                trace_steps.clone(),
                final_payload.clone(),
            )
        };

        self.route(&final_payload, &query, source, &result.answer, base_trace)
    }
}
