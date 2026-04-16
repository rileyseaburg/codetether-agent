//! Multi-run consensus validation.

use super::consensus_helpers::build_placeholder_trace;
use super::trace_types::OracleResult;
use super::types::{TraceStep, VerificationMethod};
use crate::rlm::repl::RlmAnalysisResult;

pub use super::consensus_helpers::build_base_trace;

/// Validate multiple runs with consensus agreement.
pub fn validate_with_consensus(
    results: &[RlmAnalysisResult],
    source_path: Option<&str>,
    repo_revision: Option<&str>,
    _trace_steps: Option<Vec<TraceStep>>,
    threshold: f32,
) -> OracleResult {
    if results.is_empty() {
        return OracleResult::Unverified {
            reason: "No results to validate".into(),
            trace: build_placeholder_trace(),
        };
    }

    let answers: Vec<&str> = results.iter().map(|r| r.answer.as_str()).collect();
    let total = answers.len() as f32;

    // Count most common answer using HashMap for O(N)
    let mut freq: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    for &a in &answers {
        *freq.entry(a).or_insert(0) += 1;
    }
    let best_entry = freq.iter().max_by_key(|(_, c)| *c).unwrap();
    let (best_answer, best_count) = (*best_entry.0, *best_entry.1);

    let ratio = best_count as f32 / total;
    let mut trace = build_placeholder_trace();
    trace.answer = best_answer.to_string();
    trace.source_path = source_path.map(String::from);
    trace.repo_revision = repo_revision.unwrap_or("unknown").to_string();

    if ratio >= threshold {
        trace.verification_method = VerificationMethod::Consensus;
        trace.verdict = "consensus".into();
        OracleResult::Consensus {
            trace,
            agreement_ratio: ratio,
        }
    } else {
        trace.verdict = "unverified".into();
        OracleResult::Unverified {
            reason: format!("Consensus ratio {ratio:.2} below threshold {threshold:.2}"),
            trace,
        }
    }
}
