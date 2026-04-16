//! Grep-based payload validation against source text.
//!
//! Compares claimed grep matches from a [`FinalPayload::Grep`]
//! payload with actual source content.  When the match ratio
//! exceeds the confidence threshold the result is promoted to
//! [`OracleResult::Golden`]; otherwise it falls back to
//! [`OracleResult::Unverified`].
//!
//! # Examples
//!
//! ```ignore
//! let result = validate_grep_payload(&payload, src, 0.8, || base);
//! assert!(matches!(result, OracleResult::Golden(_)));
//! ```

use super::grep_oracle::GrepOracle;
use super::schema::FinalPayload;
use super::trace_types::{OracleResult, ValidatedTrace};
use super::types::VerificationMethod;

/// Validate claimed grep matches against raw source text.
///
/// Extracts the grep payload, counts how many claimed match
/// texts actually appear in `source`, and compares the ratio
/// to `confidence_threshold`.  Returns [`OracleResult::Golden`]
/// when the ratio is at or above threshold, otherwise
/// [`OracleResult::Unverified`].  Non-grep payloads produce
/// [`OracleResult::Failed`].
///
/// # Examples
///
/// ```ignore
/// let result = validate_grep_payload(
///     &payload, &source, 0.8, || trace.clone(),
/// );
/// match result {
///     OracleResult::Golden(_) => println!("verified"),
///     _ => println!("not verified"),
/// }
/// ```
pub fn validate_grep_payload(
    payload: &FinalPayload,
    source: &str,
    confidence_threshold: f32,
    base_trace: impl FnOnce() -> ValidatedTrace,
) -> OracleResult {
    let grep_payload = match payload {
        FinalPayload::Grep(g) => g,
        _ => {
            let mut t = base_trace();
            t.verdict = "failed".into();
            return OracleResult::Failed {
                reason: "Expected grep payload".into(),
                diff: None,
                trace: t,
            };
        }
    };

    let _oracle = GrepOracle::new(source.to_string());
    let matched = grep_payload
        .matches
        .iter()
        .filter(|m| source.contains(&m.text))
        .count();
    let total = grep_payload.matches.len().max(1);
    let ratio = matched as f32 / total as f32;

    let mut trace = base_trace();
    if ratio >= confidence_threshold {
        trace.verification_method = VerificationMethod::GrepOracle;
        trace.verdict = "golden".into();
        OracleResult::Golden(trace)
    } else {
        trace.verdict = "unverified".into();
        OracleResult::Unverified {
            reason: format!("Grep match ratio {ratio:.2} below {confidence_threshold:.2}"),
            trace,
        }
    }
}
