//! Status parser for the `session_task` tool.

use crate::session::tasks::SessionTaskStatus;
use anyhow::{Result, anyhow};

pub(super) fn parse_status(s: &str) -> Result<SessionTaskStatus> {
    Ok(match s {
        "pending" => SessionTaskStatus::Pending,
        "in_progress" | "inprogress" => SessionTaskStatus::InProgress,
        "done" => SessionTaskStatus::Done,
        "blocked" => SessionTaskStatus::Blocked,
        "cancelled" | "canceled" => SessionTaskStatus::Cancelled,
        other => return Err(anyhow!("unknown status: {other}")),
    })
}
