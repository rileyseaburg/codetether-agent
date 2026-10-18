//! Final relay execution facts used to create outcome evidence.
/// Facts recorded for every final key-result outcome.
/// # Examples
/// ```
/// let report = codetether_relay_okr::progress::Report { relay_id: "r1", turns: 2, agent_count: 2, status: "converged", rlm_handoff_count: 1, dynamic_spawn_count: 0 };
/// assert_eq!(report.turns, 2);
/// ```
pub struct Report<'a> {
    /// Relay identity.
    pub relay_id: &'a str,
    /// Number of executed turns.
    pub turns: usize,
    /// Final team size.
    pub agent_count: usize,
    /// Relay completion or failure status.
    pub status: &'a str,
    /// Number of RLM-refined handoffs.
    pub rlm_handoff_count: usize,
    /// Number of dynamic specialist spawns.
    pub dynamic_spawn_count: usize,
}
