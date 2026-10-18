//! Native session specialization of revision-checked goal transitions.
use crate::tasks::{Goal, GoalStatus};
pub async fn set_status_if_current(
    session: &str,
    expected: &Goal,
    status: GoalStatus,
) -> anyhow::Result<bool> {
    codetether_goal_runtime::set_status_if_current::<super::journal::NativeJournal>(
        session, expected, status,
    )
    .await
}
