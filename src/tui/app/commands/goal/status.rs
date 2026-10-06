//! User-controlled goal lifecycle transitions.

use crate::session::tasks::GoalEditAction;
use anyhow::{Result, anyhow};

pub(super) async fn set(session_id: &str, value: &str) -> Result<String> {
    let status = match value {
        "active" => GoalEditAction::Resume,
        "paused" => GoalEditAction::Pause,
        "complete" => GoalEditAction::ForceComplete,
        _ => return Err(anyhow!("unsupported goal status: {value}")),
    };
    super::write::change(session_id, status, None, None).await
}
