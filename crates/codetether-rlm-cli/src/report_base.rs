//! Oracle variant-specific JSON report fields.
use codetether_rlm as rlm;
pub(super) fn value(result: &rlm::OracleResult) -> serde_json::Value {
    match result {
        rlm::OracleResult::Golden(trace) => serde_json::json!({
            "status": "golden",
            "verification_method": format!("{:?}", trace.verification_method),
            "verdict": trace.verdict,
            "trace_id": trace.trace_id,
        }),
        rlm::OracleResult::Consensus {
            trace,
            agreement_ratio,
        } => serde_json::json!({
            "status": "consensus",
            "agreement_ratio": agreement_ratio,
            "verification_method": format!("{:?}", trace.verification_method),
            "verdict": trace.verdict,
            "trace_id": trace.trace_id,
        }),
        rlm::OracleResult::Unverified { reason, trace } => serde_json::json!({
            "status": "unverified",
            "reason": reason,
            "verification_method": format!("{:?}", trace.verification_method),
            "verdict": trace.verdict,
            "trace_id": trace.trace_id,
        }),
        rlm::OracleResult::Failed {
            reason,
            diff,
            trace,
        } => serde_json::json!({
            "status": "failed",
            "reason": reason,
            "diff": diff,
            "verification_method": format!("{:?}", trace.verification_method),
            "verdict": trace.verdict,
            "trace_id": trace.trace_id,
        }),
    }
}
