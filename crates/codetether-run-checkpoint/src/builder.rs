//! Stream canonical history into a checkpoint without retaining raw messages.
use crate::{
    RunCheckpoint,
    checkpoint_walk::{ScanState, scan_message},
};
use codetether_provider_core::Message;
use std::path::PathBuf;

/// Ordered checkpoint accumulator for a complete canonical transcript.
/// Raw tool payloads are released after each message; source-linked evidence remains.
/// # Examples
/// ```
/// let builder = codetether_run_checkpoint::CheckpointBuilder::default();
/// let checkpoint = builder.finish("task", 10, "session", None);
/// assert_eq!(checkpoint.message_count, 0);
/// ```
#[derive(Default)]
pub struct CheckpointBuilder {
    state: ScanState,
    count: usize,
}
impl CheckpointBuilder {
    /// Fold the next canonical message, rejecting gaps or reordered sources.
    /// # Arguments
    /// * `index`, `message` — Absolute index and original message.
    /// # Returns
    /// Unit after incorporating the evidence.
    /// # Errors
    /// Returns an error unless the sequence starts at zero and is contiguous.
    /// # Examples
    /// ```
    /// use codetether_provider_core::{Message, Role};
    /// let mut builder = codetether_run_checkpoint::CheckpointBuilder::default();
    /// assert!(builder.push(1, &Message { role: Role::User, content: vec![] }).is_err());
    /// ```
    pub fn push(&mut self, index: usize, message: &Message) -> anyhow::Result<()> {
        anyhow::ensure!(
            index == self.count,
            "checkpoint source gap: expected {}, got {index}",
            self.count
        );
        self.state.source_turn = index;
        scan_message(message, &mut self.state);
        self.count += 1;
        Ok(())
    }
    /// Finish the complete stream, retaining all extracted source evidence.
    /// # Arguments
    /// `objective`, `max_steps`, `session_id`, `workspace` identify the interrupted run.
    /// # Returns
    /// A checkpoint with the absolute canonical message count.
    /// # Examples
    /// See [`CheckpointBuilder`].
    pub fn finish(
        mut self,
        objective: impl Into<String>,
        max_steps: usize,
        session_id: impl Into<String>,
        workspace: Option<PathBuf>,
    ) -> RunCheckpoint {
        self.state.next_action.get_or_insert_with(|| {
            "Continue from the last assistant/tool state toward the original objective.".into()
        });
        crate::checkpoint_assemble::assemble(
            objective,
            max_steps,
            session_id,
            workspace,
            self.count,
            self.state.into_extracted(),
        )
    }
}
