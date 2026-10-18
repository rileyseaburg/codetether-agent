//! Native journal handle specialization of cached goal-state loading.
use crate::tasks::{TaskLog, TaskState};
/// Load the session journal and its cached, folded goal state.
pub async fn current(session_id: &str) -> anyhow::Result<(TaskLog, TaskState)> {
    let (log, state) =
        codetether_goal_runtime::current::<super::journal::NativeJournal>(session_id).await?;
    Ok((TaskLog::at(log.path()), state))
}
