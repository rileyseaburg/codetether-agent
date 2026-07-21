//! Dispatch between V2 mailbox waits and hidden V1 target compatibility.

use super::Args;
use crate::tool::ToolResult;
use anyhow::{Result, anyhow};

#[path = "legacy.rs"]
mod legacy;
#[path = "outcome.rs"]
mod outcome;
#[path = "subscriptions.rs"]
mod subscriptions;
#[path = "targets.rs"]
mod targets;
#[path = "timeout.rs"]
mod timeout;

pub(super) async fn execute(args: Args) -> Result<ToolResult> {
    let owner = args
        .context
        .session_id
        .as_deref()
        .ok_or_else(|| anyhow!("session id not available"))?;
    let duration = timeout::duration(args.timeout_ms)?;
    let deadline = tokio::time::Instant::now() + duration;
    let requested = args.requested_targets();
    if !requested.is_empty() {
        return legacy::execute(owner, &requested, deadline).await;
    }
    let activity =
        crate::tool::agent::collaboration_runtime::parent_activity::until(owner, deadline).await;
    Ok(outcome::result(activity))
}
