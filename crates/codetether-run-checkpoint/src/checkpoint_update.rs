//! Field updates for checkpoint construction.

use super::{CheckpointReason, RunCheckpoint, checkpoint_state::ExtractedState};
use std::path::PathBuf;

pub(super) fn apply_identity(
    checkpoint: &mut RunCheckpoint,
    objective: String,
    max_steps: usize,
    session_id: String,
    workspace: Option<PathBuf>,
    message_count: usize,
) {
    checkpoint.reason = CheckpointReason::MaxStepsExhausted;
    checkpoint.original_objective = objective;
    checkpoint.max_step_budget = max_steps;
    checkpoint.session_id = session_id;
    checkpoint.workspace = workspace;
    checkpoint.message_count = message_count;
}

pub(super) fn apply_extracted(checkpoint: &mut RunCheckpoint, extracted: ExtractedState) {
    checkpoint.current_browser_url = extracted.browser_url;
    checkpoint.completed_actions = extracted.completed_actions;
    checkpoint.blockers = extracted.blockers;
    checkpoint.next_intended_action = extracted.next_action;
}
