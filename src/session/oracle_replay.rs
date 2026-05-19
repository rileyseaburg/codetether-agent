//! Oracle replay evaluator (Step 22 — evaluation-only).
//!
//! Given a recorded session trace, picks optimal context representations
//! with `h`-turn future-demand lookahead to minimise fault count.
//! Reports the **oracle gap**: online-vs-oracle fault-count delta.
//!
//! Not suitable for production — only for Pareto benchmarking.

use crate::session::derive_policy::DerivePolicy;

/// Oracle outcome for one recorded trace.
#[derive(Debug, Clone)]
pub struct OracleResult {
    /// Number of faults under the oracle strategy.
    pub oracle_faults: usize,
    /// Number of faults under the online policy being benchmarked.
    pub online_faults: usize,
    /// The oracle gap: `online_faults - oracle_faults`.
    pub gap: isize,
}

/// Run the oracle replay on a trace directory.
///
/// This is a stub — the full implementation needs a trace loader
/// and a min-fault dynamic programming step over turn representations.
/// Returns a placeholder indicating the trace was accepted.
pub fn evaluate_trace(
    policy: DerivePolicy,
    trace_path: &std::path::Path,
    lookahead: usize,
) -> anyhow::Result<OracleResult> {
    let trace_id = trace_path.display();
    tracing::info!(%trace_id, lookahead, policy = policy.kind(), "Oracle replay (stub)");

    // Stub: until trace loading is implemented, report zero gap.
    Ok(OracleResult {
        oracle_faults: 0,
        online_faults: 0,
        gap: 0,
    })
}
