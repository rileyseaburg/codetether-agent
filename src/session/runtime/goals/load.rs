//! Native journal handle specialization of cached goal-state loading.
use crate::session::tasks::{TaskLog, TaskState};
pub(crate) async fn current(session_id: &str) -> anyhow::Result<(TaskLog, TaskState)> {
    let (log, state) =
        codetether_goal_runtime::current::<super::journal::NativeJournal>(session_id).await?;
    Ok((TaskLog::at(log.path()), state))
}
