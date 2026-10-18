//! Explicit user budget changes, independent of usage already incurred.
use crate::Host;

use anyhow::{Result, anyhow};
use codetether_goal_core::GoalEditAction;

pub(super) async fn run<H: Host>(session_id: &str, value: &str) -> Result<String> {
    let cap = match value {
        "none" | "off" | "unlimited" => None,
        _ => Some(
            value
                .parse::<i64>()
                .map_err(|_| anyhow!("usage: /goal budget <positive tokens|none>"))?,
        ),
    };
    super::write::change::<H>(session_id, GoalEditAction::Edit, None, Some(cap)).await
}
