//! Notification-driven wait for a closing child turn to settle.

use super::super::{collaboration_runtime::execution_notify, execution_state};
use std::time::Duration;

const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(5);

pub(super) async fn until_idle(name: &str) -> bool {
    until_idle_with_timeout(name, SHUTDOWN_TIMEOUT).await
}

async fn until_idle_with_timeout(name: &str, timeout: Duration) -> bool {
    tokio::time::timeout(timeout, wait(name)).await.is_ok()
}

async fn wait(name: &str) {
    let notify = execution_notify::notification(name);
    while execution_state::is_running(name) {
        let notified = notify.notified();
        tokio::pin!(notified);
        notified.as_mut().enable();
        if execution_state::is_running(name) {
            notified.await;
        }
    }
}

#[cfg(test)]
#[path = "wait_tests.rs"]
mod tests;
