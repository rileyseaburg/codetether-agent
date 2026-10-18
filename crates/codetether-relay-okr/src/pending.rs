//! Proposed relay objective and run awaiting a user decision.
use codetether_okr::{Okr, OkrRun};

/// Pending OKR approval gate state for PRD-gated relay commands.
/// # Examples
/// ```
/// let pending = codetether_relay_okr::PendingOkrApproval::new("task".into(), 2, "model".into());
/// assert_eq!(pending.agent_count, 2);
/// ```
pub struct PendingOkrApproval {
    /// The OKR being proposed
    pub okr: Okr,
    /// The OKR run being proposed
    pub run: OkrRun,
    /// Optional note when we had to fall back to a template draft
    pub draft_note: Option<String>,
    /// Original task that triggered the OKR
    pub task: String,
    /// Agent count for the relay
    pub agent_count: usize,
    /// Model to use
    pub model: String,
}
