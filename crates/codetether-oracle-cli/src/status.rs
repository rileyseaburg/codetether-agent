//! Oracle CLI status responsibilities.
use codetether_rlm::oracle::OracleResult;

pub(super) fn oracle_status_line(result: &OracleResult) -> String {
    match result {
        OracleResult::Golden(_) => {
            "[oracle: golden ✓] deterministic verification passed".to_string()
        }
        OracleResult::Consensus {
            agreement_ratio, ..
        } => format!(
            "[oracle: consensus ✓] semantic agreement {:.1}%",
            agreement_ratio * 100.0
        ),
        OracleResult::Unverified { reason, .. } => {
            format!("[oracle: unverified —] {}", reason)
        }
        OracleResult::Failed { reason, .. } => format!("[oracle: failed ✗] {}", reason),
    }
}
