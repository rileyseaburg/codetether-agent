//! AST-based payload validation.

use super::schema::FinalPayload;
use super::trace_types::{OracleResult, ValidatedTrace};
use super::types::VerificationMethod;

/// Validate an AST-type payload against source.
pub fn validate_ast_payload(
    payload: &FinalPayload,
    source: &str,
    confidence_threshold: f32,
    base_trace: impl FnOnce() -> ValidatedTrace,
) -> OracleResult {
    let ast_payload = match payload {
        FinalPayload::Ast(a) => a,
        _ => {
            let mut t = base_trace();
            t.verdict = "failed".into();
            return OracleResult::Failed {
                reason: "Expected AST payload".into(),
                diff: None,
                trace: t,
            };
        }
    };

    // Check if AST results reference valid items
    let matched = ast_payload
        .results
        .iter()
        .filter(|r| source.contains(&r.name))
        .count();
    let total = ast_payload.results.len().max(1);
    let ratio = matched as f32 / total as f32;

    let mut trace = base_trace();
    if ratio >= confidence_threshold {
        trace.verification_method = VerificationMethod::AstOracle;
        trace.verdict = "golden".into();
        OracleResult::Golden(trace)
    } else {
        trace.verdict = "unverified".into();
        OracleResult::Unverified {
            reason: format!("AST match ratio {ratio:.2} below {confidence_threshold:.2}"),
            trace,
        }
    }
}
