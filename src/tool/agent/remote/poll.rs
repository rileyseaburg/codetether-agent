//! Bounded status polling for non-blocking A2A peer tasks.
//!
//! The deadline is an *idle* budget (see `poll_progress`): a peer that is
//! visibly working is never cut off; a hung one fails within [`IDLE_TIMEOUT`].

use crate::a2a::client::A2AClient;
use crate::a2a::types::SendMessageResponse;
use anyhow::{Context, Result, anyhow};
use std::time::Duration;

#[path = "poll_activity.rs"]
mod activity;
#[path = "poll_finished.rs"]
mod finished;
#[path = "poll_progress.rs"]
mod progress;

const POLL_INTERVAL: Duration = Duration::from_millis(500);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
const IDLE_TIMEOUT: Duration = Duration::from_secs(120);

pub(super) async fn complete(
    name: &str,
    owner: Option<&str>,
    client: &A2AClient,
    initial: SendMessageResponse,
) -> Result<SendMessageResponse> {
    let SendMessageResponse::Task(mut task) = initial else {
        return Ok(initial);
    };
    let mut idle = progress::IdleWatch::start(&task, IDLE_TIMEOUT);
    activity::record(name, owner, &task);
    while !finished::finished(task.status.state) {
        tokio::time::sleep(POLL_INTERVAL).await;
        task = tokio::time::timeout(REQUEST_TIMEOUT, client.get_task(&task.id, Some(0)))
            .await
            .map_err(|_| anyhow!("LAN peer {name} status request timed out"))?
            .with_context(|| format!("LAN peer {name} status request failed"))?;
        activity::record(name, owner, &task);
        if idle.observe(&task) {
            return Err(anyhow!(
                "LAN peer {name} made no progress for {IDLE_TIMEOUT:?}"
            ));
        }
    }
    Ok(SendMessageResponse::Task(task))
}

#[cfg(test)]
#[path = "poll_tests.rs"]
mod tests;
