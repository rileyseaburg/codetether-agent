//! Borrowed final facts needed to render the CLI summary.
/// Relay execution facts used for summary formatting.
/// # Examples
/// ```
/// let facts = codetether_relay_report::Summary { status: "converged", relay_id: "r1", agent_count: 2, turns: 2, final_handoff: "done", failure: None, planner_used: false, rlm_handoff_count: 0, rlm_context_count: 0, shared_context_items: 0, dynamic_spawn_count: 0 };
/// assert!(codetether_relay_report::summary(&facts).contains("done"));
/// ```
pub struct Summary<'a> {
    /// Final status.
    pub status: &'a str,
    /// Relay identity.
    pub relay_id: &'a str,
    /// Final team size.
    pub agent_count: usize,
    /// Executed turns.
    pub turns: usize,
    /// Final handoff, truncated only for summary display.
    pub final_handoff: &'a str,
    /// Optional failure detail.
    pub failure: Option<&'a str>,
    /// Whether model-generated team planning was used.
    pub planner_used: bool,
    /// RLM-refined handoffs.
    pub rlm_handoff_count: usize,
    /// RLM-refined shared-context deltas.
    pub rlm_context_count: usize,
    /// Final shared-context item count.
    pub shared_context_items: usize,
    /// Dynamically spawned specialists.
    pub dynamic_spawn_count: usize,
}
