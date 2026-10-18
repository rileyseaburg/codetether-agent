//! User-controlled goal lifecycle transitions.
use crate::Host;

use anyhow::{Result, anyhow};
use codetether_goal_core::GoalEditAction;

pub(super) async fn set<H: Host>(session_id: &str, value: &str) -> Result<String> {
    let status = match value {
        "active" => GoalEditAction::Resume,
        "paused" => GoalEditAction::Pause,
        "complete" => GoalEditAction::ForceComplete,
        _ => return Err(anyhow!("unsupported goal status: {value}")),
    };
    super::write::change::<H>(session_id, status, None, None).await
}
