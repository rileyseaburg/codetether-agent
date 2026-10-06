//! `/goal show` rendering.

use crate::session::tasks::{TaskLog, TaskState, goal_block};
use anyhow::Result;

pub(super) async fn run(session_id: &str) -> Result<String> {
    let log = TaskLog::for_session(session_id)?;
    let state = TaskState::from_log(&log.read_all().await?);
    Ok(goal_block(&state).unwrap_or_else(|| {
        "No session goal. Use /goal set; work items are listed with /tasks.".to_string()
    }))
}
