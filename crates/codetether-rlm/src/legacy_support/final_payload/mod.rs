//! Normalize final payloads using existing trace evidence.
//!
//! [ensure_structured_final_payload] keeps matching payloads unchanged and
//! records each correction in the supplied execution and context traces.
use super::{FinalPayload, TraceStep, coerce_grep_payload_from_trace, truncate_with_ellipsis};
use crate::context_trace::{ContextEvent, ContextTrace};
mod record;
use record::{COERCE, NORMALIZE, record_normalization as record};

/// Normalize a final answer against recorded grep evidence.
///
/// # Arguments
/// * `query` - Original analysis question.
/// * `raw_final_text` - Answer returned by the executor.
/// * `trace_steps` - Execution evidence; receives a correction step when needed.
/// * `context_trace` - Context history; receives the corrected final answer.
/// # Returns
/// The original answer or its canonical grep payload when trace evidence permits.
/// # Examples
/// ```
/// use codetether_rlm::{context_trace::ContextTrace, legacy_support::final_payload};
/// let mut steps = vec![];
/// let mut trace = ContextTrace::new(1024);
/// let answer = final_payload::ensure_structured_final_payload(
///     "explain", "answer".into(), &mut steps, &mut trace,
/// );
/// assert_eq!(answer, "answer");
/// assert!(steps.is_empty());
/// ```
pub fn ensure_structured_final_payload(
    query: &str,
    raw_final_text: String,
    trace_steps: &mut Vec<TraceStep>,
    context_trace: &mut ContextTrace,
) -> String {
    let parsed = FinalPayload::parse(&raw_final_text);

    if let Some(canonical) = coerce_grep_payload_from_trace(query, trace_steps) {
        let canonical_payload = FinalPayload::parse(&canonical);
        match &parsed {
            FinalPayload::Grep(_) => {
                if canonical_payload != parsed {
                    record(trace_steps, context_trace, &canonical, NORMALIZE);
                    tracing::info!("RLM normalized FINAL(JSON) grep payload using trace evidence");
                    return canonical;
                }
                return raw_final_text;
            }
            FinalPayload::Malformed { .. } => {
                record(trace_steps, context_trace, &canonical, COERCE);
                tracing::info!(
                    "RLM coerced malformed/prose final answer into FINAL(JSON) grep payload"
                );
                return canonical;
            }
            _ => {
                record(trace_steps, context_trace, &canonical, COERCE);
                tracing::info!(
                    "RLM coerced non-grep FINAL payload into canonical grep payload using trace evidence"
                );
                return canonical;
            }
        }
    }

    raw_final_text
}
