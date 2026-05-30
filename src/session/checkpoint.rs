//! Resumable run checkpoint data persisted beside session metadata.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[path = "checkpoint_assemble.rs"]
mod checkpoint_assemble;
#[path = "checkpoint_assistant.rs"]
mod checkpoint_assistant;
#[path = "checkpoint_build.rs"]
mod checkpoint_build;
#[path = "checkpoint_defaults.rs"]
mod checkpoint_defaults;
#[path = "checkpoint_state.rs"]
mod checkpoint_state;
#[path = "checkpoint_text.rs"]
mod checkpoint_text;
#[path = "checkpoint_tool.rs"]
mod checkpoint_tool;
#[path = "checkpoint_update.rs"]
mod checkpoint_update;
#[path = "checkpoint_walk.rs"]
mod checkpoint_walk;

/// A structured checkpoint for a step-budget-exhausted `codetether run`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RunCheckpoint {
    pub reason: CheckpointReason,
    pub original_objective: String,
    pub max_step_budget: usize,
    pub session_id: String,
    pub workspace: Option<PathBuf>,
    pub message_count: usize,
    /// Last known browser URL extracted from tool calls in the session.
    pub current_browser_url: Option<String>,
    /// Tool call names that completed successfully before checkpoint.
    pub completed_actions: Vec<String>,
    /// Errors or blockers encountered before checkpoint.
    pub blockers: Vec<String>,
    /// Inferred next action based on the last assistant message.
    pub next_intended_action: String,
    pub created_at: DateTime<Utc>,
}

/// The reason a resumable run checkpoint was created.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointReason {
    /// The run stopped because its configured maximum step budget was reached.
    MaxStepsExhausted,
}
