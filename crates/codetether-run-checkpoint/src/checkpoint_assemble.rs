//! Shared assembly logic for checkpoint construction.
//! Assembly helpers for resumable run checkpoints.
//!
//! This module combines checkpoint identity data supplied by the caller with
//! state extracted from the session transcript. It keeps construction logic
//! separate from extraction and field-update helpers so each checkpoint module
//! has a single responsibility.

use super::{RunCheckpoint, checkpoint_defaults, checkpoint_state, checkpoint_update};
use std::path::PathBuf;

/// Assemble a [`RunCheckpoint`] from identity fields and extracted state.
///
/// The caller provides the stable identity fields for the interrupted run:
/// original objective, exhausted step budget, session identifier, optional
/// workspace path, and message count. The `extracted` argument supplies runtime
/// details recovered from transcript messages, such as browser state, completed
/// actions, blockers, and the inferred next action.
///
/// The function starts from [`checkpoint_defaults::empty_checkpoint`], applies
/// identity fields, then applies extracted transcript state. It performs no I/O
/// and has no side effects outside the returned checkpoint.
///
/// # Arguments
///
/// * `objective` - Original user objective for the run being resumed.
/// * `max_steps` - Step budget that was exhausted.
/// * `session_id` - Identifier of the session that owns the checkpoint.
/// * `workspace` - Optional workspace path associated with the run.
/// * `message_count` - Number of session messages present when checkpointing.
/// * `extracted` - Transcript-derived checkpoint state.
///
/// # Returns
///
/// A populated [`RunCheckpoint`] ready to persist or turn into a resume prompt.
pub(crate) fn assemble(
    objective: impl Into<String>,
    max_steps: usize,
    session_id: impl Into<String>,
    workspace: Option<PathBuf>,
    message_count: usize,
    extracted: checkpoint_state::ExtractedState,
) -> RunCheckpoint {
    let mut checkpoint = checkpoint_defaults::empty_checkpoint();
    checkpoint_update::apply_identity(
        &mut checkpoint,
        objective.into(),
        max_steps,
        session_id.into(),
        workspace,
        message_count,
    );
    checkpoint_update::apply_extracted(&mut checkpoint, extracted);
    checkpoint
}
