//! Trace construction helpers used by the oracle validator and
//! consensus subsystem.
//!
//! [`build_base_trace`] assembles a [`ValidatedTrace`] from
//! an RLM analysis result.  Placeholder and git-revision
//! utilities live in the [`placeholders`] child module.
//!
//! # Examples
//!
//! ```ignore
//! let trace = build_base_trace(&result, Some("src/lib.rs"), None, None, payload);
//! assert!(trace.prompt.contains("async"));
//! ```

mod placeholders;

pub use placeholders::build_placeholder_trace;
#[allow(unused_imports)]
pub use placeholders::get_git_revision;

use super::schema::FinalPayload;
use super::trace_types::ValidatedTrace;
use super::types::{TraceStep, VerificationMethod};
use crate::RlmAnalysisResult;

/// Assemble a [`ValidatedTrace`] from an RLM analysis result.
///
/// Derives the prompt from the first sub-query, casts token
/// counters from `usize` → `u64`, and leaves the verdict
/// empty for the caller to fill in after oracle checks.
///
/// # Examples
///
/// ```ignore
/// let trace = build_base_trace(
///     &result, Some("src/main.rs"), Some("abc123"), None, payload,
/// );
/// assert!(trace.verdict.is_empty());
/// ```
pub fn build_base_trace(
    result: &RlmAnalysisResult,
    source_path: Option<&str>,
    repo_revision: Option<&str>,
    trace_steps: Option<Vec<TraceStep>>,
    final_payload: FinalPayload,
) -> ValidatedTrace {
    let prompt = result
        .sub_queries
        .first()
        .map(|sq| sq.query.clone())
        .unwrap_or_else(|| "unknown query".to_string());
    ValidatedTrace {
        prompt,
        trace: trace_steps.unwrap_or_default(),
        final_payload: Some(final_payload),
        verdict: String::new(),
        oracle_diff: None,
        repo_revision: repo_revision.unwrap_or("unknown").to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        answer: result.answer.clone(),
        iterations: result.iterations,
        subcalls: result.sub_queries.len(),
        input_tokens: result.stats.input_tokens as u64,
        output_tokens: result.stats.output_tokens as u64,
        elapsed_ms: result.stats.elapsed_ms,
        source_path: source_path.map(String::from),
        verification_method: VerificationMethod::None,
        trace_id: uuid::Uuid::new_v4().to_string(),
    }
}
