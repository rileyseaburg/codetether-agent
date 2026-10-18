//! Compute prior-turn demand from bounded future windows.
use crate::{
    OracleReport,
    index::{index_files_per_turn, index_tool_call_owners},
    references::references_for_turn,
};
use codetether_provider_core::Message;

/// Compute an [`OracleReport`] over `messages` with horizon `h`.
///
/// Given the demand signal (lexical file references plus tool-call id
/// matches) the report records, for every turn `t`, the set of prior
/// turns needed to satisfy demand from `[t, t+h]`. Pareto harnesses
/// compare that set to what a [`codetether_context_policy::derivation::DerivePolicy`] actually kept.
/// # Arguments
/// * `messages` — Recorded messages in chronological order.
/// * `h` — Number of future turns to include beyond the current turn.
/// # Returns
/// Sorted, unique prior-turn references for each turn and the selected horizon.
/// # Examples
/// ```
/// let report = codetether_replay_oracle::replay_oracle(&[], 2);
/// assert!(report.demand.is_empty());
/// ```
pub fn replay_oracle(messages: &[Message], h: usize) -> OracleReport {
    let files_per_turn = index_files_per_turn(messages);
    let tool_call_owner = index_tool_call_owners(messages);

    let mut demand: Vec<Vec<usize>> = vec![Vec::new(); messages.len()];

    for t in 0..messages.len() {
        let window_end = (t + h).min(messages.len().saturating_sub(1));
        for future_t in t..=window_end {
            let future = &messages[future_t];
            for u in references_for_turn(future, t, &files_per_turn, &tool_call_owner) {
                if u < t && !demand[t].contains(&u) {
                    demand[t].push(u);
                }
            }
        }
        demand[t].sort_unstable();
    }

    OracleReport { demand, horizon: h }
}
