//! Task-specific state transitions.

use super::{Task, TaskState};
use crate::session::tasks::SessionTaskStatus;

pub(super) fn add(state: &mut TaskState, id: &str, content: &str, parent: &Option<String>) {
    state.tasks.insert(
        id.into(),
        Task {
            id: id.into(),
            content: content.into(),
            parent_id: parent.clone(),
            status: SessionTaskStatus::Pending,
            last_note: None,
        },
    );
}

pub(super) fn status(
    state: &mut TaskState,
    id: &str,
    status: &SessionTaskStatus,
    note: &Option<String>,
) {
    if let Some(task) = state.tasks.get_mut(id) {
        task.status = status.clone();
        task.last_note = note.clone();
    }
}
