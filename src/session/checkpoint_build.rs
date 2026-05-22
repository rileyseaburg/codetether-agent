//! Constructors for resumable run checkpoints.

use super::{RunCheckpoint, checkpoint_assemble, checkpoint_state};
use std::path::PathBuf;

impl RunCheckpoint {
    /// Build from live session messages.
    pub fn from_session_messages(
        objective: impl Into<String>,
        max_steps: usize,
        session_id: impl Into<String>,
        workspace: Option<PathBuf>,
        message_count: usize,
        messages: &[crate::provider::Message],
    ) -> Self {
        let extracted = checkpoint_state::extract(messages);
        checkpoint_assemble::assemble(
            objective,
            max_steps,
            session_id,
            workspace,
            message_count,
            extracted,
        )
    }
}
