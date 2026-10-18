//! Native session specialization of goal status transitions.
pub(crate) async fn set_status(
    session_id: &str,
    status: crate::session::tasks::GoalStatus,
) -> anyhow::Result<bool> {
    codetether_goal_runtime::set_status::<super::journal::NativeJournal>(session_id, status).await
}
