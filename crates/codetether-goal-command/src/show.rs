//! `/goal show` rendering.
use crate::Host;

use anyhow::Result;
use codetether_goal_core::{TaskState, goal_block};

pub(super) async fn run<H: Host>(session_id: &str) -> Result<String> {
    let log = H::log(session_id)?;
    let state = TaskState::from_log(&log.read_all().await?);
    Ok(goal_block(&state).unwrap_or_else(|| {
        "No session goal. Use /goal set; work items are listed with /tasks.".to_string()
    }))
}
