//! Preserve native runtime state and model-edit controller boundaries.
use crate::session::tasks::{control, runtime};
use anyhow::Result;
use codetether_goal_core::{GoalEdit, TaskLog, TaskState, control::GoalControlError};
use serde_json::Value;
pub(super) struct Native;
impl codetether_goal_tools::Host for Native {
    async fn current(session: &str) -> Result<(TaskLog, TaskState)> {
        let (log, state) = runtime::current(session).await?;
        Ok((TaskLog::at(log.path().to_path_buf()), state))
    }
    async fn update(session: &str, edit: GoalEdit) -> Result<Value, GoalControlError> {
        control::update(session, edit).await
    }
}
