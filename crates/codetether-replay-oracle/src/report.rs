//! Demand edges observed in a replay window.

/// Summary of an oracle replay over a recorded trace.
/// # Examples
/// ```
/// let report = codetether_replay_oracle::OracleReport { demand: vec![vec![], vec![0]], horizon: 1 };
/// assert_eq!(report.reference_count(), 1);
/// ```
#[derive(Debug, Clone)]
pub struct OracleReport {
    /// For each turn `t` in the trace, the sorted list of prior turn
    /// indices `u < t` that `t` references within the `h`-turn
    /// lookahead window.
    pub demand: Vec<Vec<usize>>,
    /// Horizon the oracle used.
    pub horizon: usize,
}

impl OracleReport {
    /// Number of distinct (t, u) reference edges the oracle observed.
    ///
    /// Useful as a sanity check: zero edges on a long trace usually
    /// means the heuristic wasn't broad enough, not that the agent was
    /// uninteresting.
    pub fn reference_count(&self) -> usize {
        self.demand.iter().map(Vec::len).sum()
    }
}
