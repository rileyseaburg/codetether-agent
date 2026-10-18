//! Preserve native journal paths, runtime state and user-edit authorization.
use crate::session::tasks::{control, runtime};
use anyhow::Result;
use codetether_goal_core::{Goal, GoalEdit, TaskLog};
use serde_json::Value;
pub(super) struct Native;
impl codetether_goal_command::Host for Native {
    fn log(session: &str) -> Result<TaskLog> {
        Ok(TaskLog::at(crate::session::tasks::task_log_path(session)?))
    }
    fn goal(session: &str) -> Result<Option<Goal>> {
        Ok(runtime::answer_review::read(session)?.goal)
    }
    async fn update_user(session: &str, request: GoalEdit) -> Result<Value> {
        Ok(control::update_user(session, request).await?)
    }
    fn auto_goal_enabled() -> bool {
        runtime::auto_goal_enabled()
    }
    fn set_auto_goal(enabled: bool) {
        runtime::set_auto_goal(enabled);
    }
}
