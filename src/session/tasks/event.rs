//! Event types for the session task log.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Lifecycle status for a single task.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Done,
    Blocked,
    Cancelled,
}

/// A single durable event in the session task log.
///
/// The log is append-only; state changes are expressed as new events
/// rather than in-place edits.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum TaskEvent {
    /// Goal declared (or re-declared) for the session.
    GoalSet {
        at: DateTime<Utc>,
        objective: String,
        #[serde(default)]
        success_criteria: Vec<String>,
        #[serde(default)]
        forbidden: Vec<String>,
    },
    /// Agent explicitly reaffirmed progress toward the goal.
    GoalReaffirmed {
        at: DateTime<Utc>,
        progress_note: String,
    },
    /// Goal cleared (task complete, abandoned, or superseded).
    GoalCleared { at: DateTime<Utc>, reason: String },
    /// Task added to the session's todo list.
    TaskAdded {
        at: DateTime<Utc>,
        id: String,
        content: String,
        #[serde(default)]
        parent_id: Option<String>,
    },
    /// Task status transition.
    TaskStatus {
        at: DateTime<Utc>,
        id: String,
        status: TaskStatus,
        #[serde(default)]
        note: Option<String>,
    },
    /// Governance middleware detected drift.
    DriftDetected {
        at: DateTime<Utc>,
        tool_calls_since_reaffirm: u32,
        errors_since_reaffirm: u32,
    },
}
