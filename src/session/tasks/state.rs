//! Materialized view of a task log: current goal + open tasks.

use chrono::{DateTime, Utc};
use std::collections::BTreeMap;

use super::event::{TaskEvent, TaskStatus};

/// The session's currently-declared goal.
#[derive(Clone, Debug)]
pub struct Goal {
    pub objective: String,
    pub success_criteria: Vec<String>,
    pub forbidden: Vec<String>,
    pub set_at: DateTime<Utc>,
    pub last_reaffirmed_at: DateTime<Utc>,
}

/// A task with its latest status.
#[derive(Clone, Debug)]
pub struct Task {
    pub id: String,
    pub content: String,
    pub parent_id: Option<String>,
    pub status: TaskStatus,
    pub last_note: Option<String>,
}

/// Folded state derived from a [`super::TaskLog`].
#[derive(Clone, Debug, Default)]
pub struct TaskState {
    pub goal: Option<Goal>,
    pub tasks: BTreeMap<String, Task>,
    pub tool_calls_since_reaffirm: u32,
    pub errors_since_reaffirm: u32,
}

impl TaskState {
    /// Fold a sequence of events into current state.
    pub fn from_log(events: &[TaskEvent]) -> Self {
        let mut state = Self::default();
        for ev in events {
            state.apply(ev);
        }
        state
    }

    /// Tasks in a non-terminal state, in id order.
    pub fn open_tasks(&self) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|t| matches!(t.status, TaskStatus::Pending | TaskStatus::InProgress))
            .collect()
    }

    fn apply(&mut self, ev: &TaskEvent) {
        match ev {
            TaskEvent::GoalSet { at, objective, success_criteria, forbidden } => {
                self.goal = Some(Goal {
                    objective: objective.clone(),
                    success_criteria: success_criteria.clone(),
                    forbidden: forbidden.clone(),
                    set_at: *at,
                    last_reaffirmed_at: *at,
                });
            }
            TaskEvent::GoalReaffirmed { at, .. } => {
                if let Some(g) = self.goal.as_mut() {
                    g.last_reaffirmed_at = *at;
                }
                self.tool_calls_since_reaffirm = 0;
                self.errors_since_reaffirm = 0;
            }
            TaskEvent::GoalCleared { .. } => {
                self.goal = None;
            }
            TaskEvent::TaskAdded { id, content, parent_id, .. } => {
                self.tasks.insert(
                    id.clone(),
                    Task {
                        id: id.clone(),
                        content: content.clone(),
                        parent_id: parent_id.clone(),
                        status: TaskStatus::Pending,
                        last_note: None,
                    },
                );
            }
            TaskEvent::TaskStatus { id, status, note, .. } => {
                if let Some(t) = self.tasks.get_mut(id) {
                    t.status = status.clone();
                    t.last_note = note.clone();
                }
            }
            TaskEvent::DriftDetected { .. } => {}
        }
    }
}
