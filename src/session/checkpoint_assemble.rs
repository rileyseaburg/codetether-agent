//! Shared assembly logic for checkpoint construction.

use super::{RunCheckpoint, checkpoint_defaults, checkpoint_state, checkpoint_update};
use std::path::PathBuf;

/// Assemble a [`RunCheckpoint`] from identity fields and extracted state.
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
