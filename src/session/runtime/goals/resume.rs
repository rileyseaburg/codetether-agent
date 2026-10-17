//! Manual continuation prompt for a persisted active goal.

use super::prompt::render;
use crate::session::tasks::{TaskLog, state_cache};

pub(crate) fn prompt(session_id: &str) -> Option<String> {
    let log = TaskLog::for_session(session_id).ok()?;
    state_cache::load(&log)
        .ok()?
        .goal
        .filter(|goal| goal.status.is_active())
        .map(|goal| render(&goal))
}
