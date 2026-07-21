//! Bounded status polling for non-blocking A2A peer tasks.

use crate::a2a::client::A2AClient;
use crate::a2a::types::{SendMessageResponse, TaskState};
use anyhow::{Context, Result, anyhow};
use std::time::{Duration, Instant};

#[path = "poll_activity.rs"]
mod activity;

const POLL_INTERVAL: Duration = Duration::from_millis(500);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
const TURN_TIMEOUT: Duration = Duration::from_secs(120);

pub(super) async fn complete(
    name: &str,
    owner: Option<&str>,
    client: &A2AClient,
    initial: SendMessageResponse,
) -> Result<SendMessageResponse> {
    let SendMessageResponse::Task(mut task) = initial else {
        return Ok(initial);
    };
    let started = Instant::now();
    activity::record(name, owner, &task);
    while !finished(task.status.state) {
        if started.elapsed() >= TURN_TIMEOUT {
            return Err(anyhow!(
                "LAN peer {name} did not finish within {TURN_TIMEOUT:?}"
            ));
        }
        tokio::time::sleep(POLL_INTERVAL).await;
        task = tokio::time::timeout(REQUEST_TIMEOUT, client.get_task(&task.id, Some(0)))
            .await
            .map_err(|_| anyhow!("LAN peer {name} status request timed out"))?
            .with_context(|| format!("LAN peer {name} status request failed"))?;
        activity::record(name, owner, &task);
    }
    Ok(SendMessageResponse::Task(task))
}

fn finished(state: TaskState) -> bool {
    match state {
        TaskState::Submitted | TaskState::Working => false,
        TaskState::Completed
        | TaskState::Failed
        | TaskState::Cancelled
        | TaskState::InputRequired
        | TaskState::Rejected
        | TaskState::AuthRequired => true,
    }
}

#[cfg(test)]
#[path = "poll_tests.rs"]
mod tests;
