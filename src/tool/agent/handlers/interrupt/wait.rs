//! Notification-driven wait for an aborted child turn to release its session.

use super::super::super::{collaboration_runtime::execution_notify, execution_state};
use std::time::Duration;

const INTERRUPT_TIMEOUT: Duration = Duration::from_secs(5);

pub(super) async fn until_idle(agent_id: &str) -> bool {
    until_idle_with_timeout(agent_id, INTERRUPT_TIMEOUT).await
}

async fn until_idle_with_timeout(agent_id: &str, timeout: Duration) -> bool {
    tokio::time::timeout(timeout, wait(agent_id)).await.is_ok()
}

async fn wait(agent_id: &str) {
    let notify = execution_notify::notification(agent_id);
    while execution_state::is_running(agent_id) {
        let settled = notify.notified();
        tokio::pin!(settled);
        settled.as_mut().enable();
        if execution_state::is_running(agent_id) {
            settled.await;
        }
    }
}

#[cfg(test)]
#[path = "wait_tests.rs"]
mod tests;
