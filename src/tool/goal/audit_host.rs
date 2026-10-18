//! Native audit journal and revision-checked transitions.
use super::host::Native;
use crate::session::tasks::{Goal, GoalStatus, runtime};
use anyhow::Result;
use codetether_goal_verification::VerdictRecord;

impl codetether_goal_verification::audit::Host for Native {
    async fn read(session: &str) -> Result<Vec<VerdictRecord>> {
        crate::tool::goal::verdict_log::read(session).await
    }
    async fn append(session: &str, record: &VerdictRecord) -> Result<()> {
        crate::tool::goal::verdict_log::append(session, record).await
    }
    async fn set_status_if_current(session: &str, goal: &Goal, status: GoalStatus) -> Result<bool> {
        runtime::set_status_if_current(session, goal, status).await
    }
}
