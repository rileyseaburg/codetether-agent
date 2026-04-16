//! Main trace validator entry point.
//!
//! Orchestrates validation of RLM analysis results by:
//! 1. Classifying the query type (pattern-match vs structural vs semantic)
//! 2. Routing to the appropriate oracle
//! 3. Marking traces as "golden" (verified), "unverified", or "failed"

use std::time::Instant;

use super::grep_oracle::GrepOracle;
use super::schema::FinalPayload;
use crate::rlm::repl::RlmAnalysisResult;

pub use super::ast_validation::validate_ast_payload;
pub use super::batch::{BatchValidationStats, SplitWriteStats};
pub use super::consensus::{build_base_trace, validate_with_consensus};
#[allow(unused_imports)]
pub use super::consensus_helpers::build_placeholder_trace;
pub use super::grep_validation::validate_grep_payload;
pub use super::trace_types::{OracleResult, ValidatedTrace};
pub use super::types::{TraceStep, VerificationMethod};

/// Configuration for the trace validator.
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

/// Main trace validator for RLM REPL outputs.
#[derive(Debug, Clone)]
pub struct TraceValidator {
    config: Config,
}

impl Default for TraceValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl TraceValidator {
    /// Create a new trace validator with default configuration.
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    /// Set the confidence threshold for golden classification.
    pub fn with_confidence_threshold(mut self, threshold: f32) -> Self {
        self.config.confidence_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Set consensus threshold for semantic query verification.
    pub fn with_consensus_threshold(mut self, threshold: f32) -> Self {
        self.config.consensus_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Validate an RLM analysis result against source code.
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

        match &final_payload {
            FinalPayload::Grep(_) => validate_grep_payload(
                &final_payload,
                source,
                self.config.confidence_threshold,
                base_trace,
            ),
            FinalPayload::Ast(_) => validate_ast_payload(
                &final_payload,
                source,
                self.config.confidence_threshold,
                base_trace,
            ),
            FinalPayload::Semantic(_) => {
                let mut trace = base_trace();
                trace.verdict = "unverified".to_string();
                OracleResult::Unverified {
                    reason: "Semantic queries require LLM understanding - no deterministic oracle available".to_string(),
                    trace,
                }
            }
            FinalPayload::Malformed { error, raw } => {
                let trimmed = raw.trim_start();
                if trimmed.starts_with('{') || trimmed.starts_with('[') {
                    let mut trace = base_trace();
                    trace.verdict = "failed".to_string();
                    OracleResult::Failed {
                        reason: format!("Malformed FINAL payload: {}", error),
                        diff: None,
                        trace,
                    }
                } else {
                    self.validate_plain_text(&query, source, &result.answer, base_trace)
                }
            }
        }
    }

    fn validate_plain_text(
        &self,
        query: &str,
        source: &str,
        answer: &str,
        base_trace: impl FnOnce() -> ValidatedTrace,
    ) -> OracleResult {
        match GrepOracle::classify_query(query) {
            super::QueryType::PatternMatch => {
                let oracle = GrepOracle::new(source.to_string());
                let verification = oracle.verify(answer, query);
                self.oracle_result_from_grep_verification(verification, base_trace)
            }
            super::QueryType::Structural => {
                let mut trace = base_trace();
                trace.verdict = "unverified".to_string();
                OracleResult::Unverified {
                    reason: "Structured query result was not emitted as FINAL(JSON)".to_string(),
                    trace,
                }
            }
            super::QueryType::Semantic => {
                let mut trace = base_trace();
                trace.verdict = "unverified".to_string();
                OracleResult::Unverified {
                    reason: "Semantic query - no deterministic oracle available".to_string(),
                    trace,
                }
            }
        }
    }

    fn oracle_result_from_grep_verification(
        &self,
        verification: super::grep_oracle::GrepVerification,
        base_trace: impl FnOnce() -> ValidatedTrace,
    ) -> OracleResult {
        use super::grep_oracle::GrepVerification;

        match verification {
            GrepVerification::ExactMatch | GrepVerification::UnorderedMatch => {
                let mut trace = base_trace();
                trace.verification_method = VerificationMethod::GrepOracle;
                trace.verdict = "golden".to_string();
                OracleResult::Golden(trace)
            }
            GrepVerification::CannotVerify { reason } => {
                let mut trace = base_trace();
                trace.verdict = "unverified".to_string();
                OracleResult::Unverified { reason, trace }
            }
            _ => {
                let diff = format!("Grep verification failed: {:?}", verification);
                let mut trace = base_trace();
                trace.verification_method = VerificationMethod::GrepOracle;
                trace.verdict = "failed".to_string();
                trace.oracle_diff = Some(diff.clone());
                OracleResult::Failed {
                    reason: diff.clone(),
                    diff: Some(diff),
                    trace,
                }
            }
        }
    }

    /// Validate multiple semantic runs using strict consensus.
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

    /// Batch validate multiple traces and return statistics.
    pub fn batch_validate<'a>(
        &self,
        traces: impl IntoIterator<Item = (RlmAnalysisResult, &'a str, Option<&'a str>)>,
    ) -> BatchValidationStats {
        self.batch_validate_with_options(traces, None, None)
    }

    /// Batch validate with additional options.
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
