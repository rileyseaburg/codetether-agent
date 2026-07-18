//! Public materialized-state types.

use super::super::{GoalStatus, SessionTaskStatus};
use chrono::{DateTime, Utc};
use std::collections::BTreeMap;

/// Current persisted thread goal and its accounting totals.
#[derive(Clone, Debug)]
pub struct Goal {
    pub id: String,
    pub objective: String,
    pub success_criteria: Vec<String>,
    pub forbidden: Vec<String>,
    pub status: GoalStatus,
    pub token_budget: Option<i64>,
    pub tokens_used: i64,
    pub time_used_seconds: i64,
    pub turns_used: u32,
    pub set_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub last_reaffirmed_at: DateTime<Utc>,
}

/// Task with its latest lifecycle status.
#[derive(Clone, Debug)]
pub struct Task {
    pub id: String,
    pub content: String,
    pub parent_id: Option<String>,
    pub status: SessionTaskStatus,
    pub last_note: Option<String>,
}

/// Folded view of the current goal and session tasks.
#[derive(Clone, Debug, Default)]
pub struct TaskState {
    pub goal: Option<Goal>,
    pub tasks: BTreeMap<String, Task>,
}
