//! Confirm the edit at its journal position, not after subsequent token accounting.

use crate::session::tasks::{TaskEvent, TaskState};
use chrono::{DateTime, Utc};

pub(super) fn edit(events: &[TaskEvent], at: DateTime<Utc>, clearing: bool) -> bool {
    let Some(index) = events
        .iter()
        .rposition(|event| matches!(event, TaskEvent::GoalEdited(edit) if edit.at == at))
    else {
        return false;
    };
    let state = TaskState::from_log(&events[..=index]);
    if clearing {
        state.goal.is_none()
    } else {
        state.goal.is_some_and(|goal| goal.last_updated_at == at)
    }
}

#[cfg(test)]
#[path = "tests/accepted.rs"]
mod tests;
