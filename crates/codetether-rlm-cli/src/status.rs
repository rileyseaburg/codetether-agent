//! RLM CLI status handling.
use codetether_rlm as rlm;

/// Project RLM CLI status information.
/// # Arguments
/// * `result` — Oracle verification outcome.
/// # Returns
/// Human-readable status with the existing confidence formatting.
/// # Examples
/// ```no_run
/// # fn example(result: &codetether_rlm::OracleResult) {
/// let _ = codetether_rlm_cli::oracle_status_line(result);
/// # }
/// ```
pub fn oracle_status_line(result: &rlm::OracleResult) -> String {
    match result {
        rlm::OracleResult::Golden(_) => {
            "[oracle: golden ✓] deterministic verification passed".to_string()
        }
        rlm::OracleResult::Consensus {
            agreement_ratio, ..
        } => format!(
            "[oracle: consensus ✓] semantic agreement {:.1}%",
            agreement_ratio * 100.0
        ),
        rlm::OracleResult::Unverified { reason, .. } => {
            format!("[oracle: unverified —] {}", reason)
        }
        rlm::OracleResult::Failed { reason, .. } => format!("[oracle: failed ✗] {}", reason),
    }
}
