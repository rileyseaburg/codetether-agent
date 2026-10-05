//! Event dispatch for task-state folding.

use super::{TaskState, goal, task};
use crate::session::tasks::TaskEvent;
#[path = "goal_status_checked.rs"]
mod checked;
#[path = "goal_declaration.rs"]
mod declaration;

impl TaskState {
    pub(super) fn apply(&mut self, event: &TaskEvent) {
        match event {
            TaskEvent::GoalSet { .. } => declaration::apply(self, event),
            TaskEvent::GoalRuntime(update) => goal::runtime(self, update),
            TaskEvent::GoalStatusChecked(update) => checked::apply(self, update),
            TaskEvent::AnswerReview(update) => super::answer_review::apply(self, update),
            TaskEvent::GoalReaffirmed(note) => goal::reaffirm(self, note.at),
            TaskEvent::GoalCleared { .. } => {
                self.goal = None;
                self.answer_review = None;
            }
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
            TaskEvent::DriftDetected(_) => {}
            TaskEvent::GoalEdited(edit) => goal::edit(self, edit),
        }
    }
}
