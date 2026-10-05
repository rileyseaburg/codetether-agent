use super::super::Runner;
use crate::session::GoalStatus;

/// Status recorded when repeated provider failures stop a goal.
///
/// Runtime failures never produce `Blocked`: that status is reserved for a
/// blocker the independent verifier confirmed, and the model may clear a
/// blocked goal. Rate limits become `UsageLimited`; anything else `Paused`,
/// so the user decides whether to `/goal resume`.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(stopped_status(Some("rate_limit")), GoalStatus::UsageLimited);
/// assert_eq!(stopped_status(Some("server_error")), GoalStatus::Paused);
/// ```
pub(super) fn stopped_status(signature: Option<&str>) -> GoalStatus {
    if signature == Some("rate_limit") {
        GoalStatus::UsageLimited
    } else {
        GoalStatus::Paused
    }
}

/// Persist the stopped status for the runner's goal.
pub(super) async fn persist(runner: &Runner<'_>) {
    let status = stopped_status(runner.progress.goal_failure_signature.as_deref());
    if let Err(error) = crate::session::tasks::runtime::set_status(&runner.session.id, status).await
    {
        tracing::warn!(error = %error, "Failed to persist stopped goal status");
    }
}

#[cfg(test)]
mod tests {
    use super::stopped_status;
    use crate::session::GoalStatus;

    #[test]
    fn runtime_failures_never_mark_goal_blocked() {
        assert_eq!(stopped_status(Some("rate_limit")), GoalStatus::UsageLimited);
        assert_eq!(stopped_status(Some("server_error")), GoalStatus::Paused);
        assert_eq!(stopped_status(None), GoalStatus::Paused);
    }
}
