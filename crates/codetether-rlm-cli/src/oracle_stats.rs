//! Bucket one oracle verdict for split dataset output.
use codetether_rlm as rlm;
pub(super) fn stats(oracle_result: &rlm::OracleResult) -> rlm::oracle::BatchValidationStats {
    let mut stats = rlm::oracle::BatchValidationStats::default();
    match oracle_result {
        rlm::OracleResult::Golden(trace) => stats.golden.push(trace.clone()),
        rlm::OracleResult::Consensus { trace, .. } => stats.consensus.push(trace.clone()),
        rlm::OracleResult::Unverified { reason, trace } => {
            stats.unverified.push((trace.clone(), reason.clone()));
        }
        rlm::OracleResult::Failed { reason, trace, .. } => {
            stats.failed.push((trace.clone(), reason.clone()));
        }
    }

    stats
}
