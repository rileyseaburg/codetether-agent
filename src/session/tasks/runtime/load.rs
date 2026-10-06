//! Current-state loading for goal runtime operations.

use crate::session::tasks::{TaskLog, TaskState, state_cache};
use anyhow::Result;

/// Load the session task log and its folded state.
///
/// Uses the length/mtime-keyed fold cache so per-step usage accounting does
/// not re-parse an ever-growing log on every model step.
pub(crate) async fn current(session_id: &str) -> Result<(TaskLog, TaskState)> {
    let log = TaskLog::for_session(session_id)?;
    let state = state_cache::load(&log)?;
    Ok((log, state))
}
