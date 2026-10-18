//! Default checkpoint value construction.

use super::{CheckpointReason, RunCheckpoint};
use chrono::Utc;

pub(super) fn empty_checkpoint() -> RunCheckpoint {
    RunCheckpoint {
        reason: CheckpointReason::MaxStepsExhausted,
        original_objective: String::new(),
        max_step_budget: 0,
        session_id: String::new(),
        workspace: None,
        message_count: 0,
        current_browser_url: None,
        completed_actions: Vec::new(),
        blockers: Vec::new(),
        next_intended_action: String::new(),
        created_at: Utc::now(),
    }
}
