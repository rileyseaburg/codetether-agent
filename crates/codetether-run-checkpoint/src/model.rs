//! Persisted resumable-run checkpoint data.
use crate::CheckpointReason;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A structured checkpoint for a step-budget-exhausted `codetether run`.
/// # Examples
/// ```rust
/// let checkpoint = codetether_run_checkpoint::RunCheckpoint::from_session_messages(
///     "Review the project", 20, "session-123", None, 0, &[],
/// );
/// assert_eq!(checkpoint.original_objective, "Review the project");
/// assert_eq!(checkpoint.max_step_budget, 20);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RunCheckpoint {
    /// Why execution stopped.
    pub reason: CheckpointReason,
    /// Original user objective to preserve on resumption.
    pub original_objective: String,
    /// Step budget of the interrupted run.
    pub max_step_budget: usize,
    /// Durable identity of the existing session.
    pub session_id: String,
    /// Optional workspace belonging to the run.
    pub workspace: Option<PathBuf>,
    /// Number of messages present when checkpointing.
    pub message_count: usize,
    /// Last known browser URL extracted from tool calls in the session.
    pub current_browser_url: Option<String>,
    /// Tool call names that completed successfully before checkpoint.
    pub completed_actions: Vec<String>,
    /// Errors or blockers encountered before checkpoint.
    pub blockers: Vec<String>,
    /// Inferred next action based on the last assistant message.
    pub next_intended_action: String,
    /// Checkpoint creation timestamp.
    pub created_at: DateTime<Utc>,
}
