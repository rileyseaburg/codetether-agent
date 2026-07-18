//! Manual continuation prompt for a persisted active goal.

use super::prompt::render;
use crate::session::tasks::{TaskLog, TaskState};

pub(crate) fn prompt(session_id: &str) -> Option<String> {
    let log = TaskLog::for_session(session_id).ok()?;
    TaskState::from_log(&log.read_all_blocking().ok()?)
        .goal
        .filter(|goal| goal.status.is_active())
        .map(|goal| render(&goal))
}
