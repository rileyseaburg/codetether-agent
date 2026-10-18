//! Serializable final result of a CLI relay.
use serde::Serialize;

/// Final relay result serialized by the run command.
/// # Examples
/// ```
/// let result = codetether_relay_report::AutochatCliResult { status: "converged".into(), relay_id: "r1".into(), model: "provider/model".into(), agent_count: 2, turns: 2, agents: vec![], final_handoff: "done".into(), summary: "summary".into(), failure: None, shared_context_items: 0, rlm_context_count: 0 };
/// assert_eq!(result.turns, 2);
/// ```
#[derive(Debug, Serialize)]
pub struct AutochatCliResult {
    /// Final relay status.
    pub status: String,
    /// Relay identity.
    pub relay_id: String,
    /// Requested model reference.
    pub model: String,
    /// Final team size.
    pub agent_count: usize,
    /// Executed turn count.
    pub turns: usize,
    /// Final ordered agent names.
    pub agents: Vec<String>,
    /// Full final handoff.
    pub final_handoff: String,
    /// Rendered human-readable summary.
    pub summary: String,
    /// Optional failure detail.
    pub failure: Option<String>,
    /// Final shared-context size.
    pub shared_context_items: usize,
    /// Count of RLM context refinements.
    pub rlm_context_count: usize,
}
