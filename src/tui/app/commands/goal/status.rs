//! User-controlled goal lifecycle transitions.

use crate::session::tasks::GoalStatus;
use anyhow::{Result, anyhow};

pub(super) async fn set(session_id: &str, value: &str) -> Result<String> {
    let status = match value {
        "active" => GoalStatus::Active,
        "paused" => GoalStatus::Paused,
        "complete" => GoalStatus::Complete,
        _ => return Err(anyhow!("unsupported goal status: {value}")),
    };
    if !crate::session::tasks::runtime::set_status(session_id, status).await? {
        return Err(anyhow!("no goal exists"));
    }
    Ok(format!("Goal status: {}", status.as_str()))
}
