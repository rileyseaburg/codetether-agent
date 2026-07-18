//! Terminal state after three repeated goal-level provider failures.

use super::super::Runner;
use crate::session::GoalStatus;

pub(super) async fn persist(runner: &Runner<'_>) {
    let status = if runner.progress.goal_failure_signature.as_deref() == Some("rate_limit") {
        GoalStatus::UsageLimited
    } else {
        GoalStatus::Blocked
    };
    if let Err(error) = crate::session::tasks::runtime::set_status(&runner.session.id, status).await
    {
        tracing::warn!(error = %error, "Failed to persist stopped goal status");
    }
}
