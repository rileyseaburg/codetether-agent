//! Liveness classification for sub-agent status.
//!
//! A sub-agent reporting `Working` but silent for a long time is likely
//! wedged. This turns the last-update timestamp into a human/agent-readable
//! liveness verdict so the parent can decide whether to wait, re-message, or
//! kill a stuck sub-agent.

use chrono::{DateTime, Utc};

use crate::a2a::types::TaskState;

/// Seconds a `Working` agent may stay silent before it is flagged as stalled.
pub(super) const STALL_SECS: i64 = 120;

/// Coarse liveness verdict for a sub-agent.
pub(super) fn liveness(state: &TaskState, last_update: DateTime<Utc>) -> &'static str {
    if state.is_terminal() {
        return "settled";
    }
    let idle = (Utc::now() - last_update).num_seconds();
    match state {
        TaskState::Working if idle >= STALL_SECS => "stalled",
        TaskState::Working => "active",
        TaskState::Submitted => "queued",
        TaskState::InputRequired | TaskState::AuthRequired => "waiting",
        _ => "active",
    }
}
