//! Native audit journal and revision-checked transitions.
use crate::Host;
use anyhow::Result;
use codetether_goal_verification::VerdictRecord;
use codetether_native_goals::{Goal, GoalStatus, runtime};

impl<F> codetether_goal_verification::audit::Host for Host<F> {
    async fn read(session: &str) -> Result<Vec<VerdictRecord>> {
        crate::verdict_log::read(session).await
    }
    async fn append(session: &str, record: &VerdictRecord) -> Result<()> {
        crate::verdict_log::append(session, record).await
    }
    async fn set_status_if_current(session: &str, goal: &Goal, status: GoalStatus) -> Result<bool> {
        runtime::set_status_if_current(session, goal, status).await
    }
}
