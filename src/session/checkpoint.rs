//! Resumable run checkpoint data persisted beside session metadata.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RunCheckpoint {
    pub reason: CheckpointReason,
    pub original_objective: String,
    pub max_step_budget: usize,
    pub session_id: String,
    pub workspace: Option<PathBuf>,
    pub message_count: usize,
    pub current_browser_url: Option<String>,
    pub completed_actions: Vec<String>,
    pub blockers: Vec<String>,
    pub next_intended_action: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointReason {
    MaxStepsExhausted,
}

impl RunCheckpoint {
    pub fn exhausted(
        objective: impl Into<String>,
        max_steps: usize,
        session_id: impl Into<String>,
        workspace: Option<PathBuf>,
        message_count: usize,
    ) -> Self {
        Self {
            reason: CheckpointReason::MaxStepsExhausted,
            original_objective: objective.into(),
            max_step_budget: max_steps,
            session_id: session_id.into(),
            workspace,
            message_count,
            current_browser_url: None,
            completed_actions: Vec::new(),
            blockers: Vec::new(),
            next_intended_action:
                "Continue from the last assistant/tool state toward the original objective.".into(),
            created_at: Utc::now(),
        }
    }
}
