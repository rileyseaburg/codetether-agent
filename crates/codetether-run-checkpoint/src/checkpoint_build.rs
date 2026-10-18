//! Constructors for resumable run checkpoints.

use super::{RunCheckpoint, checkpoint_assemble, checkpoint_state};
use std::path::PathBuf;

impl RunCheckpoint {
    /// Build from live session messages.
    /// # Arguments
    /// Identity inputs preserve the original objective, budget, session, workspace
    /// and count; `messages` supplies the ordered transcript to inspect.
    /// # Returns
    /// A checkpoint with browser URL, action, blocker and next-action context.
    /// # Panics
    /// Existing byte-based text truncation may panic when its cutoff splits a
    /// multibyte UTF-8 character.
    /// # Examples
    /// ```rust
    /// let cp = codetether_run_checkpoint::RunCheckpoint::from_session_messages(
    ///     "Continue the task", 10, "session-123", None, 0, &[],
    /// );
    /// assert_eq!(cp.session_id, "session-123");
    /// ```
    pub fn from_session_messages(
        objective: impl Into<String>,
        max_steps: usize,
        session_id: impl Into<String>,
        workspace: Option<PathBuf>,
        message_count: usize,
        messages: &[codetether_provider_core::Message],
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
