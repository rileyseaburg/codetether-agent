//! Explicit user budget changes, independent of usage already incurred.

use crate::session::tasks::GoalEditAction;
use anyhow::{Result, anyhow};

pub(super) async fn run(session_id: &str, value: &str) -> Result<String> {
    let cap = match value {
        "none" | "off" | "unlimited" => None,
        _ => Some(
            value
                .parse::<i64>()
                .map_err(|_| anyhow!("usage: /goal budget <positive tokens|none>"))?,
        ),
    };
    super::write::change(session_id, GoalEditAction::Edit, None, Some(cap)).await
}
