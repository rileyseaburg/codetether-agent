//! `/goal show` rendering.

use crate::session::tasks::{TaskLog, TaskState, governance_block};
use anyhow::Result;

pub(super) async fn run(session_id: &str) -> Result<String> {
    let log = TaskLog::for_session(session_id)?;
    let state = TaskState::from_log(&log.read_all().await?);
    Ok(governance_block(&state)
        .unwrap_or_else(|| "No goal and no tasks for this session.".to_string()))
}
