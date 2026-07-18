//! Current-state loading for goal runtime operations.

use crate::session::tasks::{TaskLog, TaskState};
use anyhow::Result;

pub(crate) async fn current(session_id: &str) -> Result<(TaskLog, TaskState)> {
    let log = TaskLog::for_session(session_id)?;
    let events = log.read_all().await?;
    Ok((log, TaskState::from_log(&events)))
}
