//! Oracle CLI json responsibilities.
use codetether_rlm::oracle::OracleResult;
use serde_json::json;

pub(super) fn oracle_json(result: &OracleResult) -> serde_json::Value {
    match result {
        OracleResult::Golden(trace) => json!({
            "status": "golden",
            "verdict": trace.verdict,
            "verification_method": format!("{:?}", trace.verification_method),
            "trace_id": trace.trace_id,
        }),
        OracleResult::Consensus {
            trace,
            agreement_ratio,
        } => json!({
            "status": "consensus",
            "agreement_ratio": agreement_ratio,
            "verdict": trace.verdict,
            "verification_method": format!("{:?}", trace.verification_method),
            "trace_id": trace.trace_id,
        }),
        OracleResult::Unverified { reason, trace } => json!({
            "status": "unverified",
            "reason": reason,
            "verdict": trace.verdict,
            "verification_method": format!("{:?}", trace.verification_method),
            "trace_id": trace.trace_id,
        }),
        OracleResult::Failed {
            reason,
            diff,
            trace,
        } => json!({
            "status": "failed",
            "reason": reason,
            "diff": diff,
            "verdict": trace.verdict,
            "verification_method": format!("{:?}", trace.verification_method),
            "trace_id": trace.trace_id,
        }),
    }
}
