//! Event dispatch for task-state folding.

use super::{TaskState, goal, task};
use crate::session::tasks::TaskEvent;

impl TaskState {
    pub(super) fn apply(&mut self, event: &TaskEvent) {
        match event {
            TaskEvent::GoalSet {
                at,
                goal_id,
                objective,
                success_criteria,
                forbidden,
                ..
            } => {
                goal::set(self, *at, goal_id, objective, success_criteria, forbidden);
            }
            TaskEvent::GoalRuntime(update) => goal::runtime(self, update),
            TaskEvent::GoalReaffirmed { at, .. } => goal::reaffirm(self, *at),
            TaskEvent::GoalCleared { .. } => self.goal = None,
            TaskEvent::TaskAdded {
                id,
                content,
                parent_id,
                ..
            } => {
                task::add(self, id, content, parent_id);
            }
            TaskEvent::TaskStatus {
                id, status, note, ..
            } => {
                task::status(self, id, status, note);
            }
            TaskEvent::DriftDetected { .. } => {}
        }
    }
}
