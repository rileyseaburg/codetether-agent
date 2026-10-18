//! `/goal edit` persistence.
use crate::Host;

use anyhow::{Result, anyhow};
use codetether_goal_core::GoalEditAction;

pub(super) async fn run<H: Host>(session_id: &str, objective: &str) -> Result<String> {
    if objective.is_empty() {
        return Err(anyhow!("usage: /goal edit <objective>"));
    }
    super::change::<H>(
        session_id,
        GoalEditAction::Edit,
        Some(objective.into()),
        None,
    )
    .await
}
