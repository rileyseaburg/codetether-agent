//! Trace validator pipeline for RLM REPL outputs.
//!
//! Orchestrates validation of RLM analysis results by:
//! 1. Classifying the query type (pattern-match vs structural vs semantic)
//! 2. Routing to the appropriate oracle
//! 3. Marking traces as "golden" (verified), "unverified", or "failed"
//! 4. Outputting golden traces as JSONL for downstream SFT training
//!
//! # JSONL Output Format
//!
//! ```json
//! {
//!   "prompt": "Find all async functions in src/rlm/repl.rs",
//!   "trace": [{"iteration": 1, "action": "grep(\"async fn\")", "output": "..."}],
//!   "final_payload": { "kind": "grep", ... },
//!   "verdict": "golden",
//!   "oracle_diff": null,
//!   "repo_revision": "abc123def",
//!   "timestamp": "2026-02-21T18:40:00Z"
//! }
//! ```
//!
//! # Usage
//!
//! ```ignore
//! use codetether_agent::rlm::oracle::{TraceValidator, OracleResult};
//! use codetether_agent::rlm::RlmAnalysisResult;
//!
//! let validator = TraceValidator::new();
//! let result = validator.validate(&analysis_result, &source_code).await;
//!
//! match result {
//!     OracleResult::Golden(trace) => {
//!         // Write to JSONL file for training
//!         writeln!(jsonl_file, "{}", serde_json::to_string(&trace)?)?;
//!     }
//!     OracleResult::Unverified => {
//!         // No deterministic oracle available - skip or flag for manual review
//!     }
//!     OracleResult::Failed(reason) => {
//!         // Oracle disagrees - discard or investigate
//!     }
//! }
//! ```

use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::time::Instant;

use super::schema::{AstResult, FinalPayload};
use super::{grep_oracle::GrepOracle, tree_sitter_oracle::TreeSitterOracle};
use crate::rlm::repl::RlmAnalysisResult;

/// Result of oracle validation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OracleResult {
    /// Answer verified as correct (golden training example).
    Golden(ValidatedTrace),
    /// Weakly verified via multi-run consensus for semantic tasks.
    Consensus {
        trace: ValidatedTrace,
        agreement_ratio: f32,
    },
    /// No deterministic oracle available for this query type.
    Unverified {
        reason: String,
        trace: ValidatedTrace,
    },
    /// Oracle disagrees with the answer (failed verification).
    Failed {
        reason: String,
        diff: Option<String>,
        trace: ValidatedTrace,
    },
}

/// Canonical persisted record for oracle outcomes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OracleTraceRecord {
    pub verdict: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_ratio: Option<f32>,
    pub trace: ValidatedTrace,
}

impl OracleResult {
    /// Convert an oracle result into a canonical record for persistence.
    pub fn to_record(&self) -> OracleTraceRecord {
        match self {
            OracleResult::Golden(trace) => OracleTraceRecord {
                verdict: "golden".to_string(),
                reason: None,
                agreement_ratio: None,
                trace: trace.clone(),
            },
            OracleResult::Consensus {
                trace,
                agreement_ratio,
            } => OracleTraceRecord {
                verdict: "consensus".to_string(),
                reason: None,
                agreement_ratio: Some(*agreement_ratio),
                trace: trace.clone(),
            },
            OracleResult::Unverified { reason, trace } => OracleTraceRecord {
                verdict: "unverified".to_string(),
                reason: Some(reason.clone()),
                agreement_ratio: None,
                trace: trace.clone(),
            },
            OracleResult::Failed { reason, trace, .. } => OracleTraceRecord {
                verdict: "failed".to_string(),
                reason: Some(reason.clone()),
                agreement_ratio: None,
                trace: trace.clone(),
            },
        }
    }
}

/// A single step in the RLM trace.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraceStep {
    /// Iteration number (1-indexed)
    pub iteration: usize,
    /// Action performed (e.g., "grep(\"async fn\")")
    pub action: String,
    /// Output from the action
    pub output: String,
}

/// A validated RLM trace ready for training data export.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidatedTrace {
    /// Original query/question
    pub prompt: String,
    /// Trace of steps taken
    pub trace: Vec<TraceStep>,
    /// Parsed FINAL() payload (if JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_payload: Option<FinalPayload>,
    /// Verdict: "golden", "failed", or "unverified"
    pub verdict: String,
    /// Oracle diff (what the model got wrong - only on failures)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_diff: Option<String>,
    /// Git commit SHA for reproducibility
    pub repo_revision: String,
    /// Verification timestamp (ISO 8601)
    pub timestamp: String,
    // Legacy fields (kept for compatibility)
    /// Model's FINAL() answer (raw string)
    #[serde(skip)]
    pub answer: String,
    /// Number of RLM iterations
    #[serde(skip)]
    pub iterations: usize,
    /// Number of sub-LLM calls
    #[serde(skip)]
    pub subcalls: usize,
    /// Token usage - input
    #[serde(skip)]
    pub input_tokens: usize,
    /// Token usage - output
    #[serde(skip)]
    pub output_tokens: usize,
    /// Elapsed time in milliseconds
    #[serde(skip)]
    pub elapsed_ms: u64,
    /// Source file path (if available)
    #[serde(skip)]
    pub source_path: Option<String>,
    /// Oracle verification method used
    #[serde(skip)]
    pub verification_method: VerificationMethod,
    /// Unique trace ID
    #[serde(skip)]
    pub trace_id: String,
}

/// Method used to verify the trace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum VerificationMethod {
    /// Grep-based pattern matching
    GrepOracle,
    /// Tree-sitter AST verification
    TreeSitterOracle,
    /// Multi-run semantic consensus (weak verification)
    Consensus,
    /// No oracle available
    #[default]
    None,
}

/// Trace validator that routes to appropriate oracles.
pub struct TraceValidator {
    /// Minimum confidence threshold for golden classification
    confidence_threshold: f32,
    /// Agreement threshold for semantic consensus checks
    consensus_threshold: f32,
}

impl Default for TraceValidator {
    fn default() -> Self {
        Self {
            confidence_threshold: 0.95,
            consensus_threshold: 1.0,
        }
    }
}

impl TraceValidator {
    /// Create a new trace validator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the confidence threshold for golden classification.
    pub fn with_confidence_threshold(mut self, threshold: f32) -> Self {
        self.confidence_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Set consensus threshold for semantic query verification.
    pub fn with_consensus_threshold(mut self, threshold: f32) -> Self {
        self.consensus_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Validate an RLM analysis result against source code.
    /// Validate an RLM analysis result against source code.
    ///
    /// # Arguments
    ///
    /// * `result` - The RLM analysis result to validate
    /// * `source` - The source code that was analyzed
    /// * `source_path` - Optional path to the source file (for metadata)
    /// * `repo_revision` - Git commit SHA for reproducibility
    /// * `trace_steps` - Optional trace steps from execution
    ///
    /// # Returns
    ///
    /// - `OracleResult::Golden` if the answer is verified correct
    /// - `OracleResult::Unverified` if no oracle is available
    /// - `OracleResult::Failed` if the oracle disagrees with the answer
    pub fn validate(
        &self,
        result: &RlmAnalysisResult,
        source: &str,
        source_path: Option<&str>,
        repo_revision: Option<&str>,
        trace_steps: Option<Vec<TraceStep>>,
    ) -> OracleResult {
        let _start = Instant::now();

        // Try to parse the answer as JSON
        let final_payload = FinalPayload::parse(&result.answer);
        let query = result
            .sub_queries
            .first()
            .map(|sq| sq.query.clone())
            .unwrap_or_else(|| "unknown query".to_string());

        // Build base trace
        let base_trace = || {
            Self::build_base_trace(
                result,
                source_path,
                repo_revision,
                trace_steps.clone(),
                final_payload.clone(),
            )
        };

        // Route based on payload kind first (more reliable than query classification)
        let verdict = match &final_payload {
            FinalPayload::Grep(_) => {
                self.validate_grep_payload(&final_payload, source, source_path, &query, base_trace)
            }
            FinalPayload::Ast(_) => {
                self.validate_ast_payload(&final_payload, source, source_path, &query, base_trace)
            }
            FinalPayload::Semantic(_) => {
                // Semantic queries are unverifiable
                let mut trace = base_trace();
                trace.verdict = "unverified".to_string();
                return OracleResult::Unverified {
                    reason: "Semantic queries require LLM understanding - no deterministic oracle available".to_string(),
                    trace,
                };
            }
            FinalPayload::Malformed { error, raw } => {
                // If the model attempted JSON and failed to parse, keep a failed verdict.
                // If the answer is plain text, attempt a deterministic fallback route.
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
                    match GrepOracle::classify_query(&query) {
                        super::QueryType::PatternMatch => {
                            let oracle = GrepOracle::new(source.to_string());
                            let verification = oracle.verify(&result.answer, &query);
                            self.oracle_result_from_grep_verification(verification, base_trace)
                        }
                        super::QueryType::Structural => {
                            let mut trace = base_trace();
                            trace.verdict = "unverified".to_string();
                            OracleResult::Unverified {
                                reason: "Structured query result was not emitted as FINAL(JSON); deterministic AST verification requires typed payload".to_string(),
                                trace,
                            }
                        }
                        super::QueryType::Semantic => {
                            let mut trace = base_trace();
                            trace.verdict = "unverified".to_string();
                            OracleResult::Unverified {
                                reason: "Semantic/free-form answer without FINAL(JSON) payload"
                                    .to_string(),
                                trace,
                            }
                        }
                    }
                }
            }
        };

        verdict
    }

    /// Validate multiple semantic runs using strict consensus.
    ///
    /// Deterministic oracle-verifiable payloads are always routed through `validate()`.
    /// Consensus is only attempted when payload kind is `semantic`.
    pub fn validate_with_consensus(
        &self,
        results: &[RlmAnalysisResult],
        source: &str,
        source_path: Option<&str>,
        repo_revision: Option<&str>,
        trace_steps: Option<Vec<TraceStep>>,
    ) -> OracleResult {
        let trace_steps_for_consensus = trace_steps.clone();
        let Some(first) = results.first() else {
            let trace = Self::build_placeholder_trace(source_path, repo_revision, None);
            return OracleResult::Unverified {
                reason: "Consensus validation requires at least one result".to_string(),
                trace,
            };
        };

        let first_payload = FinalPayload::parse(&first.answer);
        if !matches!(first_payload, FinalPayload::Semantic(_)) {
            return self.validate(first, source, source_path, repo_revision, trace_steps);
        }

        let mut counts: std::collections::HashMap<String, (String, usize)> =
            std::collections::HashMap::new();

        for result in results {
            let payload = FinalPayload::parse(&result.answer);
            let semantic = match payload {
                FinalPayload::Semantic(p) => p.answer,
                _ => {
                    let trace = Self::build_base_trace(
                        first,
                        source_path,
                        repo_revision,
                        trace_steps.clone(),
                        first_payload.clone(),
                    );
                    return OracleResult::Unverified {
                        reason: "Consensus validation requires semantic payloads for all runs"
                            .to_string(),
                        trace,
                    };
                }
            };
            let canonical = canonicalize_semantic_answer(&semantic);
            let entry = counts.entry(canonical).or_insert((semantic, 0));
            entry.1 += 1;
        }

        let Some((canonical, (winning_answer, winning_count))) =
            counts.into_iter().max_by_key(|(_, (_, count))| *count)
        else {
            let trace = Self::build_base_trace(
                first,
                source_path,
                repo_revision,
                trace_steps.clone(),
                first_payload.clone(),
            );
            return OracleResult::Unverified {
                reason: "Consensus validation could not determine a winning answer".to_string(),
                trace,
            };
        };

        let agreement_ratio = winning_count as f32 / results.len() as f32;
        if agreement_ratio < self.consensus_threshold {
            let mut trace = Self::build_base_trace(
                first,
                source_path,
                repo_revision,
                trace_steps.clone(),
                first_payload.clone(),
            );
            trace.verdict = "unverified".to_string();
            return OracleResult::Unverified {
                reason: format!(
                    "Consensus not reached: best agreement {:.1}% for answer '{}'",
                    agreement_ratio * 100.0,
                    canonical
                ),
                trace,
            };
        }

        let mut trace = Self::build_base_trace(
            first,
            source_path,
            repo_revision,
            trace_steps_for_consensus,
            FinalPayload::Semantic(super::schema::SemanticPayload {
                file: source_path.unwrap_or("unknown").to_string(),
                answer: winning_answer,
            }),
        );
        trace.verdict = "consensus".to_string();
        trace.verification_method = VerificationMethod::Consensus;

        OracleResult::Consensus {
            trace,
            agreement_ratio,
        }
    }

    fn build_base_trace(
        result: &RlmAnalysisResult,
        source_path: Option<&str>,
        repo_revision: Option<&str>,
        trace_steps: Option<Vec<TraceStep>>,
        final_payload: FinalPayload,
    ) -> ValidatedTrace {
        let revision = repo_revision
            .map(ToString::to_string)
            .or_else(|| Self::get_git_revision().ok())
            .unwrap_or_else(|| "unknown".to_string());

        let query = result
            .sub_queries
            .first()
            .map(|sq| sq.query.clone())
            .unwrap_or_else(|| "unknown query".to_string());

        ValidatedTrace {
            prompt: query,
            trace: trace_steps.unwrap_or_default(),
            final_payload: Some(final_payload),
            verdict: "unverified".to_string(),
            oracle_diff: None,
            repo_revision: revision,
            timestamp: Utc::now().to_rfc3339(),
            answer: result.answer.clone(),
            iterations: result.iterations,
            subcalls: result.sub_queries.len(),
            input_tokens: result.stats.input_tokens,
            output_tokens: result.stats.output_tokens,
            elapsed_ms: result.stats.elapsed_ms,
            source_path: source_path.map(ToString::to_string),
            verification_method: VerificationMethod::None,
            trace_id: uuid::Uuid::new_v4().to_string(),
        }
    }

    fn build_placeholder_trace(
        source_path: Option<&str>,
        repo_revision: Option<&str>,
        final_payload: Option<FinalPayload>,
    ) -> ValidatedTrace {
        let revision = repo_revision
            .map(ToString::to_string)
            .or_else(|| Self::get_git_revision().ok())
            .unwrap_or_else(|| "unknown".to_string());
        ValidatedTrace {
            prompt: "unknown query".to_string(),
            trace: Vec::new(),
            final_payload,
            verdict: "unverified".to_string(),
            oracle_diff: None,
            repo_revision: revision,
            timestamp: Utc::now().to_rfc3339(),
            answer: String::new(),
            iterations: 0,
            subcalls: 0,
            input_tokens: 0,
            output_tokens: 0,
            elapsed_ms: 0,
            source_path: source_path.map(ToString::to_string),
            verification_method: VerificationMethod::None,
            trace_id: uuid::Uuid::new_v4().to_string(),
        }
    }

    /// Validate using grep payload directly.
    fn validate_grep_payload(
        &self,
        payload: &FinalPayload,
        source: &str,
        _source_path: Option<&str>,
        _query: &str,
        base_trace: impl FnOnce() -> ValidatedTrace,
    ) -> OracleResult {
        let grep_payload = match payload {
            FinalPayload::Grep(p) => p,
            _ => unreachable!(),
        };

        let oracle = GrepOracle::new(source.to_string());

        // Run actual grep to get ground truth
        let ground_truth = match oracle.grep(&grep_payload.pattern) {
            Ok(m) => m,
            Err(e) => {
                let mut trace = base_trace();
                trace.verdict = "unverified".to_string();
                return OracleResult::Unverified {
                    reason: format!("Could not run grep: {}", e),
                    trace,
                };
            }
        };

        // Convert payload matches to the format expected by verification
        let claimed: Vec<(usize, String)> = grep_payload
            .matches
            .iter()
            .map(|m| (m.line, m.text.clone()))
            .collect();

        let verification = oracle.verify_matches(&claimed, &ground_truth);

        self.oracle_result_from_grep_verification(verification, base_trace)
    }

    fn oracle_result_from_grep_verification(
        &self,
        verification: super::grep_oracle::GrepVerification,
        base_trace: impl FnOnce() -> ValidatedTrace,
    ) -> OracleResult {
        match verification {
            super::grep_oracle::GrepVerification::ExactMatch
            | super::grep_oracle::GrepVerification::UnorderedMatch => {
                let mut trace = base_trace();
                trace.verification_method = VerificationMethod::GrepOracle;
                trace.verdict = "golden".to_string();
                OracleResult::Golden(trace)
            }
            super::grep_oracle::GrepVerification::SubsetMatch { claimed, actual } => {
                let coverage = claimed as f32 / actual.max(1) as f32;
                if coverage >= self.confidence_threshold {
                    let mut trace = base_trace();
                    trace.verification_method = VerificationMethod::GrepOracle;
                    trace.verdict = "golden".to_string();
                    OracleResult::Golden(trace)
                } else {
                    let diff = format!(
                        "Subset match: model claimed {} but source has {} (coverage: {:.1}%)",
                        claimed,
                        actual,
                        coverage * 100.0
                    );
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
            super::grep_oracle::GrepVerification::HasFalsePositives { false_positives } => {
                let diff = format!(
                    "False positives: {} claims not found in source: {:?}",
                    false_positives.len(),
                    false_positives
                );
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
            super::grep_oracle::GrepVerification::HasFalseNegatives { false_negatives } => {
                let diff = format!(
                    "False negatives: {} items in source not claimed: {:?}",
                    false_negatives.len(),
                    false_negatives
                );
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
            super::grep_oracle::GrepVerification::Mismatch => {
                let diff = "Complete mismatch between claimed and actual matches".to_string();
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
            super::grep_oracle::GrepVerification::CannotVerify { reason } => {
                let mut trace = base_trace();
                trace.verdict = "unverified".to_string();
                OracleResult::Unverified { reason, trace }
            }
        }
    }

    /// Validate using AST payload directly.
    fn validate_ast_payload(
        &self,
        payload: &FinalPayload,
        source: &str,
        _source_path: Option<&str>,
        _query: &str,
        base_trace: impl FnOnce() -> ValidatedTrace,
    ) -> OracleResult {
        let ast_payload = match payload {
            FinalPayload::Ast(p) => p,
            _ => unreachable!(),
        };

        let mut oracle = TreeSitterOracle::new(source.to_string());

        // Get actual AST results based on query type.
        let actual_results: Vec<AstResult> = match ast_payload.query.as_str() {
            "functions" => match oracle.get_functions() {
                Ok(funcs) => funcs
                    .iter()
                    .map(|f| AstResult {
                        name: f.name.clone(),
                        args: parse_params(&f.params),
                        return_type: f
                            .return_type
                            .as_deref()
                            .map(|r| r.trim().trim_start_matches("->").trim().to_string()),
                        span: Some((f.line, f.line)),
                    })
                    .collect(),
                Err(e) => {
                    let mut trace = base_trace();
                    trace.verdict = "unverified".to_string();
                    return OracleResult::Unverified {
                        reason: format!("Failed to parse AST: {}", e),
                        trace,
                    };
                }
            },
            "structs" => match oracle.get_structs() {
                Ok(structs) => structs
                    .iter()
                    .map(|s| AstResult {
                        name: s.name.clone(),
                        args: s.fields.clone(),
                        return_type: None,
                        span: Some((s.line, s.line)),
                    })
                    .collect(),
                Err(e) => {
                    let mut trace = base_trace();
                    trace.verdict = "unverified".to_string();
                    return OracleResult::Unverified {
                        reason: format!("Failed to parse AST: {}", e),
                        trace,
                    };
                }
            },
            "enums" => match oracle.get_enums() {
                Ok(enums) => enums
                    .iter()
                    .map(|e| AstResult {
                        name: e.name.clone(),
                        args: e.variants.clone(),
                        return_type: None,
                        span: Some((e.line, e.line)),
                    })
                    .collect(),
                Err(e) => {
                    let mut trace = base_trace();
                    trace.verdict = "unverified".to_string();
                    return OracleResult::Unverified {
                        reason: format!("Failed to parse AST: {}", e),
                        trace,
                    };
                }
            },
            "impls" => match oracle.get_impls() {
                Ok(impls) => impls
                    .iter()
                    .map(|i| AstResult {
                        name: i.type_name.clone(),
                        args: i.trait_name.clone().map(|t| vec![t]).unwrap_or_default(),
                        return_type: Some(format!("methods:{}", i.method_count)),
                        span: Some((i.line, i.line)),
                    })
                    .collect(),
                Err(e) => {
                    let mut trace = base_trace();
                    trace.verdict = "unverified".to_string();
                    return OracleResult::Unverified {
                        reason: format!("Failed to parse AST: {}", e),
                        trace,
                    };
                }
            },
            _ => {
                // Generic fallback: function symbols.
                match oracle.get_functions() {
                    Ok(funcs) => funcs
                        .iter()
                        .map(|f| AstResult {
                            name: f.name.clone(),
                            args: parse_params(&f.params),
                            return_type: f
                                .return_type
                                .as_deref()
                                .map(|r| r.trim().trim_start_matches("->").trim().to_string()),
                            span: Some((f.line, f.line)),
                        })
                        .collect(),
                    Err(_) => vec![],
                }
            }
        };

        // Canonicalized comparison (stable ordering + normalized formatting).
        let claimed: std::collections::HashSet<_> = ast_payload
            .results
            .iter()
            .map(normalize_ast_result)
            .collect();
        let actual: std::collections::HashSet<_> =
            actual_results.iter().map(normalize_ast_result).collect();

        if claimed == actual {
            let mut trace = base_trace();
            trace.verification_method = VerificationMethod::TreeSitterOracle;
            trace.verdict = "golden".to_string();

            OracleResult::Golden(trace)
        } else if claimed.is_subset(&actual) {
            let coverage = claimed.len() as f32 / actual.len().max(1) as f32;
            if coverage >= self.confidence_threshold {
                let mut trace = base_trace();
                trace.verification_method = VerificationMethod::TreeSitterOracle;
                trace.verdict = "golden".to_string();

                OracleResult::Golden(trace)
            } else {
                let diff = format!(
                    "Partial structural match. Claimed {} entries, actual {} entries.",
                    claimed.len(),
                    actual.len()
                );
                let mut trace = base_trace();
                trace.verification_method = VerificationMethod::TreeSitterOracle;
                trace.verdict = "failed".to_string();
                trace.oracle_diff = Some(diff.clone());

                OracleResult::Failed {
                    reason: diff.clone(),
                    diff: Some(diff),
                    trace,
                }
            }
        } else {
            let extra: Vec<_> = claimed.difference(&actual).cloned().collect();
            let missing: Vec<_> = actual.difference(&claimed).cloned().collect();
            let diff = format!(
                "Structural mismatch. extra={} missing={} extra_items={:?} missing_items={:?}",
                extra.len(),
                missing.len(),
                extra,
                missing
            );
            let mut trace = base_trace();
            trace.verification_method = VerificationMethod::TreeSitterOracle;
            trace.verdict = "failed".to_string();
            trace.oracle_diff = Some(diff.clone());

            OracleResult::Failed {
                reason: diff.clone(),
                diff: Some(diff),
                trace,
            }
        }
    }

    /// Get the current git revision.
    fn get_git_revision() -> Result<String> {
        let output = std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()?;

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
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
                OracleResult::Golden(trace) => {
                    stats.golden.push(trace);
                }
                OracleResult::Consensus { trace, .. } => {
                    stats.consensus.push(trace);
                }
                OracleResult::Unverified { reason, trace } => {
                    let _ = result;
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

/// Statistics from batch validation.
#[derive(Debug, Clone, Default)]
pub struct BatchValidationStats {
    /// Traces verified as golden (ready for training)
    pub golden: Vec<ValidatedTrace>,
    /// Traces weakly verified by consensus (kept separate from golden)
    pub consensus: Vec<ValidatedTrace>,
    /// Traces that could not be verified
    pub unverified: Vec<(ValidatedTrace, String)>,
    /// Traces that failed verification
    pub failed: Vec<(ValidatedTrace, String)>,
}

impl BatchValidationStats {
    /// Total number of traces processed.
    pub fn total(&self) -> usize {
        self.golden.len() + self.consensus.len() + self.unverified.len() + self.failed.len()
    }

    /// Percentage of traces verified as golden.
    pub fn golden_rate(&self) -> f32 {
        let total = self.total();
        if total == 0 {
            0.0
        } else {
            self.golden.len() as f32 / total as f32
        }
    }

    /// Write golden traces to a JSONL file.
    pub fn write_jsonl(&self, path: &str) -> Result<usize> {
        use std::fs::File;
        use std::io::{BufWriter, Write};

        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        let mut count = 0;
        for trace in &self.golden {
            let json = serde_json::to_string(trace)?;
            writeln!(writer, "{}", json)?;
            count += 1;
        }

        writer.flush()?;
        Ok(count)
    }

    /// Write separated JSONL files for deterministic data hygiene.
    ///
    /// Output files:
    /// - `{prefix}.golden.jsonl`
    /// - `{prefix}.consensus.jsonl`
    /// - `{prefix}.failed.jsonl`
    /// - `{prefix}.unverified.jsonl`
    pub fn write_jsonl_split(&self, out_dir: &str, prefix: &str) -> Result<SplitWriteStats> {
        use std::fs::File;
        use std::io::{BufWriter, Write};
        use std::path::Path;

        let dir = Path::new(out_dir);
        std::fs::create_dir_all(dir)?;

        let golden_path = dir.join(format!("{prefix}.golden.jsonl"));
        let consensus_path = dir.join(format!("{prefix}.consensus.jsonl"));
        let failed_path = dir.join(format!("{prefix}.failed.jsonl"));
        let unverified_path = dir.join(format!("{prefix}.unverified.jsonl"));

        let mut golden_writer = BufWriter::new(File::create(&golden_path)?);
        let mut consensus_writer = BufWriter::new(File::create(&consensus_path)?);
        let mut failed_writer = BufWriter::new(File::create(&failed_path)?);
        let mut unverified_writer = BufWriter::new(File::create(&unverified_path)?);

        for trace in &self.golden {
            let rec = OracleTraceRecord {
                verdict: "golden".to_string(),
                reason: None,
                agreement_ratio: None,
                trace: trace.clone(),
            };
            writeln!(golden_writer, "{}", serde_json::to_string(&rec)?)?;
        }

        for trace in &self.consensus {
            let rec = OracleTraceRecord {
                verdict: "consensus".to_string(),
                reason: None,
                agreement_ratio: None,
                trace: trace.clone(),
            };
            writeln!(consensus_writer, "{}", serde_json::to_string(&rec)?)?;
        }

        for (trace, reason) in &self.failed {
            let rec = OracleTraceRecord {
                verdict: "failed".to_string(),
                reason: Some(reason.clone()),
                agreement_ratio: None,
                trace: trace.clone(),
            };
            writeln!(failed_writer, "{}", serde_json::to_string(&rec)?)?;
        }

        for (trace, reason) in &self.unverified {
            let rec = OracleTraceRecord {
                verdict: "unverified".to_string(),
                reason: Some(reason.clone()),
                agreement_ratio: None,
                trace: trace.clone(),
            };
            writeln!(unverified_writer, "{}", serde_json::to_string(&rec)?)?;
        }

        golden_writer.flush()?;
        consensus_writer.flush()?;
        failed_writer.flush()?;
        unverified_writer.flush()?;

        Ok(SplitWriteStats {
            golden_path: golden_path.to_string_lossy().to_string(),
            consensus_path: consensus_path.to_string_lossy().to_string(),
            failed_path: failed_path.to_string_lossy().to_string(),
            unverified_path: unverified_path.to_string_lossy().to_string(),
            golden_count: self.golden.len(),
            consensus_count: self.consensus.len(),
            failed_count: self.failed.len(),
            unverified_count: self.unverified.len(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SplitWriteStats {
    pub golden_path: String,
    pub consensus_path: String,
    pub failed_path: String,
    pub unverified_path: String,
    pub golden_count: usize,
    pub consensus_count: usize,
    pub failed_count: usize,
    pub unverified_count: usize,
}

fn normalize_type_text(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn parse_params(params: &str) -> Vec<String> {
    let trimmed = params.trim().trim_start_matches('(').trim_end_matches(')');
    if trimmed.is_empty() {
        return Vec::new();
    }
    trimmed
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string)
        .collect()
}

/// Canonicalize semantic text with whitespace normalization only.
///
/// This is intentionally lightweight and deterministic: paraphrases and
/// reordered clauses are treated as different answers.
fn canonicalize_semantic_answer(answer: &str) -> String {
    answer.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn normalize_ast_result(result: &AstResult) -> String {
    let args = if result.args.is_empty() {
        String::new()
    } else {
        result.args.join(",").replace(' ', "")
    };
    let return_type = result
        .return_type
        .as_deref()
        .map(normalize_type_text)
        .unwrap_or_default();
    let span = result
        .span
        .map(|(s, e)| format!("{s}:{e}"))
        .unwrap_or_default();
    format!("{}|{}|{}|{}", result.name.trim(), args, return_type, span)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rlm::{RlmStats, SubQuery};

    fn make_result(answer: &str, query: &str) -> RlmAnalysisResult {
        RlmAnalysisResult {
            answer: answer.to_string(),
            iterations: 2,
            sub_queries: vec![SubQuery {
                query: query.to_string(),
                context_slice: None,
                response: answer.to_string(),
                tokens_used: 0,
            }],
            stats: RlmStats {
                input_tokens: 100,
                output_tokens: 50,
                iterations: 2,
                subcalls: 0,
                elapsed_ms: 500,
                compression_ratio: 1.0,
            },
        }
    }

    fn sample_rust_code() -> &'static str {
        r#"
pub async fn process(input: &str) -> Result<String> {
    let data = parse(input)?;
    Ok(data)
}

async fn parse(input: &str) -> Result<String> {
    Ok(input.to_uppercase())
}

pub struct Config {
    pub debug: bool,
}
"#
    }

    #[test]
    fn validate_grep_match() {
        let validator = TraceValidator::new();
        let source = sample_rust_code();
        let result = make_result(
            r#"{"kind": "grep", "file": "test.rs", "pattern": "async fn", "matches": [{"line": 2, "text": "pub async fn process(input: &str) -> Result<String> {"}, {"line": 7, "text": "async fn parse(input: &str) -> Result<String> {"}]}"#,
            "Find all async functions",
        );

        match validator.validate(&result, source, Some("test.rs"), Some("abc123"), None) {
            OracleResult::Golden(trace) => {
                assert_eq!(trace.verification_method, VerificationMethod::GrepOracle);
                assert_eq!(trace.verdict, "golden");
            }
            OracleResult::Consensus { .. } => panic!("Expected deterministic golden"),
            OracleResult::Unverified { .. } => panic!("Expected golden"),
            OracleResult::Failed { .. } => panic!("Expected golden"),
        }
    }

    #[test]
    fn validate_semantic_unverified() {
        let validator = TraceValidator::new();
        let source = sample_rust_code();
        let result = make_result(
            r#"{"kind": "semantic", "file": "test.rs", "answer": "This function processes input by parsing it and returning uppercase"}"#,
            "Explain what the process function does",
        );

        match validator.validate(&result, source, Some("test.rs"), Some("abc123"), None) {
            OracleResult::Unverified { reason, .. } => {
                assert!(reason.contains("Semantic"));
            }
            OracleResult::Golden(_) => panic!("Expected unverified"),
            OracleResult::Consensus { .. } => panic!("Expected unverified"),
            OracleResult::Failed { .. } => panic!("Expected unverified"),
        }
    }

    #[test]
    fn validate_semantic_consensus() {
        let validator = TraceValidator::new();
        let source = sample_rust_code();
        let results = vec![
            make_result(
                r#"{"kind":"semantic","file":"x.rs","answer":"Auth middleware is mandatory for all routes except /health."}"#,
                "Explain auth",
            ),
            make_result(
                r#"{"kind":"semantic","file":"x.rs","answer":"Auth middleware is mandatory for all routes except /health."}"#,
                "Explain auth",
            ),
            make_result(
                r#"{"kind":"semantic","file":"x.rs","answer":"Auth middleware is mandatory for all routes except /health."}"#,
                "Explain auth",
            ),
        ];

        match validator.validate_with_consensus(
            &results,
            source,
            Some("x.rs"),
            Some("abc123"),
            None,
        ) {
            OracleResult::Consensus {
                trace,
                agreement_ratio,
            } => {
                assert_eq!(trace.verification_method, VerificationMethod::Consensus);
                assert_eq!(trace.verdict, "consensus");
                assert_eq!(agreement_ratio, 1.0);
            }
            _ => panic!("Expected consensus verification"),
        }
    }

    #[test]
    fn validate_plain_text_pattern_query_without_json_is_verified() {
        let validator = TraceValidator::new();
        let source = sample_rust_code();
        let result = make_result(
            "There are 2 occurrences of async functions in test.rs",
            "Find all async functions",
        );

        match validator.validate(&result, source, Some("test.rs"), Some("abc123"), None) {
            OracleResult::Golden(trace) => {
                assert_eq!(trace.verification_method, VerificationMethod::GrepOracle);
                assert_eq!(trace.verdict, "golden");
            }
            other => panic!(
                "Expected golden from plaintext grep fallback, got {:?}",
                other
            ),
        }
    }

    #[test]
    fn validate_json_like_malformed_payload_stays_failed() {
        let validator = TraceValidator::new();
        let source = sample_rust_code();
        let result = make_result("{ this is not valid json", "Find all async functions");

        match validator.validate(&result, source, Some("test.rs"), Some("abc123"), None) {
            OracleResult::Failed { reason, .. } => {
                assert!(reason.contains("Malformed FINAL payload"));
            }
            other => panic!(
                "Expected failed verdict for malformed JSON-like payload, got {:?}",
                other
            ),
        }
    }

    #[test]
    fn batch_validate_mixed() {
        let validator = TraceValidator::new();
        let source = sample_rust_code();

        let traces = vec![
            (
                make_result(
                    r#"{"kind": "grep", "file": "x.rs", "pattern": "async fn", "matches": [{"line": 2, "text": "pub async fn process(input: &str) -> Result<String> {"}, {"line": 7, "text": "async fn parse(input: &str) -> Result<String> {"}]}"#,
                    "Find async",
                ),
                source,
                None,
            ),
            (
                make_result(
                    r#"{"kind": "semantic", "file": "x.rs", "answer": "text"}"#,
                    "Explain",
                ),
                source,
                None,
            ),
        ];

        let stats = validator.batch_validate(traces);

        assert!(stats.golden.len() >= 1);
        assert!(stats.unverified.len() >= 1);
        assert!(stats.total() == 2);
    }

    #[test]
    fn write_jsonl_split_creates_all_buckets() {
        let validator = TraceValidator::new();
        let source = sample_rust_code();
        let traces = vec![
            (
                make_result(
                    r#"{"kind":"grep","file":"x.rs","pattern":"async fn","matches":[{"line":2,"text":"pub async fn process(input: &str) -> Result<String> {"},{"line":7,"text":"async fn parse(input: &str) -> Result<String> {"}]}"#,
                    "Find async",
                ),
                source,
                None,
            ),
            (
                make_result(
                    r#"{"kind":"semantic","file":"x.rs","answer":"This is semantic"}"#,
                    "Explain",
                ),
                source,
                None,
            ),
        ];

        let stats = validator.batch_validate(traces);
        let tmp = tempfile::tempdir().expect("tempdir");
        let split = stats
            .write_jsonl_split(tmp.path().to_str().unwrap_or("."), "oracle")
            .expect("write split jsonl");

        assert_eq!(split.golden_count, stats.golden.len());
        assert_eq!(split.consensus_count, stats.consensus.len());
        assert_eq!(split.unverified_count, stats.unverified.len());
        assert_eq!(split.failed_count, stats.failed.len());
        assert!(std::path::Path::new(&split.golden_path).exists());
        assert!(std::path::Path::new(&split.consensus_path).exists());
        assert!(std::path::Path::new(&split.failed_path).exists());
        assert!(std::path::Path::new(&split.unverified_path).exists());
    }
}
