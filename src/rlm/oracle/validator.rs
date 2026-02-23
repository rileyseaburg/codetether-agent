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
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Instant;

use super::schema::FinalPayload;
use super::{grep_oracle::GrepOracle, tree_sitter_oracle::TreeSitterOracle, QueryType};
use crate::rlm::repl::RlmAnalysisResult;

/// Result of oracle validation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OracleResult {
    /// Answer verified as correct (golden training example).
    Golden(ValidatedTrace),
    /// No deterministic oracle available for this query type.
    Unverified {
        reason: String,
    },
    /// Oracle disagrees with the answer (failed verification).
    Failed {
        reason: String,
        diff: Option<String>,
        trace: ValidatedTrace,
    },
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
    /// No oracle available
    #[default]
    None,
}

/// Trace validator that routes to appropriate oracles.
pub struct TraceValidator {
    /// Minimum confidence threshold for golden classification
    confidence_threshold: f32,
}

impl Default for TraceValidator {
    fn default() -> Self {
        Self {
            confidence_threshold: 0.95,
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
        
        // Get current git revision if not provided
        let revision = repo_revision
            .map(|s| s.to_string())
            .or_else(|| Self::get_git_revision().ok())
            .unwrap_or_else(|| "unknown".to_string());
        
        // Extract the original query from sub_queries or use a placeholder
        let query = result
            .sub_queries
            .first()
            .map(|sq| sq.query.clone())
            .unwrap_or_else(|| "unknown query".to_string());
        
        // Try to parse the answer as JSON
        let final_payload = FinalPayload::parse(&result.answer);
        
        // Build base trace
        let base_trace = || ValidatedTrace {
            prompt: query.clone(),
            trace: trace_steps.unwrap_or_default(),
            final_payload: Some(final_payload.clone()),
            verdict: "unverified".to_string(),
            oracle_diff: None,
            repo_revision: revision.clone(),
            timestamp: Utc::now().to_rfc3339(),
            // Legacy fields
            answer: result.answer.clone(),
            iterations: result.iterations,
            subcalls: result.sub_queries.len(),
            input_tokens: result.stats.input_tokens,
            output_tokens: result.stats.output_tokens,
            elapsed_ms: result.stats.elapsed_ms,
            source_path: source_path.map(|s| s.to_string()),
            verification_method: VerificationMethod::None,
            trace_id: uuid::Uuid::new_v4().to_string(),
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
                return OracleResult::Unverified {
                    reason: "Semantic queries require LLM understanding - no deterministic oracle available".to_string(),
                };
            }
            FinalPayload::Malformed { error, .. } => {
                // Return failed because the payload is malformed
                let mut trace = base_trace();
                trace.verdict = "failed".to_string();
                OracleResult::Failed {
                    reason: format!("Malformed FINAL payload: {}", error),
                    diff: None,
                    trace,
                }
            }
        };
        
        verdict
    }

    /// Validate using grep payload directly.
    fn validate_grep_payload(
        &self,
        payload: &FinalPayload,
        source: &str,
        source_path: Option<&str>,
        query: &str,
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
                return OracleResult::Unverified {
                    reason: format!("Could not run grep: {}", e),
                };
            }
        };
        
        // Convert payload matches to the format expected by verification
        let claimed: Vec<(usize, String)> = grep_payload.matches
            .iter()
            .map(|m| (m.line, m.text.clone()))
            .collect();
        
        let verification = oracle.verify_matches(&claimed, &ground_truth);
        
        match verification {
            super::grep_oracle::GrepVerification::ExactMatch
            | super::grep_oracle::GrepVerification::UnorderedMatch => {
                let mut trace = base_trace();
                trace.verification_method = VerificationMethod::GrepOracle;
                trace.verdict = "golden".to_string();
                
                tracing::info!(
                    query = %query,
                    pattern = %grep_payload.pattern,
                    "Grep oracle verified trace as golden"
                );
                
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
                        claimed, actual, coverage * 100.0
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
                OracleResult::Unverified { reason }
            }
        }
    }

    /// Validate using AST payload directly.
    fn validate_ast_payload(
        &self,
        payload: &FinalPayload,
        source: &str,
        source_path: Option<&str>,
        query: &str,
        base_trace: impl FnOnce() -> ValidatedTrace,
    ) -> OracleResult {
        let ast_payload = match payload {
            FinalPayload::Ast(p) => p,
            _ => unreachable!(),
        };
        
        let mut oracle = TreeSitterOracle::new(source.to_string());
        
        // Get actual AST results based on query type
        let actual_results = match ast_payload.query.as_str() {
            "functions" => {
                match oracle.get_functions() {
                    Ok(funcs) => funcs.iter().map(|f| f.name.clone()).collect(),
                    Err(e) => {
                        return OracleResult::Unverified {
                            reason: format!("Failed to parse AST: {}", e),
                        };
                    }
                }
            }
            "structs" => {
                match oracle.get_structs() {
                    Ok(structs) => structs.iter().map(|s| s.name.clone()).collect(),
                    Err(e) => {
                        return OracleResult::Unverified {
                            reason: format!("Failed to parse AST: {}", e),
                        };
                    }
                }
            }
            "enums" => {
                match oracle.get_enums() {
                    Ok(enums) => enums.iter().map(|e| e.name.clone()).collect(),
                    Err(e) => {
                        return OracleResult::Unverified {
                            reason: format!("Failed to parse AST: {}", e),
                        };
                    }
                }
            }
            _ => {
                // Generic query - try to match against all
                match oracle.get_functions() {
                    Ok(funcs) => funcs.iter().map(|f| f.name.clone()).collect(),
                    Err(_) => vec![],
                }
            }
        };
        
        // Compare with claimed results
        let claimed: std::collections::HashSet<_> = ast_payload.results
            .iter()
            .map(|r| r.name.clone())
            .collect();
        let actual: std::collections::HashSet<_> = actual_results.iter().cloned().collect();
        
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
                    "Partial match: claimed {:?}, actual {:?}",
                    claimed, actual
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
            let diff = format!(
                "Mismatch: claimed {:?}, actual {:?}",
                claimed, actual
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
            match self.validate(&result, source, source_path, repo_revision, trace_steps.clone()) {
                OracleResult::Golden(trace) => {
                    stats.golden.push(trace);
                }
                OracleResult::Unverified { reason } => {
                    stats.unverified.push((result, reason));
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
    /// Traces that could not be verified
    pub unverified: Vec<(RlmAnalysisResult, String)>,
    /// Traces that failed verification
    pub failed: Vec<(ValidatedTrace, String)>,
}

impl BatchValidationStats {
    /// Total number of traces processed.
    pub fn total(&self) -> usize {
        self.golden.len() + self.unverified.len() + self.failed.len()
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rlm::RlmStats;

    fn make_result(answer: &str, query: &str) -> RlmAnalysisResult {
        RlmAnalysisResult {
            answer: answer.to_string(),
            iterations: 2,
            sub_queries: vec![],
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
            r#"{"kind": "grep", "file": "test.rs", "pattern": "async fn", "matches": [{"line": 1, "text": "pub async fn process(input: &str) -> Result<String> {"}, {"line": 5, "text": "async fn parse(input: &str) -> Result<String> {"}]}"#,
            "Find all async functions",
        );
        
        match validator.validate(&result, source, Some("test.rs"), Some("abc123"), None) {
            OracleResult::Golden(trace) => {
                assert_eq!(trace.verification_method, VerificationMethod::GrepOracle);
                assert_eq!(trace.verdict, "golden");
            }
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
            OracleResult::Unverified { reason } => {
                assert!(reason.contains("Semantic"));
            }
            OracleResult::Golden(_) => panic!("Expected unverified"),
            OracleResult::Failed { .. } => panic!("Expected unverified"),
        }
    }

    #[test]
    fn batch_validate_mixed() {
        let validator = TraceValidator::new();
        let source = sample_rust_code();
        
        let traces = vec![
            (make_result(r#"{"kind": "grep", "file": "x.rs", "pattern": "async", "matches": []}"#, "Find async"), source, None),
            (make_result(r#"{"kind": "semantic", "file": "x.rs", "answer": "text"}"#, "Explain"), source, None),
        ];
        
        let stats = validator.batch_validate(traces);
        
        assert!(stats.golden.len() >= 1);
        assert!(stats.unverified.len() >= 1);
        assert!(stats.total() == 2);
    }
}
